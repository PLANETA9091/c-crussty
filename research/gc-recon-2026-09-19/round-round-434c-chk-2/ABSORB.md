# absorb ROUND (round-434c-chk-2, run 35917646721, branch round-434c-chk-2, head a17cde0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6652581 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6652581 (поллов=6); TPS_exp=2.20; normalized=+6.8%
- GC: young=103, Full=9, total=21.9s, avg=195ms, max=2944ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=104110 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.97% (+0.25%) флэт
  - broadphase: 15.66% -> 9.44% (-6.22%) спад
  - nav_ai: 14.16% -> 3.28% (-10.88%) спад
  - inside_volatile: 12.01% -> 15.61% (+3.60%) РОСТ
  - fastutil: 8.54% -> 6.67% (-1.87%) спад
  - java_util: 7.01% -> 8.93% (+1.92%) РОСТ
  - paletted: 6.41% -> 5.82% (-0.59%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
