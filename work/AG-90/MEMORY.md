# AG-90 MEMORY (волна-526, cycle-1) — уроки ≤15 строк
1. WBP-инпуты брать с живого мастера (radius/seconds/.../population_target) — локальный
   клон протух: yml-блоб верифить по API, не по локали.
2. 2x2-интеракции (rt×pop, fp×pop) — свободный класс клеток против midpoint-штампедов:
   rt8@450k/fp8@400k 0-клейм при полном шторме мидов (клетки живут <3 мин).
3. Пары 2x2: AG-32 rt4@450k, AG-198 rt8@150k, AG-2 fp4@400k, AG-266 fp8@150k — все queued,
   harvest x527 закроет матрицу без новых POST.
4. population_seed=42 = same-seed пары к pop-когорте (канон AG-243/257), не рандом-сиды.
5. POST /git/refs FULL-sha 201 x2 (swarm-526-90/90b @b0642438, tree 4256 truncated=False),
   диспатчи разнесены 32s — 0 фантомов (оба run-id в /actions/runs queued).
6. Board CAS: 409-ретраи не понадобились, но gвардить свой ID в хвосте надо маркером темы.
