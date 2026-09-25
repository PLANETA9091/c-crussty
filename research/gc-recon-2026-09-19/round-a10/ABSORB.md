# absorb ROUND (a10, run 36104789102, branch round-455-anchor-10, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6608247 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6608247 (поллов=5); TPS_exp=2.19; normalized=-4.1%
- GC: young=108, Full=9, total=21.7s, avg=186ms, max=2735ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117659 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.50% (-1.67%) спад
  - fluid: 16.72% -> 15.59% (-1.13%) спад
  - broadphase: 15.66% -> 15.50% (-0.16%) флэт
  - nav_ai: 14.16% -> 13.56% (-0.60%) флэт
  - inside_volatile: 12.01% -> 11.08% (-0.93%) флэт
  - fastutil: 8.54% -> 8.73% (+0.20%) флэт
  - java_util: 7.01% -> 7.03% (+0.02%) флэт
  - paletted: 6.41% -> 6.36% (-0.04%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
