# absorb ROUND (round-428-a2-l6, run 35856003806, branch round-428-a2-l6, head fe4ee57)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6850777 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 6850777 (поллов=6); TPS_exp=2.24; normalized=+24.9%
- GC: young=111, Full=9, total=20.3s, avg=169ms, max=2866ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104132 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.88% (+0.16%) флэт
  - broadphase: 15.66% -> 10.30% (-5.36%) спад
  - nav_ai: 14.16% -> 3.44% (-10.73%) спад
  - inside_volatile: 12.01% -> 13.61% (+1.61%) РОСТ
  - fastutil: 8.54% -> 6.84% (-1.70%) спад
  - java_util: 7.01% -> 7.56% (+0.54%) флэт
  - paletted: 6.41% -> 5.50% (-0.91%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
