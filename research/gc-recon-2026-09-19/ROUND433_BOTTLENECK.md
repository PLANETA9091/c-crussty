# ROUND-433 BOTTLENECK (tick-433, 02:08-03:5x +08, глубокая ночь)

## Входный контекст
master d282985 = МЁРЖ ×432 (композит mobsoa⊕inside +27.3% pair-stable @53c7879 + GOAL ×115/CLAIMS TASK-433).
Канон TASK-433 п.1: FIRST TASK = root-cause inside_snap collect CCE ×18k + ре-сертификация композита
на новом master (мёрж сбрасывает носитель). Окно: глубокая ночь 01:30-02:3x = здоровый эталон (2.5пп ×432).

## CCE-433 ROOT-CAUSED И ПОЧИНЕН (step-1, ветка round-433-ccefix @513c4835)
- Источник: InsideSnapOps.collect() строка 291 `BlockState[] pal = d.moonrise$getPalette()` —
  метод объявлен `()[Ljava/lang/Object;` (javap), на этой сборке ядро аллоцирует палитру как
  СТЁРТЫЙ Object[] → call-site checkcast к BlockState[] кидал CCE на первой же pending-секции →
  весь stage-1c (снап-плоскость) был МЁРТВ С РОЖДЕНИЯ (fail-closed; +27.3% несёт stage-1b pre-gate + композит).
- Фикс: (Object[])(Object) double-cast (erase-safe) + ELEMENT-level casts ×3 сайта (элементы —
  настоящие BlockState, стирается только тип массива); javap-verified: checkcast "[Ljava/lang/Object;".
- Блобы: build_430b_blobs один javac-проход, flat==nested OK, BLOB-SYNC OK, delta vs HEAD =
  ТОЛЬКО InsideSnapOps (java+flat+nested); INDY WARN InsideBitmaskOps = pre-existing (байт-идентично HEAD).

## ТОП-ЛЕЙНЫ (носитель композита, leg 432-ins-l1r3: items 0.00 / nav_ai 3.78 / broadphase 10.21 / inside_volatile 15.91)
После фикса ожидание: inside_volatile 15.9% адресуется stage-1c (снапы начнут собираться),
serve-путь LevelChunk.getBlockStateFinal 0.498% + readPalette → снап-хит.

## БАТЧ-433 (интерлив ×12, 02:1x +08): якоря ×6 @master + cce-ноги ×4 @513c4835 (cmp430_inside:
l1r4/l1r5/l3r3/l2r4) + wgen-ноги ×2 @898650c (cmp429_wgen l7/l8, закон-8 pair-hunt, norm-линия ×6 медиана ~15.2).
Вердикт двойного гейта + депресс-гейт (якоря norm ≥ −2%) + CCE-гейт новый: grep "collect failed" ≤ десятки
(шторм = регресс); мерж ccefix при ≥+20% pair min-of-3 (композит-линия на новом master).

## Сабагенты (рестарт по анти-застой канону: предшественники 50-90 мин без прогресса)
- TASK-433-A: МЕГА-КОМПОЗИТ master⊕wgen (закон 7) — merge round-429-a-wgen в master, STRICT-OR унион
  флагов, блобы+cargo, диспатч lever cmp429_wgen на композите = измерение wgen ПОВЕРХ +27.3%.
- TASK-433-B: inside2 NCDFE RC7-фикс (lazy init=false) + cherry-pick на базу 513c4835 → диспатч cmp432_inside2.

## chunk/worldgen ось (закон 8)
wgen pair-hunt в батче (l7/l8) + TASK-433-A мост wgen→master = игрокам-видимая GEN-ось поверх композита.
