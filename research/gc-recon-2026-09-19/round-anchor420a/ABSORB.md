# absorb ROUND (anchor420a, run 35788810224, branch round-420-anchora, head 41456af)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8654482 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.75 @ 8654482 (поллов=6); TPS_exp=2.62; normalized=+4.9%
- GC: young=117, Full=10, total=21.6s, avg=170ms, max=2110ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112530 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.78% (-2.39%) спад
  - fluid: 16.72% -> 16.48% (-0.24%) флэт
  - broadphase: 15.66% -> 15.16% (-0.50%) флэт
  - nav_ai: 14.16% -> 13.73% (-0.44%) флэт
  - inside_volatile: 12.01% -> 10.27% (-1.73%) спад
  - fastutil: 8.54% -> 8.77% (+0.23%) флэт
  - java_util: 7.01% -> 6.76% (-0.25%) флэт
  - paletted: 6.41% -> 6.99% (+0.58%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
