# absorb ROUND (round409cleg2b, run 35667449721, branch round-405-c-l2b, head 36ea2a3)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6557502 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6557502 (поллов=5); TPS_exp=2.18; normalized=+5.5%
- GC: young=112, Full=9, total=22.2s, avg=184ms, max=2839ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116326 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.23% (-1.95%) спад
  - fluid: 16.72% -> 15.53% (-1.18%) спад
  - broadphase: 15.66% -> 15.29% (-0.37%) флэт
  - nav_ai: 14.16% -> 14.05% (-0.11%) флэт
  - inside_volatile: 12.01% -> 11.36% (-0.65%) флэт
  - fastutil: 8.54% -> 8.59% (+0.05%) флэт
  - java_util: 7.01% -> 6.60% (-0.41%) флэт
  - paletted: 6.41% -> 6.06% (-0.34%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
