# absorb ROUND (anchor422ar, run 35809401962, branch round-422-anchorar, head 57a2e67)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6681241 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6681241 (поллов=6); TPS_exp=2.21; normalized=+8.8%
- GC: young=115, Full=9, total=20.7s, avg=167ms, max=2376ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117043 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.79% (-1.38%) спад
  - fluid: 16.72% -> 15.97% (-0.75%) флэт
  - broadphase: 15.66% -> 14.82% (-0.84%) флэт
  - nav_ai: 14.16% -> 13.42% (-0.74%) флэт
  - inside_volatile: 12.01% -> 11.82% (-0.19%) флэт
  - fastutil: 8.54% -> 8.43% (-0.11%) флэт
  - java_util: 7.01% -> 6.88% (-0.14%) флэт
  - paletted: 6.41% -> 6.17% (-0.23%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
