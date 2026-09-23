# absorb S7-174 P2-OFFLOAD v2 PUMP s7198 (run 35500343881, head aa1682d) — PROTOCOL v8-REGRESSION DUAL BAR

- PG-T1: region_steal=OK, travel_diet=OK, zero_alloc=OK, skip_store_bb=OK, bu_defer=OK, inside_cache=OK, flush_diet=OK, region_threads=OK, batch_collector=OK, NCDFE=0, pop=VALID, mode=WORKERS4-TELEMETRY, guarded-marker=OK, strict-violated=False -> **PASS**
- PG-T2: threw=0, unexpected=0, s7180-class=0, TPS-поллов=5 -> **PASS**
- PG-T3: runner=8912496 (широкий банд 6000000..9500000: OK), median5=1.5
  DUAL BAR (v8-REGRESSION): normalized=-29.7%, absolute=-6.2% (бар: ОБЕ >= +10%; оси страхуют index-конфаунд в обе стороны)
  -> **< +10% хотя бы по одной оси** -> лейн ОТКРЫТ -> P2-продолжение по RECON-37: v2 residual overlap (tracker 2.2% — парити-ревью; random/block 1.4% — RNG-риск) или REBALANCE, если I деградировал на 4 хелперах — реализация+диспатч в том же тике
- PG-T4: young=166, Full=0 -> **PASS**
- PG-T5: park=1/1199 (0.1%; класс ~1.4%) -> PASS/N/A

## VERDICT: **LANE-OPEN**
