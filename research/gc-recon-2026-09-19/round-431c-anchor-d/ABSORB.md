# absorb ROUND (431c-anchor-d, run 35887035323, branch round-431c-anchor-d, head 3065047)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7104607 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 7104607 (поллов=5); TPS_exp=2.30; normalized=+0.2%
- GC: young=110, Full=9, total=20.7s, avg=174ms, max=2721ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117041 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.40% (-1.77%) спад
  - fluid: 16.72% -> 15.55% (-1.17%) спад
  - broadphase: 15.66% -> 14.67% (-0.98%) флэт
  - nav_ai: 14.16% -> 13.66% (-0.51%) флэт
  - inside_volatile: 12.01% -> 11.64% (-0.36%) флэт
  - fastutil: 8.54% -> 8.30% (-0.24%) флэт
  - java_util: 7.01% -> 6.63% (-0.38%) флэт
  - paletted: 6.41% -> 6.08% (-0.33%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
