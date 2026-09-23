# absorb ROUND (agent-a-430anchor-c, run 35873139270, branch round-430-anchor-c, head 3dea992)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6877022 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.15 @ 6877022 (поллов=6); TPS_exp=2.25; normalized=-4.3%
- GC: young=106, Full=5, total=14.3s, avg=129ms, max=555ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115820 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.67% (-1.50%) спад
  - fluid: 16.72% -> 15.48% (-1.23%) спад
  - broadphase: 15.66% -> 16.24% (+0.58%) флэт
  - nav_ai: 14.16% -> 14.37% (+0.20%) флэт
  - inside_volatile: 12.01% -> 11.35% (-0.65%) флэт
  - fastutil: 8.54% -> 8.84% (+0.30%) флэт
  - java_util: 7.01% -> 7.00% (-0.01%) флэт
  - paletted: 6.41% -> 6.40% (-0.01%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
