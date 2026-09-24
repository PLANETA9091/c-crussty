# absorb ROUND (round-434-anchor-w2, run 35919330196, branch round-434-anchor-w2, head 8141548)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6969567 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6969567 (поллов=6); TPS_exp=2.27; normalized=+3.7%
- GC: young=111, Full=10, total=24.4s, avg=202ms, max=2458ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117302 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.41% (-1.76%) спад
  - fluid: 16.72% -> 15.93% (-0.78%) флэт
  - broadphase: 15.66% -> 15.29% (-0.37%) флэт
  - nav_ai: 14.16% -> 13.50% (-0.66%) флэт
  - inside_volatile: 12.01% -> 11.35% (-0.66%) флэт
  - fastutil: 8.54% -> 8.67% (+0.13%) флэт
  - java_util: 7.01% -> 6.97% (-0.04%) флэт
  - paletted: 6.41% -> 6.30% (-0.10%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
