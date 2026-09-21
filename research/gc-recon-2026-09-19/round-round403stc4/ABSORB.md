# absorb ROUND (round403stc4, run 35608298608, branch round-403-stcomp3, head 60fe902)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6811639 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 6811639 (поллов=6); TPS_exp=2.23; normalized=+20.9%
- GC: young=111, Full=9, total=21.3s, avg=177ms, max=2430ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=111111 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.48% (+0.76%) флэт
  - broadphase: 15.66% -> 13.81% (-1.85%) спад
  - nav_ai: 14.16% -> 8.82% (-5.34%) спад
  - inside_volatile: 12.01% -> 11.67% (-0.33%) флэт
  - fastutil: 8.54% -> 7.27% (-1.26%) спад
  - java_util: 7.01% -> 7.42% (+0.41%) флэт
  - paletted: 6.41% -> 7.32% (+0.91%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
