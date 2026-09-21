# absorb ROUND (round403cl2, run 35606648953, branch round-403-c-leg2, head bab6b74)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7103401 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 7103401 (поллов=5); TPS_exp=2.29; normalized=+13.3%
- GC: young=112, Full=9, total=19.2s, avg=158ms, max=2206ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112697 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.08% (+0.36%) флэт
  - broadphase: 15.66% -> 15.57% (-0.08%) флэт
  - nav_ai: 14.16% -> 7.08% (-7.09%) спад
  - inside_volatile: 12.01% -> 12.26% (+0.26%) флэт
  - fastutil: 8.54% -> 7.10% (-1.44%) спад
  - java_util: 7.01% -> 6.92% (-0.10%) флэт
  - paletted: 6.41% -> 6.17% (-0.24%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
