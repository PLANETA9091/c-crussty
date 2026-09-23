# ABSORB S7-153 — FLUID-DIRTY leg (ран 35341241628) vs CUMULATIVE base (35330129145)

Дата: 2026-09-18, тик 19:43+08. Протокол: preregistered гейты §S7-150 (S7-148-протокол, A/B min-of-2 vs CUMULATIVE-конфиг).

## Конфигурация лега

run 35341241628, head 38afaa3 (пломбинг fluid_dirty), SUCCESS 11:46:00→12:03:29 UTC (~17.5 мин).
inputs: radius=640, seconds=300, fake_players=4, fluid_guard=1, paletted_demux=0, alloc_diet=0,
inside_cache=1, flush_diet=1, fluid_free=0, **fluid_dirty=1**, population 150000/seed42, xmx10G.
run-env.txt подтверждает: inside_cache=1, flush_diet=1, fluid_dirty=1 — конфиг точен, девиаций нет.

## Валидность (G2/G3)

- **G2 PASS**: 0 NoClassDefFoundError (stdout 275KB); ARMED-цепочка полна живьём:
  `fluid_dirty: defined FluidPushOps + $ScanOut in kernel loader` →
  `computed patch LevelChunk (48104→48329, Retargeted{sites:1})` →
  `entity chain composed (Retargeted{sites:2})` → `hook serve` → `armed, retransform rc=0` (обе цели);
  compose с inside_cache на одних Entity-байтах — живьём (inside Retargeted{1} + fluid_dirty Retargeted{2}).
- **G3 PASS**: POPULATION INJECT DONE 150000/150000 (items=105000, hostiles=30000, passives=15000),
  FIXTURE-VALIDITY: VALID.
- Runtime-доказательство обслуживания: cpu-collapsed содержит 1592 сэмпла с кадром
  `FluidPushOps.scan` — ретаргетнутые байты реально работали на горячем пути.

## Вердикт: G1 FAIL ⇒ REFUTED-BY-ECONOMICS (hit-rate ≈ 0% на живой сцене) ⇒ fluid_dirty = 0

| Метрика | base (CUMULATIVE) | leg (FLUID-DIRTY) | Гейт | Итог |
|---|---|---|---|---|
| fluid-family CPU (total CPU, взвешенно) | 5185/52341 = 9.91% | 5691/54711 = 10.40% | ↓≥60% | **FAIL** |
| fluid в ItemEntity.tick (доля лейна) | 29.9% | 33.0% | — | не снят |
| fluid в Zombie.tick (доля лейна) | 7.6% | 7.7% | — | не снят |
| hit-rate моста (по стекам: scan без vanilla ниже) | — | ≈0% (scan 1592 ≈ vanilla 1590) | ≥80% | **FAIL** |
| PalettedContainer.get | 5.20% | 5.32% | — | не снят |
| young GC (gc,start+Pause Young) | 118 | 140 (+18.6%) | не выше базы | **FAIL** |
| TPS crawl (1m-медианы) | 0.7–0.9 | 0.6–0.9 | не хуже min-of-2 −10% | ~паритет |
| 0 NCDFE / INJECT VALID | — | — | обязательные | PASS |

REFUTED-критерий §S7-150 выполнен: **при живом ARMED hit-rate ≥80% недостижим** (фактически 0%) —
кэш не Economically работает на живой сцене. fluid_dirty забанкован off (default 0, как
fluid_free/paletted_demux/alloc_diet до него). Оффлайн-достижения S7-151/S7-152 (lockstep G5 бит-в-бит,
ARMED-механика, 0 NCDFE на живой ноге) остаются в силе — рычаг инженерно корректен, но экономика нулевая.

## Корневая причина (честно, из профиля)

0% хитов означает: **span каждого сканируемого существа меняется каждый тик** — на живой сцене X150K
популяция fluid-скана почти полностью ДВИЖУЩАЯСЯ:

1. Items (105k): topup-цикл (drain → respawn) держит долю свежих падающих предметов; предметы,
   попавшие в воду, плавают и гоняются потоком бесконечно (в воде предмет никогда не покоится).
2. Zombies (30k): AI-блуждание — движение каждый тик по дизайну (этот лейн был не-кэшируемым
   с самого начала).
3. Сценовые мутации воды (fluid-tick/redstone/falling) добавляют инвалидации, но не являются
   главным фактором (для 100% miss нужны движущиеся спаны, не точечные бампы).

Это ТРЕТЬЕ независимое подтверждение одной сигнатуры: FLUID-FREE (S7-148d, «0 хитов в водном мире»)
→ FLUID-DIRTY (настоящая нога) → вывод: **fluid-scan лейн живой сцены X150K позиционно-нестабилен;
любая per-ENTITY/per-SECTION мемоизация позиции-зависимого чистого скана на нём не бьёт**.
Гипотеза S7-150 «масса 100k items покоится» опровергнута живым профилем.

## Чему учит вердикт (для следующих рычагов)

- Лейн fluid-push ~10% CPU закрыт для кэш-архитектур: движение — это vanilla-поведение, его нельзя
  убрать (паритет). Сканирование движущегося сущности обязано выполняться.
- Оставшиеся ранжированные лейны (move/collision 5.4%, inside-blocks residual 5.1%, tracker ~2%)
  — каждый на границе микро-класса; кэш-рычаги там упрутся в ту же позиционную нестабильность.
- Единственный крупный непокрытый фронт — **unclassified 33–39% entity-фазы** (AI/behavior/goal
  selectors и пр.) — требует свежего RECON-2 с раскладкой до классов поведения, прежде чем бить.

## Артефакты

- run dir: research/fluid-dirty-2026-09-18/run-s7153-fluid-dirty/ (gitignored, 28MB)
- Анализатор: research/fluid-dirty-2026-09-18/absorb_s7153.py (санити на базе: воспроизводит
  9.91%/118/0-NCDFE бит-в-бит)
- sha256: artifact_hashes_s7153.txt
