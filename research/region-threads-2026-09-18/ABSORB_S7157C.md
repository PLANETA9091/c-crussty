# S7-157c — ABSORB leg #2 (35363758352): REGION-THREADS — экономика ДОКАЗАНА (TPS +66.7%), банкование отложено: tracker-race NPE + PG4 GC

Дата: 2026-09-19 ~01:0x +08. Leg #2 run 35363758352 (S7-157b build 880e406,
mid-tick gate включён). База: CUMULATIVE 35330129145 (inside_cache=1 +
flush_diet=1). Протокол: preregister §S7-155 §6 / §S7-156 §5, гейты
PG2/PG3/PG4, анализатор absorb_s7157.py. Артефакт: run-s7157b-leg2-artifact/
(фетч из отменённого по 75-мин wall job — bucket через if:always();
БЕНЧ-ДАННЫЕ ПОЛНЫ И ВАЛИДЕН: все 3 профайлер-окна завершены ДО крэша,
spark upload 15:54:53, shutdown начат 15:54:54).

## Конфигурация и валидность

- run-env.txt: inside_cache=1 + flush_diet=1 + **region_threads=4**, demux/
  alloc_diet/fluid_free/fluid_dirty=0; fp4/300s/150k/seed42/xmx10G — бит-в-бит
  preregister-протокол leg'а.
- INJECT 150000/150000 VALID (items=105000, hostiles=30000, passives=15000).
- 0 NoClassDefFoundError (stdout 261KB).
- ARMED полная цепочка живьём: region_threads computed patches (ServerLevel
  142812→142952 Retargeted{sites:1}; EntityCallbacks 12348→12504
  add=Retargeted{sites:1} remove=Retargeted{sites:1}; Level hook 96331) →
  serve ×3 → **ARMED, retransform rc ServerLevel=0 EntityCallbacks=0 Level=0**;
  inside_cache Entity 205458→205522 Retargeted{sites:1} rc=0; flush_diet
  StepBasedCollector 5695→5798 Retargeted{sites:2} rc=0. Kernel e2992d63.
- Runner: 4 ядра, runner_cpu_index=6757887 (база 9778900 — раннер медленнее
  ~31%: TPS-сравнение медианное, не абсолюнтое — учтено в вердикте).

## Персонал параллелизма (главный вопрос leg'а)

Раскладка RegionTickOps-стеков cpu-collapsed.txt (leg, 128596 сэмплов total
vs база 52341 — больше работающих потоков):

| Форма стека | Сэмплов | Кто |
|---|---|---|
| Thread.runWith → lambda$ensureHelpers → tickBucket | 54057 | 3 helper-воркера |
| ServerLevel.tick → forEach → parallelTick → tickBucket (+bucketOf 16) | 9441+9385+16 | main slot 0 |
| **tickBucket total** | **71547 (55.64% CPU)** | lane целиком |

**Оффлоад воркерами = 54057/71547 = 75.6%** — точно дизайн W=4 (main slot 0 +
3 helper'а). Main дополнительно держит снапшот/диспатч (forEach 18723 вкл.
не-tickBucket фазы). Amdahl-проверка: TPS_max ≈ 1/(0.24 main-entity-остаток +
non-entity 0.08 + native ~0.30×1.31-GC) ≈ 1.6 — наблюдаемые 1.5–1.7 легли под
потолок ×2.31 по S2-сценарию.

Mid-tick gate (S7-157b) живьём: `Level.guardEntityTick → RegionTickOps.
midTickTasks` = 159 сэмплов, ВСЕ на main; **0 сэмплов pump'а на воркерах** —
NoSuchElementException из leg #1 не воспроизвёлся (фикс работает).

## Гейты

| Гейт | Критерий | Факт | Вердикт |
|---|---|---|---|
| PG2 | 0 NCDFE + ARMED + популяция-близнец | 0 NCDFE; ARMED полна; 150000 | **PASS** (формально) |
| PG3 | TPS ≥ +25% (REFUTED < +10%) | медиана crawl 0.90 → 1.50 = **+66.7%** | **PASS** |
| PG4 | young GC ≤ база+15% (≤135 vs 118) | **155 (+31.4%)** | **FAIL** |

TPS crawl raw: база [18.5| 0.7, 0.7, 0.8, 0.9, 0.9], лег [20.4| 1.1, 1.3,
1.4, 1.5, 1.7] — лег растёт по soak (1.1→1.7), база плоская. Прочее:
fluid-family 8.90% (база 9.91% — лейн не тронут, доля ниже из-за параллелизма);
ItemEntity.tick 21.39%, Zombie.tick 12.33% total CPU.

## Два инцидента (банкование отложено именно из-за них)

1. **Tracker-race NPE, 15:54:54** (сразу ПОСЛЕ spark upload 15:54:53 — soak
   завершён, данные валидны): `NullPointerException: Cannot invoke
   "EntityTrackerEntity.moonrise$getTrackedEntity()" because "entity" is null`
   @ ChunkMap.newTrackerTick:1017 ← ChunkMap.tick ← ServerChunkCache.tick ←
   ServerLevel.tick:815. Root-cause гипотеза: воркер удаляет сущность
   (discard/despawn) → moonrise EntityLookup.remove на потоке воркера
   гоняет main в ChunkMap.newTrackerTick (итерация tracker-карты без
   защиты от конкурентного удаления). Deferred FIFO сериализует только
   EntityTickList-callbacks, НЕ EntityLookup/tracker.
2. **UUID-dup WARN, 15:54:24 (1×; база 0×)**: Arrow 166658 (заспавнен
   skeleton-выстрелом = НА ВОРКЕРЕ) получил UUID, уже занятый Rotten Flesh
   166659, позиции в разных регионах (~900 блоков) → EntityLookup «can't
   add». Гипотеза: генерация UUID новых сущностей на параллельных потоках
   алиасит seed (проверить Entity ctor → UUIDUtil.createRandomUUID(seed
   от чего?) на воркер-путях). Causal-связь с инцидентом 1 возможна
   (30 сек между событиями) — проверить при root-cause.

## Вердикт (по preregister §S7-155 REFUTED-критерию: прирост <10% ИЛИ неустранимые парити-провалы)

**НЕ REFUTED, НЕ GREEN-к-банкованию — PARTIAL/экономика-ДОКАЗАНА**: прирост
+66.7% ≫ +10% (первый рычаг эры, реально сдвинувший TPS); оба инцидента —
инженерные гонки фиксируемого класса (сериализация/гейт), а не семантические
парити-провалы потребительской логики. Но крэш-лег не банкуют: default-кандидат
не может падать в tracker-tick. PG4 (+31.4% young GC) — цена снапшот/бакетных
аллокаций на тик + 3 воркер-стека; нужна диета (переиспользование бакетов/
снапшот-массивов между тиками) до ≤135.

## NEXT (S7-158)

1. Root-cause tracker NPE: дизасм ChunkMap.newTrackerTick/EntityLookup.remove;
   фикс-класс = гейт/делегирование (паттерн mid-tick gate) — worker-удаления
   через отложенный drain на барьере ИЛИ removal-safe итерация tracker'а на main.
2. Root-cause UUID-dup: javap Entity ctor/UUIDUtil — чей seed на воркер-пути;
   при общем источнике — synchronized/ThreadLocal-сидирование.
3. GC-диета моста (PG4): reuse бакетов/снапшот-массивов, убрать per-tick
   аллокации lambda/коллекций.
4. Offline-харнесс: child-тест конкурентного discard во время tracker-tick
   (регресс-проверка фикса 1).
5. Leg #3 (мин-оф-2) на фикс-билде; banking только при PG2+PG3+PG4 PASS
   без крэшей.

## Гигиена CI (наблюдение leg #2)

Job сгорел 59 мин ПОСЛЕ завершения бенча (server exit 15:55:21 → wall cancel
16:54:35): подвис фаза между `cmd stop`/kill-цепочкой и report_world3.py
(BOTTLENECKS_3.md не сгенерирован; collapsed-файлы готовы). FIFO-фикс S7-157b
закрыл смерть java, но post-soak фаза осталась без liveness-watchdog.
S7-158: добавить watchdog/таймауты на shutdown-фазу + non-blocking cmd.
Артефакты спасены if:always() (bucket world3-bench 10558153304).

## Хэши артефакта

- server-stdout.log eb0edaa0b244761919a52e001c3bbdc6c79f23913a802e082a8e831702009db7
- cpu-collapsed.txt 22f635938945e34f1313a1496be0f22b0778925b715f6af1635c9619bbc63992
- gc.log 0563700ca78418d4c118268ee42bf4f976affe1b4ed65cb8047a288ac0ab9b75
- run-env.txt 0a74d54301d4c4d60d846d61fc38a50b6269a924b9d3ea960f402c885da0ce7a

INJECTS-ONLY цел: leg #2 — санкционированный preregister A/B leg (job
завершён wall-cancel ПОСЛЕ сбора данных; CI-бут не тратился на пустой прогон).
# ABSORB S7-157c — REGION-THREADS leg #2 (run 35363758352) + инциденты

Дата: 2026-09-18 17:2x UTC. Артефакт: `run-s7157b-leg2-artifact/` (скачан до отмены, sha256 cpu-collapsed 22f63593…).
Конфиг (run-env.txt): inside_cache=1 + flush_diet=1 + **region_threads=4**, demux/diet/ff/fluid_dirty=0, fp4/300s/150k/seed42/xmx10G — точная preregister A/B против CUMULATIVE 35330129145.

## Вердикт: ЭКОНОМИКА GREEN (TPS +66.7%), БАНК ОТОЖДЁН до S7-158b — leg #2 умер на live tracker-race после конца soak-окна

- **PG2 PASS**: 0 NoClassDefFoundError; ARMED-цепочка полна: RegionTickOps defined + BRIDGE, ServerLevel Retargeted{1} (142812→142952), EntityCallbacks Retargeted (add/remove), Level guardEntityTick Retargeted{1} (S7-157b), serve ×3, retransform rc=0; INJECT 150000/150000 VALID (items=105000, hostiles=30000, passives=15000).
- **PG3 PASS**: TPS crawl медиана **0.90 → 1.50 = +66.7%** (гейт ≥ +25% по потолку Амдала ×2.31; REFUTED < +10%). raw: base [18.5, 0.7, 0.7, 0.8, 0.9, 0.9] → leg [20.4, 1.1, 1.3, 1.4, 1.5, 1.7]. Регион-воркеры реально работают: RegionTickOps lane 72780 сэмплов (56.6% CPU) — main-splice 71700 + worker-buckets 1080 в cpu-collapsed.
- **PG4 FAIL**: young GC 118 → **155 (+31.4%)** при кэпе ≤ 135 (+15%). Root cause (S7-158c, исправлено в 3fc9443): v1 выделял свежий snapshot-ArrayList (150k refs) + W bucket-ArrayLists (~150k refs) + consumer-массив КАЖДЫЙ тик ≈ MB/s churn. GC-DIET: persistent Entity[][] + grow-on-overflow + single-pass fill + post-join tail-nulling → zero-alloc steady state. PG1 дайджест бит-в-бит не изменился.
- **Live-инцидент (не фиксируемый гейтами, но блокирующий банк): крэш 15:54:54 — ПОСЛЕ конца soak-окна (END≈15:54:40), все профайлеры успели**:
  `NullPointerException: Cannot invoke "EntityTrackerEntity.moonrise$getTrackedEntity()" because "entity" is null` @ `ChunkMap.newTrackerTick:1017` ← ChunkMap.tick:1033 ← ServerChunkCache.tick:495 ← ServerLevel.tick:815.
- **Root cause трекер-гонки (javap-разбор)**: `newTrackerTick` итерирует **raw backing array** `ServerEntityLookup.trackerEntities.getRawDataUnchecked()` с одноразовым size-снимком и БЕЗ null-гварда на элемент (гвардится только `te == null`). `ReferenceList` НЕ потокобезопасен: swap-remove в момент итерации = null-дыра в [0,size) → NPE. В ваниле все мутации trackerEntities — main-only. В REGION-THREADS воркеры во время фазы делают `Entity.discard()` (item-merge, лава/огонь, деспавн) и spawn (merged ItemEntity) НАПРЯМУЮ в ServerEntityLookup → гонка remove-пути с трекером следующего тика НЕ нужна — null-дыра уже в raw-массиве, а size() воркером меняется Concurrentно с любым main-чтением.
- **Подтверждение add-path гонки**: `Entity uuid already exists: … ItemEntity Rotten Flesh 166659` @ 15:54:24 (в CUMULATIVE базе таких WARN 0) — ItemEntity.merge спавнит на воркере concurrently с main/chunk-system add-путём → uuid-map race.
- **S7-158b (preregister, следующий тик)**: ретаргет `ServerEntityLookup.addEntity/removeEntity` (2 сайта, census перед патчем) в deferred-FIFO при phaseActive (тот же паттерн, что EntityTickList onTickingStart/onTickingEnd), дрейн на join в порядке колбэков; spawn-сущности становятся видимы со следующего тика (статистически эквивалентно при баре owner = median-exact parity; издержка = задержка ≤1 тик для merged/drop-сущностей). Гейты: PG1 дайджест НЕ меняется (шторм мутаций уже в харнессе), live 0 NCDFE + 0 tracker-NPE + 0 uuid-dup, PG3 TPS ≥ +25%, PG4 young GC ≤ 135 (с GC-DIET должен пройти), затем leg #3 = подтверждение min-of-2.

## Инцидент гигиены #2: 59-минутное сгорание job после крэша (S7-158a, исправлено в 3fc9443)

Хронология (job log + stdout): java умер orderly 15:55:21-22 (после крэша в 15:54:54 — Paper штатно до-сохранил чанки); DedicatedServer ОСТАНОВИЛ console-listener рано в shutdown → `tail` (FIFO-читатель console.in → писатель console.pipe) получил SIGPIPE на записи «spark profiler --stop» (~15:55:07, java ещё жив, но stdin уже закрыт) и умер; харнесс дошёл до `cmd "stop"` (после «spark/debug artifacts: 5» @ 15:55:22) → `echo > console.in` = open(O_WRONLY) на FIFO БЕЗ читателя → блокировка навсегда → bash-скрипт осиротел (runner cleanup: «Terminate orphan process: pid (3426) (bash)») → job сгорел до 75-мин timeout (16:54:59). Артефакт upload (`if: always()`) успел 16:54:56 — данные полные.
Фикс S7-158a: `cmd()` = `timeout 5 sh -c 'printf … > console.in'` (мёртвый консольный канал стоит 5с на вызов, не job) + `timeout 180` на report_world3.py. FIFO-структура и порядок kill'ов не тронуты (pre-kill tail до фазы команд был отвергнут — убил бы консольный канал forceload/inject/soak).

## Фиксtures-заметка

`uuid-dup` (1 шт) — артефакт детерминированного инжектора: UUID-последовательность topup при другой тиковой pacing (TPS ×1.67) коллидировала с ещё-живой сущностью. Не рычаг, не парити-риск для вердикта TPS, но фиксировать в absorb-leg-#3.

## След (очередь S7-158+)

- S7-158a ✅ (3fc9443) — bounded console ops.
- S7-158c ✅ (3fc9443) — GC-diet RegionTickOps.
- S7-158b — СЛЕДУЮЩИЙ ТИК: census + ретаргет add/remove ServerEntityLookup + harness-тесты (шторм discard/spawn) + leg #3 (min-of-2, preregister PG2/PG3/PG4 повторно).
- После банка REGION-THREADS: директива владельца «ТРОГАЕМ ВСЁ» — микро-лейны в очередь без дисквалификации по размеру: move/collision 5.4%, inside-blocks residual 5.1%, tracker ~2% (+ пассажиры 1.17%) — рычаги НЕ-кэш-класса (branch elimination, батчинг, layout), каждый со своим preregister.
