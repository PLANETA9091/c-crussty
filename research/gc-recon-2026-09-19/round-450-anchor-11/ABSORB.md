# absorb ROUND (450-anchor-11, run 36050636282, branch round-450-anchor-11, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6577962 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6577962 (поллов=5); TPS_exp=2.18; normalized=+5.3%
- GC: young=113, Full=9, total=23.5s, avg=193ms, max=2811ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115681 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.10% (-2.07%) спад
  - fluid: 16.72% -> 16.34% (-0.38%) флэт
  - broadphase: 15.66% -> 14.81% (-0.85%) флэт
  - nav_ai: 14.16% -> 13.92% (-0.25%) флэт
  - inside_volatile: 12.01% -> 11.10% (-0.91%) флэт
  - fastutil: 8.54% -> 8.68% (+0.14%) флэт
  - java_util: 7.01% -> 6.68% (-0.34%) флэт
  - paletted: 6.41% -> 6.85% (+0.44%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
