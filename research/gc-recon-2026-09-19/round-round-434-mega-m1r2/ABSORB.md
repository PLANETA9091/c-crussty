# absorb ROUND (round-434-mega-m1r2, run 35914731471, branch round-434-mega-m1r2, head b15f329)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6917188 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.75 @ 6917188 (поллов=6); TPS_exp=2.26; normalized=+21.9%
- GC: young=103, Full=9, total=18.3s, avg=164ms, max=2404ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103458 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.72% (+0.00%) флэт
  - broadphase: 15.66% -> 10.13% (-5.53%) спад
  - nav_ai: 14.16% -> 3.45% (-10.71%) спад
  - inside_volatile: 12.01% -> 12.99% (+0.99%) флэт
  - fastutil: 8.54% -> 7.24% (-1.29%) спад
  - java_util: 7.01% -> 7.78% (+0.76%) флэт
  - paletted: 6.41% -> 5.64% (-0.77%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
