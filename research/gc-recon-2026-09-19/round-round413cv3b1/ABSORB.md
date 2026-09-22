# absorb ROUND (round413cv3b1, run 35717612486, branch round-413-cv3b-1, head 43a8318)

- T1: lever=(n/a в run-env), pop=VALID, NCDFE=0, col=PARALLEL, runner=6756401 (band OK) | inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK fake_players=OK fluid_bitmask=OK fluid_dirty=OK inside_bitmask=OK skip_store_bb=OK region_steal=OK bu_defer=OK -> **PASS**
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median=2.40 @ 6756401 (поллов=5); TPS_exp=2.22; normalized=+8.0%
- GC: young=1133, Full=9, total=28.3s, avg=25ms, max=2415ms (банк-справка 18.8s/162ms/2400ms/Full=7)
- T5 ЛЕЙНЫ: total=111317 сэмплов (базлайн 115655)
  - items: 31.17% -> 34.23% (+3.06%) РОСТ
  - fluid: 16.72% -> 18.37% (+1.65%) РОСТ
  - broadphase: 15.66% -> 15.45% (-0.21%) флэт
  - nav_ai: 14.16% -> 8.45% (-5.71%) спад
  - inside_volatile: 12.01% -> 11.65% (-0.35%) флэт
  - fastutil: 8.54% -> 8.03% (-0.51%) флэт
  - java_util: 7.01% -> 7.81% (+0.79%) флэт
  - paletted: 6.41% -> 7.18% (+0.78%) флэт
  - players_packets: 0.01% -> 0.01% (-0.00%) флэт

## VERDICT: **GREEN-CANDIDATE**
