# absorb ROUND (anchor-529, run 36206794479, branch round-463-anchor-529, head 97f1c9c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6528747 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6528747 (поллов=5); TPS_exp=2.17; normalized=+1.2%
- GC: young=105, Full=7, total=15.7s, avg=140ms, max=1238ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116054 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.26% (-0.91%) флэт
  - fluid: 16.72% -> 15.98% (-0.74%) флэт
  - broadphase: 15.66% -> 15.96% (+0.30%) флэт
  - nav_ai: 14.16% -> 13.91% (-0.25%) флэт
  - inside_volatile: 12.01% -> 11.84% (-0.17%) флэт
  - fastutil: 8.54% -> 8.82% (+0.28%) флэт
  - java_util: 7.01% -> 6.74% (-0.27%) флэт
  - paletted: 6.41% -> 6.17% (-0.23%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
