# absorb ROUND (mg420c, run 35795528708, branch round-420-mgc, head 77b22b7)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6391216 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.75 @ 6391216 (поллов=6); TPS_exp=2.14; normalized=+28.2%
- GC: young=1136, Full=9, total=31.2s, avg=27ms, max=2477ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107416 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.49% (+0.78%) флэт
  - broadphase: 15.66% -> 14.11% (-1.55%) спад
  - nav_ai: 14.16% -> 9.51% (-4.65%) спад
  - inside_volatile: 12.01% -> 12.07% (+0.06%) флэт
  - fastutil: 8.54% -> 8.40% (-0.14%) флэт
  - java_util: 7.01% -> 8.13% (+1.12%) РОСТ
  - paletted: 6.41% -> 6.55% (+0.14%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
