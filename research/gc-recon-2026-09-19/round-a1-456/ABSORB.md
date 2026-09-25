# absorb ROUND (a1-456, run 36112077248, branch round-456-anchor-1, head 0716075)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8935474 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 8935474 (поллов=5); TPS_exp=2.68; normalized=+4.4%
- GC: young=125, Full=10, total=21.6s, avg=160ms, max=2112ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113660 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.85% (-2.32%) спад
  - fluid: 16.72% -> 16.51% (-0.20%) флэт
  - broadphase: 15.66% -> 15.10% (-0.55%) флэт
  - nav_ai: 14.16% -> 13.80% (-0.37%) флэт
  - inside_volatile: 12.01% -> 10.73% (-1.28%) спад
  - fastutil: 8.54% -> 9.09% (+0.55%) флэт
  - java_util: 7.01% -> 6.87% (-0.15%) флэт
  - paletted: 6.41% -> 6.87% (+0.46%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
