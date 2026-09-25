# absorb ROUND (a11-457, run 36131690086, branch round-457-anchor-11, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6884499 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6884499 (поллов=5); TPS_exp=2.25; normalized=+2.3%
- GC: young=112, Full=9, total=20.7s, avg=171ms, max=2367ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116741 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.11% (-1.06%) спад
  - fluid: 16.72% -> 15.86% (-0.85%) флэт
  - broadphase: 15.66% -> 15.12% (-0.54%) флэт
  - nav_ai: 14.16% -> 13.67% (-0.49%) флэт
  - inside_volatile: 12.01% -> 11.84% (-0.16%) флэт
  - fastutil: 8.54% -> 8.16% (-0.38%) флэт
  - java_util: 7.01% -> 6.78% (-0.24%) флэт
  - paletted: 6.41% -> 6.40% (-0.00%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
