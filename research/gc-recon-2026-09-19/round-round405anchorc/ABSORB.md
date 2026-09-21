# absorb ROUND (round405anchorc, run 35631465535, branch round-405-anchorc, head 0eb844c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8841704 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 8841704 (поллов=5); TPS_exp=2.66; normalized=+1.5%
- GC: young=134, Full=9, total=20.2s, avg=141ms, max=2030ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112889 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.48% (-1.69%) спад
  - fluid: 16.72% -> 16.66% (-0.06%) флэт
  - broadphase: 15.66% -> 15.40% (-0.25%) флэт
  - nav_ai: 14.16% -> 13.42% (-0.74%) флэт
  - inside_volatile: 12.01% -> 11.80% (-0.21%) флэт
  - fastutil: 8.54% -> 8.51% (-0.02%) флэт
  - java_util: 7.01% -> 6.65% (-0.36%) флэт
  - paletted: 6.41% -> 6.98% (+0.57%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
