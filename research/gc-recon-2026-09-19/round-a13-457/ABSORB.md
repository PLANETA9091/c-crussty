# absorb ROUND (a13-457, run 36131709399, branch round-457-anchor-13, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6887097 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6887097 (поллов=5); TPS_exp=2.25; normalized=-2.2%
- GC: young=115, Full=9, total=21.3s, avg=172ms, max=2365ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116794 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.94% (-1.23%) спад
  - fluid: 16.72% -> 16.05% (-0.67%) флэт
  - broadphase: 15.66% -> 14.85% (-0.80%) флэт
  - nav_ai: 14.16% -> 13.77% (-0.39%) флэт
  - inside_volatile: 12.01% -> 11.77% (-0.24%) флэт
  - fastutil: 8.54% -> 8.67% (+0.13%) флэт
  - java_util: 7.01% -> 6.81% (-0.20%) флэт
  - paletted: 6.41% -> 6.34% (-0.06%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
