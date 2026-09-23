# absorb ROUND (396-f-confirm, run 35539017250, branch round-396-f-items_mono, head 36988be)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7101557 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 7101557 (поллов=6); TPS_exp=2.29; normalized=+0.2%
- GC: young=116, Full=9, total=21.3s, avg=170ms, max=2440ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115230 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.64% (-0.53%) флэт
  - fluid: 16.72% -> 16.77% (+0.05%) флэт
  - broadphase: 15.66% -> 15.88% (+0.23%) флэт
  - nav_ai: 14.16% -> 14.66% (+0.50%) флэт
  - inside_volatile: 12.01% -> 11.99% (-0.01%) флэт
  - fastutil: 8.54% -> 9.60% (+1.07%) РОСТ
  - java_util: 7.01% -> 6.48% (-0.53%) флэт
  - paletted: 6.41% -> 6.35% (-0.06%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
