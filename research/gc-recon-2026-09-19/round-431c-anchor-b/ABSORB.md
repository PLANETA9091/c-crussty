# absorb ROUND (431c-anchor-b, run 35886978370, branch round-431c-anchor-b, head 3065047)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8891273 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.60 @ 8891273 (поллов=5); TPS_exp=2.67; normalized=-2.7%
- GC: young=126, Full=10, total=22.2s, avg=164ms, max=2085ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112945 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.52% (-2.65%) спад
  - fluid: 16.72% -> 16.31% (-0.40%) флэт
  - broadphase: 15.66% -> 14.81% (-0.84%) флэт
  - nav_ai: 14.16% -> 14.07% (-0.09%) флэт
  - inside_volatile: 12.01% -> 11.04% (-0.97%) флэт
  - fastutil: 8.54% -> 8.80% (+0.26%) флэт
  - java_util: 7.01% -> 6.56% (-0.45%) флэт
  - paletted: 6.41% -> 7.19% (+0.78%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
