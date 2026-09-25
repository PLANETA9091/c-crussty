# absorb ROUND (a14-457, run 36131720754, branch round-457-anchor-14, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6986856 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6986856 (поллов=6); TPS_exp=2.27; normalized=+1.3%
- GC: young=107, Full=9, total=20.7s, avg=178ms, max=2510ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116479 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.66% (-1.51%) спад
  - fluid: 16.72% -> 15.93% (-0.78%) флэт
  - broadphase: 15.66% -> 15.29% (-0.37%) флэт
  - nav_ai: 14.16% -> 13.76% (-0.40%) флэт
  - inside_volatile: 12.01% -> 11.15% (-0.86%) флэт
  - fastutil: 8.54% -> 8.69% (+0.15%) флэт
  - java_util: 7.01% -> 6.83% (-0.18%) флэт
  - paletted: 6.41% -> 5.98% (-0.43%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
