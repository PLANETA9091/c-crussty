# absorb ROUND (a24, run 36108691375, branch round-455-anchor-24, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6601880 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6601880 (поллов=5); TPS_exp=2.19; normalized=+5.1%
- GC: young=108, Full=8, total=23.2s, avg=200ms, max=2565ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115580 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.87% (-2.30%) спад
  - fluid: 16.72% -> 16.44% (-0.27%) флэт
  - broadphase: 15.66% -> 14.78% (-0.88%) флэт
  - nav_ai: 14.16% -> 13.81% (-0.35%) флэт
  - inside_volatile: 12.01% -> 10.73% (-1.28%) спад
  - fastutil: 8.54% -> 8.64% (+0.10%) флэт
  - java_util: 7.01% -> 6.56% (-0.45%) флэт
  - paletted: 6.41% -> 7.00% (+0.60%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
