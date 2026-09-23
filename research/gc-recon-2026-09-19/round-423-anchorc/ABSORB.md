# absorb ROUND (round-423-anchorc, run 35815692082, branch round-423-anchorc, head 4789ca7)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6970260 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6970260 (поллов=5); TPS_exp=2.27; normalized=+5.9%
- GC: young=113, Full=9, total=20.2s, avg=165ms, max=2292ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116537 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.14% (-1.03%) спад
  - fluid: 16.72% -> 15.85% (-0.86%) флэт
  - broadphase: 15.66% -> 14.78% (-0.88%) флэт
  - nav_ai: 14.16% -> 13.71% (-0.45%) флэт
  - inside_volatile: 12.01% -> 11.67% (-0.34%) флэт
  - fastutil: 8.54% -> 8.67% (+0.13%) флэт
  - java_util: 7.01% -> 6.72% (-0.29%) флэт
  - paletted: 6.41% -> 6.37% (-0.04%) флэт
  - players_packets: 0.01% -> 0.01% (-0.01%) флэт

## VERDICT: **GREEN-CANDIDATE**
