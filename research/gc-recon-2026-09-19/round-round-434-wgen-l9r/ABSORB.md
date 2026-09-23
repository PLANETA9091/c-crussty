# absorb ROUND (round-434-wgen-l9r, run 35914841683, branch round-434-wgen-l9r, head 898650c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6910221 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.75 @ 6910221 (поллов=6); TPS_exp=2.25; normalized=+22.0%
- GC: young=107, Full=9, total=19.2s, avg=165ms, max=2422ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=102649 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.02% (+0.31%) флэт
  - broadphase: 15.66% -> 10.52% (-5.14%) спад
  - nav_ai: 14.16% -> 3.34% (-10.83%) спад
  - inside_volatile: 12.01% -> 12.80% (+0.79%) флэт
  - fastutil: 8.54% -> 6.62% (-1.92%) спад
  - java_util: 7.01% -> 7.65% (+0.64%) флэт
  - paletted: 6.41% -> 5.71% (-0.69%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
