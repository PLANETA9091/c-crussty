# absorb ROUND (436c-chk3-2, run 35930849659, branch round-436c-chk3-2, head fb82607)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8787612 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=3.10 @ 8787612 (поллов=5); TPS_exp=2.65; normalized=+17.0%
- GC: young=128, Full=8, total=16.1s, avg=119ms, max=1249ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=102318 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.03% (-0.68%) флэт
  - broadphase: 15.66% -> 9.26% (-6.40%) спад
  - nav_ai: 14.16% -> 3.18% (-10.98%) спад
  - inside_volatile: 12.01% -> 15.49% (+3.49%) РОСТ
  - fastutil: 8.54% -> 6.12% (-2.42%) спад
  - java_util: 7.01% -> 8.31% (+1.30%) РОСТ
  - paletted: 6.41% -> 5.86% (-0.55%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
