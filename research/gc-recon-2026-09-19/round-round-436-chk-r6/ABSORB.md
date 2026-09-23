# absorb ROUND (round-436-chk-r6, run 35930430974, branch round-436-chk-r6, head b64b189)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6720684 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6720684 (поллов=5); TPS_exp=2.21; normalized=+3.9%
- GC: young=101, Full=9, total=21.3s, avg=194ms, max=2856ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=103300 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.27% (+0.56%) флэт
  - broadphase: 15.66% -> 9.81% (-5.85%) спад
  - nav_ai: 14.16% -> 3.09% (-11.08%) спад
  - inside_volatile: 12.01% -> 15.65% (+3.65%) РОСТ
  - fastutil: 8.54% -> 6.85% (-1.69%) спад
  - java_util: 7.01% -> 8.60% (+1.59%) РОСТ
  - paletted: 6.41% -> 6.06% (-0.35%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
