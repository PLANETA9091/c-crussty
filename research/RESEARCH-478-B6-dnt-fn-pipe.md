# RESEARCH 478-B6 — DnT-бенч 19c: census function-пайплайна (datapack-плоскость) на свежем master

Тик ×478, cmd z-478-B6, 2026-09-28. Вход: master-линия 386887a8 (92443914 → 5cf2e172, обе пост-МЕРЖ-№16; HEAD уплыл на 8f9dec37/63289ab3 конкуренцией — docs-only).

## Метод
- Ценз ALL-CPU cpu-collapsed in-place zipfile (закон Л-478-A1.2d) ×3 fresh **ваниль-**canon-ноги:
  - Y4-36369037286 @5cf2e172 (116,374 сэмплов)
  - B5a2-36367092915 @92443914 (115,335)
  - B5a3-36367098236 @92443914 (117,147)
  - Σ **348,656** сэмплов, мир afb3a0b3, окна пост-warmup.
- Скрипт: /home/z/rounds/ROUND-478/B6/b6_fn_pipe_census.py (вывод census_out.txt).
- Контракт-якоря: constant-pool скан классов из patched-kernel.jar того же рана (javap в песочнице нет; CP-скан эквивалентен по якорям).
- Kernel-ценз: rg по src/*.rs, cplug-sdk/src, cplug-abi/src.

## Числа (ALL-CPU, %)
| лейн | Y4 | a2 | a3 |
|---|---|---|---|
| FN-QUEUE (ServerFunctionManager) | 0.0000 | 0.0000 | 0.0000 |
| CommandFunction/CallFunction/functions-executor | 0.0000 | 0.0000 | 0.0000 |
| CommandDispatcher.execute/parse (brigadier-deep) | 0.0000 | 0.0000 | 0.0000 |
| pack/datapack-load | 0.0000 | 0.0000 | 0.0000 |
| jigsaw/template (без broad StructureManager) | 0.0000 | 0.0000 | 0.0000 |
| jigsaw/template∪StructureManager (upper bound) | 0.0009 | 0.0009 | 0.0017 |
| Commands-union (net/minecraft/commands∪brigadier∪ExecutionContext∪CommandQueueEntry∪runCommandQueue∪ExecuteCommand) | 0.0361 | 0.0477 | 0.0307 |
| ↳ из них paper EntityCommand.listEntities | 95.2% | 96.4% | 94.4% |
| spark-инструмент (отдельная плоскость) | 0.0756 | 0.0832 | 0.0657 |
| тик-анкер (tickChildren) | 28.05 | 27.81 | 27.60 |

## Контракт (fresh master jar, 1.21.10 mojang-mapped)
1. `net/minecraft/server/ServerFunctionManager`: TICK_FUNCTION_TAG + executeTagFunctions + сигнатура (InstantiatedFunction, CommandSourceStack, ExecutionContext) — вся per-tick function-плоскость = тег #minecraft:tick; **0 сэмплов**.
2. `MinecraftServer`: поле ServerFunctionManager + tickChildren — вайринг есть, трафика нет.
3. `CallFunction.execute` → ExecutionContext/CommandQueueEntry/Frame — per-dispatch очередь (консистент C23, глоб-шедулера нет).
4. `ServerFunctionLibrary` reload/apply = PreparableReloadListener — загрузка только boot/reload-фаза (в окне бенча 0).
5. `CommandFunction.instantiate` присутствует (макро-путь жив семантически, нагрузка = 0).
6. Kernel: dp-хуков 0 (rg datapack|brigadier|jigsaw|function-manager в src/*.rs + cplug-sdk/src → только JNI-функциональные указатели; C22-консистент: crussty dp-ops 0).

## Capture-матем (метод C22: потолок = capture/тик-анкер × 100)
- Строгий ваниль-датапак-путь (ServerFunctionManager ∪ CommandFunction ∪ CallFunction ∪ fn-executor ∪ brigadier-deep ∪ jigsaw/template ∪ pack-load): **0/348,656 = 0.0000%** → потолок 100%-kill **+0.000пп конструктивно** (upper-bound с broad StructureManager: ≤+0.006пп).
- Kill всего Commands-union: +0.129 / +0.171 / +0.111пп (мед **+0.129пп**) — дефицит **×15.5 к гейту +2пп**, **×155 к бару +20пп**.
- Даже синтетический тик-потолок оси (C41-475 ×100) +0.8..1.2пп < бара — реальный ваниль-путь = 0.

## Вердикт
**REFUTED_CENS dp-плоскости** (capture 0.0000% ≪ 1пп порога гипотезы → диспатча нет, placebo-запрет закон 5). Свежий ценз ×3 ужесточает C22 (0/117,035) до **0/348,656** и замещает stale-якорь C40 (sched-lanes 0.17-0.80пп) → Commands-union 0.031-0.048% с 94-96% harness-атрибуцией.

## NEXT (отдельный пункт, НЕ dp)
Harness-приборка canon-стенда: commands-union 0.031-0.048% ALL-CPU = 94-96% paper EntityCommand.listEntities (полл инжектора), + spark-инструмент 0.066-0.083% отдельной плоскостью. Кандидат: фиксация как константа прибора / вынос за окно проф-цензуры (не семантика, прибыль ≤+0.13пп потолок — численно легален как REFUTED_CENS-потолок, не как merge-кандидат).

## Reopen
dp-мир (DnT/Terralith/BACAP), прошедший POP-гейт (сейчас 4/4 FAILURE, инъектор-стилл) И имеющий #minecraft:tick ≥100 fn/тик ИЛИ Commands-lane ≥1% ALL-CPU.
