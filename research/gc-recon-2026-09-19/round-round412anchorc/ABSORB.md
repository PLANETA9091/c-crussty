# absorb ROUND (round412anchorc, run 35704304263, branch round-412-anchorc, head 3bd07fe)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6435981 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6435981 (поллов=6); TPS_exp=2.15; normalized=+9.1%
- GC: young=110, Full=9, total=21.3s, avg=179ms, max=2674ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114697 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.87% (-0.30%) флэт
  - fluid: 16.72% -> 16.36% (-0.36%) флэт
  - broadphase: 15.66% -> 15.93% (+0.27%) флэт
  - nav_ai: 14.16% -> 14.28% (+0.11%) флэт
  - inside_volatile: 12.01% -> 11.89% (-0.12%) флэт
  - fastutil: 8.54% -> 9.05% (+0.52%) флэт
  - java_util: 7.01% -> 6.85% (-0.16%) флэт
  - paletted: 6.41% -> 6.37% (-0.03%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
