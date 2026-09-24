# absorb ROUND (round-437-anchor-5, run 35935343055, branch round-437-anchor-5, head e4d39b1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6726738 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6726738 (поллов=5); TPS_exp=2.22; normalized=+3.8%
- GC: young=111, Full=9, total=23.9s, avg=199ms, max=2897ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115636 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.06% (-2.11%) спад
  - fluid: 16.72% -> 16.57% (-0.15%) флэт
  - broadphase: 15.66% -> 14.81% (-0.85%) флэт
  - nav_ai: 14.16% -> 13.32% (-0.85%) флэт
  - inside_volatile: 12.01% -> 10.78% (-1.23%) спад
  - fastutil: 8.54% -> 8.28% (-0.25%) флэт
  - java_util: 7.01% -> 6.50% (-0.51%) флэт
  - paletted: 6.41% -> 7.13% (+0.73%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
