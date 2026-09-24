# absorb ROUND (round-437-anchor-3r2, run 35938699587, branch round-437-anchor-3r2, head e4d39b1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6931670 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 6931670 (поллов=6); TPS_exp=2.26; normalized=-0.4%
- GC: young=109, Full=9, total=20.8s, avg=176ms, max=2448ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116342 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.57% (-1.60%) спад
  - fluid: 16.72% -> 16.03% (-0.69%) флэт
  - broadphase: 15.66% -> 15.91% (+0.25%) флэт
  - nav_ai: 14.16% -> 14.31% (+0.15%) флэт
  - inside_volatile: 12.01% -> 11.19% (-0.82%) флэт
  - fastutil: 8.54% -> 8.70% (+0.17%) флэт
  - java_util: 7.01% -> 6.48% (-0.53%) флэт
  - paletted: 6.41% -> 5.91% (-0.50%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
