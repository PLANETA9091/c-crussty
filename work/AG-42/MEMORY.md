# AG-42 MEMORY (волна-526, ≤15 уроков)
1. API-only саб (0 ворктри/0 gc/0 локальных коммитов) — диск-футпринт ноль, Д1-Д5 чисты.
2. Клетки брать по живому contents-GET + regex-boundary по ПОЛНОЙ истории (хвост протухает за минуты).
3. fp-ось @2171d6da (sim32/1d/9000s/dcp900) — canon press-коhорта; зазор 88-96 был пуст → fp92 (0-клейм).
4. rt-ось WBP @e49e8984 (dp3v2/pop150k/seed42/band 5.5-13.5M) — зазор 16-20 пуст → rt18.
5. Сиды: 526042 grep-чист; seed42-когорта WBP для сран-вимости с rt-кривой (AG-243/226/262).
6. POST /git/refs FULL 40-sha; 422 → GET-ref и сравнение sha; dispatch-ы разносить ≥31s (AG-338-мина).
7. yml-blob assert перед dispatch: b4e9e05b (fp+sim входы живы), 7c021f41 (WBP) — 422-drift ловится заранее.
8. SameFileError при mirror в собственный каталог RD — skip src==dst; mkdir зеркал до copy.
9. board_append идемпотентен ("already-appended") — падение finalize после append не дублирует строки.
10. Пивоты держать наготове: fp100/fp76/fp132; rt20/rt9/rt28 — гонка клеток <3 мин (AG-274).
11. Бюджет саба ~20 мин: CLAIM→2 POST→FACT/DISP/PS уложились в ~12 мин, 0 wasted POST.
12. Терминалов моих ног ждать харвестом следующих волн (run 36990096741/36990150816).
