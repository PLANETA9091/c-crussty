# absorb ROUND (451b-anchor-24, run 36073689329, branch round-451-anchor-24, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6867285 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6867285 (поллов=6); TPS_exp=2.25; normalized=+4.7%
- GC: young=107, Full=10, total=22.7s, avg=194ms, max=2428ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116208 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.84% (-1.33%) спад
  - fluid: 16.72% -> 15.89% (-0.82%) флэт
  - broadphase: 15.66% -> 15.31% (-0.35%) флэт
  - nav_ai: 14.16% -> 14.06% (-0.11%) флэт
  - inside_volatile: 12.01% -> 11.56% (-0.45%) флэт
  - fastutil: 8.54% -> 8.86% (+0.33%) флэт
  - java_util: 7.01% -> 6.68% (-0.33%) флэт
  - paletted: 6.41% -> 6.18% (-0.22%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
