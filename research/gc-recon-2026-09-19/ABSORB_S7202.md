# absorb COLLECTOR-A/B #2 ZGC-gen s7202 (run 35513964778, head baf4abc) — PROTOCOL v8-REGRESSION DUAL BAR vs БАНК v4 (2-точки: 2.6 @ 8551924, 2.2 @ 6653417)

- PG-T1: gc_tune=OK, region_steal=OK, travel_diet=OK, skip_store_bb=OK, bu_defer=OK, inside_cache=OK, flush_diet=OK, region_threads=OK, batch_collector=OK, fluid_guard=OK, NCDFE=0, pop=VALID, mode=WORKERS4-TELEMETRY, col=ZGC -> **PASS**
- PG-T2: threw=0, unexpected=0, s7180-class=0, TPS-поллов=6 -> **PASS**
- PG-T3: runner=6680532 (широкий банд 6000000..9500000: OK), median5=1.6
  DUAL BAR (v8-REGRESSION, банк v4 2-точки): normalized=min(-21.2%, -27.6%)=-27.6%, absolute=-27.5% (TPS_exp@6680532=2.21; бар: ОБЕ >= +10%)
  -> **< +10% хотя бы по одной оси** -> лейн ОТКРЫТ -> ZGC НЕ ПРЕВЗОШЁЛ ParallelGC-банк; следующий кандидат очереди (ParallelGCThreads-tuning / THP) в новом тике
- PG-T4: pauses=334, Full=0, total_pause=0.0s (гейт <= 10.0s), avg=0.019ms, max=0.041ms (гейт <= 50ms) -> **PASS** (банк v4 leg#1 ParallelGC: 24.6s/max 2954ms/Full=9)
- PG-T5: park=64/1119 (5.7%; класс ~1.4%) -> PASS/N/A

## VERDICT: **LANE-OPEN**
