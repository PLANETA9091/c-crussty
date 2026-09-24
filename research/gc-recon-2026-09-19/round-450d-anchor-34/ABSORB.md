# absorb ROUND (450d-anchor-34, run 36063373379, branch round-450-anchor-34, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6754473 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6754473 (поллов=6); TPS_exp=2.22; normalized=+3.5%
- GC: young=109, Full=9, total=23.2s, avg=197ms, max=2545ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115127 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.45% (-1.72%) спад
  - fluid: 16.72% -> 16.51% (-0.21%) флэт
  - broadphase: 15.66% -> 14.82% (-0.84%) флэт
  - nav_ai: 14.16% -> 13.56% (-0.60%) флэт
  - inside_volatile: 12.01% -> 10.85% (-1.15%) спад
  - fastutil: 8.54% -> 8.63% (+0.10%) флэт
  - java_util: 7.01% -> 6.36% (-0.65%) флэт
  - paletted: 6.41% -> 7.01% (+0.61%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
