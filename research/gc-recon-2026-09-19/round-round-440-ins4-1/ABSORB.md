# absorb ROUND (round-440-ins4-1, run 35950559494, branch round-440-ins4-1, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8533964 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.55 @ 8533964 (поллов=6); TPS_exp=2.60; normalized=-1.8%
- GC: young=103, Full=9, total=23.0s, avg=206ms, max=2877ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=107234 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 18.04% (+1.32%) РОСТ
  - broadphase: 15.66% -> 10.06% (-5.59%) спад
  - nav_ai: 14.16% -> 3.96% (-10.20%) спад
  - inside_volatile: 12.01% -> 17.46% (+5.45%) РОСТ
  - fastutil: 8.54% -> 6.84% (-1.70%) спад
  - java_util: 7.01% -> 9.13% (+2.12%) РОСТ
  - paletted: 6.41% -> 6.64% (+0.23%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **RED**
