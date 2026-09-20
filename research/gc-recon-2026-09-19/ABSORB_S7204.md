# absorb #16 FLUIDPUSH-BITMASK s7206 (run 35528326290, head a95e601) — PROTOCOL v8-REGRESSION DUAL BAR vs БАНК v4 (2.6 @ 8551924 / 2.2 @ 6653417, 2-точечная модель)

- PG-T1: gc_tune=OK, inside_bitmask=OK, region_steal=OK, travel_diet=OK, skip_store_bb=OK, bu_defer=OK, fluid_bitmask=OK, fluid_dirty_ledger=OK, fluid_dirty=OK, inside_cache=OK, flush_diet=OK, region_threads=OK, batch_collector=OK, fluid_guard=OK, NCDFE=0, pop=VALID, ARMED=OK+READY=OK+composed=OK+arm-fail=нет+window-miss=нет+rng-cascade=нет+dormant=нет, mode=WORKERS4-TELEMETRY, col=PARALLEL -> **PASS**
- PG-T2: threw=0, unexpected=0, s7180-class=0, TPS-поллов=6 -> **PASS**
- PG-T3: runner=7023564 (широкий банд 6000000..9500000: OK), median5=2.3
  DUAL BAR (v8-REGRESSION, банк v4 2-точки): normalized=min(+7.7%, -1.0%)=-1.0%, absolute=+1.0% (TPS_exp@7023564=2.28; бар: ОБЕ >= +10%)
  -> **< +10% хотя бы по одной оси** -> лейн ОТКРЫТ -> #15 НЕ ПРЕВЗОШЁЛ банк v4 под ParallelGC-экономикой (G1-вердикты RECON-32/33 подтверждены и на новой базе); следующий кандидат очереди в новом тике
- PG-T4: young=114, Full=9, total_pause=21.5s (гейт <= 20.0s), avg=175.1ms, max=2869.8ms (гейт <= 3000ms) -> **FAIL** (банк v4 leg#2: 18.8s/162ms/2400ms/Full=7)
- PG-T5: fluid-family=15931/115655 (13.77% vs baseline 14.59%; гейт <= 5.84%) [cpu-collapsed.txt] -> **FAIL — семейство не снята**

## VERDICT: **LANE-OPEN**
