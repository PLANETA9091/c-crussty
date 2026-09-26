# absorb ROUND (anchor-502, run 36206822607, branch round-463-anchor-502, head 97f1c9c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6471744 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6471744 (поллов=5); TPS_exp=2.16; normalized=+1.8%
- GC: young=108, Full=10, total=25.3s, avg=214ms, max=2553ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116317 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.24% (-1.93%) спад
  - fluid: 16.72% -> 16.44% (-0.28%) флэт
  - broadphase: 15.66% -> 15.56% (-0.10%) флэт
  - nav_ai: 14.16% -> 13.95% (-0.21%) флэт
  - inside_volatile: 12.01% -> 10.59% (-1.42%) спад
  - fastutil: 8.54% -> 8.79% (+0.25%) флэт
  - java_util: 7.01% -> 6.45% (-0.56%) флэт
  - paletted: 6.41% -> 7.05% (+0.65%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
