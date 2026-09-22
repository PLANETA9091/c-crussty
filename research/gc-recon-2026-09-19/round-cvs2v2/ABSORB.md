# absorb ROUND (cvs2v2, run 35728198347, branch round-414-b-cvs2, head 1aec4f8)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6593164 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 6593164 (поллов=6); TPS_exp=2.19; normalized=+16.6%
- GC: young=1131, Full=9, total=26.5s, avg=23ms, max=2363ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107858 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.69% (-0.02%) флэт
  - broadphase: 15.66% -> 14.04% (-1.61%) спад
  - nav_ai: 14.16% -> 10.57% (-3.59%) спад
  - inside_volatile: 12.01% -> 11.24% (-0.76%) флэт
  - fastutil: 8.54% -> 8.29% (-0.25%) флэт
  - java_util: 7.01% -> 7.76% (+0.75%) флэт
  - paletted: 6.41% -> 6.28% (-0.12%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
