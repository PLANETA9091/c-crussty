# absorb ROUND (round-433-anchor-j, run 35907657727, branch round-433-anchor-j, head d282985)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6924148 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6924148 (поллов=6); TPS_exp=2.26; normalized=+4.1%
- GC: young=111, Full=10, total=23.9s, avg=198ms, max=2414ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117449 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.89% (-1.28%) спад
  - fluid: 16.72% -> 15.96% (-0.76%) флэт
  - broadphase: 15.66% -> 14.98% (-0.68%) флэт
  - nav_ai: 14.16% -> 13.62% (-0.55%) флэт
  - inside_volatile: 12.01% -> 11.40% (-0.60%) флэт
  - fastutil: 8.54% -> 8.54% (+0.00%) флэт
  - java_util: 7.01% -> 6.42% (-0.59%) флэт
  - paletted: 6.41% -> 6.19% (-0.22%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
