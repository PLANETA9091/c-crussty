# absorb ROUND (round-448-anchor-3, run 36015068760, branch round-448-anchor-3, head 97afb42)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6905572 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.25 @ 6905572 (поллов=6); TPS_exp=2.25; normalized=-0.1%
- GC: young=107, Full=9, total=21.5s, avg=185ms, max=2420ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116223 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.41% (-1.76%) спад
  - fluid: 16.72% -> 15.51% (-1.20%) спад
  - broadphase: 15.66% -> 15.26% (-0.39%) флэт
  - nav_ai: 14.16% -> 14.22% (+0.06%) флэт
  - inside_volatile: 12.01% -> 11.12% (-0.88%) флэт
  - fastutil: 8.54% -> 8.93% (+0.39%) флэт
  - java_util: 7.01% -> 6.96% (-0.05%) флэт
  - paletted: 6.41% -> 6.27% (-0.13%) флэт
  - players_packets: 0.01% -> 0.00% (-0.01%) флэт

## VERDICT: **PARITY/LOW**
