# absorb ROUND (round409anchorc, run 35666762102, branch round-408-anchorc, head 010a07e)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6846036 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6846036 (поллов=6); TPS_exp=2.24; normalized=+2.7%
- GC: young=115, Full=7, total=19.2s, avg=157ms, max=2410ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115380 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.59% (-0.58%) флэт
  - fluid: 16.72% -> 15.97% (-0.74%) флэт
  - broadphase: 15.66% -> 15.26% (-0.40%) флэт
  - nav_ai: 14.16% -> 13.97% (-0.20%) флэт
  - inside_volatile: 12.01% -> 12.35% (+0.34%) флэт
  - fastutil: 8.54% -> 8.85% (+0.31%) флэт
  - java_util: 7.01% -> 6.73% (-0.29%) флэт
  - paletted: 6.41% -> 6.19% (-0.22%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
