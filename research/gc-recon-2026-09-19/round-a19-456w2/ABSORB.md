# absorb ROUND (a19-456w2, run 36116702563, branch round-456-anchor-19, head f70402e)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6803823 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6803823 (поллов=6); TPS_exp=2.23; normalized=+7.5%
- GC: young=111, Full=10, total=24.1s, avg=199ms, max=2469ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116439 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.28% (-1.89%) спад
  - fluid: 16.72% -> 15.74% (-0.98%) флэт
  - broadphase: 15.66% -> 15.42% (-0.24%) флэт
  - nav_ai: 14.16% -> 13.73% (-0.44%) флэт
  - inside_volatile: 12.01% -> 11.41% (-0.60%) флэт
  - fastutil: 8.54% -> 8.66% (+0.12%) флэт
  - java_util: 7.01% -> 6.84% (-0.17%) флэт
  - paletted: 6.41% -> 6.06% (-0.35%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
