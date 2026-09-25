# absorb ROUND (451a-ins4-3, run 36069573439, branch round-451-ins4-3, head 0707800)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6753946 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6753946 (поллов=5); TPS_exp=2.22; normalized=+3.5%
- GC: young=100, Full=9, total=21.4s, avg=196ms, max=2942ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=105077 сэмплов (базлайн 115655)
  - items: 31.17% -> 0.00% (-31.17%) спад
  - fluid: 16.72% -> 17.04% (+0.32%) флэт
  - broadphase: 15.66% -> 9.51% (-6.14%) спад
  - nav_ai: 14.16% -> 4.00% (-10.16%) спад
  - inside_volatile: 12.01% -> 16.34% (+4.33%) РОСТ
  - fastutil: 8.54% -> 6.72% (-1.82%) спад
  - java_util: 7.01% -> 9.00% (+1.98%) РОСТ
  - paletted: 6.41% -> 5.69% (-0.71%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
