# absorb ROUND (round-427-a2-anchorf, run 35851936925, branch round-427-a2-anchorf, head 790dc2f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6747081 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 6747081 (поллов=6); TPS_exp=2.22; normalized=+1.4%
- GC: young=109, Full=9, total=23.2s, avg=196ms, max=2676ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115723 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.28% (-1.89%) спад
  - fluid: 16.72% -> 16.82% (+0.11%) флэт
  - broadphase: 15.66% -> 14.83% (-0.83%) флэт
  - nav_ai: 14.16% -> 13.67% (-0.50%) флэт
  - inside_volatile: 12.01% -> 10.73% (-1.28%) спад
  - fastutil: 8.54% -> 8.59% (+0.05%) флэт
  - java_util: 7.01% -> 6.21% (-0.81%) флэт
  - paletted: 6.41% -> 6.84% (+0.43%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
