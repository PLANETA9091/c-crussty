# absorb #15 INSIDE-BITMASK s7204 (run 35518054033, head 30cd34f) — PROTOCOL v8-REGRESSION DUAL BAR vs БАНК v4 (2.6 @ 8551924 / 2.2 @ 6653417, 2-точечная модель)

- PG-T1: gc_tune=OK, inside_bitmask=OK, region_steal=OK, travel_diet=OK, skip_store_bb=OK, bu_defer=OK, inside_cache=OK, flush_diet=OK, region_threads=OK, batch_collector=OK, fluid_guard=OK, NCDFE=0, pop=VALID, ARMED=OK+READY=OK+composed=OK+arm-fail=нет+window-miss=нет+rng-cascade=нет+dormant=нет, mode=WORKERS4-TELEMETRY, col=PARALLEL -> **PASS**
- PG-T2: threw=0, unexpected=0, s7180-class=0, TPS-поллов=5 -> **PASS**
- PG-T3: runner=6687429 (широкий банд 6000000..9500000: OK), median5=2.2
  DUAL BAR (v8-REGRESSION, банк v4 2-точки): normalized=min(+8.2%, -0.5%)=-0.5%, absolute=-0.3% (TPS_exp@6687429=2.21; бар: ОБЕ >= +10%)
  -> **< +10% хотя бы по одной оси** -> лейн ОТКРЫТ -> #15 НЕ ПРЕВЗОШЁЛ банк v4 под ParallelGC-экономикой (G1-вердикты RECON-32/33 подтверждены и на новой базе); следующий кандидат очереди в новом тике
- PG-T4: young=110, Full=8, total_pause=18.9s (гейт <= 20.0s), avg=160.1ms, max=2270.7ms (гейт <= 3000ms) -> **PASS** (банк v4 leg#2: 18.8s/162ms/2400ms/Full=7)
- PG-T5: park=37/1164 (3.2%; класс ~1.4%) -> PASS/N/A

## VERDICT: **LANE-OPEN**
