# absorb ROUND (round-438-chk3-1, run 35941747880, branch round-438-chk3-1, head 7afe6d1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6856573 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6856573 (поллов=5); TPS_exp=2.24; normalized=-6.4%
- GC: young=107, Full=9, total=18.9s, avg=163ms, max=2446ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105274 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.71% (-1.01%) спад
  - broadphase: 15.66% -> 9.68% (-5.98%) спад
  - nav_ai: 14.16% -> 3.21% (-10.96%) спад
  - inside_volatile: 12.01% -> 15.42% (+3.41%) РОСТ
  - fastutil: 8.54% -> 6.49% (-2.05%) спад
  - java_util: 7.01% -> 8.85% (+1.84%) РОСТ
  - paletted: 6.41% -> 5.13% (-1.27%) спад
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
