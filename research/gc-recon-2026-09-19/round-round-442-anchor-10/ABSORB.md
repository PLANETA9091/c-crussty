# absorb ROUND (round-442-anchor-10, run 35957137129, branch round-442-anchor-10, head 335f170)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6900573 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6900573 (поллов=5); TPS_exp=2.25; normalized=+2.1%
- GC: young=109, Full=10, total=22.9s, avg=193ms, max=2520ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117264 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.77% (-1.40%) спад
  - fluid: 16.72% -> 16.04% (-0.67%) флэт
  - broadphase: 15.66% -> 15.10% (-0.56%) флэт
  - nav_ai: 14.16% -> 13.55% (-0.61%) флэт
  - inside_volatile: 12.01% -> 11.59% (-0.41%) флэт
  - fastutil: 8.54% -> 8.54% (+0.01%) флэт
  - java_util: 7.01% -> 6.91% (-0.10%) флэт
  - paletted: 6.41% -> 6.26% (-0.15%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
