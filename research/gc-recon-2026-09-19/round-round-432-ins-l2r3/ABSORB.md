# absorb ROUND (round-432-ins-l2r3, run 35896502999, branch round-432-ins-l2r3, head 3f6f6e6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8833269 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.90 @ 8833269 (поллов=5); TPS_exp=2.66; normalized=+9.1%
- GC: young=132, Full=10, total=19.8s, avg=139ms, max=2616ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=101822 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.00% (+0.28%) флэт
  - broadphase: 15.66% -> 9.52% (-6.14%) спад
  - nav_ai: 14.16% -> 3.43% (-10.74%) спад
  - inside_volatile: 12.01% -> 13.92% (+1.91%) РОСТ
  - fastutil: 8.54% -> 6.90% (-1.64%) спад
  - java_util: 7.01% -> 9.04% (+2.03%) РОСТ
  - paletted: 6.41% -> 6.12% (-0.29%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
