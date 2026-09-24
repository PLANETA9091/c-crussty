# absorb ROUND (450c-anchor-21, run 36058393798, branch round-450-anchor-21, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6705744 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6705744 (поллов=6); TPS_exp=2.21; normalized=+6.3%
- GC: young=110, Full=9, total=21.0s, avg=177ms, max=2332ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116737 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.36% (-1.81%) спад
  - fluid: 16.72% -> 15.62% (-1.10%) спад
  - broadphase: 15.66% -> 15.45% (-0.21%) флэт
  - nav_ai: 14.16% -> 13.98% (-0.19%) флэт
  - inside_volatile: 12.01% -> 11.54% (-0.46%) флэт
  - fastutil: 8.54% -> 8.51% (-0.03%) флэт
  - java_util: 7.01% -> 6.40% (-0.61%) флэт
  - paletted: 6.41% -> 6.30% (-0.10%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
