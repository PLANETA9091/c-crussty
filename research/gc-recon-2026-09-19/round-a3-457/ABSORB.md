# absorb ROUND (a3-457, run 36131596365, branch round-457-anchor-3, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6977973 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6977973 (поллов=6); TPS_exp=2.27; normalized=+3.6%
- GC: young=111, Full=10, total=24.1s, avg=199ms, max=2486ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117010 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.08% (-1.09%) спад
  - fluid: 16.72% -> 15.45% (-1.27%) спад
  - broadphase: 15.66% -> 15.25% (-0.41%) флэт
  - nav_ai: 14.16% -> 13.72% (-0.44%) флэт
  - inside_volatile: 12.01% -> 11.61% (-0.40%) флэт
  - fastutil: 8.54% -> 8.74% (+0.20%) флэт
  - java_util: 7.01% -> 6.49% (-0.52%) флэт
  - paletted: 6.41% -> 5.97% (-0.44%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
