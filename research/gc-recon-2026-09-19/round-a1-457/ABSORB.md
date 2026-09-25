# absorb ROUND (a1-457, run 36131572543, branch round-457-anchor-1, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6919577 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6919577 (поллов=5); TPS_exp=2.26; normalized=-6.9%
- GC: young=105, Full=10, total=23.7s, avg=206ms, max=2587ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116374 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.94% (-1.23%) спад
  - fluid: 16.72% -> 15.99% (-0.73%) флэт
  - broadphase: 15.66% -> 15.61% (-0.05%) флэт
  - nav_ai: 14.16% -> 13.58% (-0.58%) флэт
  - inside_volatile: 12.01% -> 11.44% (-0.56%) флэт
  - fastutil: 8.54% -> 8.74% (+0.21%) флэт
  - java_util: 7.01% -> 6.81% (-0.20%) флэт
  - paletted: 6.41% -> 6.28% (-0.13%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
