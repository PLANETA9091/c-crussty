# absorb ROUND (round-435-anchor-u3, run 35924492649, branch round-435-anchor-u3, head 536cf06)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6895840 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6895840 (поллов=6); TPS_exp=2.25; normalized=+2.2%
- GC: young=108, Full=9, total=19.6s, avg=168ms, max=2402ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116785 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.88% (-1.29%) спад
  - fluid: 16.72% -> 16.00% (-0.71%) флэт
  - broadphase: 15.66% -> 15.06% (-0.59%) флэт
  - nav_ai: 14.16% -> 13.61% (-0.55%) флэт
  - inside_volatile: 12.01% -> 11.47% (-0.53%) флэт
  - fastutil: 8.54% -> 8.92% (+0.38%) флэт
  - java_util: 7.01% -> 7.09% (+0.08%) флэт
  - paletted: 6.41% -> 6.24% (-0.17%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
