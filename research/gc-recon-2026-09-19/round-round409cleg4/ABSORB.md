# absorb ROUND (round409cleg4, run 35668283159, branch round-405-c-l4, head 71cce9f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6667072 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6667072 (поллов=5); TPS_exp=2.20; normalized=-0.1%
- GC: young=111, Full=9, total=21.4s, avg=178ms, max=2573ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116155 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.49% (-1.68%) спад
  - fluid: 16.72% -> 16.05% (-0.66%) флэт
  - broadphase: 15.66% -> 15.02% (-0.64%) флэт
  - nav_ai: 14.16% -> 14.28% (+0.12%) флэт
  - inside_volatile: 12.01% -> 11.36% (-0.64%) флэт
  - fastutil: 8.54% -> 8.58% (+0.05%) флэт
  - java_util: 7.01% -> 6.77% (-0.24%) флэт
  - paletted: 6.41% -> 6.35% (-0.06%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
