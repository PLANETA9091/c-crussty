# absorb ROUND (round-441-anchor-6, run 35954846118, branch round-441-anchor-6, head 6db49fb)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6815983 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.90 @ 6815983 (поллов=5); TPS_exp=2.23; normalized=-15.0%
- GC: young=110, Full=9, total=20.7s, avg=174ms, max=2429ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116632 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.87% (-2.30%) спад
  - fluid: 16.72% -> 15.58% (-1.13%) спад
  - broadphase: 15.66% -> 14.91% (-0.75%) флэт
  - nav_ai: 14.16% -> 14.09% (-0.07%) флэт
  - inside_volatile: 12.01% -> 11.26% (-0.75%) флэт
  - fastutil: 8.54% -> 8.54% (+0.01%) флэт
  - java_util: 7.01% -> 7.07% (+0.06%) флэт
  - paletted: 6.41% -> 5.85% (-0.56%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
