# absorb ROUND (round-434c-chk-1, run 35917619575, branch round-434c-chk-1, head a17cde0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6652817 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6652817 (поллов=5); TPS_exp=2.20; normalized=+9.1%
- GC: young=106, Full=9, total=19.7s, avg=172ms, max=2435ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104988 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.91% (+0.19%) флэт
  - broadphase: 15.66% -> 9.84% (-5.82%) спад
  - nav_ai: 14.16% -> 3.22% (-10.94%) спад
  - inside_volatile: 12.01% -> 16.28% (+4.27%) РОСТ
  - fastutil: 8.54% -> 6.27% (-2.27%) спад
  - java_util: 7.01% -> 8.38% (+1.37%) РОСТ
  - paletted: 6.41% -> 5.11% (-1.29%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
