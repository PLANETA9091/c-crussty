# absorb ROUND (ch420a, run 35790657454, branch round-420-c-cha, head 85f74b8)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6758845 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.10 @ 6758845 (поллов=5); TPS_exp=2.22; normalized=-5.5%
- GC: young=109, Full=9, total=21.1s, avg=179ms, max=2479ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116276 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.82% (-1.35%) спад
  - fluid: 16.72% -> 15.87% (-0.85%) флэт
  - broadphase: 15.66% -> 15.42% (-0.23%) флэт
  - nav_ai: 14.16% -> 13.48% (-0.68%) флэт
  - inside_volatile: 12.01% -> 11.16% (-0.85%) флэт
  - fastutil: 8.54% -> 8.67% (+0.13%) флэт
  - java_util: 7.01% -> 6.28% (-0.73%) флэт
  - paletted: 6.41% -> 6.33% (-0.08%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **RED**
