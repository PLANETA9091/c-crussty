# absorb ROUND (round405comp3, run 35633645282, branch round-405-comp-l3, head fef3746)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7619630 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.30 @ 7619630 (поллов=5); TPS_exp=2.40; normalized=+37.3%
- GC: young=123, Full=10, total=19.7s, avg=148ms, max=2099ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107040 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.93% (+0.22%) флэт
  - broadphase: 15.66% -> 14.34% (-1.32%) спад
  - nav_ai: 14.16% -> 9.81% (-4.35%) спад
  - inside_volatile: 12.01% -> 11.55% (-0.45%) флэт
  - fastutil: 8.54% -> 7.69% (-0.84%) флэт
  - java_util: 7.01% -> 7.27% (+0.25%) флэт
  - paletted: 6.41% -> 6.17% (-0.24%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
