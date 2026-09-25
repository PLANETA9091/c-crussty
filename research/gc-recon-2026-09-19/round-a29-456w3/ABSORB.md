# absorb ROUND (a29-456w3, run 36119642502, branch round-456-anchor-29, head d4deb33)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7051107 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 7051107 (поллов=6); TPS_exp=2.28; normalized=+2.9%
- GC: young=111, Full=10, total=23.8s, avg=197ms, max=2421ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=117062 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.76% (-1.41%) спад
  - fluid: 16.72% -> 16.01% (-0.71%) флэт
  - broadphase: 15.66% -> 14.78% (-0.88%) флэт
  - nav_ai: 14.16% -> 13.49% (-0.68%) флэт
  - inside_volatile: 12.01% -> 11.47% (-0.54%) флэт
  - fastutil: 8.54% -> 9.19% (+0.65%) флэт
  - java_util: 7.01% -> 6.67% (-0.34%) флэт
  - paletted: 6.41% -> 6.14% (-0.27%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
