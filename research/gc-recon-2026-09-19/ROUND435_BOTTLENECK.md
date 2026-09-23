# ROUND-435 BOTTLENECK (tick-435, 05:43 +08)

## Входный контекст
master 536cf06 (код 8141548 = d282985: композит mobsoa⊕inside +27.3% pair-cert ×432).
×434: мега-композиция composite⊕wgen НУЛИФИЦИРОВАНА (юнион-пары +6.4..+16.9 ≈ cce +13.6 то же окно) — ре-композиции этих осей не тратить ноги. wgen +27.3 pair-cert 1/3 на старой базе (мержить некуда без потери). Honest NO MERGE ×3.
CLAIMS TASK-435 п.1 = ПЕРВАЯ ЗАДАЧА: golden-window сертификация ЖИВЫХ линий, батч СРАЗУ на старте тика.

## КАНДИДАТЫ ТИКА (все ветки пережили тик-434)
1. **inside2-fixed @round-434-b-inside4 @92cd1e5 (lever cmp432_inside2) — ГЛАВНЫЙ КАНДИДАТ ЛЕСЕНКИ**: B root-caused NCDFE round-3 ($Lane never-defined в kernel-loader + indy clinit + BRIDGE_READY ordering; KernelLoaderSim A/B канон), фикс 4-нога, ноги NCDFE=0 ×3 ВПЕРВЫЕ. Внутренняя плоскость (stage-1c + inside2) впервые ПОЛНОСТЬЮ живая. Эра-пик внутренней линии +34.2 (431b-l1) — золотое окно покажет истинную линию.
2. ccefix re-cert @513c4835 (cmp430_inside): stage-1c alive, полуденные пары +13.6 max, золотые norm +16.5..+23.6.
3. wgen3 @round-434-a-wgen3 @062baea (cmp434_wgen3): PALETTED-DEMUX (kernel-leaf demux) + wgen set; батч ×434 рухнул в холодное окно (свой якорь −18.2) — ре-бенч.
4. chunkpl @round-434-c-chunkpl (cmp434_chunkpl): biomes-parse cache; chk-3 +21.9 norm high-зона no-pair — ноги агента C.

## ТОП-ЛЕЙНЫ (носитель композита, leg 432-ins-l1r3: items 0.00 / nav_ai 3.78 / broadphase 10.21 / inside_volatile 15.91)
- inside_volatile 15.9% — адресуется ПОЛНОСТЬЮ живой inside2-плоскостью (главный рычаг лесенки)
- wgen/GEN-ось: юнион с композитом ≈ 0 → wgen живёт отдельной веткой (закон 8: игрокам-видимая ось — chunkpl/wgen ноги в батче)

## БАТЧ-435 (интерлив ×15, 05:4x +08): якоря ×7 @master (p3..v3) + inside2-ноги ×4 @92cd1e5 (b-ins-r1..r4)
+ cce-ноги ×2 @513c4835 (l1r8/l2r7) + wgen3-ноги ×2 @062baea (w3-1/w3-2). Депресс-гейт + band + pair-by-runner
Δ≤50k + min-of-3. Мерж при ≥+20% pair min-of-3 (лесенка: +27.3 → inside2-полная плоскость).

## Сабагенты (волна ≤3, рестарт с контекстом: worktrees удалены для диска, RESULT-*.json сохранены в ROUND-434/)
- TASK-435-A: wgen3 ре-бенч + цикл закона 3 (PALETTED-DEMUX, round-434-a-wgen3)
- TASK-435-B: inside2 углубление ПОВЕРХ 92cd1e5 (свой цикл, golden-window ноги)
- TASK-435-C: chunkpl pair-hunt (high-зона якоря 8.4-8.9M ИЛИ re-роллы ядро 6.7-7.0M)

## chunk/worldgen ось (закон 8)
wgen3-ноги + chunkpl-ноги в батче = GEN/chunk-parse ось измеряется каждый тик; A/C циклы продолжают ось.
