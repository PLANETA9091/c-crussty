# absorb ROUND (anchor-14, run 36097268580, branch round-454-anchor-14, head ccfa6f0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6684972 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6684972 (поллов=5); TPS_exp=2.21; normalized=+4.2%
- GC: young=116, Full=9, total=21.7s, avg=174ms, max=2780ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117600 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.79% (-1.38%) спад
  - fluid: 16.72% -> 15.76% (-0.95%) флэт
  - broadphase: 15.66% -> 14.83% (-0.82%) флэт
  - nav_ai: 14.16% -> 13.88% (-0.29%) флэт
  - inside_volatile: 12.01% -> 11.83% (-0.17%) флэт
  - fastutil: 8.54% -> 8.63% (+0.10%) флэт
  - java_util: 7.01% -> 6.52% (-0.49%) флэт
  - paletted: 6.41% -> 6.30% (-0.11%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
