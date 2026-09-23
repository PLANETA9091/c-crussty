# absorb ROUND (round-432-anchor-g, run 35891413990, branch round-432-anchor-g, head e05681f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7194312 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 7194312 (поллов=5); TPS_exp=2.31; normalized=-9.2%
- GC: young=106, Full=10, total=23.5s, avg=202ms, max=2504ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116644 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.56% (-1.61%) спад
  - fluid: 16.72% -> 15.47% (-1.24%) спад
  - broadphase: 15.66% -> 15.81% (+0.16%) флэт
  - nav_ai: 14.16% -> 13.96% (-0.20%) флэт
  - inside_volatile: 12.01% -> 11.26% (-0.75%) флэт
  - fastutil: 8.54% -> 8.70% (+0.16%) флэт
  - java_util: 7.01% -> 6.56% (-0.45%) флэт
  - paletted: 6.41% -> 6.01% (-0.40%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
