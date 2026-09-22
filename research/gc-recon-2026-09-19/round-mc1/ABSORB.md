# absorb ROUND (mc1, run 35740258288, branch round-415-a-mc1, head e1771f1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7073190 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=0.95 @ 7073190 (поллов=6); TPS_exp=2.29; normalized=-58.5%
- GC: young=71, Full=7, total=10.8s, avg=139ms, max=1105ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=111694 сэмплов (базлайн 115655)
  - items: 31.17% -> 27.13% (-4.04%) спад
  - fluid: 16.72% -> 13.76% (-2.95%) спад
  - broadphase: 15.66% -> 9.02% (-6.64%) спад
  - nav_ai: 14.16% -> 2.74% (-11.43%) спад
  - inside_volatile: 12.01% -> 9.03% (-2.98%) спад
  - fastutil: 8.54% -> 4.43% (-4.11%) спад
  - java_util: 7.01% -> 6.17% (-0.84%) флэт
  - paletted: 6.41% -> 5.02% (-1.39%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
