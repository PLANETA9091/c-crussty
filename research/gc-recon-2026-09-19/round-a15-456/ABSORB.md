# absorb ROUND (a15-456, run 36112224970, branch round-456-anchor-15, head 0716075)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6756575 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6756575 (поллов=6); TPS_exp=2.22; normalized=+8.0%
- GC: young=110, Full=9, total=21.3s, avg=179ms, max=2867ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116524 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.01% (-1.16%) спад
  - fluid: 16.72% -> 15.80% (-0.92%) флэт
  - broadphase: 15.66% -> 15.26% (-0.40%) флэт
  - nav_ai: 14.16% -> 14.43% (+0.27%) флэт
  - inside_volatile: 12.01% -> 11.78% (-0.22%) флэт
  - fastutil: 8.54% -> 8.90% (+0.36%) флэт
  - java_util: 7.01% -> 6.80% (-0.21%) флэт
  - paletted: 6.41% -> 6.19% (-0.22%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
