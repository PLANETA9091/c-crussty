# absorb ROUND (diet454-11, run 36100442432, branch round-454c-diet-11, head 47ea8b2)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7310117 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.00 @ 7310117 (поллов=5); TPS_exp=2.34; normalized=+28.3%
- GC: young=113, Full=9, total=22.7s, avg=186ms, max=2987ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105954 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.78% (+2.07%) РОСТ
  - broadphase: 15.66% -> 9.84% (-5.82%) спад
  - nav_ai: 14.16% -> 3.44% (-10.73%) спад
  - inside_volatile: 12.01% -> 17.65% (+5.64%) РОСТ
  - fastutil: 8.54% -> 6.33% (-2.21%) спад
  - java_util: 7.01% -> 8.67% (+1.65%) РОСТ
  - paletted: 6.41% -> 5.60% (-0.81%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
