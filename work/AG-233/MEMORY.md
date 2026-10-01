# AG-233 MEMORY — волна-523 (харвест #16f-вериф-ног + G4-фикс-диспатч)

## Что сделано
- **Харвест 10 приоритет-ног ×523 API-форензикой** (все 9 completed = FAILURE, run-36869031041 AG-132 ещё in-flight на 14:2xZ):
  - **run-36869746650 (AG-120, window-256+single-dim) = СКРЫТЫЙ GREEN**: marked **20449/20449=100%**, ch/s **730.32**, G-DIM **PASS** (21609≥19426, n_dims=1), NCDFE=0, AIOOBE=0 — FAIL=1 дал ТОЛЬКО G4 (expect 58279 = ×3-хардкод). Единственная нога волны с полным pregen.
  - **run-36869466433 (AG-114, bounded-inflight-2048, 3-dim)**: 25626/61347 (41.8%) @21.4 ch/s за 1200s, смерть ТОЛЬКО drain-бюджет (нужно ~2900s) — жив, но в 34× медленнее window-256 staged.
  - **run-36869235123 (AG-115, drain2700) / run-36869768680 (AG-118, drain3000)**: DRAIN сработал на +28s (mspt-idle false-trigger при неготовом gen — «#16h»: триггер дрена не требует marked-прогресса) → marked=0/3837 на гейте.
  - **run-36868762311 (AG-146)**: чистый GEN-STALL 25/61347 весь ран, DRAIN-STALL-детектор (600s no-progress) работает — паттерн валиден.
  - **run-36869509148 (AG-106, staged≤256, 3-dim)**: заморозка 25/0/4 — staged-окно 256 НЕ лечит 3-dim fanout.
  - **run-36871302011 (canary-7 leg-2, master @ab113ff2 warn-дефолт, idx 8.16M)**: RED = #16f-столл на master (marked=0, G-DIM 29/58279, DRAIN +689s false-PASS) → canary-7 остаётся заблокирован до мёржа #16f-чемпиона; ΔS=0 9-я волна.
- **ROOT-CAUSE найден и зафикшен**: report_benchv2.py n_dims-парс был МЁРТВ — `re.match(r"dims=...")` якорится на начало строки, а dims= сидит mid-line после radius_blocks/seed/run_seconds/xmx в run-env.txt → n_dims ВСЕГДА 3 → G4 для легального single-dim невозможен (20449 < 58279). Фикс `re.search(r"(?:^|\s)dims=...")`, коммит **b578c543** на ветке **swarm-523-233** (base 7e54d922=AG-120, tree 3240==3240, diff 1 файл +2/−1).
- **Диспатч**: run-**36876480445** bench-v2 ref=swarm-523-233, seed 523233 (уникален, grep 0 совпадений), run_seconds=600, band_gate_action=warn. Прогноз: G4+G5+G-DIM+NCDFE+G3+G-HB = FULL GREEN → первый полный SUCCESS r1136-канона эры.

## Потолки/числа
- window-256 staged single-dim pregen: 730 ch/s (20449 за ≤28s) vs bounded-inflight-2048: 21.4 ch/s — **window-256 доминирует ×34**.
- Early-drain-на-mspt-idle триггер (+28s) — отдельный баг #16h: не фатален для marked (ген доигрывает в sustain), но портит sustain-метрики; фикс = требовать marked-прогресс/GEN-DONE в триггере.

## Следующий шаг ×524
- ХАРВЕСТ run-36876480445: FULL GREEN → мёрж b578c543 как #16g-G4-чемпиона + re-fire canary-7 на master с AG-120-рецептом (bench_dims=minecraft:overworld) → S_BV2 TPS@20k-компонент наконец измерим.
- Дрени-триггер: gate на marked-progress (AG-146 детектор как основа), не mspt-idle.
