# absorb ROUND (anchor424c, run 35825641995, branch round-424-anchorc, head a8ef2fc)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6874937 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6874937 (поллов=5); TPS_exp=2.25; normalized=+2.4%
- GC: young=113, Full=10, total=23.1s, avg=188ms, max=2307ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116497 сэмплов (базлайн 115655)
  - items: 31.17% -> 30.12% (-1.05%) спад
  - fluid: 16.72% -> 15.97% (-0.74%) флэт
  - broadphase: 15.66% -> 15.43% (-0.22%) флэт
  - nav_ai: 14.16% -> 14.21% (+0.05%) флэт
  - inside_volatile: 12.01% -> 11.87% (-0.13%) флэт
  - fastutil: 8.54% -> 8.81% (+0.28%) флэт
  - java_util: 7.01% -> 6.59% (-0.43%) флэт
  - paletted: 6.41% -> 6.32% (-0.09%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
