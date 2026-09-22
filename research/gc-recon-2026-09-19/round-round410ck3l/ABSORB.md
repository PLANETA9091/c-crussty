# absorb ROUND (round410ck3l, run 35679297810, branch round-410-c-k3l, head 1b68a87)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7135980 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 7135980 (поллов=6); TPS_exp=2.30; normalized=-0.1%
- GC: young=107, Full=10, total=24.0s, avg=205ms, max=2668ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116376 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.74% (-1.43%) спад
  - fluid: 16.72% -> 16.11% (-0.61%) флэт
  - broadphase: 15.66% -> 14.76% (-0.90%) флэт
  - nav_ai: 14.16% -> 14.22% (+0.06%) флэт
  - inside_volatile: 12.01% -> 11.13% (-0.88%) флэт
  - fastutil: 8.54% -> 8.64% (+0.11%) флэт
  - java_util: 7.01% -> 6.55% (-0.46%) флэт
  - paletted: 6.41% -> 6.08% (-0.33%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
