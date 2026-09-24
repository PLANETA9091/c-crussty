# absorb ROUND (round-433-anchor-g2, run 35910288222, branch round-433-anchor-g2, head d282985)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6653426 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6653426 (поллов=6); TPS_exp=2.20; normalized=+6.8%
- GC: young=107, Full=9, total=20.9s, avg=180ms, max=2779ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117200 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.55% (-1.63%) спад
  - fluid: 16.72% -> 15.50% (-1.21%) спад
  - broadphase: 15.66% -> 15.67% (+0.01%) флэт
  - nav_ai: 14.16% -> 13.80% (-0.36%) флэт
  - inside_volatile: 12.01% -> 11.52% (-0.48%) флэт
  - fastutil: 8.54% -> 8.47% (-0.06%) флэт
  - java_util: 7.01% -> 6.38% (-0.63%) флэт
  - paletted: 6.41% -> 6.03% (-0.38%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
