# absorb ROUND (a7-456, run 36112141209, branch round-456-anchor-7, head 0716075)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7264766 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.50 @ 7264766 (поллов=5); TPS_exp=2.33; normalized=+7.4%
- GC: young=125, Full=10, total=28.5s, avg=211ms, max=2862ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117211 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.11% (-1.06%) спад
  - fluid: 16.72% -> 18.17% (+1.46%) РОСТ
  - broadphase: 15.66% -> 15.75% (+0.09%) флэт
  - nav_ai: 14.16% -> 13.97% (-0.20%) флэт
  - inside_volatile: 12.01% -> 12.33% (+0.33%) флэт
  - fastutil: 8.54% -> 8.08% (-0.46%) флэт
  - java_util: 7.01% -> 6.15% (-0.86%) флэт
  - paletted: 6.41% -> 6.32% (-0.08%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
