# absorb ROUND (brain421l4, run 35805621238, branch round-421-a-brain-l4, head 3e8c431)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6615418 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.80 @ 6615418 (поллов=5); TPS_exp=2.19; normalized=+27.7%
- GC: young=1118, Full=9, total=24.2s, avg=21ms, max=2198ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=108695 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 15.65% (-1.06%) спад
  - broadphase: 15.66% -> 14.65% (-1.00%) спад
  - nav_ai: 14.16% -> 9.06% (-5.10%) спад
  - inside_volatile: 12.01% -> 12.34% (+0.34%) флэт
  - fastutil: 8.54% -> 8.09% (-0.45%) флэт
  - java_util: 7.01% -> 7.79% (+0.78%) флэт
  - paletted: 6.41% -> 5.56% (-0.85%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
