# absorb ROUND (cvs1r, run 35729389765, branch round-414-b-cvs1, head 1aec4f8)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6655252 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 6655252 (поллов=5); TPS_exp=2.20; normalized=+27.3%
- GC: young=1129, Full=9, total=27.2s, avg=24ms, max=2272ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107374 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.78% (+0.07%) флэт
  - broadphase: 15.66% -> 13.53% (-2.12%) спад
  - nav_ai: 14.16% -> 9.14% (-5.02%) спад
  - inside_volatile: 12.01% -> 11.74% (-0.27%) флэт
  - fastutil: 8.54% -> 7.69% (-0.85%) флэт
  - java_util: 7.01% -> 8.16% (+1.15%) РОСТ
  - paletted: 6.41% -> 6.37% (-0.04%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
