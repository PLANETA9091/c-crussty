# absorb ROUND (round410anchorb, run 35673618349, branch round-410-anchorb, head 8ef9e5b)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8880170 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.95 @ 8880170 (поллов=6); TPS_exp=2.67; normalized=+10.5%
- GC: young=127, Full=9, total=20.4s, avg=150ms, max=2136ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=111420 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.19% (-1.98%) спад
  - fluid: 16.72% -> 16.60% (-0.12%) флэт
  - broadphase: 15.66% -> 16.00% (+0.34%) флэт
  - nav_ai: 14.16% -> 13.83% (-0.33%) флэт
  - inside_volatile: 12.01% -> 10.94% (-1.07%) спад
  - fastutil: 8.54% -> 8.86% (+0.32%) флэт
  - java_util: 7.01% -> 6.80% (-0.22%) флэт
  - paletted: 6.41% -> 6.83% (+0.43%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
