# absorb ROUND (450c-chunk-2r1, run 36053671041, branch round-450c-chunk-2r1, head f53979c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8608618 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 8608618 (поллов=5); TPS_exp=2.61; normalized=-0.5%
- GC: young=108, Full=9, total=23.5s, avg=201ms, max=3248ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=106798 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.64% (+0.92%) флэт
  - broadphase: 15.66% -> 9.71% (-5.94%) спад
  - nav_ai: 14.16% -> 3.37% (-10.79%) спад
  - inside_volatile: 12.01% -> 16.63% (+4.62%) РОСТ
  - fastutil: 8.54% -> 6.28% (-2.26%) спад
  - java_util: 7.01% -> 9.12% (+2.10%) РОСТ
  - paletted: 6.41% -> 6.31% (-0.10%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
