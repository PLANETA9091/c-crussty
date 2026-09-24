# absorb ROUND (round-439-anchor-4, run 35946066262, branch round-439-anchor-4, head afbcde6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7031569 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 7031569 (поллов=6); TPS_exp=2.28; normalized=+5.3%
- GC: young=112, Full=10, total=24.9s, avg=204ms, max=2722ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117189 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.81% (-1.36%) спад
  - fluid: 16.72% -> 15.87% (-0.84%) флэт
  - broadphase: 15.66% -> 14.96% (-0.70%) флэт
  - nav_ai: 14.16% -> 13.80% (-0.36%) флэт
  - inside_volatile: 12.01% -> 11.18% (-0.82%) флэт
  - fastutil: 8.54% -> 8.60% (+0.06%) флэт
  - java_util: 7.01% -> 6.19% (-0.82%) флэт
  - paletted: 6.41% -> 6.18% (-0.23%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
