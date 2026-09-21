# absorb ROUND (round403cl3, run 35612388192, branch round-403-c-leg3, head 67d5225)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6960776 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6960776 (поллов=5); TPS_exp=2.26; normalized=+6.0%
- GC: young=107, Full=9, total=20.0s, avg=172ms, max=2322ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113275 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.51% (-0.20%) флэт
  - broadphase: 15.66% -> 14.88% (-0.77%) флэт
  - nav_ai: 14.16% -> 8.73% (-5.43%) спад
  - inside_volatile: 12.01% -> 12.13% (+0.12%) флэт
  - fastutil: 8.54% -> 7.54% (-1.00%) флэт
  - java_util: 7.01% -> 6.97% (-0.04%) флэт
  - paletted: 6.41% -> 6.26% (-0.14%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
