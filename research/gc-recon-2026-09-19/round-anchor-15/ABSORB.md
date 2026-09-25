# absorb ROUND (anchor-15, run 36101831504, branch round-454-anchor-15, head ccfa6f0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6887003 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6887003 (поллов=6); TPS_exp=2.25; normalized=+4.5%
- GC: young=115, Full=10, total=24.8s, avg=199ms, max=2391ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117040 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.54% (-1.63%) спад
  - fluid: 16.72% -> 15.68% (-1.03%) спад
  - broadphase: 15.66% -> 15.28% (-0.37%) флэт
  - nav_ai: 14.16% -> 13.49% (-0.67%) флэт
  - inside_volatile: 12.01% -> 11.87% (-0.14%) флэт
  - fastutil: 8.54% -> 8.62% (+0.09%) флэт
  - java_util: 7.01% -> 6.52% (-0.50%) флэт
  - paletted: 6.41% -> 5.89% (-0.51%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
