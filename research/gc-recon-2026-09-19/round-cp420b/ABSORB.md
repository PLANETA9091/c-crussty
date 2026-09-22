# absorb ROUND (cp420b, run 35791184136, branch round-420-a-cpb, head 9bde8bf)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6565183 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 6565183 (поллов=5); TPS_exp=2.18; normalized=+14.6%
- GC: young=1139, Full=10, total=27.2s, avg=24ms, max=2290ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=108642 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.72% (-1.00%) флэт
  - broadphase: 15.66% -> 14.07% (-1.59%) спад
  - nav_ai: 14.16% -> 9.21% (-4.95%) спад
  - inside_volatile: 12.01% -> 12.49% (+0.49%) флэт
  - fastutil: 8.54% -> 7.69% (-0.84%) флэт
  - java_util: 7.01% -> 7.91% (+0.89%) флэт
  - paletted: 6.41% -> 5.28% (-1.13%) спад
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
