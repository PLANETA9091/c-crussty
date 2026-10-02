# S7-157b — MID-TICK GATE (регионы × Paper mid-tick pump): фикс живого крэша 35353820223

Дата: 2026-09-18 ~23:35 +08. Прецедент: первая живая нога REGION-THREADS
(run 35353820223, leg #1) упала через 40 с тиков:
`ReportedException: Exception ticking world` ← `NoSuchElementException` в
`BlockableEventLoop.pollTask` ← `ServerChunkCache$MainThreadExecutor.pollTask`
← `MinecraftServer.tickMidTickTasks` ← `moonrise$executeMidTickTasks` ←
`ServerLevel.moonrise$midTickTasks` ← **`Level.guardEntityTick:1489`** ←
`ServerLevel.lambda$tick$4` ← `RegionTickOps.tickBucket` (воркер-поток).
Затем job сгорел до 75-мин таймаута (второй корень — отдельно ниже).

## Корень 1: Paper mid-tick pump не потокобезопасен с воркеров

Paper исполняет mid-tick задачи (chunk system queue) ПОСЛЕ КАЖДОГО
entity-тика из `Level.guardEntityTick` → `moonrise$midTickTasks`.
Очередь — main-thread-only; параллельный poll из воркера = гонка
(NoSuchElementException на пустой очереди + в принципе недопустимое
исполнение main-задач не-главным потоком).

**Census (javap, kernel e2992d63) — все вызывающие `executeMidTickTasks`:**
`runAllTasksAtTickStart` ×2, `pollTaskInternal` (MinecraftServer),
`tickFluid`/`tickBlock` (ServerLevel, блок-фаза), `iterateTickingChunksFaster`
(ServerChunkCache), `tickBlockEntities` (Level, invokeinterface,
tileTickPosition-петля) — ВСЕ main-only. Worker-достижим ровно ОДИН сайт:
`guardEntityTick` (invokevirtual `moonrise$midTickTasks()V`, безусловно после
каждого entity-тика, включая catch-путь). Census S7-155/156 пропустил его —
урок: структурный census поверхjavap-методов обязан включать ВСЁ тело
consumer-пути (guardEntityTick — обёртка каждого entity-тика).

## Фикс (S7-157b, MidTickGate)

- `RegionTickOps.midTickTasks(Level)` — публичный мост: worker (ThreadLocal
  `WORKER_FLAG`, ставится в ensureHelpers перед циклом) → return (suppress);
  main → `instanceof ChunkSystemLevel` → точная ванильная виртуальная
  диспетчеризация (наиболее производный override; Level-стаб = return).
  НЕ reimplementация — вызов нетронутого ванильного тела.
- Третий байт-хук (strict): `Level.guardEntityTick`
  `invokevirtual moonrise$midTickTasks()V` → `invokestatic
  RegionTickOps.midTickTasks(Level)` (receiver-prepended 1:1).
  classfile.rs `patch_region_tick_guardentity` + 4 roundtrip-теста
  (Retargeted{1}, Methodref-резолв, idempotent, fail-closed);
  дамп `tests/out/Level.regionthreads.patched.class`.
- region_threads.rs v2: третий таргет (hook → pristine → patch → strict
  sites==1 → retransform ×3, audit_wire "…/midTickTasks", "region_threads v2").
- Живость mid-tick задач сохранена: main качает свой bucket-0 через
  guardEntityTick, плюс `tickMidTickTasks` (каждый тик), плюс
  `tickBlockEntities`-pump. Средний дрейф исполнения chunk-задач —
  статистический домен (тот же, что кросс-регионное отставание ≤1 тик).

## Верификация

- rust suite 144/0/1 (+4 guardentity-теста), release build чист.
- Харнесс OFFLINE PASS: structural (верификатор принимает патченный Level),
  wiring (Methodref в 3 классах), `midTickTasks` static, `isWorker()==false`
  на main, dormant-семантика без изменений, parallel child W=2 + НОВЫЙ
  per-visit флаг-чек: `isWorker()` true ТОЛЬКО на helper-потоках
  (0 нарушений на 200 сущностей).
- PG1 LOCKSTEP PASS на новом мосте: дайджест
  `61e3c3742a0dfd85857f438dbbcdd0be372e436b175c74a7bb19ae7f15e941d5`
  НЕ изменился (W=1 == W=2 == W=4, 60 тиков, 413 финал).
- Урок рефлексии харнесса: параметр `Level` моста резолвится через
  patch-loader (там Level переопределён) — рефлексия обязана брать Class
  объект того же загрузчика; Level ABSTRACT (Unsafe-инвок невозможен,
  реальный pump на Unsafe-ServerLevel NPE — офлайн-инвок main-делегации
  опущен, делегация верна по построению).

## Корень 2 (CI-гигиена): 68-мин сгорание job после крэша

`tail -f console.in | java &` — при смерти JVM `tail -f` остаётся жить
вечно, наследуя stderr шага CI → раннер ждёт закрытия пайпа → 75-мин
cancel. Фиксы run_world3.sh:
- FIFO-паттерн: `mkfifo console.pipe`; `tail -f console.in > pipe &`;
  java `< pipe`; TAIL_PID убивается при shutdown (TERM+KILL, как и JVM).
- Liveness-watchdog во всех трёх ожиданиях (boot / pop-inject / soak):
  `kill -0 $SERVER_PID` → FATAL-лог + ранняя остановка, артефакты
  сохраняются (полный crash-stdout в артефакте).

## Статус ноги

Leg #1 (35353820223) = CANCELLED (крэш до soak; валидного A/B-сэмпла нет,
CI-бут санкционирован). Leg #2 (S7-157b) диспатчен тем же preregister
протоколом (region_threads=4 vs CUMULATIVE 35330129145, гейты PG2/PG3/PG4).
