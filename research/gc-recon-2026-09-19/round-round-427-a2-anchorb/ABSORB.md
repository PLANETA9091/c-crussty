# absorb ROUND (round-427-a2-anchorb, run 35851866015, branch round-427-a2-anchorb, head 790dc2f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8936480 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 8936480 (поллов=5); TPS_exp=2.68; normalized=-10.5%
- GC: young=111, Full=8, total=23.2s, avg=195ms, max=2770ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117449 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.69% (-1.48%) спад
  - fluid: 16.72% -> 17.48% (+0.76%) флэт
  - broadphase: 15.66% -> 14.60% (-1.05%) спад
  - nav_ai: 14.16% -> 13.39% (-0.78%) флэт
  - inside_volatile: 12.01% -> 11.59% (-0.41%) флэт
  - fastutil: 8.54% -> 8.67% (+0.13%) флэт
  - java_util: 7.01% -> 6.85% (-0.16%) флэт
  - paletted: 6.41% -> 7.42% (+1.01%) РОСТ
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **RED**
