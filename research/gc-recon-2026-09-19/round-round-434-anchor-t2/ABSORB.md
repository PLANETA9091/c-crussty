# absorb ROUND (round-434-anchor-t2, run 35919255158, branch round-434-anchor-t2, head 8141548)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6599503 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6599503 (поллов=5); TPS_exp=2.19; normalized=+5.1%
- GC: young=110, Full=9, total=20.9s, avg=175ms, max=2435ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116226 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.02% (-1.15%) спад
  - fluid: 16.72% -> 16.11% (-0.61%) флэт
  - broadphase: 15.66% -> 15.35% (-0.31%) флэт
  - nav_ai: 14.16% -> 14.04% (-0.12%) флэт
  - inside_volatile: 12.01% -> 11.23% (-0.78%) флэт
  - fastutil: 8.54% -> 9.30% (+0.77%) флэт
  - java_util: 7.01% -> 6.61% (-0.40%) флэт
  - paletted: 6.41% -> 6.44% (+0.04%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
