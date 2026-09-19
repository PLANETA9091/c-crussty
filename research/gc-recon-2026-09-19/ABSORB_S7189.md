# absorb #14 v2a ISOLATION s7189 (run 35467929550, head a5c353d) — PROTOCOL v8-REGRESSION DUAL BAR

- PG-T1: travel_diet=OK, zero_alloc=OK, skip_store_bb=OK, region_steal=OK, bu_defer=OK, inside_cache=OK, flush_diet=OK, region_threads=OK, batch_collector=OK, NCDFE=0, pop=VALID, arm=COMPOSED, guarded-marker=N/A (pre-S7-170 head), strict-violated=False -> **PASS**
- PG-T2: threw=1, unexpected=0, s7180-class=1, TPS-поллов=5 -> **FAIL**
- PG-T3: runner=6782504 (широкий банд 6000000..9500000: OK), median5=1.6
  DUAL BAR (v8-REGRESSION): normalized=-1.5%, absolute=+0.0% (бар: ОБЕ >= +10%; оси страхуют index-конфаунд в обе стороны)
  -> **< +10% хотя бы по одной оси** -> лейн ОТКРЫТ -> #14 v2b travel-math (RECON-21 контракт: handleRelativeFrictionAndCalculateMovement 48 строк + travelInFluid + getInputVector) — реализация+диспатч в том же тике
- PG-T4: young=158, Full=0 -> **PASS**
- PG-T5: park=168/1181 (14.2%; класс ~1.4%) -> PASS/N/A

## VERDICT: **LANE-OPEN**
