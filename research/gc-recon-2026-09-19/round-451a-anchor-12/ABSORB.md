# absorb ROUND (451a-anchor-12, run 36069542090, branch round-451-anchor-12, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6800196 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6800196 (поллов=5); TPS_exp=2.23; normalized=-1.4%
- GC: young=111, Full=9, total=20.8s, avg=174ms, max=2523ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117409 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.67% (-1.50%) спад
  - fluid: 16.72% -> 15.66% (-1.05%) спад
  - broadphase: 15.66% -> 14.93% (-0.72%) флэт
  - nav_ai: 14.16% -> 13.93% (-0.23%) флэт
  - inside_volatile: 12.01% -> 11.41% (-0.60%) флэт
  - fastutil: 8.54% -> 8.58% (+0.04%) флэт
  - java_util: 7.01% -> 6.53% (-0.49%) флэт
  - paletted: 6.41% -> 6.11% (-0.29%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
