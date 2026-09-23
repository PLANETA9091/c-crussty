# absorb ROUND (round-435-anchor-p3, run 35924370909, branch round-435-anchor-p3, head 536cf06)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6817331 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 6817331 (поллов=6); TPS_exp=2.23; normalized=+5.2%
- GC: young=108, Full=9, total=20.3s, avg=173ms, max=2354ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117122 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.22% (-1.95%) спад
  - fluid: 16.72% -> 15.62% (-1.10%) спад
  - broadphase: 15.66% -> 14.45% (-1.21%) спад
  - nav_ai: 14.16% -> 13.33% (-0.84%) флэт
  - inside_volatile: 12.01% -> 11.40% (-0.61%) флэт
  - fastutil: 8.54% -> 8.27% (-0.27%) флэт
  - java_util: 7.01% -> 6.21% (-0.80%) флэт
  - paletted: 6.41% -> 6.21% (-0.19%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
