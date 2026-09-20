# absorb COLLECTOR-A/B ParallelGC s7201 (run 35512885689, head e377f41) — PROTOCOL v8-REGRESSION DUAL BAR

- PG-T1: gc_tune=OK, region_steal=OK, travel_diet=OK, skip_store_bb=OK, bu_defer=OK, inside_cache=OK, flush_diet=OK, region_threads=OK, batch_collector=OK, fluid_guard=OK, NCDFE=0, pop=VALID, mode=WORKERS4-TELEMETRY, col=PARALLEL -> **PASS**
- PG-T2: threw=0, unexpected=0, s7180-class=0, TPS-поллов=6 -> **PASS**
- PG-T3: runner=6653417 (широкий банд 6000000..9500000: OK), median5=2.2
  DUAL BAR (v8-REGRESSION): normalized=+38.1%, absolute=+37.5% (бар: ОБЕ >= +10%)
  -> **CANDIDATE GREEN** -> подтверждающий лег min-of-2 (dispatch_s7201.py повторно) -> banking v4 = v3 + gc_tune
- PG-T4: young=109, Full=7, total_pause=18.8s (гейт <= 19.5s), avg=162.0ms (гейт <= 300ms), young-банд 30..250, Full<= 2, max=2400.6ms (гейт <= 2000ms) -> **FAIL** (база s7198: 19.5s/79.3ms/178.7ms)
- PG-T5: park=40/1180 (3.4%; класс ~1.4%) -> PASS/N/A

## VERDICT: **CANDIDATE-GREEN**
