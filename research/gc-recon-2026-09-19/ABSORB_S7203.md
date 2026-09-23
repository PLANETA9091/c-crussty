# absorb A/B #3 THP s7203 (run 35514929587, head fb0a9ad) — PROTOCOL v8-REGRESSION DUAL BAR vs БАНК v4 (2.6 @ 8551924 / 2.2 @ 6653417, 2-точечная модель)

- PG-T1: gc_tune=OK, region_steal=OK, travel_diet=OK, skip_store_bb=OK, bu_defer=OK, inside_cache=OK, flush_diet=OK, region_threads=OK, batch_collector=OK, fluid_guard=OK, NCDFE=0, pop=VALID, mode=WORKERS4-TELEMETRY, col=PARALLEL, THP-warn=нет -> **PASS**
- PG-T2: threw=0, unexpected=0, s7180-class=0, TPS-поллов=5 -> **PASS**
- PG-T3: runner=7014414 (широкий банд 6000000..9500000: OK), median5=2.3
  DUAL BAR (v8-REGRESSION, банк v4 2-точки): normalized=min(+7.9%, -0.8%)=-0.8%, absolute=+1.1% (TPS_exp@7014414=2.28; бар: ОБЕ >= +10%)
  -> **< +10% хотя бы по одной оси** -> лейн ОТКРЫТ -> THP НЕ ПРЕВЗОШЁЛ банк v4; следующий кандидат очереди в новом тике
- PG-T4: young=112, Full=9, total_pause=20.9s (гейт <= 20.0s), avg=172.4ms, max=2406.4ms (гейт <= 3000ms) -> **FAIL** (банк v4 leg#2: 18.8s/162ms/2400ms/Full=7)
- PG-T5: park=43/1198 (3.6%; класс ~1.4%) -> PASS/N/A

## VERDICT: **LANE-OPEN**
