# absorb ROUND (round-441-sense-1, run 35954884128, branch round-441-sense-1, head b6134c6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6870539 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6870539 (поллов=6); TPS_exp=2.25; normalized=+6.9%
- GC: young=106, Full=8, total=19.2s, avg=169ms, max=2398ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112048 сэмплов (базлайн 115655)
  - items: 31.17% -> 34.77% (+3.60%) РОСТ
  - fluid: 16.72% -> 16.90% (+0.18%) флэт
  - broadphase: 15.66% -> 15.09% (-0.56%) флэт
  - nav_ai: 14.16% -> 7.81% (-6.35%) спад
  - inside_volatile: 12.01% -> 13.87% (+1.87%) РОСТ
  - fastutil: 8.54% -> 7.01% (-1.53%) спад
  - java_util: 7.01% -> 7.56% (+0.55%) флэт
  - paletted: 6.41% -> 6.19% (-0.21%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
