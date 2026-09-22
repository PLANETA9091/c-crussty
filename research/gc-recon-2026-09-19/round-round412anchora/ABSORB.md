# absorb ROUND (round412anchora, run 35704269872, branch round-412-anchora, head 3bd07fe)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6803065 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6803065 (поллов=5); TPS_exp=2.23; normalized=+3.1%
- GC: young=109, Full=8, total=23.6s, avg=202ms, max=2529ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113784 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.72% (-0.46%) флэт
  - fluid: 16.72% -> 16.61% (-0.11%) флэт
  - broadphase: 15.66% -> 14.89% (-0.77%) флэт
  - nav_ai: 14.16% -> 13.61% (-0.56%) флэт
  - inside_volatile: 12.01% -> 11.27% (-0.74%) флэт
  - fastutil: 8.54% -> 8.96% (+0.43%) флэт
  - java_util: 7.01% -> 6.64% (-0.37%) флэт
  - paletted: 6.41% -> 7.26% (+0.85%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
