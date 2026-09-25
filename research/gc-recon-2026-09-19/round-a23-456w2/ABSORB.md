# absorb ROUND (a23-456w2, run 36116748602, branch round-456-anchor-23, head f70402e)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6958213 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6958213 (поллов=6); TPS_exp=2.26; normalized=+6.0%
- GC: young=107, Full=6, total=14.6s, avg=130ms, max=826ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116728 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.38% (-0.79%) флэт
  - fluid: 16.72% -> 16.02% (-0.70%) флэт
  - broadphase: 15.66% -> 15.34% (-0.32%) флэт
  - nav_ai: 14.16% -> 14.13% (-0.03%) флэт
  - inside_volatile: 12.01% -> 11.70% (-0.30%) флэт
  - fastutil: 8.54% -> 8.56% (+0.02%) флэт
  - java_util: 7.01% -> 7.07% (+0.06%) флэт
  - paletted: 6.41% -> 6.15% (-0.26%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
