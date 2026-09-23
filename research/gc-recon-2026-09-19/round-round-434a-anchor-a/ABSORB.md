# absorb ROUND (round-434a-anchor-a, run 35917099549, branch round-434a-anchor-a, head 8141548)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7235732 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=1.90 @ 7235732 (поллов=5); TPS_exp=2.32; normalized=-18.2%
- GC: young=111, Full=9, total=21.5s, avg=179ms, max=2408ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116384 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.76% (-2.41%) спад
  - fluid: 16.72% -> 15.48% (-1.24%) спад
  - broadphase: 15.66% -> 15.08% (-0.58%) флэт
  - nav_ai: 14.16% -> 13.64% (-0.52%) флэт
  - inside_volatile: 12.01% -> 11.15% (-0.86%) флэт
  - fastutil: 8.54% -> 8.86% (+0.32%) флэт
  - java_util: 7.01% -> 6.34% (-0.68%) флэт
  - paletted: 6.41% -> 6.04% (-0.36%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
