# absorb ROUND (ch420c, run 35790317317, branch round-420-c-chc, head 85f74b8)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6686983 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6686983 (поллов=5); TPS_exp=2.21; normalized=+4.2%
- GC: young=112, Full=9, total=23.7s, avg=196ms, max=2592ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115320 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.46% (-1.71%) спад
  - fluid: 16.72% -> 16.41% (-0.30%) флэт
  - broadphase: 15.66% -> 15.21% (-0.45%) флэт
  - nav_ai: 14.16% -> 13.74% (-0.42%) флэт
  - inside_volatile: 12.01% -> 11.08% (-0.92%) флэт
  - fastutil: 8.54% -> 8.84% (+0.30%) флэт
  - java_util: 7.01% -> 6.52% (-0.49%) флэт
  - paletted: 6.41% -> 7.13% (+0.72%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
