# absorb ROUND (450-anchor-1, run 36050495535, branch round-450-anchor-1, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6692262 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6692262 (поллов=5); TPS_exp=2.21; normalized=-0.4%
- GC: young=111, Full=9, total=20.5s, avg=170ms, max=2410ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116158 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.08% (-1.09%) спад
  - fluid: 16.72% -> 15.79% (-0.92%) флэт
  - broadphase: 15.66% -> 15.20% (-0.45%) флэт
  - nav_ai: 14.16% -> 14.22% (+0.06%) флэт
  - inside_volatile: 12.01% -> 11.77% (-0.23%) флэт
  - fastutil: 8.54% -> 8.83% (+0.29%) флэт
  - java_util: 7.01% -> 7.10% (+0.08%) флэт
  - paletted: 6.41% -> 6.45% (+0.04%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
