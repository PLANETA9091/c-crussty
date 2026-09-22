# absorb ROUND (anchorc2, run 35749856755, branch round-416-anchorc2, head 5869010)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6736936 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6736936 (поллов=5); TPS_exp=2.22; normalized=-0.8%
- GC: young=114, Full=9, total=21.1s, avg=171ms, max=2499ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115218 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.56% (-0.61%) флэт
  - fluid: 16.72% -> 16.65% (-0.07%) флэт
  - broadphase: 15.66% -> 15.79% (+0.13%) флэт
  - nav_ai: 14.16% -> 14.56% (+0.40%) флэт
  - inside_volatile: 12.01% -> 12.23% (+0.23%) флэт
  - fastutil: 8.54% -> 8.77% (+0.23%) флэт
  - java_util: 7.01% -> 6.74% (-0.28%) флэт
  - paletted: 6.41% -> 6.43% (+0.02%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
