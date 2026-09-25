# absorb ROUND (a31-457, run 36135877432, branch round-457-anchor-31, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6637931 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6637931 (поллов=6); TPS_exp=2.20; normalized=+0.1%
- GC: young=109, Full=9, total=22.1s, avg=187ms, max=2564ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116330 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.14% (-2.04%) спад
  - fluid: 16.72% -> 15.35% (-1.37%) спад
  - broadphase: 15.66% -> 15.23% (-0.43%) флэт
  - nav_ai: 14.16% -> 14.03% (-0.13%) флэт
  - inside_volatile: 12.01% -> 11.08% (-0.92%) флэт
  - fastutil: 8.54% -> 8.69% (+0.15%) флэт
  - java_util: 7.01% -> 6.40% (-0.61%) флэт
  - paletted: 6.41% -> 6.07% (-0.34%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
