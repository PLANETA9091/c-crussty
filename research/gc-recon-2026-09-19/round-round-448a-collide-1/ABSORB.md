# absorb ROUND (round-448a-collide-1, run 36015052561, branch round-448a-collide-1, head 18f37d5)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6983622 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6983622 (поллов=5); TPS_exp=2.27; normalized=+10.2%
- GC: young=119, Full=8, total=17.4s, avg=137ms, max=2701ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=100219 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.60% (+0.88%) флэт
  - broadphase: 15.66% -> 10.06% (-5.59%) спад
  - nav_ai: 14.16% -> 6.14% (-8.03%) спад
  - inside_volatile: 12.01% -> 12.91% (+0.91%) флэт
  - fastutil: 8.54% -> 6.12% (-2.42%) спад
  - java_util: 7.01% -> 7.38% (+0.37%) флэт
  - paletted: 6.41% -> 5.84% (-0.57%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
