# absorb ROUND (anchor-627, run 36207463578, branch round-463-anchor-627, head 97f1c9c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6146935 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6146935 (поллов=5); TPS_exp=2.09; normalized=+0.3%
- GC: young=104, Full=9, total=20.7s, avg=183ms, max=2475ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117483 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.57% (-1.61%) спад
  - fluid: 16.72% -> 15.55% (-1.17%) спад
  - broadphase: 15.66% -> 15.04% (-0.62%) флэт
  - nav_ai: 14.16% -> 14.13% (-0.03%) флэт
  - inside_volatile: 12.01% -> 11.41% (-0.59%) флэт
  - fastutil: 8.54% -> 8.85% (+0.32%) флэт
  - java_util: 7.01% -> 6.97% (-0.04%) флэт
  - paletted: 6.41% -> 5.95% (-0.46%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
