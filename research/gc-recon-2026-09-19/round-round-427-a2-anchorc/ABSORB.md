# absorb ROUND (round-427-a2-anchorc, run 35851874982, branch round-427-a2-anchorc, head 790dc2f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6602607 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6602607 (поллов=5); TPS_exp=2.19; normalized=+0.5%
- GC: young=112, Full=10, total=24.4s, avg=200ms, max=2441ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117115 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.19% (-0.99%) флэт
  - fluid: 16.72% -> 16.11% (-0.61%) флэт
  - broadphase: 15.66% -> 15.15% (-0.50%) флэт
  - nav_ai: 14.16% -> 13.75% (-0.42%) флэт
  - inside_volatile: 12.01% -> 11.50% (-0.51%) флэт
  - fastutil: 8.54% -> 8.87% (+0.34%) флэт
  - java_util: 7.01% -> 6.32% (-0.70%) флэт
  - paletted: 6.41% -> 6.37% (-0.03%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
