# absorb ROUND (round-449b-items-2, run 36023838241, branch round-449b-items-2, head a6e63cf)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7164217 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 7164217 (поллов=6); TPS_exp=2.31; normalized=-2.5%
- GC: young=107, Full=9, total=19.8s, avg=171ms, max=2381ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116969 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.07% (-1.10%) спад
  - fluid: 16.72% -> 15.93% (-0.79%) флэт
  - broadphase: 15.66% -> 14.77% (-0.88%) флэт
  - nav_ai: 14.16% -> 13.70% (-0.46%) флэт
  - inside_volatile: 12.01% -> 11.03% (-0.98%) флэт
  - fastutil: 8.54% -> 8.80% (+0.26%) флэт
  - java_util: 7.01% -> 6.37% (-0.64%) флэт
  - paletted: 6.41% -> 6.21% (-0.19%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
