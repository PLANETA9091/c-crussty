# absorb ROUND (451b-anchor-16, run 36073612361, branch round-451-anchor-16, head b5baf54)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6651180 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6651180 (поллов=5); TPS_exp=2.20; normalized=+0.0%
- GC: young=112, Full=10, total=23.7s, avg=194ms, max=2407ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116145 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.77% (-1.40%) спад
  - fluid: 16.72% -> 16.10% (-0.61%) флэт
  - broadphase: 15.66% -> 15.71% (+0.06%) флэт
  - nav_ai: 14.16% -> 13.71% (-0.45%) флэт
  - inside_volatile: 12.01% -> 11.10% (-0.91%) флэт
  - fastutil: 8.54% -> 9.11% (+0.57%) флэт
  - java_util: 7.01% -> 6.66% (-0.35%) флэт
  - paletted: 6.41% -> 6.19% (-0.21%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
