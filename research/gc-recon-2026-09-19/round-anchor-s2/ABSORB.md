# absorb ROUND (anchor-s2, run 35914827847, branch round-434-anchor-s2, head 8141548)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6726470 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6726470 (поллов=5); TPS_exp=2.22; normalized=-0.7%
- GC: young=108, Full=9, total=20.5s, avg=175ms, max=2527ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116316 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.64% (-1.53%) спад
  - fluid: 16.72% -> 15.78% (-0.93%) флэт
  - broadphase: 15.66% -> 15.26% (-0.39%) флэт
  - nav_ai: 14.16% -> 13.94% (-0.22%) флэт
  - inside_volatile: 12.01% -> 11.49% (-0.51%) флэт
  - fastutil: 8.54% -> 8.80% (+0.27%) флэт
  - java_util: 7.01% -> 6.76% (-0.25%) флэт
  - paletted: 6.41% -> 6.09% (-0.31%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
