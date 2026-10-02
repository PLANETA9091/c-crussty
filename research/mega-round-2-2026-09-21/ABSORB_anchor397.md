# absorb ROUND (anchor397, run 35543961438, branch master, head 3c48a96)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6918472 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6918472 (поллов=5); TPS_exp=2.26; normalized=-2.5%
- GC: young=114, Full=9, total=21.3s, avg=173ms, max=2345ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=115699 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.77% (-0.40%) флэт
  - fluid: 16.72% -> 16.45% (-0.26%) флэт
  - broadphase: 15.66% -> 15.66% (+0.00%) флэт
  - nav_ai: 14.16% -> 14.10% (-0.06%) флэт
  - inside_volatile: 12.01% -> 12.22% (+0.22%) флэт
  - fastutil: 8.54% -> 8.98% (+0.44%) флэт
  - java_util: 7.01% -> 7.11% (+0.10%) флэт
  - paletted: 6.41% -> 6.78% (+0.38%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
