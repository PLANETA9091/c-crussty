# absorb ROUND (round-433-anchor-h, run 35907605232, branch round-433-anchor-h, head d282985)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6958652 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.15 @ 6958652 (поллов=6); TPS_exp=2.26; normalized=-5.0%
- GC: young=108, Full=9, total=22.0s, avg=188ms, max=2824ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116607 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.25% (-1.92%) спад
  - fluid: 16.72% -> 15.67% (-1.04%) спад
  - broadphase: 15.66% -> 15.59% (-0.06%) флэт
  - nav_ai: 14.16% -> 13.74% (-0.42%) флэт
  - inside_volatile: 12.01% -> 10.77% (-1.23%) спад
  - fastutil: 8.54% -> 8.56% (+0.02%) флэт
  - java_util: 7.01% -> 6.82% (-0.19%) флэт
  - paletted: 6.41% -> 5.96% (-0.45%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
