# absorb ROUND (round-440-anchor-4, run 35950601328, branch round-440-anchor-4, head 6db49fb)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7042437 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 7042437 (поллов=5); TPS_exp=2.28; normalized=+5.2%
- GC: young=111, Full=10, total=24.0s, avg=198ms, max=2706ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117214 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.26% (-1.91%) спад
  - fluid: 16.72% -> 15.43% (-1.28%) спад
  - broadphase: 15.66% -> 14.80% (-0.86%) флэт
  - nav_ai: 14.16% -> 13.83% (-0.33%) флэт
  - inside_volatile: 12.01% -> 11.47% (-0.53%) флэт
  - fastutil: 8.54% -> 8.33% (-0.21%) флэт
  - java_util: 7.01% -> 7.84% (+0.83%) флэт
  - paletted: 6.41% -> 6.08% (-0.32%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
