# absorb ROUND (anchor-593, run 36207368064, branch round-463-anchor-593, head 97f1c9c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6433455 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6433455 (поллов=5); TPS_exp=2.15; normalized=+2.2%
- GC: young=110, Full=10, total=23.9s, avg=199ms, max=2589ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117444 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.98% (-1.19%) спад
  - fluid: 16.72% -> 15.79% (-0.93%) флэт
  - broadphase: 15.66% -> 15.75% (+0.09%) флэт
  - nav_ai: 14.16% -> 13.41% (-0.75%) флэт
  - inside_volatile: 12.01% -> 11.68% (-0.33%) флэт
  - fastutil: 8.54% -> 8.40% (-0.13%) флэт
  - java_util: 7.01% -> 6.71% (-0.30%) флэт
  - paletted: 6.41% -> 5.88% (-0.52%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
