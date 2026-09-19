# RECON-16 — absorb лега s7176 (REGION-STEAL lever #13 v1): СЕРВЕР-КРЭШ mid-soak, руут-кауз + план v2

Лег: run 35452002378 (world-bench-3), head fd91c51, dispatched 23:30:08 +08 (TASK-333), conclusion **failure**.
Верификация dispatch-коммита 99b741b = worklog-only (diff fd91c51..99b741b = worklog.md, без кода) — ре-диспатч того же кода воспроизведёт крэш.

## Хронология (job 105920676485 / server-stdout.log)

- 15:40:19 SEEN_DONE; forceload sweep r640; 15:41:30 `POPULATION INJECT DONE target=150000 injected=150000` (FIXTURE-VALIDITY: VALID, 45.9s)
- 15:41:35 профайлер-фаза (cpu-окно), TPS-поллы в цикле: 15:42:40 → `TPS: 1.7, 1.6, 2.0, 2.0` (cцена жива, TPS-класс ожидаемый)
- 15:42:45 batch_collector telemetry tick=600 (workers=4) — STEAL-путь работает
- 15:43:05 **`Entity threw exception`** на воркере → NPE → **`Encountered an unexpected exception`** на Server thread → crash-report → `Stopping server` 15:43:05, JVM exit 15:43:23 (uptime 197s)
- 15:43:59 harness: `FATAL: server process died mid-soak`; все asprof-стопы «Process 3561 not found» → collapsed НЕ собраны (cpu/wall/alloc = 0 сэмплов); BENCH-4 fixture gate 1b FAIL (polls=1, delta=n/a) — **каскад смерти сервера, не самостоятельный дефект**

## РУТ-КАУЗ (стектрейсы точные)

Слой 1 — воркер-тред (RegionTickOps.lambda$ensureHelpers$3 → **stealChunks:324** → ServerLevel.lambda$tick$4 → guardEntityTick → tickNonPassenger → Zombie.tick → travel → move → checkFallDamage → **FarmBlock.fallOn → turnToDirt → Level.setBlockAndUpdate → notifyAndUpdatePhysics → ServerLevel.sendBlockUpdated:1883**):

```
java.lang.NullPointerException: Cannot invoke "it.unimi.dsi.fastutil.objects.ObjectArrayList.get(int)"
  because "this.wrapped" is null
  at it.unimi.dsi.fastutil.objects.ObjectOpenHashSet$SetIterator.next(ObjectOpenHashSet.java:575)
  at net.minecraft.server.level.ServerLevel.sendBlockUpdated(ServerLevel.java:1883)
```

Слой 2 — Server thread: `ReportedException: Exception while updating neighbours` (MinecraftServer.tickChildren:1839) `Caused by: java.lang.NullPointerException` — та же точка.

Механика: в ваниле/Paper `ServerLevel.sendBlockUpdated` выполняется ИСКЛЮЧИТЕЛЬНО на Server thread; внутри — итерация fastutil `ObjectOpenHashSet`, чей итератор при попадании в «wrapped»-слот требует ненулевой `wrapped`-список (фастутил-контракт добавления во время итерации). STEAL v1 тикает сущности на воркерах → воркер + главный итерируют/мутируют один и тот же set конкурентно → сломанный индекс итератора → NPE. Это RACE на разделяемой структуре, НЕ отсутствие инициализации: главный упал той же точкой после воркер-инцидента.

Доказательство «не флейк»: 1-й инцидент на ~65-й секунде соака при 150k живых сущностей (уtopленник на ферме — бытовой блок-апдейт), graceful shutdown, GC до смерти здоров (152 паузы / 9872ms STW), память 5.6/6.1GB — НЕ OOM.

## Вердикт lever #13 v1

**CRASH-REFUTED (мех. дефект канализации блок-апдейтов из воркера)** — PG-S2 не достигается (краш-фри = FAIL). DONE-park/TPS-эффекты не измеряемы (профили 0 сэмплов). По v7 лейн НЕ закрыт: DONE-park 13.4% MSPT-фазы 3 остаётся валидной целью (RECON-15 методика 121/901 воспроизведена в absorb_s7176.py на базе s7169).

## План v2 (след. тик, коммит-пригодный)

1. javap точки 1883 (JDK в CI; локально javap отсутствует — jre-headless only): идентифицировать поле-Set (Purpur sendBlockUpdated) и его инвариант владельца-треда.
2. Канализация: воркер НЕ зовёт sendBlockUpdated напрямую — routed batch-очередь на владельца чанка/главного (тот же класс структуры, что batch_collector), drain в фазе главного; либо per-worker init контекста, если поле per-thread.
3. Гварды парити: lockstep-харнесс RegionLocklockstepHarness + сценарий «FarmBlock.fallOn на воркере» (воспроизведение 15:43:05); cargo-тесты на classfile-редиректы; honest A/B min-of-2 лег s7177 (region_steal=2 = STEAL v2 с канализацией).
4. Rollback-состояние: region_steal=0 в дефолте dispatch (v3 банк); STEAL не банкингован.
