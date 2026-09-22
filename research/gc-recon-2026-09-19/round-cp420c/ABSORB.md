# absorb ROUND (cp420c, run 35791191062, branch round-420-a-cpc, head 9bde8bf)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6690540 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6690540 (поллов=5); TPS_exp=2.21; normalized=+17.8%
- GC: young=1128, Full=9, total=26.0s, avg=23ms, max=2265ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107140 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.27% (-0.45%) флэт
  - broadphase: 15.66% -> 14.37% (-1.29%) спад
  - nav_ai: 14.16% -> 9.19% (-4.98%) спад
  - inside_volatile: 12.01% -> 12.51% (+0.51%) флэт
  - fastutil: 8.54% -> 8.03% (-0.51%) флэт
  - java_util: 7.01% -> 8.15% (+1.14%) РОСТ
  - paletted: 6.41% -> 5.48% (-0.92%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
