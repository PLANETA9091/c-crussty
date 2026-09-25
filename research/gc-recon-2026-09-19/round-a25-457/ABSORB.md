# absorb ROUND (a25-457, run 36135804562, branch round-457-anchor-25, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7060169 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 7060169 (поллов=5); TPS_exp=2.29; normalized=+5.0%
- GC: young=114, Full=9, total=21.1s, avg=172ms, max=2560ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116751 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.54% (-1.63%) спад
  - fluid: 16.72% -> 15.57% (-1.14%) спад
  - broadphase: 15.66% -> 14.75% (-0.91%) флэт
  - nav_ai: 14.16% -> 13.64% (-0.52%) флэт
  - inside_volatile: 12.01% -> 12.14% (+0.13%) флэт
  - fastutil: 8.54% -> 8.99% (+0.45%) флэт
  - java_util: 7.01% -> 7.11% (+0.10%) флэт
  - paletted: 6.41% -> 6.18% (-0.23%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
