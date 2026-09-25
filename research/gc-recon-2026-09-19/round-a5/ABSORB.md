# absorb ROUND (a5, run 36104744433, branch round-455-anchor-5, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7071664 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.00 @ 7071664 (поллов=6); TPS_exp=2.29; normalized=-12.6%
- GC: young=104, Full=9, total=20.6s, avg=182ms, max=2468ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115892 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.02% (-2.15%) спад
  - fluid: 16.72% -> 15.54% (-1.18%) спад
  - broadphase: 15.66% -> 15.82% (+0.16%) флэт
  - nav_ai: 14.16% -> 14.11% (-0.05%) флэт
  - inside_volatile: 12.01% -> 11.02% (-0.98%) флэт
  - fastutil: 8.54% -> 8.77% (+0.23%) флэт
  - java_util: 7.01% -> 6.45% (-0.56%) флэт
  - paletted: 6.41% -> 5.84% (-0.56%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
