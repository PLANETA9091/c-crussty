# ROUND-436 BOTTLENECK (tick-436, 06:43+08, холодное окно; золотой слот = тик 02:08)

## Входный контекст
master e05994c (код 8141548: композит mobsoa⊕inside +27.3% pair-cert). NO MERGE ×4 честно.
Канон v2 (×435): golden 02:2x-03:3x = сертификация; полуздоровое = инфо-ноги + якоря; холодное (05:4x-07:2x+) = инфо-ноги + циклы. Внезолотные пары ≤+9.1 (×435 подтверждено).

## ЛИНИИ (после ×434-435)
- **chunkpl (8cd2028/b64b189, cmp434_chunkpl) — сильнейшая живая**: +22.1/+17.7/+21.9 norm (×2 тика no-pair — зазор 6.65-6.71M против якорей 6.76M+); C углубляет (cmp435_chunk3) + нужен pair-completion в golden: якоря целённо 6.60-6.75M.
- **inside2-fixed (92cd1e5, cmp432_inside2)**: +8.2..+13.7 norm вне golden (≈cce); golden покажет истинную линию (эра-пик +34.2). B: cmp435_inside3 углубление ВРЕДНОЕ ×2 (−16/−17.6) → пере-рисёрч на базе 92cd1e5.
- **wgen3 (062baea, cmp434_wgen3 PALETTED-DEMUX)**: негатив/нуль ×2 окна — финальная попытка в golden (тик 02:08), иначе закрытие вектора, слот → новая подсистема закона 6 (despawn+spawn сканы / entity-query слой).

## ТОП-ЛЕЙНЫ (носитель leg 432-ins-l1r3: items 0.00 / nav_ai 3.78 / broadphase 10.21 / inside_volatile 15.91)
- inside_volatile 15.9% — внутри2-плоскость (golden-измерение завтра)
- chunk-parse/wgen (закон 8): chunkpl ноги в инфо-батче + C-цикл

## ИНФО-БАТЧ-436 (×8, 06:5x +08): якоря ×4 @master (w3..z3, coverage вкл. зазор-зоны) + chunkpl-ноги ×2 (chk-r5/r6 @b64b189 cmp434_chunkpl) + inside2-ноги ×2 (ins-r5/r6 @92cd1e5 cmp432_inside2). Инфо-линия, сертификацию не форсировать (канон v2).

## Сабагенты (волна ≤3)
- A: wgen3 golden-prep (ре-бенч готов к 02:08) + research запасного kernel-leaf
- B: пере-рисёрч лейна на базе 92cd1e5 (serve-путь getBlockStateFinal / readPalette / снап-мисс-рейт), имплементация, бенч
- C: chunk3 углубление (b64b189 база) + бенч; pair-completion план для golden
