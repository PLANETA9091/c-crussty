# absorb ROUND (450c-anchor-23, run 36058421472, branch round-450-anchor-23, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6688724 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6688724 (поллов=6); TPS_exp=2.21; normalized=+4.2%
- GC: young=113, Full=10, total=24.6s, avg=200ms, max=2438ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117572 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.27% (-0.90%) флэт
  - fluid: 16.72% -> 16.13% (-0.58%) флэт
  - broadphase: 15.66% -> 15.18% (-0.47%) флэт
  - nav_ai: 14.16% -> 13.98% (-0.18%) флэт
  - inside_volatile: 12.01% -> 11.45% (-0.56%) флэт
  - fastutil: 8.54% -> 8.59% (+0.05%) флэт
  - java_util: 7.01% -> 6.58% (-0.43%) флэт
  - paletted: 6.41% -> 6.25% (-0.15%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
