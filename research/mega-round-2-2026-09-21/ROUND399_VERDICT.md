# TASK-399 VERDICT (тик 10:08→13:5x +08, cron 401462, 2026-09-21)

МЕГА-РАУНД-3: 10 векторов на БАЗЕ round-398-j-subsys2 (композит J+X, флаги cmp399_*),
все ноги world-bench-parallel.yml банк v4 (fp=4, 150k, seed42, 300s, band 6.0-9.5M),
ARM-пруф обязателен, вердикты vs v4-якорь (2.6@8551924 / 2.2@6653417).

## КАРТА ВЕКТОРОВ (композит J+X)

| Вектор | Флаг | Ноги (Δ normalized) | ARM | Вердикт |
|---|---|---|---|---|
| **B shardgrid** (шардированный grid-индекс 64 шардов + seqlock-риды) | cmp399_shard | **+8.7% / +9.8%** (runs 35556027094/35557216438) | ✓✓ | **РЕПЛИЦИРОВАН, медиана +9.25% — ПОБЕДИТЕЛЬ, но <10% бара → карта векторов** |
| **F despawn2** (rust lifetime-minheap + батч-деспавн) | cmp399_despawn2 | **+6.0% / +6.0%** (35555915599/35557224931) | ✓✓ | РЕПЛИЦИРОВАН +6.0% — карта векторов |
| A batchjni (батч-JNI транспорт idx_batch_apply) | cmp399_batch | −7.1% (35556385953) | ✓ | RED — батч-протокол дороже per-item |
| E rustpre (rust-side AABB pre-filter f64 bit-match) | cmp399_rustpre | −0.0% (35555829366) | ✓ | PARITY — фильтр не конвертировал |
| J2 foot2 (footprint-диет пост-J остатка, 15 sites) | cmp399_foot2 | −4.4% (35555490360) | ✓ | RED |
| C navstagger (N-фазный AI scheduler) | cmp399_navstag | «+12.6%» — **patch failed → НЕ-ARMED = ФЕЙК** (35556455097) | ✗ | INVALID (урок ARM-пруф раунда-2 повторился бит-в-бит: не-armed нога дала фейковые +12%) |
| G devirt (девиртуализация SynchedEntityData-цепочек) | cmp399_devirt | −2.4% + **ReportedException** Colliding entity with block в checkInsideBlocks (35556464768) | ✓ | CRASH-REFUTED (фикс следующий тик: byte-hook ломает checkInsideBlocks lambda) |
| D sensebatch (батч-сенсорика getEntities) | cmp399_sensebatch | BAND-DISCARD ×2 (10.09M/10.08M) — не измерен | — | INFRA-BAND (бюджет ре-роллов исчерпан) |
| I wakeup (event-driven AI wakeup-list) | cmp399_wakeup | leg1: boot-NPE scriptEngine (root-caused, фикс 4140600 — force-load удалён); leg2: boot ЧИСТЫЙ (фикс работает) но BENCH-4 fixture-gate INVALID (35557524097) | ✗ | INFRA-FIXTURE — root-cause следующий тик |
| H offpath (off-thread pathfinding) | cmp399_offpath | агент не начал (infra-дедлайн Task-вызовов) | — | DROP |

## РЕШЕНИЕ ФАЗЫ-3: МЕРЖА НЕТ
B = победитель (реплицирован, parity, RAM/GC норма: 21.0s/Full=9 и 18.7s/Full=7,
items 0.00% ×2 — J-композиция жива) НО медиана +9.25% < двойного бара ≥+10%
(обе оси) → по правилу фиксируется как карта векторов, амбиция следующего
раунда ↑. Урок TASK-398 (items_oss «+11.3%» оказался ≈паритетом) подтверждает
строгость бара: при якорном шуме ±14пп мерж +9.25% = риск фантома.

## ГЛАВНОЕ МЕТОДОЛОГИЧЕСКОЕ ОТКРЫТИЕ: ЯКОРЬ САМ ШУМИТ ±14пп
Ре-анкер банк-v5 (master dceb7f9 + items_oss) — 6 ног ОДНОГО sha через
alias-refs (master/round-399-anchorb/round-399-anchord): Δ = **−7.1% .. +14.1%**
(ноги: +0.0/+12.7/−0.9/+14.1/+14.1/−7.1; runs 35550244357, 35552405859,
35553864920, 35556255770, 35556264955, 35556274059). Линия TPS_exp от 2
старых точек НЕ воспроизводит банк-TPS на новом раннере: дрейф + GC-состояние
(Full=7..10) + раннер-вариация. ВЫВОД для TASK-400: вердикты только (а)
min-of-N ≥3 ног И (б) pair-by-runner (S7-96d) ИЛИ (в) новый якорь
same-day same-runner A/B. Старая практика «одна нога → вердикт» умерла
окончательно (3 репликации TASK-398 + 6 якорных ног этого тика).

## ИНФРА/КОД-УРОКИ ТИКА (§5)
1. 10 параллельных Task-вызовов → ВСЕ легли на adapter-timeout (тяжелее
   раунда-2), НО агенты доделали пайплайны (5 веток запушены, RESEARCH ×10,
   worktrees ×10) — верхний агент дотянул c/g/i (cargo-fix+пуш+диспатч) и
   D (include_bytes-литералы + **Code attribute_length = u4, был u2 — битый
   класс** + cp_count scope) — собирать артефакты, не рестартить.
2. force-load классов через чужой loader ДО инициализации миров =
   Entity.<clinit> в неверном контексте → getEngineByName("rhino")=null →
   scriptEngine null → PurpurWorldConfig NPE на boot (I leg1). Фикс: passive
   sighting poll (прецедент brainhook). force-load только после boot-marker.
3. BAND-DISCARD ×5 за тик (ночной пул бимодальный: 4.9M/10.0-11.9M вне
   6.0-9.5M) — паринг-закон работает, но съедает бюджет ре-роллов.
4. Диск: research-артефакты (unz-дубли, world3-bench.zip, flamegraphs) +
   /tmp-логи = 8.8G/9.9G — чистка −4G. Абсорб-директории держать lean.

## NEXT (TASK-400)
1. **Парный A/B вердикт B-shardgrid**: same-runner min-of-3 (или парный
   банк-vs-B на одном раннере) → если ≥+10% стабильно → мерж (первый
   настоящий композит в master: J-подсистема + шардированный grid).
2. C-fix (navstagger patch failed: сверить LDCW-сайты с J-патчами
   RegionTickOps — конфликт цепочки classfile) → ре-диспатч.
3. G-fix (checkInsideBlocks crash — byte-hook vs lambda$checkInsideBlocks$2),
   I-fix (fixture-gate: wakeup gate() vs population churn-check), D re-run.
4. Композиция B+F (шардgrid + despawn2, ортогональны: read-scaling vs
   lifetime-heap) — кандидат на ≥+12% при парном методе.
5. Якорная методика: новый 3-точечный якорь same-day + pair-law в absorb.
