# absorb ROUND (431c-anchor-a, run 35886950729, branch round-431c-anchor-a, head 3065047)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7094984 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 7094984 (поллов=6); TPS_exp=2.29; normalized=+4.7%
- GC: young=106, Full=9, total=19.8s, avg=172ms, max=2431ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116051 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.31% (-1.86%) спад
  - fluid: 16.72% -> 15.87% (-0.84%) флэт
  - broadphase: 15.66% -> 14.85% (-0.81%) флэт
  - nav_ai: 14.16% -> 14.01% (-0.15%) флэт
  - inside_volatile: 12.01% -> 11.30% (-0.71%) флэт
  - fastutil: 8.54% -> 8.77% (+0.24%) флэт
  - java_util: 7.01% -> 6.87% (-0.15%) флэт
  - paletted: 6.41% -> 6.04% (-0.36%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
