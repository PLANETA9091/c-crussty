# absorb ROUND (round-440-anchor-3, run 35950575468, branch round-440-anchor-3, head 6db49fb)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8460119 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 8460119 (поллов=6); TPS_exp=2.58; normalized=-14.8%
- GC: young=107, Full=10, total=27.1s, avg=232ms, max=2902ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116254 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.34% (-1.83%) спад
  - fluid: 16.72% -> 17.59% (+0.88%) флэт
  - broadphase: 15.66% -> 15.50% (-0.15%) флэт
  - nav_ai: 14.16% -> 13.37% (-0.80%) флэт
  - inside_volatile: 12.01% -> 10.87% (-1.14%) спад
  - fastutil: 8.54% -> 8.87% (+0.33%) флэт
  - java_util: 7.01% -> 6.76% (-0.25%) флэт
  - paletted: 6.41% -> 7.27% (+0.87%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **RED**
