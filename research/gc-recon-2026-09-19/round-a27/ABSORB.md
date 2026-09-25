# absorb ROUND (a27, run 36108722769, branch round-455-anchor-27, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7044077 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 7044077 (поллов=6); TPS_exp=2.28; normalized=-1.4%
- GC: young=109, Full=9, total=21.2s, avg=180ms, max=2368ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116267 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.70% (-2.47%) спад
  - fluid: 16.72% -> 15.47% (-1.25%) спад
  - broadphase: 15.66% -> 14.83% (-0.82%) флэт
  - nav_ai: 14.16% -> 13.19% (-0.98%) флэт
  - inside_volatile: 12.01% -> 11.03% (-0.97%) флэт
  - fastutil: 8.54% -> 7.79% (-0.75%) флэт
  - java_util: 7.01% -> 6.56% (-0.45%) флэт
  - paletted: 6.41% -> 6.17% (-0.23%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
