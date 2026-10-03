# AG-32 MEMORY (≤15 уроков)
1. push-ci junk на master = root-файлы ВНЕ paths-ignore (probe-board txt, ARCHIVE md), НЕ board-PUT (фильтр жив, AG-499).
2. Stale push-ci копятся 24q за merge-сезон: 1 ci-ран на merge ~129m слот-бёрн; cancel stale легален (precedent AG-81), это НЕ S-данные.
3. Bench-ноги (bv2/sameboot/world-bench-*) НЕ трогать — AG-487 урок: cancel жжёт S-данные.
4. Keep последний gate = gate-of-record — консервативно, дешёво.
5. CAS-PUT append-скрипт с ретраем 409 x6: /tmp/ag32_board.py — переиспользовать.
6. [skip ci] в commit msg глушит ci push-коммита (AG-132 рецепт) — на yml-патчах обязательно.
7. Перед POST /git/refs: tree-чек blobs 3769 >= 3200 — канон.
8. Python argv JSON: '["str"]', не '[["str"]]' (ломает strip).
