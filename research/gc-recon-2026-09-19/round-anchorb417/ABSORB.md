# absorb ROUND (anchorb417, run 35759265422, branch round-417-anchorb, head 87fc0cb)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6960975 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6960975 (поллов=6); TPS_exp=2.26; normalized=+3.8%
- GC: young=114, Full=9, total=21.3s, avg=174ms, max=2382ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114758 сэмплов (базлайн 115655)
  - items: 31.17% -> 31.00% (-0.18%) флэт
  - fluid: 16.72% -> 16.61% (-0.11%) флэт
  - broadphase: 15.66% -> 15.42% (-0.24%) флэт
  - nav_ai: 14.16% -> 14.43% (+0.26%) флэт
  - inside_volatile: 12.01% -> 12.09% (+0.08%) флэт
  - fastutil: 8.54% -> 8.85% (+0.31%) флэт
  - java_util: 7.01% -> 6.91% (-0.11%) флэт
  - paletted: 6.41% -> 6.43% (+0.02%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
