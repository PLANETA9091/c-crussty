# absorb ROUND (round-434-anchor-o2, run 35914718278, branch round-434-anchor-o2, head 8141548)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6861409 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6861409 (поллов=5); TPS_exp=2.24; normalized=+2.5%
- GC: young=113, Full=10, total=23.6s, avg=192ms, max=2391ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117279 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.47% (-1.70%) спад
  - fluid: 16.72% -> 15.91% (-0.81%) флэт
  - broadphase: 15.66% -> 14.98% (-0.67%) флэт
  - nav_ai: 14.16% -> 13.94% (-0.22%) флэт
  - inside_volatile: 12.01% -> 11.47% (-0.53%) флэт
  - fastutil: 8.54% -> 8.41% (-0.13%) флэт
  - java_util: 7.01% -> 6.60% (-0.41%) флэт
  - paletted: 6.41% -> 6.32% (-0.09%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
