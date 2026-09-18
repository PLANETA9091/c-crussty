# S7-157 — PG1 LOCKSTEP (REGION-THREADS lever #7): PASS

Дата: 2026-09-18 ~22:00 +08. Гейт preregistered в СТАТУС S7-155 §6 (PG1):
«OFFLINE lockstep (region-параллель vs ваниль-последовательность, per-entity
бит-в-бит)». Инструмент: `scripts/run_region_lockstep_harness.sh` →
`entityinside/harness/RegionLockstepHarness.java` (plain JVM, реальный kernel,
NO server boot, INJECTS-ONLY).

## Результат

**PASS — W=1 == W=2 == W=4, дайджест бит-в-бит:**
`61e3c3742a0dfd85857f438dbbcdd0be372e436b175c74a7bb19ae7f15e941d5`

- 60 тиков, 400 базовых сущностей + шторм мутаций, финал 413 (400 − 7 + 20).
- W=1 = dormant-путь моста = `list.forEach(consumer)` — ВАНИЛЬ байт-в-байт
  (эталон); W=2/W=4 — полная параллельная фаза (снапшот → бакеты → GO/DONE →
  drain) в child-процессах с реальным env `CRUSSTY_REGION_THREADS`.
- 20 отложенных добавлений через РЕАЛЬНЫЙ ретаргет `onTickingStart` — во всех
  конфигурациях начинают тикаться СЛЕДУЮЩИМ тиком (совпадает с ванилью:
  javap-доказательство `BaseIterator` — `maxIndex` пиннится на момент
  создания итератора, mid-iteration добавки не посещаются).
- 7 mid-tick самоудалений через РЕАЛЬНЫЙ ретаргет `onTickingEnd` — W=1:
  null-слот без defrag (javap: defrag только при `iteratorCount==0`),
  W≥2: FIFO-drain на барьере; наблюдаемо идентично с тика T+1.
- Без дедлоков (child timeout-сторож 240s), workerError отсутствует.

## Сценарий (детерминизм)

- Per-entity `Random(ordinal*126271+7)` — модель S7-155 (per-tick RNG
  пер-сущностный; общий Level.random в сценарии отсутствует). Ветвящееся
  потребление 1..3 draws/тик — чувствительно к любому перерасходу/недобору.
- 100 партнёрских пар в одном 8-чанковом регионе (взаимные чтения состояния
  — домен PG1 «при одинаковых соседях»: same-bucket порядок = insertion =
  ванильная последовательность).
- Дайджест = SHA-256 канонического вектора по тикам (ordinal-sorted:
  state/draws/tickCount/кардинальность) — иммунен к гонке порядка drain
  между бакетами, чувствителен к любому отклонению per-entity семантики
  (двойной тик, пропуск, перерасход RNG, потеря мутации).

## Значение для живой ноги

Scheduler сохраняет per-entity семантику при W=2/W=4: exactly-once,
порядок внутри бакета = insertion, отложенные мутации = ванильный
snapshot-итератор. Остаточный парити-домен живой сцены — межбакетные чтения
(документированное отставание ≤1 тик, bar владельца = median-exact parity).
Гейты диспатча: PG2 (0 NCDFE/ARMED/популяция 150k), PG3 (TPS ≥ +25% vs
CUMULATIVE 35330129145, REFUTED < +10%), PG4 (young GC ≤ база+15% = 135).
