# absorb ROUND (round-448-anchor-2, run 36015037486, branch round-448-anchor-2, head 97afb42)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6826709 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6826709 (поллов=5); TPS_exp=2.24; normalized=+2.8%
- GC: young=112, Full=9, total=20.6s, avg=170ms, max=2389ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116829 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.52% (-1.65%) спад
  - fluid: 16.72% -> 15.67% (-1.04%) спад
  - broadphase: 15.66% -> 15.42% (-0.24%) флэт
  - nav_ai: 14.16% -> 13.86% (-0.30%) флэт
  - inside_volatile: 12.01% -> 11.36% (-0.65%) флэт
  - fastutil: 8.54% -> 8.39% (-0.15%) флэт
  - java_util: 7.01% -> 6.73% (-0.28%) флэт
  - paletted: 6.41% -> 6.08% (-0.33%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
