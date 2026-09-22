# absorb ROUND (g3a2, run 35748314494, branch round-416-b-g3a, head b1dbc66)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6843210 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6843210 (поллов=5); TPS_exp=2.24; normalized=+7.1%
- GC: young=107, Full=9, total=19.1s, avg=164ms, max=2315ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112812 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 14.70% (-2.02%) спад
  - broadphase: 15.66% -> 14.65% (-1.01%) спад
  - nav_ai: 14.16% -> 7.04% (-7.13%) спад
  - inside_volatile: 12.01% -> 11.74% (-0.26%) флэт
  - fastutil: 8.54% -> 8.53% (-0.01%) флэт
  - java_util: 7.01% -> 7.93% (+0.91%) флэт
  - paletted: 6.41% -> 5.64% (-0.77%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
