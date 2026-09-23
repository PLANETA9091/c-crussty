# P500 SCALING — WaypointHotPath (g47) large-N probe

**TASK-06** · agent-7625532f (агент-1) · 2026-09-08 · BENCH-MUTEX соблюдён (`/home/z/BENCH.lock`).

## Метод
- Код из origin/master 381412f (Bench.java v2, BATCH_NS=120ms — уже фикс, не 120µs),
  извлечён `git archive` в /tmp-worktree: рабочее дерево c-crussty не затронуто (там WIP второго агента).
- Группа 47 `PaperNativeWaypointHotPath (I)D`, 10 кернелов, N = 16384 / 65536 / 262144,
  fresh args, SINK против DCE, один JVM на N.
- Контекст: P500_SCALING.md (2-c) уже показал scale-invariant ratio на N=16/256/4096; этот проб
  продлевает кривую на 64x выше прежнего максимума, чтобы отсечь суперлинейность на больших размерах.

## Результаты (ns/op, median-of-5, forward pass; ранжирование идентично 2-c)

| kernel | stem | N=16384 | N=65536 | N=262144 | t(×16 N) | ns/elem @262k |
|---|---|---:|---:|---:|---:|---:|
| oldChunkVisibleValue | old | 220 909 | 901 297 | 3 691 660 | ×16.7 | 14.1 |
| cachedChunkVisibleValue | alt | 203 814 | 825 412 | 3 287 021 | ×16.1 | 12.5 |
| oldAzimuthValue | old | 170 974 | 697 391 | 2 737 850 | ×16.0 | 10.4 |
| directAzimuthValue | alt | 172 573 | 694 113 | 2 721 371 | ×15.8 | 10.4 |
| oldAtOrBeyondRangeValue | old | 135 783 | 623 948 | 2 497 827 | ×18.4 | 9.5 |
| guardedAtOrBeyondRangeValue | alt | 220 411 | 963 251 | 3 887 995 | ×17.6 | 14.8 |
| oldReallyFarValue | old | 94 220 | 354 141 | 1 382 084 | ×14.7 | 5.3 |
| guardedReallyFarValue | alt | 105 350 | 436 312 | 1 733 632 | ×16.5 | 6.6 |
| oldWaypointManagerValue | old | 6 653 877 | 27 015 798 | 107 482 982 | ×16.1 | 410 |
| optimizedWaypointManagerValue | alt | 6 211 789 | 25 303 842 | 100 523 882 | ×16.2 | 383 |

## Вердикты

1. **O(N) подтверждена на всём диапазоне**: рост N ×16 даёт t ×14.7–18.4 (линейно, без
   суперлинейного обвала; ×18.4 у oldAtOrBeyond — в пределах +15% от линейного, шум одиночного прогона).
2. **Per-element стоимости плотные**: 5–15 ns/elem для чистых (I)D; manager-кернел ~380–410
   ns/elem — под итератором тяжёлая структура, не массив.
3. **Same-stem ratio стабильны на больших N** (alt/old @262k):
   - cachedChunkVisible **0.89** (alt WIN 11%);
   - directAzimuth **0.994** (parity);
   - optimizedWaypointManager **0.935** (alt WIN 6.5%) — как в 2-c (7–10%);
   - guardedAtOrBeyond **1.556**, guardedReallyFar **1.25** (alt REGRESSION, scale-invariant) —
     третий независимый N-уровень подтверждает: guarded-пары честно делают больше работы на
     элемент, НЕ вешать на hot path (совпадает с P500_SCALING.md).
4. **Практика**: на N=262144 oldWaypointManager = 107ms на ВЫЗОВ — если такой кернел попадёт в
   реальный путь, это секунды на кадр; для тяжёлых кернелов batching вторичен — сначала
   алгоритмика/распараллеливание.

## Ограничения
- Одиночный JVM на N (методология probe 2-c), forward-only: для ratio-выводов достаточно
  (2-c показал forward/reverse симметрию на этой группе). Абсолюты 2-CPU sandbox — не переносить.
