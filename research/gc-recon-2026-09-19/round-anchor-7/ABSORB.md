# absorb ROUND (anchor-7, run 36093448843, branch round-454-anchor-7, head ccfa6f0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8901576 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.65 @ 8901576 (поллов=6); TPS_exp=2.67; normalized=-0.9%
- GC: young=125, Full=10, total=22.7s, avg=168ms, max=2282ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112641 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.89% (-2.28%) спад
  - fluid: 16.72% -> 16.39% (-0.32%) флэт
  - broadphase: 15.66% -> 15.14% (-0.51%) флэт
  - nav_ai: 14.16% -> 13.67% (-0.50%) флэт
  - inside_volatile: 12.01% -> 10.50% (-1.51%) спад
  - fastutil: 8.54% -> 8.48% (-0.05%) флэт
  - java_util: 7.01% -> 6.69% (-0.32%) флэт
  - paletted: 6.41% -> 6.82% (+0.42%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
