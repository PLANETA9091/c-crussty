# absorb ROUND (round-433-anchor-c, run 35901485687, branch round-433-anchor-c, head d282985)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6921279 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6921279 (поллов=6); TPS_exp=2.26; normalized=+4.1%
- GC: young=109, Full=9, total=19.9s, avg=168ms, max=2370ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117308 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.39% (-1.79%) спад
  - fluid: 16.72% -> 15.61% (-1.10%) спад
  - broadphase: 15.66% -> 15.41% (-0.24%) флэт
  - nav_ai: 14.16% -> 13.68% (-0.48%) флэт
  - inside_volatile: 12.01% -> 11.43% (-0.57%) флэт
  - fastutil: 8.54% -> 8.55% (+0.01%) флэт
  - java_util: 7.01% -> 6.62% (-0.40%) флэт
  - paletted: 6.41% -> 6.19% (-0.21%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
