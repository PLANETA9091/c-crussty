# absorb ROUND (454-a8, run 36093457274, branch round-454-anchor-8, head ccfa6f0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6559473 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.00 @ 6559473 (поллов=5); TPS_exp=2.18; normalized=-8.3%
- GC: young=111, Full=9, total=21.4s, avg=179ms, max=2458ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117155 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.84% (-2.33%) спад
  - fluid: 16.72% -> 15.58% (-1.14%) спад
  - broadphase: 15.66% -> 15.21% (-0.45%) флэт
  - nav_ai: 14.16% -> 13.68% (-0.48%) флэт
  - inside_volatile: 12.01% -> 11.12% (-0.88%) флэт
  - fastutil: 8.54% -> 8.81% (+0.27%) флэт
  - java_util: 7.01% -> 6.35% (-0.66%) флэт
  - paletted: 6.41% -> 6.06% (-0.35%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
