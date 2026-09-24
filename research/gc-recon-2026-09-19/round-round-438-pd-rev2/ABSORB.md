# absorb ROUND (round-438-pd-rev2, run 35944215348, branch round-438-pd-rev2, head 4836276)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6661785 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6661785 (поллов=5); TPS_exp=2.20; normalized=-4.6%
- GC: young=107, Full=9, total=21.4s, avg=185ms, max=2433ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116901 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.54% (-1.63%) спад
  - fluid: 16.72% -> 16.16% (-0.56%) флэт
  - broadphase: 15.66% -> 15.50% (-0.16%) флэт
  - nav_ai: 14.16% -> 13.12% (-1.04%) спад
  - inside_volatile: 12.01% -> 10.92% (-1.09%) спад
  - fastutil: 8.54% -> 7.99% (-0.54%) флэт
  - java_util: 7.01% -> 6.34% (-0.68%) флэт
  - paletted: 6.41% -> 6.74% (+0.33%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
