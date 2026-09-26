# absorb ROUND (anchor-599, run 36206859378, branch round-463-anchor-599, head 97f1c9c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6513537 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6513537 (поллов=5); TPS_exp=2.17; normalized=+1.4%
- GC: young=110, Full=10, total=23.9s, avg=200ms, max=2606ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117065 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.47% (-1.70%) спад
  - fluid: 16.72% -> 15.59% (-1.13%) спад
  - broadphase: 15.66% -> 14.99% (-0.67%) флэт
  - nav_ai: 14.16% -> 13.97% (-0.19%) флэт
  - inside_volatile: 12.01% -> 11.39% (-0.61%) флэт
  - fastutil: 8.54% -> 8.58% (+0.05%) флэт
  - java_util: 7.01% -> 6.59% (-0.42%) флэт
  - paletted: 6.41% -> 6.21% (-0.20%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
