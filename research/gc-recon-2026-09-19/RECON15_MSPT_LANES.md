# RECON-15 — MSPT-ось декомпозирована: DONE-park 13.4% + дисбаланс стат-бакетов; lever #13 STEAL реализован+диспатчен в тике — 2026-09-19 ~16:0x UTC (TASK-333)

Инструмент: `recon15_mspt_lanes.py` (deep-first классификация лист→корень, срез корня до последнего `ServerLevel.tick;`; инпуты: чистый банк s7169 + диетный s7174 кросс-чек). Профилировочная база: s7169 = последний ЧИСТЫЙ банк v3 (900 wall-сэмплов Server thread за span 473s; 127k CPU-сэмплов; CPUs: 4 total/4 available, G1 Parallel=4, Concurrent Refinement=4 — из gc,init).

## 1. MSPT-ось (Server thread wall, ранее не раскладывалась)

Семейства (deep-first, чистый s7169): planner-рамка+серия 239 (26.6%), ai/goals/nav 129 (14.3%), travel-collide 98 (10.9%), chunk-access 93 (10.3%), tracking 83 (9.2%), fluid-sim 83 (9.2%), inside 52 (5.8%), прочее ~23 (2.6%).
Server thread НЕ спит между тиками (0 sleep-сэмплов вне тика) — TPS 1.8 = непрерывный catch-up; wall-сэмплы = MSPT-бюджет.

## 2. Ключевая находка: Server thread PARK на DONE-барьере 121/900 = 13.4%

Точный стэк: `ServerLevel.tick → RegionTickOps.forEach → parallelTick → tickBucket(0) → CyclicBarrier.await(DONE) → ForkJoinPool.managedBlock → pthread_cond_wait` (120 сэмплов) + safepoint 31 (3.4%) + VMThread 13.
Арифметика фазы 3: главный (бакет 0) работа 478, каждый воркер 651 (суммарно воркеры 1954), общая работа 2432, критический путь 651 → средний параллелизм 2432/651/4 = 93% эффективности → **НЕ CPU-голод, а ДИСБАЛАНС статических квадрантов** (главный недогружен, воркеры — критический путь; главный park 121 ждёт хвост).
Дополнительно: системные PrioritisedQueueExecutor (чанк-воркеры moonrise) 2402 wall — 65.6% их времени спят; G1-семья = 33% CPU всего процесса (refine 4 воркера + conc mark) — фоновое давление, но арифметика фазы 3 доказывает, что тикеры его почти не ощущают.

## 3. Lever #13 v1 (CRUSSTY_REGION_STEAL) — реализован ДО КОНЦА в тике

STEAL-режим в RegionTickOps: ОДИН общий снапшот-массив в точном ванильном порядке итерации (reuse, zero-alloc steady) + AtomicInteger-курсор чанков (STEAL_CHUNK=512, getAndAdd); главный и воркеры тянут чанки до исчерпания очереди → DONE-park → ~0 (главный заканчивает последним, не первым), критический путь → ~total/4 (608 вместо 651), GC/джиттер воркера поглощается очередью вместо простоя главного. Per-entity логика нетронута (ванильный consumer бит-в-бит); intra-chunk порядок = snapshot; меж-чанковый интерливинг = уже принятый класс паритета region-тика (S7-155, median-exact). Rollback = env (без CRUSSTY_REGION_STEAL статический путь бит-в-бит банку v3).
Оффлайн-гейты: RegionLockstepHarness расширен W=4+STEAL ребёнком — **LOCKSTEP PG1 PASS: W=1 == W=2 == W=4 == W=4+STEAL дайджесты** (60 тиков, 20 deferred-adds/7 mid-tick-removals через реальные ретаргнутые гвард-сайты, no deadlock); cargo 189/189 PASS; release PASS.
Прегистер гейт-лега s7176 (dispatch_s7176.py, S7-108 + remote-head==local-head): PG-S2 доставка (ARMED + region_steal: 1 в run-env), PG-S3a DONE-park ≤ 40/900 (база 121), PG-S3b баланс ≤ 0.20 (база 0.29), PG-S4 TPS median5 ≥ 1.98 (банк 1.80 + 10%), PG-S5 young ≤ 174 / 0 Full, CRASH-FREE. Честная оценка в прегистере: только park-kill даёт ~+4-6%; < 1.98 = REFUTED-by-TPS → rollback (bridges = infrastructure), лейн НЕ закрыт (v7) → RECON-16 (remset/серия-аттрибуция).
