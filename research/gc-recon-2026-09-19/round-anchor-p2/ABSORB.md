# absorb ROUND (anchor-p2, run 35914745111, branch round-434-anchor-p2, head 8141548)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6781398 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6781398 (поллов=5); TPS_exp=2.23; normalized=-1.2%
- GC: young=112, Full=9, total=21.1s, avg=175ms, max=2350ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116434 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.48% (-1.69%) спад
  - fluid: 16.72% -> 16.18% (-0.54%) флэт
  - broadphase: 15.66% -> 14.53% (-1.12%) спад
  - nav_ai: 14.16% -> 13.61% (-0.55%) флэт
  - inside_volatile: 12.01% -> 11.58% (-0.43%) флэт
  - fastutil: 8.54% -> 8.19% (-0.35%) флэт
  - java_util: 7.01% -> 6.74% (-0.27%) флэт
  - paletted: 6.41% -> 6.30% (-0.10%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
