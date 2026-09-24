# absorb ROUND (round-440-pd-3, run 35950618087, branch round-440-pd-3, head 4836276)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6839805 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.00 @ 6839805 (поллов=6); TPS_exp=2.24; normalized=-10.7%
- GC: young=113, Full=10, total=25.6s, avg=208ms, max=2514ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115618 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.71% (-1.46%) спад
  - fluid: 16.72% -> 15.97% (-0.74%) флэт
  - broadphase: 15.66% -> 16.06% (+0.41%) флэт
  - nav_ai: 14.16% -> 13.06% (-1.10%) спад
  - inside_volatile: 12.01% -> 11.05% (-0.95%) флэт
  - fastutil: 8.54% -> 8.49% (-0.05%) флэт
  - java_util: 7.01% -> 6.47% (-0.55%) флэт
  - paletted: 6.41% -> 6.44% (+0.03%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
