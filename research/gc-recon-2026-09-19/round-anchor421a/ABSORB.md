# absorb ROUND (anchor421a, run 35804136042, branch round-421-anchora, head 2d23f45)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8979059 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 8979059 (поллов=5); TPS_exp=2.69; normalized=+0.4%
- GC: young=125, Full=10, total=22.1s, avg=164ms, max=2231ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113381 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.55% (-2.63%) спад
  - fluid: 16.72% -> 16.48% (-0.24%) флэт
  - broadphase: 15.66% -> 15.04% (-0.61%) флэт
  - nav_ai: 14.16% -> 13.97% (-0.20%) флэт
  - inside_volatile: 12.01% -> 10.73% (-1.28%) спад
  - fastutil: 8.54% -> 8.57% (+0.03%) флэт
  - java_util: 7.01% -> 6.53% (-0.48%) флэт
  - paletted: 6.41% -> 6.93% (+0.53%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
