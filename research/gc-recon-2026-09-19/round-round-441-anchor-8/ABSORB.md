# absorb ROUND (round-441-anchor-8, run 35954893159, branch round-441-anchor-8, head 6db49fb)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8788920 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 8788920 (поллов=5); TPS_exp=2.65; normalized=-1.9%
- GC: young=115, Full=8, total=23.6s, avg=192ms, max=2704ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116193 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.02% (-2.15%) спад
  - fluid: 16.72% -> 16.92% (+0.21%) флэт
  - broadphase: 15.66% -> 14.40% (-1.25%) спад
  - nav_ai: 14.16% -> 13.62% (-0.54%) флэт
  - inside_volatile: 12.01% -> 11.72% (-0.28%) флэт
  - fastutil: 8.54% -> 9.03% (+0.50%) флэт
  - java_util: 7.01% -> 6.77% (-0.24%) флэт
  - paletted: 6.41% -> 7.44% (+1.04%) РОСТ
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
