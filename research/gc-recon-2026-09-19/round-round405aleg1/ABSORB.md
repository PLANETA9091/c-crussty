# absorb ROUND (round405aleg1, run 35649236693, branch round-405-a-l1, head 1aba03e)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6507187 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6507187 (поллов=6); TPS_exp=2.17; normalized=+6.0%
- GC: young=108, Full=9, total=21.8s, avg=187ms, max=2493ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112436 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.27% (-0.91%) флэт
  - fluid: 16.72% -> 16.89% (+0.17%) флэт
  - broadphase: 15.66% -> 15.91% (+0.26%) флэт
  - nav_ai: 14.16% -> 13.28% (-0.89%) флэт
  - inside_volatile: 12.01% -> 11.17% (-0.84%) флэт
  - fastutil: 8.54% -> 8.59% (+0.05%) флэт
  - java_util: 7.01% -> 8.23% (+1.22%) РОСТ
  - paletted: 6.41% -> 7.16% (+0.76%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
