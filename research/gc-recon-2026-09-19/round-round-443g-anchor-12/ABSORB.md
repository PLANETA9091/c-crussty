# absorb ROUND (round-443g-anchor-12, run 36038772008, branch round-443g-anchor-12, head 9061566)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7064743 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.45 @ 7064743 (поллов=6); TPS_exp=2.29; normalized=+7.1%
- GC: young=119, Full=10, total=24.5s, avg=190ms, max=2390ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117009 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.50% (-1.67%) спад
  - fluid: 16.72% -> 16.06% (-0.65%) флэт
  - broadphase: 15.66% -> 15.49% (-0.17%) флэт
  - nav_ai: 14.16% -> 13.88% (-0.28%) флэт
  - inside_volatile: 12.01% -> 11.90% (-0.10%) флэт
  - fastutil: 8.54% -> 8.93% (+0.40%) флэт
  - java_util: 7.01% -> 6.93% (-0.08%) флэт
  - paletted: 6.41% -> 6.17% (-0.23%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
