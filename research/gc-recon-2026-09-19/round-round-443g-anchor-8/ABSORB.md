# absorb ROUND (round-443g-anchor-8, run 36042417040, branch round-443g-anchor-8, head 9061566)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6428986 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 6428986 (поллов=6); TPS_exp=2.15; normalized=+4.5%
- GC: young=107, Full=9, total=21.5s, avg=185ms, max=2999ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116393 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.76% (-1.41%) спад
  - fluid: 16.72% -> 15.89% (-0.83%) флэт
  - broadphase: 15.66% -> 15.27% (-0.39%) флэт
  - nav_ai: 14.16% -> 13.28% (-0.88%) флэт
  - inside_volatile: 12.01% -> 11.24% (-0.77%) флэт
  - fastutil: 8.54% -> 8.59% (+0.05%) флэт
  - java_util: 7.01% -> 6.29% (-0.72%) флэт
  - paletted: 6.41% -> 5.97% (-0.43%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
