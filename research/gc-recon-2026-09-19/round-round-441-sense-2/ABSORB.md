# absorb ROUND (round-441-sense-2, run 35954926328, branch round-441-sense-2, head b6134c6)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6798089 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6798089 (поллов=5); TPS_exp=2.23; normalized=+3.1%
- GC: young=113, Full=9, total=19.7s, avg=161ms, max=2340ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112171 сэмплов (базлайн 115655)
  - items: 31.17% -> 35.14% (+3.97%) РОСТ
  - fluid: 16.72% -> 16.98% (+0.26%) флэт
  - broadphase: 15.66% -> 15.70% (+0.04%) флэт
  - nav_ai: 14.16% -> 8.00% (-6.16%) спад
  - inside_volatile: 12.01% -> 14.30% (+2.29%) РОСТ
  - fastutil: 8.54% -> 7.29% (-1.25%) спад
  - java_util: 7.01% -> 8.09% (+1.08%) РОСТ
  - paletted: 6.41% -> 5.89% (-0.51%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
