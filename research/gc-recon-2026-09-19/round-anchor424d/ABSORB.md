# absorb ROUND (anchor424d, run 35829381716, branch round-424-anchord, head a8ef2fc)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6929838 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6929838 (поллов=6); TPS_exp=2.26; normalized=+6.3%
- GC: young=110, Full=9, total=19.3s, avg=162ms, max=2362ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116548 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.06% (-1.11%) спад
  - fluid: 16.72% -> 16.10% (-0.62%) флэт
  - broadphase: 15.66% -> 15.26% (-0.40%) флэт
  - nav_ai: 14.16% -> 14.06% (-0.11%) флэт
  - inside_volatile: 12.01% -> 11.48% (-0.52%) флэт
  - fastutil: 8.54% -> 8.69% (+0.15%) флэт
  - java_util: 7.01% -> 6.80% (-0.21%) флэт
  - paletted: 6.41% -> 6.33% (-0.08%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
