# absorb ROUND (anchor-553, run 36206883138, branch round-463-anchor-553, head 97f1c9c)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6097651 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.15 @ 6097651 (поллов=6); TPS_exp=2.08; normalized=+3.2%
- GC: young=109, Full=9, total=21.1s, avg=179ms, max=2470ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116496 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.65% (-1.52%) спад
  - fluid: 16.72% -> 15.84% (-0.88%) флэт
  - broadphase: 15.66% -> 14.95% (-0.71%) флэт
  - nav_ai: 14.16% -> 13.61% (-0.56%) флэт
  - inside_volatile: 12.01% -> 11.17% (-0.83%) флэт
  - fastutil: 8.54% -> 8.27% (-0.27%) флэт
  - java_util: 7.01% -> 6.49% (-0.52%) флэт
  - paletted: 6.41% -> 6.12% (-0.28%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
