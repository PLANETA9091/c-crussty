# absorb MEGA-ROUND-1 items_mono (run 35534936408, head 3b6c418, branch round-396-f-items_mono) — банк v4 + lever

- T1: fp=4:OK, pop=VALID, NCDFE=0, col=PARALLEL, runner=6558621 (band OK), lever=ARMED | fluid_bitmask=0:OK fluid_dirty=0:OK inside_bitmask=0:OK skip_store_bb=0:OK region_steal=0:OK bu_defer=0:OK inside_cache=OK flush_diet=OK region_threads=OK batch_collector=OK fluid_guard=OK gc_tune=OK population_target=OK -> **PASS**
  - arm-marker: [crussty-plugin] items_mono: LEVER ARMED (CRUSSTY_LEVER_FLAG=items_mono, lever_arg="1") — tickNonPassenger Entity.tick -> RegionTickOps.entityTick type-test spl
  - arm-marker: [20:23:05 ERROR]: [STDERR] [net.minecraft.world.entity.RegionTickOps] [S7-F] items_mono ARMED: tickNonPassenger Entity.tick -> RegionTickOps.entityTick type-tes
- T2: threw=0, TPS-поллов=6 -> **PASS**
- T3: median5=2.40 @ 6558621; TPS_exp(банк-интерполяция)=2.18; DELTA=+10.1% (БЫЛО 2.18 -> СТАЛО 2.40 @ тот же runner)
- T4 (справка банка 18.8s/162ms/2400ms/Full=7): young=111, Full=9, total=22.3s, avg=186ms, max=2563ms
- T5 ЛЕЙН-КАРТА: total=113434 сэмплов (базлайн 115655)
  - fluid: банк 16.72% -> лег 17.48% (+0.76%) [19332->19827] флэт
  - paletted: банк 6.41% -> лег 7.15% (+0.74%) [7408->8109] флэт
  - fastutil: банк 8.54% -> лег 8.55% (+0.01%) [9874->9696] флэт
  - players_packets: банк 0.01% -> лег 0.01% (-0.00%) [12->8] флэт
  - java_util: банк 7.01% -> лег 6.88% (-0.14%) [8110->7801] флэт
  - nav_ai: банк 14.16% -> лег 13.87% (-0.29%) [16380->15732] флэт
  - inside_volatile: банк 12.01% -> лег 11.48% (-0.52%) [13885->13023] флэт
  - broadphase: банк 15.66% -> лег 15.13% (-0.53%) [18108->17161] флэт
  - items: банк 31.17% -> лег 30.16% (-1.01%) [36051->34213] спад

## VERDICT: **MEASURED**
