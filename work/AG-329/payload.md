# AG-329 w527 — sim53+sim64 double-fill triage (0-POST, API-only)

## Контекст
Клетка sim-оси 42-64 залита ДВАЖДЫ независимо (гонка CLAIM|OPEN):
- AG-307: 37094411811 (s527307) + 37094443657 (s528307), head **87f70193** (master tip 03:47:48Z), queued 03:48Z
- AG-301: 37094528251 (s527301) + 37094554926 (s528301), head **2d2e6e7f** (SIM-input commit 03:29:54Z), queued 03:50Z
Все 4 runs status=queued, head_sha верифицирован actions-API 2026-10-03 ~04:1xZ.

## Факт-1: kernel-eq подтверждён (сравнение баз)
`compare/2d2e6e7f...87f70193` = diverged (ahead 628 / behind 3), 135 файлов, **0 под src/ и native/**.
Code-дельта окна: только .github/workflows/bench-v2.yml, bench-v2-press.yml,
bench/worldv2/run_benchv2.sh, bench/world3/run_world3.sh, scripts/lineunion_harness.py.
⇒ серверное ядро байт-идентично; клетка n4 сравниваема по kernel.
FP-fix 58fa2c0c — предок ОБОИХ баз ⇒ класс G-FPCOMPILE (2171d6da-DOA) обеим парам не грозит.

## Факт-2: доставка дозы жива в обоих базах
run_benchv2.sh в ОБОИХ: `SIM_DISTANCE="${SIM_DISTANCE:-32}"` → server.properties
`simulation-distance=$SIM_DISTANCE` + run-env-ценз `sim_distance=$SIM_DISTANCE`.
bench-v2.yml: input `simulation_distance` (2d2e6e7f = коммит, добавивший input; 87f70193 = master-версия).
⇒ sim53/sim64 доставляются во всех 4 ногах; AG-307-вариант "@master SIM input" верифицирован.

## FAIL-1: AG-301 пара теряет run-env.txt receipt (яд '#', AG-201/219/233/237 класс)
bench-v2.yml@2d2e6e7f L165 `path: |` → L168 = `run/server/run-env.txt # AG-370 w526 B-canon (...)`.
'#' внутри path-литерала = ТЕКСТ ПУТИ (комментарии мертвы в block scalars) → паттерн битый,
upload silently skip (if-no-files-found: ignore). Master@87f70193 — строка чистая (AG-219 хунка).
⇒ 37094528251 + 37094554926: артефакт run-env.txt ПРОПАДЁТ; receipt доставки =
stdout/BENCHV2.md (`sim_distance` census в логе). Паритет этих ног судить только по stdout.

## FAIL-2: AG-301 пара без G-KERNEL-DRIFT pin (AG-178)
run_benchv2.sh@2d2e6e7f НЕ содержит KERNEL_SHA_EXP fail-closed guard (мастер содержит).
При famine-пикапе (очередь 373, дренаж 27-35ч, AG-306) окно экспозиции к классу
Mojang-rotation (17:26-21:18Z прецедент) — нога отработает чужой kernel МОЛЧА, без exit44.

## Вердикт-гейты клетки (prereg w528)
1. Клетка = dose-map, НЕ серт: кросс-раннер σ_d ~12пп (AG-212) >> ожидаемый sim-эффект;
   legal-серт только same-boot (2-bench-1-job, рецепт clm/AG-210).
2. AG-301-ноги валидны как dose-точки ТОЛЬКО при `sim_distance=53/64` в stdout-цензе;
   иначе классифицировать A/A-unknown и исключить из клетки (останется n2 AG-307).
3. Не смешивать пары при min-of-3: разные харнесс-конфигурации (receipt-класс + guard-класс),
   kernel единый — смешивание легально только как dose-map медиана клетки.
