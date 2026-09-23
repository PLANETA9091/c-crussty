# absorb ROUND (round-423-anchora, run 35815682445, branch round-423-anchora, head 4789ca7)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6769427 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6769427 (поллов=5); TPS_exp=2.22; normalized=+3.4%
- GC: young=110, Full=9, total=20.7s, avg=174ms, max=2673ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116322 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.59% (-1.58%) спад
  - fluid: 16.72% -> 15.88% (-0.84%) флэт
  - broadphase: 15.66% -> 15.16% (-0.50%) флэт
  - nav_ai: 14.16% -> 13.88% (-0.28%) флэт
  - inside_volatile: 12.01% -> 11.43% (-0.57%) флэт
  - fastutil: 8.54% -> 8.89% (+0.35%) флэт
  - java_util: 7.01% -> 6.56% (-0.45%) флэт
  - paletted: 6.41% -> 6.43% (+0.02%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
