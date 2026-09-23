# ROUND-434 BOTTLENECK (tick-434, 04:08+ +08, пост-золотое окно)

## Входный контекст
master 8141548 = код d282985 (мерж ×432 композит mobsoa⊕inside +27.3% pair-stable) + учёт ×433.
×433 честный NO MERGE (×2): пары {+10.9, -2.1, 0.0} медиана 0.0 < BAR — бимодальное окно
(золотое 02:2x-03:3x vs смешанное 04:0x). CCE-433 починен @513c4835 (stage-1c ALIVE впервые,
эффект-линия медиана 15.9 → 19-21).

## КАНДИДАТ ТИКА: МЕГА-КОМПОЗИТ round-434-mega @b15f329 (закон 7)
- База = wmaster-юнион 5a85a6b (композит mobsoa⊕inside + wgen STRICT-OR: cmp430_inside + cmp429_wgen live)
- + мерж ccefix 513c4835 (CCE-фикс InsideSnapOps: erase-safe double-cast + element casts ×3)
- Гейты: merge clean (только InsideSnapOps delta) / check_blobs_sync ALL IN SYNC / javap checkcast
  "[Ljava/lang/Object;" + BlockState element casts ПРИСУТСТВУЮТ / cargo check --lib зелёный
- Смысл: полный юнион эры (внутренняя плоскость stage-1c ALIVE + wgen GEN-ось + mobsoa) поверх master.
  wgen-норма ×8: +12.9..+23.0 best-ever; cce-ноги +16.5..+23.6; если складываются хотя бы частично —
  пара ≥+20% достижима даже в смешанном окне.

## ТОП-ЛЕЙНЫ (носитель leg 432-ins-l1r3: items 0.00 / nav_ai 3.78 / broadphase 10.21 / inside_volatile 15.91)
- inside_volatile 15.9% → stage-1c теперь ALIVE (снап-хиты на getBlockStateFinal 0.498% + readPalette)
- wgen/GEN-ось: noise_fill — law-8 игрокам-видимая ось (чанки грузятся медленно — директива владельца)

## chunk/worldgen ось (закон 8) — ОБЯЗАТЕЛЬНАЯ
- Мега-ноги @round-434-mega несут wgen-вектор (cmp429_wgen live) = GEN-ось измеряется в этом тике
- Сабагент A: углубление wgen (noise-подсистема целиком, round-434-a-wgen3)
- Сабагент C: chunk-pipeline R5 (parse/serialization/chunk-send) — новый подсистемный вектор
- B: inside3 NCDFE round-3 root-cause (артефакт run 35902792520 на GitHub)

## БАТЧ-434 (интерлив ×14, 04:1x +08): якоря ×7 @8141548 (lever="") + cce-ноги ×2 @513c4835
(cmp430_inside, ре-роллы сильного семейства l3r4r/l3r5r) + мега-ноги ×4 @b15f329 (cmp429_wgen
= юнион live: m1r1/m1r2/m2r1/m3r1) + wgen-нога ×1 @898650c (cmp429_wgen l9r pair-hunt).
Вердикт двойного гейта + депресс-гейт (якоря norm ≥ −2%) + CCE-гейт (grep "collect failed" ≤ десятки).
Мерж меги при ≥+20% pair min-of-3 (лесенка эры: композит +27.3 → мега = композит+stage-1c+wgen).
