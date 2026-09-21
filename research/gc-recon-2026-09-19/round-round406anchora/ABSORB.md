# absorb ROUND (round406anchora, run 35640571735, branch round-406-anchora, head ca5e1a4)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6992836 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6992836 (поллов=6); TPS_exp=2.27; normalized=+3.5%
- GC: young=111, Full=9, total=21.2s, avg=177ms, max=2500ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115171 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.46% (-0.71%) флэт
  - fluid: 16.72% -> 16.34% (-0.37%) флэт
  - broadphase: 15.66% -> 16.05% (+0.39%) флэт
  - nav_ai: 14.16% -> 15.03% (+0.86%) флэт
  - inside_volatile: 12.01% -> 11.70% (-0.31%) флэт
  - fastutil: 8.54% -> 9.44% (+0.91%) флэт
  - java_util: 7.01% -> 6.85% (-0.17%) флэт
  - paletted: 6.41% -> 6.37% (-0.03%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
