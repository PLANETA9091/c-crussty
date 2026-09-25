# absorb ROUND (451b-anchor-20, run 36073651804, branch round-451-anchor-20, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6809679 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6809679 (поллов=6); TPS_exp=2.23; normalized=+5.2%
- GC: young=106, Full=8, total=20.6s, avg=181ms, max=2406ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117157 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.10% (-1.07%) спад
  - fluid: 16.72% -> 15.77% (-0.94%) флэт
  - broadphase: 15.66% -> 15.13% (-0.52%) флэт
  - nav_ai: 14.16% -> 14.13% (-0.03%) флэт
  - inside_volatile: 12.01% -> 11.42% (-0.58%) флэт
  - fastutil: 8.54% -> 8.52% (-0.02%) флэт
  - java_util: 7.01% -> 6.62% (-0.39%) флэт
  - paletted: 6.41% -> 6.42% (+0.02%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
