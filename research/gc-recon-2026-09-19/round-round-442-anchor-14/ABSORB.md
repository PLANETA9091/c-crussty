# absorb ROUND (round-442-anchor-14, run 35957211173, branch round-442-anchor-14, head 335f170)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6460946 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6460946 (поллов=5); TPS_exp=2.16; normalized=+6.5%
- GC: young=119, Full=8, total=24.0s, avg=189ms, max=2519ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116255 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.79% (-1.38%) спад
  - fluid: 16.72% -> 16.57% (-0.14%) флэт
  - broadphase: 15.66% -> 14.11% (-1.55%) спад
  - nav_ai: 14.16% -> 13.21% (-0.95%) флэт
  - inside_volatile: 12.01% -> 11.47% (-0.53%) флэт
  - fastutil: 8.54% -> 8.94% (+0.40%) флэт
  - java_util: 7.01% -> 6.80% (-0.21%) флэт
  - paletted: 6.41% -> 6.73% (+0.32%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
