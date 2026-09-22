# absorb ROUND (round409eleg2b, run 35669097430, branch round-406-e-l2b, head b405fe1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6598051 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 6598051 (поллов=5); TPS_exp=2.19; normalized=+18.8%
- GC: young=112, Full=9, total=21.6s, avg=179ms, max=2495ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=111243 сэмплов (базлайн 115655)
  - items: 31.17% -> 33.40% (+2.23%) РОСТ
  - fluid: 16.72% -> 18.52% (+1.81%) РОСТ
  - broadphase: 15.66% -> 14.95% (-0.70%) флэт
  - nav_ai: 14.16% -> 8.51% (-5.65%) спад
  - inside_volatile: 12.01% -> 11.59% (-0.41%) флэт
  - fastutil: 8.54% -> 7.48% (-1.06%) спад
  - java_util: 7.01% -> 7.24% (+0.22%) флэт
  - paletted: 6.41% -> 7.24% (+0.83%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
