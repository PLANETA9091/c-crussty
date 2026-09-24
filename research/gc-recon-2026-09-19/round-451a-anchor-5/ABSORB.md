# absorb ROUND (451a-anchor-5, run 36069468667, branch round-451-anchor-5, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7124924 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 7124924 (поллов=5); TPS_exp=2.30; normalized=+0.0%
- GC: young=111, Full=9, total=20.3s, avg=169ms, max=2340ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117120 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.68% (-1.49%) спад
  - fluid: 16.72% -> 15.70% (-1.01%) спад
  - broadphase: 15.66% -> 15.00% (-0.66%) флэт
  - nav_ai: 14.16% -> 13.75% (-0.41%) флэт
  - inside_volatile: 12.01% -> 11.84% (-0.17%) флэт
  - fastutil: 8.54% -> 8.69% (+0.16%) флэт
  - java_util: 7.01% -> 7.02% (+0.01%) флэт
  - paletted: 6.41% -> 6.21% (-0.20%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
