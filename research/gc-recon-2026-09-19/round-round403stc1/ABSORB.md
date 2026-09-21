# absorb ROUND (round403stc1, run 35604168413, branch round-403-stagcompa, head 60fe902)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8785618 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.90 @ 8785618 (поллов=5); TPS_exp=2.65; normalized=+9.5%
- GC: young=124, Full=9, total=19.0s, avg=143ms, max=1942ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=109543 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.59% (+0.88%) флэт
  - broadphase: 15.66% -> 13.77% (-1.89%) спад
  - nav_ai: 14.16% -> 8.60% (-5.56%) спад
  - inside_volatile: 12.01% -> 11.46% (-0.55%) флэт
  - fastutil: 8.54% -> 7.52% (-1.02%) спад
  - java_util: 7.01% -> 7.16% (+0.15%) флэт
  - paletted: 6.41% -> 6.74% (+0.33%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
