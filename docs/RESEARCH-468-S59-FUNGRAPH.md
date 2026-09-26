# RESEARCH-468-S59 — 19c scheduler/command-graph профиль: javap-контракты function-пайплайна ванили

R468-S59, 2026-09-26. Полный док: /home/z/rounds/ROUND-468/S59.md (+ /home/z/rounds/ROUND-468/s59_art/ — javap-дампы 622,837 B). Источник: /home/z/tools/patched-kernel.jar (Paper/Purpur 1.21.10 kernel), javap JDK-21.

## Контракты (числа)
1. Tick-позиция: `MinecraftServer.tickChildren` @offset 110-123 — profiler push(`"commandFunctions"`) -> `ServerFunctionManager.tick()` -> popPush(`"levels"`); порядок: CraftScheduler -> FoliaGlobalRegionScheduler -> EntityScheduler -> ClickCallback -> **commandFunctions** -> levels. Main-thread, до энтити/чанк-тика.
2. Гейт: только `TickRateManager.runsNormally()`; корни = `List.copyOf` кэш TICK_FUNCTION_TAG (postReload), 0 re-lookup/тик.
3. Fix-цена корня/тик >=5 alloc: `getGameLoopSender()` (createCommandSourceStack+withPermission(2)+withSuppressedOutput) + ExecutionContext (new ArrayDeque + fastutil ObjectArrayList) через ThreadLocal CURRENT_EXECUTION_CONTEXT (nested -> reuse); commandLimit=max(1,RULE_MAX_COMMAND_CHAIN_LENGTH=65536), forkLimit=RULE_MAX_COMMAND_FORK_COUNT=65536.
4. Дренаж: `ExecutionContext.runCommandQueue` pollFirst-цикл; MAX_QUEUE_DEPTH капа 10,000,000; `incrementCost` = quota--.
5. Per-command: `ExecuteCommand.execute` = profiler-lambda + `ContextChain.runExecutable` (Brigadier); parse один раз на load (`CommandFunction.fromLines` -> parseCommand/line); bind-lambda на каждое исполнение.
6. Per nested call: `CallFunction.execute` = new Frame + ContinuationTask + per-entry CommandQueueEntry + bind-lambda = 4+N allocs.
7. Макро: `MacroFunction.instantiate` LRU Object2ObjectLinkedOpenHashMap (load 0.25), **MAX_CACHE_ENTRIES=8** (bipush @169); >8 комбинаций = substituteAndParse re-parse/call.
8. Reload (MC-267491 /reload×5): `ServerFunctionLibrary.reload` CompletableFuture; BACAP 170,675 строк × 5 = **853,375 parse** — load-lane.
9. Census: 42 .class / 371,061 B (MinecraftServer 141,065; Commands 43,136; CommandSourceStack 30,845; ServerFunctionLibrary 16,570; MacroFunction 10,186; ServerFunctionManager 9,207; ExecutionContext 9,141).

## Профили стресс-ранов
- CI stress-v1 ×3 ценза + локальный репро c93-run: level.dat-wall NbtFormatException 5.5-6.6s + 594s limbo -> **CI-профиль датапак-тика не существует до v2-мира (S75/S76/S71)**; все 2600+ ранов дня — MineShield-3, 0 datapack-маркеров (job-log run 2635: 0 hits).
- synth-tick200 (C93): 200 every-tick × 1 cmd; G-D1 >=3.0% lt_drain; BACAP 5230 fn/170,675 строк/tick.json -> 1 root.
- 19a-протокол (S59/S67 R467): wall=Σmax(N·sleep, C·1.26ms); 0.4s->354.3 c/s; COMMAND_SLEEP=0.05->687-748 c/s >= BAR 708 (run 36246612235); zero-sleep потолок 794 c/s.

## Топ-ботлнек
1. Per-command Brigadier runExecutable + scoreboard-семантика (BACAP-класс, тысячи cmd/тик) — venue=стресс-v2. 2. Макро-LRU 8 -> кандидат #19c-M (8->512, venue-only). 3. Reload-lane 853k parse. 4. Оболочка НЕ ботлнек (5 alloc/корень/тик = наносекунды).

## Capture-матем
- Банк v5: lane(commandFunctions)≈0% CPU (0 tick-функций; fp4 без команд) -> 0%×X = **0пп < 20пп** — REFUTED_CENS банк-веню лейн-класса function-pipeline.
- Стресс-веню: G-D1 маркер 3% × 100% capture ≈ **+3пп** < 20пп -> верикт-платформа 19c = стресс-v2-пары.

## Диспатч (закон 16 prereg)
Ветки round-468-s59a/b/c = ref-push origin/master c1196321a7413c23c271d73026a362bb7c9b8b2b, 0 код-дельт, бан-EXACT вектор: run **36253822654** / **36253832361** / **36253840110** (same-minute burst 15:59Z). Гипотеза: в-точки банка; E-окно chkclimb-5 [6427199,6527199] <=-1.60 = нога МЕРЖ №11; 3-й..5-й ваниль-сэмплы вопроса базы МЕРЖ №10 (-13.32 @6.92M / -13.61 @6.97M vs флор-шум z=-1.83σ).

## ИНФРА-урок
`dispatch_467_wave.py::ref_push` молча no-op'ит НОВЫЕ ветки (api() глотает HTTP 404 -> PATCH-в-никуда «успешен» -> холостой таймаут). Фикс: /home/z/rounds/ROUND-468/s59_art/dispatch_s59.py (api_raw, явные статусы, verify sha).
