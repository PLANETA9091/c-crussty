# absorb ROUND (mc2, run 35740281981, branch round-415-a-mc2, head e1771f1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6966222 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=5 -> **PASS**
- T3: median=1.00 @ 6966222 (поллов=4); TPS_exp=2.27; normalized=-55.9%
- GC: young=64, Full=8, total=12.5s, avg=174ms, max=2801ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=111311 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.38% (-1.79%) спад
  - fluid: 16.72% -> 14.81% (-1.90%) спад
  - broadphase: 15.66% -> 9.99% (-5.66%) спад
  - nav_ai: 14.16% -> 2.98% (-11.18%) спад
  - inside_volatile: 12.01% -> 9.89% (-2.11%) спад
  - fastutil: 8.54% -> 4.75% (-3.79%) спад
  - java_util: 7.01% -> 6.17% (-0.84%) флэт
  - paletted: 6.41% -> 5.32% (-1.09%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
