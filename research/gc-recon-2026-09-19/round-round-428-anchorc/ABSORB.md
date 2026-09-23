# absorb ROUND (round-428-anchorc, run 35853614303, branch round-428-anchorc, head dc704c9)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6284226 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6284226 (поллов=5); TPS_exp=2.12; normalized=+3.7%
- GC: young=111, Full=9, total=23.0s, avg=192ms, max=2601ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115274 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.40% (-1.77%) спад
  - fluid: 16.72% -> 16.82% (+0.11%) флэт
  - broadphase: 15.66% -> 14.98% (-0.68%) флэт
  - nav_ai: 14.16% -> 13.63% (-0.53%) флэт
  - inside_volatile: 12.01% -> 10.74% (-1.26%) спад
  - fastutil: 8.54% -> 8.71% (+0.17%) флэт
  - java_util: 7.01% -> 6.48% (-0.53%) флэт
  - paletted: 6.41% -> 7.01% (+0.61%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
