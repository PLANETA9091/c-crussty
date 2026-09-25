# absorb ROUND (a31-456w3, run 36119665366, branch round-456-anchor-31, head d4deb33)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6740240 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6740240 (поллов=5); TPS_exp=2.22; normalized=+3.7%
- GC: young=108, Full=10, total=23.8s, avg=202ms, max=2696ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116601 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.84% (-1.33%) спад
  - fluid: 16.72% -> 15.59% (-1.13%) спад
  - broadphase: 15.66% -> 15.16% (-0.50%) флэт
  - nav_ai: 14.16% -> 13.83% (-0.33%) флэт
  - inside_volatile: 12.01% -> 11.41% (-0.59%) флэт
  - fastutil: 8.54% -> 9.05% (+0.51%) флэт
  - java_util: 7.01% -> 6.84% (-0.17%) флэт
  - paletted: 6.41% -> 6.13% (-0.28%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
