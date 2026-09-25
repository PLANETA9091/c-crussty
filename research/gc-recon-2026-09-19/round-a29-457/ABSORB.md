# absorb ROUND (a29-457, run 36135852144, branch round-457-anchor-29, head 1838ae1)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6916268 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6916268 (поллов=5); TPS_exp=2.26; normalized=-6.9%
- GC: young=109, Full=9, total=21.7s, avg=184ms, max=2613ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116329 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.93% (-1.24%) спад
  - fluid: 16.72% -> 16.08% (-0.64%) флэт
  - broadphase: 15.66% -> 14.93% (-0.73%) флэт
  - nav_ai: 14.16% -> 13.41% (-0.75%) флэт
  - inside_volatile: 12.01% -> 11.06% (-0.95%) флэт
  - fastutil: 8.54% -> 8.41% (-0.12%) флэт
  - java_util: 7.01% -> 6.49% (-0.52%) флэт
  - paletted: 6.41% -> 6.24% (-0.17%) флэт
  - players_packets: 0.01% -> 0.01% (+0.00%) флэт

## VERDICT: **RED**
