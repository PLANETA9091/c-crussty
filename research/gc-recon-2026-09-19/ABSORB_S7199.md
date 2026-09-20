# absorb GC-TUNE s7199 (run 35503592912, head fd20533) — PROTOCOL v8-REGRESSION DUAL BAR

- PG-T1: gc_tune=OK, region_steal=OK, travel_diet=OK, skip_store_bb=OK, bu_defer=OK, inside_cache=OK, flush_diet=OK, region_threads=OK, batch_collector=OK, fluid_guard=OK, NCDFE=0, pop=VALID, mode=WORKERS4-TELEMETRY, region8M=OK -> **PASS**
- PG-T2: threw=0, unexpected=0, s7180-class=0, TPS-поллов=5 -> **PASS**
- PG-T3: runner=6450600 (широкий банд 6000000..9500000: OK), median5=1.5
  DUAL BAR (v8-REGRESSION): normalized=-2.9%, absolute=-6.2% (бар: ОБЕ >= +10%)
  -> **< +10% хотя бы по одной оси** -> лейн ОТКРЫТ -> GC-под-лейны (young-mix/region-size/pause-target варьирование) или следующий крит-путь рычаг
- PG-T4: young=621, Full=0, total_pause=44.4s (гейт <= 14.0s), avg=71.4ms (гейт <= 60ms), max=153.6ms -> **FAIL** (база s7198: 19.5s/79.3ms/178.7ms)
- PG-T5: park=151/1178 (12.8%; класс ~1.4%) -> PASS/N/A

## VERDICT: **LANE-OPEN**
