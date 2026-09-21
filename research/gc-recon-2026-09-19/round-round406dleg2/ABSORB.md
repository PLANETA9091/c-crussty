# absorb ROUND (round406dleg2, run 35652772569, branch round-406-d-l2, head a46a094)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6791121 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6791121 (поллов=5); TPS_exp=2.23; normalized=+16.6%
- GC: young=114, Full=9, total=21.6s, avg=176ms, max=2391ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=111283 сэмплов (базлайн 115655)
  - items: 31.17% -> 33.79% (+2.62%) РОСТ
  - fluid: 16.72% -> 18.52% (+1.80%) РОСТ
  - broadphase: 15.66% -> 15.11% (-0.54%) флэт
  - nav_ai: 14.16% -> 8.72% (-5.45%) спад
  - inside_volatile: 12.01% -> 11.48% (-0.52%) флэт
  - fastutil: 8.54% -> 7.00% (-1.53%) спад
  - java_util: 7.01% -> 7.36% (+0.34%) флэт
  - paletted: 6.41% -> 7.29% (+0.89%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
