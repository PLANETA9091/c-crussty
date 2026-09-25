# absorb ROUND (a28-457, run 36135841083, branch round-457-anchor-28, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6982027 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 6982027 (поллов=6); TPS_exp=2.27; normalized=-0.8%
- GC: young=109, Full=10, total=24.3s, avg=204ms, max=2477ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116769 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.47% (-1.70%) спад
  - fluid: 16.72% -> 15.50% (-1.22%) спад
  - broadphase: 15.66% -> 15.34% (-0.32%) флэт
  - nav_ai: 14.16% -> 14.31% (+0.15%) флэт
  - inside_volatile: 12.01% -> 11.07% (-0.94%) флэт
  - fastutil: 8.54% -> 8.99% (+0.45%) флэт
  - java_util: 7.01% -> 6.34% (-0.67%) флэт
  - paletted: 6.41% -> 6.14% (-0.26%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
