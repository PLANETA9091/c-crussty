# absorb ROUND (450d-anchor-35, run 36063385955, branch round-450-anchor-35, head d02a297)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6499754 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6499754 (поллов=6); TPS_exp=2.17; normalized=+10.7%
- GC: young=115, Full=10, total=24.0s, avg=192ms, max=2377ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116889 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.69% (-1.49%) спад
  - fluid: 16.72% -> 16.03% (-0.68%) флэт
  - broadphase: 15.66% -> 14.91% (-0.75%) флэт
  - nav_ai: 14.16% -> 13.95% (-0.21%) флэт
  - inside_volatile: 12.01% -> 11.40% (-0.60%) флэт
  - fastutil: 8.54% -> 8.75% (+0.22%) флэт
  - java_util: 7.01% -> 6.88% (-0.13%) флэт
  - paletted: 6.41% -> 6.27% (-0.14%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
