# RESEARCH-C-416 — FLUID-BULK (вектор TASK-416-C: fluid-апдейт слой целиком → Rust-крейт)

## STEP0 — посимвольный профиль fluid-лейна (до имплементации; стиль RESEARCH-B-k3)

Источник: round-mc1 ABSORB (fluid 13.76% после meganav-эры; vanilla-лейн 16.0-16.7% в
каждом ABSORB) + mc1 BOTTLENECKS_3 top-40 leaf + javap transcript
research/fluid-dirty-2026-09-18/step0_updateFluidHeightAndDoFluidPushing.txt.

Fluid-семья (lane_map mc1 = 13.76% CPU, ~84k сэмплов) раскладывается по листьям:

| лист | доля от CPU | доля от лейна | механизм |
|---|---|---|---|
| PalettedContainer.get | 3.0% | ~22% | SimpleBitStorage + VarHandle volatile-цепи при чтении BlockState ячейки |
| Entity.updateFluidHeightAndDoFluidPushing (self) | 2.4% | ~17% | собственное тело свипа: bounds-математика AABB.deflate, Mth.floor/ceil, циклы |
| SimpleBitStorage.get | 1.2% | ~9% | unpack ячейки (то же чтение, ниже get) |
| VarHandle ...ReadOnly.getVolatile | 1.1% | ~8% | volatile-прочтения palette/storage (часть — fluid-путь) |
| Mth.floor | 1.0% | ~7% | floor/ceil координат бокса на каждый вызов |
| LevelChunk.getFluidState | 0.8% | ~6% | чанк-путь до секции |
| FluidState.getType | 0.5% | ~4% | is(tag) проверка |
| ConcurrentLong2ReferenceChainedHashTable.getNode | ~1.1% | ~8% | hasChunksAt/touchingUnloadedChunk concurrent-map чтения |

КЛЮЧЕВОЙ ФАКТ (recon C-415, подтверждён mc1): 150k сущностей ПЕРЕЧИТЫВАЮТ ОДНИ И ТЕ ЖЕ
ячейки 10-100×/тик (items кучкуются в 0.5 блока; каждая сущность кучи свипает тот же
набор ячеек). При fluid_guard=1 хит-путь гварда всё равно делает ПОЛНОЕ пере-прочтение
бокса (cellsUnchanged: chunk fetch + states.get на каждую ячейку) — dedup ТОЛЬКО на
уровне «box = чистый», не на уровне ячейки.

## Анти-уроки в дизайне
- bl2 (секционный bulk-JNI, RED −9пп): НЕ делать отдельный gather-обход мира.
  Решение: LAZY LUT — первая сущность, коснувшаяся ячейки, собирает её для всей кучи
  (probe-dedup), отдельного прохода НЕТ. Zero gather-cost.
- bleg1 (секционный): отклонён.
- fluid_dirty-мемо / fluid_bitmask: НЕ используются (закон 5). Инвалидация = GLOBAL
  GEN-bump (RECON-43 ec880c2 write-bump контракт — счётчик поколений, не мемо, не битмап):
  единственный retarget-сайт LevelChunk.setBlockState → FluidBulkOps.secWrite бампает GEN
  при реальной смене FluidState (ref-compare синглтонов).

## Архитектура (закон 6: подсистема целиком, ОДИН bulk-JNI/тик)
- Entity.updateFluidHeightAndDoFluidPushing → whole-body bridge (asm replace_body, G-BODY
  shape fluid_guard/TASK-80) → FluidBulkOps.updateFluidHeightAndDoFluidPushing.
- Data-plane: пер-потоковый LUT (key[]+tag[]+val[]), валидность слота = (key, tickGen-stamp);
  fetch — бит-в-байт ванильное чтение (flat-section body гварда, javap-proven).
- Control-plane: RUST (src/fluid_bulk.rs) — natives fluidProbe/fluidBulk: ОДИН вызов/тик
  (epoch handshake, census-drain, policy, fail-closed disarm). Per-entity JNI отсутствует
  по построению.
- LevelChunk.setBlockState: 1 сайт invokevirtual LevelChunkSection.setBlockState →
  invokestatic FluidBulkOps.secWrite (ретаргет = клей миграции подсистемы, закон 6).
- Ванильность (закон 4): пустой флаг = ни один модуль не регистрируется (bit-in-byte);
  slowVanilla = посимвольная копия FluidPushGuardHook.slow (javap transcript); wet-математика
  (getHeight/getFlow, float-арифметика глубины, normalize/scale/0.003/0.0045 tail) не тронута.

## Отличия от FluidBulkOps.java C-415 (исправления до имплементации)
1. DEADLOCK-BUG C-415: `if (!ENABLED || !rustOk) return slowVanilla` делал maybeEpoch()
   недостижимым (rustOk ставился только внутри maybeEpoch) — LUT не вставал НИКОГДА.
   Фикс: отдельная ветка `if (!rustOk) { maybeEpoch(); ... }`.
2. Unbounded retry: sweepRetry→sweep→GEN-bump→sweepRetry — рекурсия. Фикс: ровно 1 ретрай,
   дальше slowVanilla (бит-в-байт).
3. Per-tick Arrays.fill(65536) ref-array на каждом (tick,GEN)-rollover = 65536 write-barrier
   записей/тик/поток (card-table мусор, анти-урок bl2 «скрытые за-тик costs»). Фикс:
   per-slot tag[] (упакованный tickGen), rollover = смена штампа, БЕЗ bulk-clear.
4. lever = cmp416_fluid (STRICT-eq), маркеры/метки cmp416_fluid.

## Метрики успеха
- ARM-пруф: "cmp416_fluid: ARMED fluid-bulk" + "EFFECT rust-bulk epoch ok" в server-stdout.log.
- ЛЕЙН: fluid 16.0-16.7 → ≤8 (цель), TPS pair-by-runner не хуже якорей-416.
- grep AIOOBE=0, band 6.0-9.5M, items 0.00 (носитель жив), RAM/GC без регресса.
