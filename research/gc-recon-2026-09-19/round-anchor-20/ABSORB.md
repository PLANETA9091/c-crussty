# absorb ROUND (anchor-20, run 36101872832, branch round-454-anchor-20, head ccfa6f0)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=7018724 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.35 @ 7018724 (поллов=6); TPS_exp=2.28; normalized=+3.2%
- GC: young=111, Full=9, total=20.7s, avg=173ms, max=2449ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=116618 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.74% (-1.43%) спад
  - fluid: 16.72% -> 15.75% (-0.97%) флэт
  - broadphase: 15.66% -> 14.98% (-0.68%) флэт
  - nav_ai: 14.16% -> 13.95% (-0.21%) флэт
  - inside_volatile: 12.01% -> 11.44% (-0.56%) флэт
  - fastutil: 8.54% -> 8.65% (+0.11%) флэт
  - java_util: 7.01% -> 7.05% (+0.03%) флэт
  - paletted: 6.41% -> 6.23% (-0.18%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
