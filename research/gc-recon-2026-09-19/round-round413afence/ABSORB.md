# absorb ROUND (round413afence, run 35717258406, branch round-413-a-fence, head 3642210)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6741043 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.20 @ 6741043 (поллов=5); TPS_exp=2.22; normalized=-0.8%
- GC: young=115, Full=9, total=23.8s, avg=192ms, max=2817ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=112301 сэмплов (базлайн 115655)
  - items: 31.17% -> 29.71% (-1.46%) спад
  - fluid: 16.72% -> 16.66% (-0.06%) флэт
  - broadphase: 15.66% -> 15.42% (-0.23%) флэт
  - nav_ai: 14.16% -> 13.34% (-0.83%) флэт
  - inside_volatile: 12.01% -> 11.31% (-0.70%) флэт
  - fastutil: 8.54% -> 9.00% (+0.46%) флэт
  - java_util: 7.01% -> 7.00% (-0.01%) флэт
  - paletted: 6.41% -> 6.94% (+0.54%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **PARITY/LOW**
