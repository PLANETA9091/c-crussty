# absorb ROUND (a8-456, run 36112151010, branch round-456-anchor-8, head 0716075)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6731200 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6731200 (поллов=6); TPS_exp=2.22; normalized=-0.7%
- GC: young=107, Full=9, total=20.2s, avg=174ms, max=2446ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116508 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.70% (-1.47%) спад
  - fluid: 16.72% -> 16.08% (-0.64%) флэт
  - broadphase: 15.66% -> 15.84% (+0.18%) флэт
  - nav_ai: 14.16% -> 14.16% (-0.01%) флэт
  - inside_volatile: 12.01% -> 11.36% (-0.64%) флэт
  - fastutil: 8.54% -> 8.93% (+0.40%) флэт
  - java_util: 7.01% -> 7.09% (+0.08%) флэт
  - paletted: 6.41% -> 6.00% (-0.41%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
