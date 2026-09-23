# absorb ROUND (mega421c, run 35806676546, branch round-421-mgc, head 99ffefa)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8727515 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 8727515 (поллов=5); TPS_exp=2.64; normalized=+2.4%
- GC: young=1127, Full=9, total=31.2s, avg=27ms, max=2680ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=108766 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.46% (+0.74%) флэт
  - broadphase: 15.66% -> 14.18% (-1.48%) спад
  - nav_ai: 14.16% -> 9.24% (-4.93%) спад
  - inside_volatile: 12.01% -> 11.88% (-0.13%) флэт
  - fastutil: 8.54% -> 7.56% (-0.98%) флэт
  - java_util: 7.01% -> 7.85% (+0.84%) флэт
  - paletted: 6.41% -> 6.59% (+0.19%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
