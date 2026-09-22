# absorb ROUND (anchor419a, run 35770227563, branch round-419-anchora, head fbb06e3)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6841181 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6841181 (поллов=5); TPS_exp=2.24; normalized=+2.7%
- GC: young=113, Full=10, total=24.2s, avg=196ms, max=2500ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117107 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.55% (-1.62%) спад
  - fluid: 16.72% -> 15.80% (-0.91%) флэт
  - broadphase: 15.66% -> 15.11% (-0.55%) флэт
  - nav_ai: 14.16% -> 13.71% (-0.46%) флэт
  - inside_volatile: 12.01% -> 11.58% (-0.43%) флэт
  - fastutil: 8.54% -> 8.27% (-0.27%) флэт
  - java_util: 7.01% -> 6.90% (-0.11%) флэт
  - paletted: 6.41% -> 6.18% (-0.23%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
