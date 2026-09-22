# absorb ROUND (mc3b, run 35765659501, branch round-417-a-mc3b, head 20c9fdc)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8678220 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 8678220 (поллов=5); TPS_exp=2.63; normalized=+2.8%
- GC: young=108, Full=9, total=18.8s, avg=161ms, max=2484ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=109469 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.54% (+0.83%) флэт
  - broadphase: 15.66% -> 9.82% (-5.83%) спад
  - nav_ai: 14.16% -> 4.28% (-9.89%) спад
  - inside_volatile: 12.01% -> 12.00% (-0.01%) флэт
  - fastutil: 8.54% -> 6.04% (-2.50%) спад
  - java_util: 7.01% -> 7.65% (+0.64%) флэт
  - paletted: 6.41% -> 6.69% (+0.29%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **PARITY/LOW**
