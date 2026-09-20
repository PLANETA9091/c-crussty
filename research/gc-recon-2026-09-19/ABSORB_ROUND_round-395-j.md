# absorb ROUND (395-j, run 35534620455, branch round-395-j-manager, head 4e8fc34)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6775348 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6775348 (поллов=5); TPS_exp=2.23; normalized=+3.3%
- GC: young=116, Full=9, total=21.9s, avg=175ms, max=2480ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113737 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.08% (-0.64%) флэт
  - broadphase: 15.66% -> 15.69% (+0.04%) флэт
  - nav_ai: 14.16% -> 14.56% (+0.39%) флэт
  - inside_volatile: 12.01% -> 12.22% (+0.22%) флэт
  - fastutil: 8.54% -> 8.88% (+0.34%) флэт
  - java_util: 7.01% -> 6.57% (-0.44%) флэт
  - paletted: 6.41% -> 6.61% (+0.21%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
