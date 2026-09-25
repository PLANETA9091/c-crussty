# absorb ROUND (anchor-4, run 36093423348, branch round-454-anchor-4, head ccfa6f0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7059636 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 7059636 (поллов=5); TPS_exp=2.29; normalized=-8.1%
- GC: young=108, Full=9, total=21.7s, avg=186ms, max=2791ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116550 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.35% (-1.82%) спад
  - fluid: 16.72% -> 15.37% (-1.35%) спад
  - broadphase: 15.66% -> 15.19% (-0.46%) флэт
  - nav_ai: 14.16% -> 13.78% (-0.38%) флэт
  - inside_volatile: 12.01% -> 11.16% (-0.84%) флэт
  - fastutil: 8.54% -> 8.47% (-0.06%) флэт
  - java_util: 7.01% -> 6.45% (-0.56%) флэт
  - paletted: 6.41% -> 5.94% (-0.46%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
