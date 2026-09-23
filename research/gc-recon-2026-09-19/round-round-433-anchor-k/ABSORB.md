# absorb ROUND (round-433-anchor-k, run 35907685958, branch round-433-anchor-k, head d282985)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6952698 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6952698 (поллов=6); TPS_exp=2.26; normalized=+6.1%
- GC: young=111, Full=9, total=20.7s, avg=172ms, max=2391ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116336 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.97% (-1.20%) спад
  - fluid: 16.72% -> 15.83% (-0.89%) флэт
  - broadphase: 15.66% -> 14.98% (-0.68%) флэт
  - nav_ai: 14.16% -> 13.82% (-0.35%) флэт
  - inside_volatile: 12.01% -> 11.25% (-0.76%) флэт
  - fastutil: 8.54% -> 8.78% (+0.24%) флэт
  - java_util: 7.01% -> 6.85% (-0.16%) флэт
  - paletted: 6.41% -> 6.18% (-0.23%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
