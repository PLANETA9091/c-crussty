# absorb ROUND (450e-anchor-50, run 36066577014, branch round-450-anchor-50, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6842561 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6842561 (поллов=6); TPS_exp=2.24; normalized=+4.9%
- GC: young=110, Full=10, total=23.7s, avg=197ms, max=2405ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117185 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.34% (-1.83%) спад
  - fluid: 16.72% -> 15.70% (-1.01%) спад
  - broadphase: 15.66% -> 14.70% (-0.95%) флэт
  - nav_ai: 14.16% -> 13.65% (-0.52%) флэт
  - inside_volatile: 12.01% -> 11.15% (-0.85%) флэт
  - fastutil: 8.54% -> 8.47% (-0.06%) флэт
  - java_util: 7.01% -> 6.72% (-0.30%) флэт
  - paletted: 6.41% -> 6.35% (-0.05%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
