# absorb ROUND (a13, run 36104815756, branch round-455-anchor-13, head 2443542)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6619580 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6619580 (поллов=6); TPS_exp=2.19; normalized=+4.9%
- GC: young=107, Full=10, total=25.3s, avg=216ms, max=2568ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115800 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.77% (-1.40%) спад
  - fluid: 16.72% -> 16.94% (+0.22%) флэт
  - broadphase: 15.66% -> 15.34% (-0.32%) флэт
  - nav_ai: 14.16% -> 13.84% (-0.32%) флэт
  - inside_volatile: 12.01% -> 10.70% (-1.31%) спад
  - fastutil: 8.54% -> 8.62% (+0.09%) флэт
  - java_util: 7.01% -> 6.36% (-0.66%) флэт
  - paletted: 6.41% -> 7.03% (+0.62%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
