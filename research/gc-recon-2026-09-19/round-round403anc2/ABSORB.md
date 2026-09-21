# absorb ROUND (round403anc2, run 35604155834, branch round-403-anchora, head f19d5f5)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7089030 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 7089030 (поллов=6); TPS_exp=2.29; normalized=+4.7%
- GC: young=112, Full=9, total=21.6s, avg=178ms, max=2454ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115090 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.06% (-1.11%) спад
  - fluid: 16.72% -> 16.07% (-0.65%) флэт
  - broadphase: 15.66% -> 15.61% (-0.05%) флэт
  - nav_ai: 14.16% -> 14.58% (+0.42%) флэт
  - inside_volatile: 12.01% -> 11.47% (-0.54%) флэт
  - fastutil: 8.54% -> 8.67% (+0.13%) флэт
  - java_util: 7.01% -> 6.85% (-0.16%) флэт
  - paletted: 6.41% -> 6.61% (+0.20%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
