# absorb ROUND (a14, run 36104824658, branch round-455-anchor-14, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8580467 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 8580467 (поллов=5); TPS_exp=2.61; normalized=-0.2%
- GC: young=123, Full=10, total=22.3s, avg=168ms, max=2183ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=111962 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.69% (-2.49%) спад
  - fluid: 16.72% -> 16.88% (+0.16%) флэт
  - broadphase: 15.66% -> 15.25% (-0.40%) флэт
  - nav_ai: 14.16% -> 14.17% (+0.00%) флэт
  - inside_volatile: 12.01% -> 10.75% (-1.26%) спад
  - fastutil: 8.54% -> 8.85% (+0.31%) флэт
  - java_util: 7.01% -> 6.37% (-0.64%) флэт
  - paletted: 6.41% -> 7.06% (+0.65%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
