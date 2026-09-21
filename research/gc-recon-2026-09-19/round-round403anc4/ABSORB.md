# absorb ROUND (round403anc4, run 35604239745, branch round-403-anchor2, head f19d5f5)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6706928 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6706928 (поллов=5); TPS_exp=2.21; normalized=+4.0%
- GC: young=110, Full=9, total=19.7s, avg=166ms, max=2391ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115647 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.65% (-0.52%) флэт
  - fluid: 16.72% -> 16.42% (-0.30%) флэт
  - broadphase: 15.66% -> 15.55% (-0.11%) флэт
  - nav_ai: 14.16% -> 14.35% (+0.19%) флэт
  - inside_volatile: 12.01% -> 11.84% (-0.16%) флэт
  - fastutil: 8.54% -> 8.63% (+0.09%) флэт
  - java_util: 7.01% -> 6.93% (-0.08%) флэт
  - paletted: 6.41% -> 6.37% (-0.03%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
