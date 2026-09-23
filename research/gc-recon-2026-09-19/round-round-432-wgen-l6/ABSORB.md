# absorb ROUND (round-432-wgen-l6, run 35891400665, branch round-432-wgen-l6, head 898650c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7030184 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.65 @ 7030184 (поллов=6); TPS_exp=2.28; normalized=+16.3%
- GC: young=106, Full=9, total=20.5s, avg=179ms, max=3048ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=102641 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 16.64% (-0.08%) флэт
  - broadphase: 15.66% -> 10.13% (-5.53%) спад
  - nav_ai: 14.16% -> 3.37% (-10.80%) спад
  - inside_volatile: 12.01% -> 12.80% (+0.79%) флэт
  - fastutil: 8.54% -> 6.67% (-1.87%) спад
  - java_util: 7.01% -> 7.49% (+0.48%) флэт
  - paletted: 6.41% -> 5.56% (-0.84%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
