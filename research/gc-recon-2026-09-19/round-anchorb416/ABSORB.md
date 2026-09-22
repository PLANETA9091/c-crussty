# absorb ROUND (anchorb416, run 35745492696, branch round-416-anchorb, head 5869010)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6944574 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6944574 (поллов=5); TPS_exp=2.26; normalized=-7.1%
- GC: young=112, Full=9, total=22.2s, avg=183ms, max=2651ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114777 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.78% (-0.39%) флэт
  - fluid: 16.72% -> 16.57% (-0.15%) флэт
  - broadphase: 15.66% -> 15.66% (+0.00%) флэт
  - nav_ai: 14.16% -> 14.51% (+0.35%) флэт
  - inside_volatile: 12.01% -> 11.91% (-0.10%) флэт
  - fastutil: 8.54% -> 8.96% (+0.42%) флэт
  - java_util: 7.01% -> 7.04% (+0.03%) флэт
  - paletted: 6.41% -> 6.24% (-0.16%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
