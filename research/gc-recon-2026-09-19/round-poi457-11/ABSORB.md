# absorb ROUND (poi457-11, run 36144116531, branch round-456b-poi-11, head 5ecd841)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6953086 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6953086 (поллов=6); TPS_exp=2.26; normalized=+3.8%
- GC: young=107, Full=9, total=20.1s, avg=173ms, max=2689ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104147 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.37% (-0.35%) флэт
  - broadphase: 15.66% -> 9.57% (-6.08%) спад
  - nav_ai: 14.16% -> 3.21% (-10.95%) спад
  - inside_volatile: 12.01% -> 16.91% (+4.90%) РОСТ
  - fastutil: 8.54% -> 6.36% (-2.17%) спад
  - java_util: 7.01% -> 8.57% (+1.56%) РОСТ
  - paletted: 6.41% -> 5.08% (-1.32%) спад
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **PARITY/LOW**
