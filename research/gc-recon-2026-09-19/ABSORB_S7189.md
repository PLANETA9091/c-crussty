# absorb S7-172 P2-OFFLOAD v1 s7197 (run 35499022752, head b18b3a4) — PROTOCOL v8-REGRESSION DUAL BAR

- PG-T1: region_steal=OK, travel_diet=OK, zero_alloc=OK, skip_store_bb=OK, bu_defer=OK, inside_cache=OK, flush_diet=OK, region_threads=OK, batch_collector=OK, NCDFE=0, pop=VALID, mode=WORKERS4-TELEMETRY, guarded-marker=OK, strict-violated=False -> **PASS**
- PG-T2: threw=0, unexpected=0, s7180-class=0, TPS-поллов=5 -> **PASS**
- PG-T3: runner=6878968 (широкий банд 6000000..9500000: OK), median5=1.3
  DUAL BAR (v8-REGRESSION): normalized=-21.1%, absolute=-18.8% (бар: ОБЕ >= +10%; оси страхуют index-конфаунд в обе стороны)
  -> **< +10% хотя бы по одной оси** -> лейн ОТКРЫТ -> P2-продолжение по RECON-37: v2 residual overlap (tracker 2.2% — парити-ревью; random/block 1.4% — RNG-риск) или REBALANCE, если I деградировал на 4 хелперах — реализация+диспатч в том же тике
- PG-T4: young=157, Full=0 -> **PASS**
- PG-T5: park=933/1195 (78.1%; класс ~1.4%) -> PASS/N/A

## VERDICT: **LANE-OPEN**
