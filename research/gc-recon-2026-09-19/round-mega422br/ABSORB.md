# absorb ROUND (mega422br, run 35809415663, branch round-422-mgbr, head 99ffefa)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6520365 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6520365 (поллов=5); TPS_exp=2.17; normalized=+1.3%
- GC: young=1121, Full=9, total=25.0s, avg=22ms, max=2226ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=108266 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.83% (-0.88%) флэт
  - broadphase: 15.66% -> 14.29% (-1.37%) спад
  - nav_ai: 14.16% -> 9.22% (-4.95%) спад
  - inside_volatile: 12.01% -> 12.34% (+0.34%) флэт
  - fastutil: 8.54% -> 8.33% (-0.21%) флэт
  - java_util: 7.01% -> 8.03% (+1.02%) РОСТ
  - paletted: 6.41% -> 5.38% (-1.03%) спад
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
