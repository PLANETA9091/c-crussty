# absorb ROUND (round-438-anchor-3, run 35941718749, branch round-438-anchor-3, head bcfb18f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6276479 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6276479 (поллов=5); TPS_exp=2.12; normalized=+3.7%
- GC: young=109, Full=9, total=22.7s, avg=193ms, max=2654ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115205 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.45% (-1.72%) спад
  - fluid: 16.72% -> 16.76% (+0.05%) флэт
  - broadphase: 15.66% -> 15.03% (-0.62%) флэт
  - nav_ai: 14.16% -> 13.80% (-0.36%) флэт
  - inside_volatile: 12.01% -> 10.85% (-1.15%) спад
  - fastutil: 8.54% -> 8.75% (+0.22%) флэт
  - java_util: 7.01% -> 6.44% (-0.57%) флэт
  - paletted: 6.41% -> 7.03% (+0.62%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
