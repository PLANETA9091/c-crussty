# absorb ROUND (round-428-anchorb, run 35853603155, branch round-428-anchorb, head dc704c9)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8935861 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 8935861 (поллов=5); TPS_exp=2.68; normalized=+4.4%
- GC: young=127, Full=10, total=21.8s, avg=159ms, max=2100ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112835 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.56% (-2.61%) спад
  - fluid: 16.72% -> 16.24% (-0.47%) флэт
  - broadphase: 15.66% -> 15.48% (-0.18%) флэт
  - nav_ai: 14.16% -> 14.12% (-0.04%) флэт
  - inside_volatile: 12.01% -> 11.11% (-0.90%) флэт
  - fastutil: 8.54% -> 9.05% (+0.52%) флэт
  - java_util: 7.01% -> 6.59% (-0.42%) флэт
  - paletted: 6.41% -> 6.78% (+0.37%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
