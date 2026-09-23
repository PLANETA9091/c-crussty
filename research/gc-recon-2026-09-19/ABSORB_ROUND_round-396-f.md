# absorb ROUND (396-f, run 35534936408, branch round-396-f-items_mono, head 3b6c418)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6558621 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6558621 (поллов=5); TPS_exp=2.18; normalized=+10.1%
- GC: young=111, Full=9, total=22.3s, avg=186ms, max=2563ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113434 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.16% (-1.01%) спад
  - fluid: 16.72% -> 17.48% (+0.76%) флэт
  - broadphase: 15.66% -> 15.13% (-0.53%) флэт
  - nav_ai: 14.16% -> 13.87% (-0.29%) флэт
  - inside_volatile: 12.01% -> 11.48% (-0.52%) флэт
  - fastutil: 8.54% -> 8.55% (+0.01%) флэт
  - java_util: 7.01% -> 6.88% (-0.14%) флэт
  - paletted: 6.41% -> 7.15% (+0.74%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
