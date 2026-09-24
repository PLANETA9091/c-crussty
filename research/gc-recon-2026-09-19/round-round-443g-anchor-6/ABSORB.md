# absorb ROUND (round-443g-anchor-6, run 36038582424, branch round-443g-anchor-6, head 9061566)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=8679052 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.70 @ 8679052 (поллов=5); TPS_exp=2.63; normalized=+2.8%
- GC: young=124, Full=10, total=22.5s, avg=168ms, max=2245ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=113744 сэмплов (базлайн 115655)
  - items: 31.17% -> 28.78% (-2.39%) спад
  - fluid: 16.72% -> 16.26% (-0.46%) флэт
  - broadphase: 15.66% -> 15.62% (-0.04%) флэт
  - nav_ai: 14.16% -> 14.04% (-0.12%) флэт
  - inside_volatile: 12.01% -> 10.80% (-1.21%) спад
  - fastutil: 8.54% -> 9.27% (+0.73%) флэт
  - java_util: 7.01% -> 6.76% (-0.25%) флэт
  - paletted: 6.41% -> 6.69% (+0.28%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
