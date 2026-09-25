# absorb ROUND (a4-457, run 36131607242, branch round-457-anchor-4, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6737702 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6737702 (поллов=5); TPS_exp=2.22; normalized=-0.8%
- GC: young=104, Full=8, total=20.7s, avg=185ms, max=2432ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116337 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.49% (-1.68%) спад
  - fluid: 16.72% -> 15.33% (-1.38%) спад
  - broadphase: 15.66% -> 15.14% (-0.52%) флэт
  - nav_ai: 14.16% -> 14.12% (-0.05%) флэт
  - inside_volatile: 12.01% -> 11.43% (-0.57%) флэт
  - fastutil: 8.54% -> 8.87% (+0.34%) флэт
  - java_util: 7.01% -> 6.65% (-0.36%) флэт
  - paletted: 6.41% -> 6.20% (-0.21%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
