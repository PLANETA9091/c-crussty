# absorb ROUND (round413anchorb, run 35715005958, branch round-413-anchorb, head dab32b4)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6978562 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6978562 (поллов=6); TPS_exp=2.27; normalized=+5.8%
- GC: young=111, Full=9, total=21.3s, avg=178ms, max=2447ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115249 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.28% (-0.89%) флэт
  - fluid: 16.72% -> 16.24% (-0.48%) флэт
  - broadphase: 15.66% -> 15.65% (-0.01%) флэт
  - nav_ai: 14.16% -> 14.27% (+0.10%) флэт
  - inside_volatile: 12.01% -> 11.81% (-0.19%) флэт
  - fastutil: 8.54% -> 8.96% (+0.43%) флэт
  - java_util: 7.01% -> 6.85% (-0.16%) флэт
  - paletted: 6.41% -> 6.49% (+0.09%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
