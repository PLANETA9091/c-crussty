# absorb ROUND (round-434-wgen-l10r, run 35919369448, branch round-434-wgen-l10r, head 898650c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6836462 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 6836462 (поллов=6); TPS_exp=2.24; normalized=+25.1%
- GC: young=107, Full=9, total=21.3s, avg=183ms, max=2566ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103575 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.37% (+0.65%) флэт
  - broadphase: 15.66% -> 9.38% (-6.28%) спад
  - nav_ai: 14.16% -> 3.43% (-10.74%) спад
  - inside_volatile: 12.01% -> 12.16% (+0.15%) флэт
  - fastutil: 8.54% -> 6.90% (-1.64%) спад
  - java_util: 7.01% -> 7.42% (+0.41%) флэт
  - paletted: 6.41% -> 6.44% (+0.03%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
