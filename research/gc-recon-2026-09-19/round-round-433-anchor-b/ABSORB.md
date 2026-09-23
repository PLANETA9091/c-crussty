# absorb ROUND (round-433-anchor-b, run 35901458030, branch round-433-anchor-b, head d282985)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8958774 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.85 @ 8958774 (поллов=6); TPS_exp=2.69; normalized=+6.1%
- GC: young=126, Full=10, total=22.2s, avg=163ms, max=2112ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113466 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.67% (-2.51%) спад
  - fluid: 16.72% -> 16.62% (-0.10%) флэт
  - broadphase: 15.66% -> 15.01% (-0.65%) флэт
  - nav_ai: 14.16% -> 13.80% (-0.37%) флэт
  - inside_volatile: 12.01% -> 10.67% (-1.34%) спад
  - fastutil: 8.54% -> 8.46% (-0.08%) флэт
  - java_util: 7.01% -> 6.58% (-0.43%) флэт
  - paletted: 6.41% -> 6.96% (+0.56%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
