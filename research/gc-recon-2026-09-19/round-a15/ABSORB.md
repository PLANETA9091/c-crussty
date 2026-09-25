# absorb ROUND (a15, run 36104833559, branch round-455-anchor-15, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8914125 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 8914125 (поллов=5); TPS_exp=2.68; normalized=+4.6%
- GC: young=127, Full=10, total=21.5s, avg=157ms, max=1987ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113208 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.42% (-2.75%) спад
  - fluid: 16.72% -> 16.43% (-0.29%) флэт
  - broadphase: 15.66% -> 14.63% (-1.02%) спад
  - nav_ai: 14.16% -> 13.75% (-0.41%) флэт
  - inside_volatile: 12.01% -> 10.96% (-1.05%) спад
  - fastutil: 8.54% -> 8.76% (+0.22%) флэт
  - java_util: 7.01% -> 6.60% (-0.41%) флэт
  - paletted: 6.41% -> 6.90% (+0.49%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
