# absorb ROUND (a32-457, run 36135889784, branch round-457-anchor-32, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6579335 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6579335 (поллов=5); TPS_exp=2.18; normalized=+0.7%
- GC: young=115, Full=9, total=22.8s, avg=184ms, max=2615ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115868 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.38% (-1.79%) спад
  - fluid: 16.72% -> 16.54% (-0.18%) флэт
  - broadphase: 15.66% -> 15.14% (-0.52%) флэт
  - nav_ai: 14.16% -> 13.59% (-0.57%) флэт
  - inside_volatile: 12.01% -> 11.06% (-0.95%) флэт
  - fastutil: 8.54% -> 8.62% (+0.08%) флэт
  - java_util: 7.01% -> 7.30% (+0.29%) флэт
  - paletted: 6.41% -> 7.11% (+0.70%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
