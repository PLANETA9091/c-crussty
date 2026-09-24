# absorb ROUND (round-438-anchor-2, run 35941700643, branch round-438-anchor-2, head bcfb18f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6895751 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6895751 (поллов=5); TPS_exp=2.25; normalized=-2.3%
- GC: young=111, Full=9, total=21.2s, avg=177ms, max=2410ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116402 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.24% (-0.93%) флэт
  - fluid: 16.72% -> 16.04% (-0.68%) флэт
  - broadphase: 15.66% -> 15.27% (-0.39%) флэт
  - nav_ai: 14.16% -> 13.96% (-0.20%) флэт
  - inside_volatile: 12.01% -> 11.82% (-0.18%) флэт
  - fastutil: 8.54% -> 8.83% (+0.29%) флэт
  - java_util: 7.01% -> 6.80% (-0.21%) флэт
  - paletted: 6.41% -> 6.18% (-0.23%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
