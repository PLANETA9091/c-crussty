# absorb ROUND (430a-anchor-c2, run 35876708604, branch round-430a-anchor-c2, head 3dea992)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6688321 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6688321 (поллов=5); TPS_exp=2.21; normalized=-0.3%
- GC: young=111, Full=10, total=24.3s, avg=201ms, max=2419ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116819 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.61% (-1.56%) спад
  - fluid: 16.72% -> 15.66% (-1.05%) спад
  - broadphase: 15.66% -> 15.41% (-0.24%) флэт
  - nav_ai: 14.16% -> 13.79% (-0.37%) флэт
  - inside_volatile: 12.01% -> 11.16% (-0.84%) флэт
  - fastutil: 8.54% -> 8.54% (+0.00%) флэт
  - java_util: 7.01% -> 6.82% (-0.20%) флэт
  - paletted: 6.41% -> 6.30% (-0.11%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
