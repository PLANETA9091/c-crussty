# absorb ROUND (round-438-anchor-5, run 35941756661, branch round-438-anchor-5, head bcfb18f)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6793686 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.30 @ 6793686 (поллов=5); TPS_exp=2.23; normalized=+3.2%
- GC: young=108, Full=9, total=21.6s, avg=185ms, max=2539ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=114727 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.69% (-1.48%) спад
  - fluid: 16.72% -> 16.51% (-0.20%) флэт
  - broadphase: 15.66% -> 15.10% (-0.55%) флэт
  - nav_ai: 14.16% -> 13.27% (-0.89%) флэт
  - inside_volatile: 12.01% -> 10.98% (-1.03%) спад
  - fastutil: 8.54% -> 8.69% (+0.15%) флэт
  - java_util: 7.01% -> 7.05% (+0.04%) флэт
  - paletted: 6.41% -> 7.08% (+0.68%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
