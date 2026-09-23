# absorb ROUND (round-423-anchorb2, run 35818552334, branch round-423-anchorb2, head 4789ca7)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6700832 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6700832 (поллов=5); TPS_exp=2.21; normalized=+4.1%
- GC: young=113, Full=10, total=26.4s, avg=214ms, max=2611ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115841 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.07% (-2.10%) спад
  - fluid: 16.72% -> 16.46% (-0.26%) флэт
  - broadphase: 15.66% -> 14.41% (-1.25%) спад
  - nav_ai: 14.16% -> 13.44% (-0.72%) флэт
  - inside_volatile: 12.01% -> 10.87% (-1.13%) спад
  - fastutil: 8.54% -> 8.39% (-0.15%) флэт
  - java_util: 7.01% -> 6.41% (-0.60%) флэт
  - paletted: 6.41% -> 6.99% (+0.58%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
