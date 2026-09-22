# absorb ROUND (anchor419c, run 35770245316, branch round-419-anchorc, head fbb06e3)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6971994 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6971994 (поллов=6); TPS_exp=2.27; normalized=+3.7%
- GC: young=112, Full=9, total=21.6s, avg=178ms, max=2629ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117252 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.50% (-1.67%) спад
  - fluid: 16.72% -> 15.53% (-1.19%) спад
  - broadphase: 15.66% -> 14.83% (-0.83%) флэт
  - nav_ai: 14.16% -> 13.64% (-0.52%) флэт
  - inside_volatile: 12.01% -> 11.19% (-0.82%) флэт
  - fastutil: 8.54% -> 8.52% (-0.02%) флэт
  - java_util: 7.01% -> 6.31% (-0.70%) флэт
  - paletted: 6.41% -> 5.81% (-0.60%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
