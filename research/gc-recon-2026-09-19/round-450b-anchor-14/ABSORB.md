# absorb ROUND (450b-anchor-14, run 36055294508, branch round-450-anchor-14, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8931072 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 8931072 (поллов=5); TPS_exp=2.68; normalized=-3.0%
- GC: young=120, Full=10, total=21.6s, avg=166ms, max=2122ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113407 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.92% (-2.25%) спад
  - fluid: 16.72% -> 17.36% (+0.64%) флэт
  - broadphase: 15.66% -> 15.28% (-0.38%) флэт
  - nav_ai: 14.16% -> 13.76% (-0.41%) флэт
  - inside_volatile: 12.01% -> 10.78% (-1.22%) спад
  - fastutil: 8.54% -> 8.85% (+0.31%) флэт
  - java_util: 7.01% -> 6.69% (-0.32%) флэт
  - paletted: 6.41% -> 6.76% (+0.35%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
