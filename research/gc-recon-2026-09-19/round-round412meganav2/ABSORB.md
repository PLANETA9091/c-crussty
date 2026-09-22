# absorb ROUND (round412meganav2, run 35705669376, branch round-412-meganav2, head f579c1c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6781278 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 6781278 (поллов=6); TPS_exp=2.23; normalized=+21.2%
- GC: young=107, Full=9, total=18.6s, avg=160ms, max=2280ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106989 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.51% (-0.21%) флэт
  - broadphase: 15.66% -> 14.06% (-1.59%) спад
  - nav_ai: 14.16% -> 8.90% (-5.26%) спад
  - inside_volatile: 12.01% -> 12.29% (+0.29%) флэт
  - fastutil: 8.54% -> 7.29% (-1.24%) спад
  - java_util: 7.01% -> 7.46% (+0.45%) флэт
  - paletted: 6.41% -> 5.73% (-0.68%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
