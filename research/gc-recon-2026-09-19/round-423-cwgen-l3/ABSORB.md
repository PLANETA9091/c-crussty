# absorb ROUND (round-423-cwgen-l1, run 35817976963, branch round-423-c-wgen-l3, head b582bc3)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8876555 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 8876555 (поллов=5); TPS_exp=2.67; normalized=+1.2%
- GC: young=127, Full=10, total=21.8s, avg=159ms, max=2113ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113932 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.86% (-2.32%) спад
  - fluid: 16.72% -> 16.55% (-0.16%) флэт
  - broadphase: 15.66% -> 14.69% (-0.96%) флэт
  - nav_ai: 14.16% -> 13.41% (-0.76%) флэт
  - inside_volatile: 12.01% -> 10.80% (-1.20%) спад
  - fastutil: 8.54% -> 8.40% (-0.14%) флэт
  - java_util: 7.01% -> 6.46% (-0.56%) флэт
  - paletted: 6.41% -> 6.78% (+0.38%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
