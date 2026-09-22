# absorb ROUND (mc2c, run 35748285387, branch round-416-a-mc2c, head 079eb4a)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7172726 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=5 -> **PASS**
- T3: median=0.70 @ 7172726 (поллов=4); TPS_exp=2.31; normalized=-69.7%
- GC: young=63, Full=8, total=13.4s, avg=189ms, max=2169ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113612 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 11.70% (-5.01%) спад
  - broadphase: 15.66% -> 6.28% (-9.37%) спад
  - nav_ai: 14.16% -> 2.20% (-11.96%) спад
  - inside_volatile: 12.01% -> 8.14% (-3.86%) спад
  - fastutil: 8.54% -> 3.26% (-5.28%) спад
  - java_util: 7.01% -> 4.36% (-2.65%) спад
  - paletted: 6.41% -> 3.65% (-2.76%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
