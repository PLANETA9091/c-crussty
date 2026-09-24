# absorb ROUND (450b-anchor-15, run 36055311286, branch round-450-anchor-15, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6923569 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6923569 (поллов=6); TPS_exp=2.26; normalized=+1.9%
- GC: young=108, Full=9, total=19.8s, avg=169ms, max=2446ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116560 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.04% (-1.13%) спад
  - fluid: 16.72% -> 16.04% (-0.67%) флэт
  - broadphase: 15.66% -> 15.36% (-0.29%) флэт
  - nav_ai: 14.16% -> 14.06% (-0.10%) флэт
  - inside_volatile: 12.01% -> 11.63% (-0.37%) флэт
  - fastutil: 8.54% -> 8.92% (+0.39%) флэт
  - java_util: 7.01% -> 7.01% (-0.00%) флэт
  - paletted: 6.41% -> 6.15% (-0.25%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
