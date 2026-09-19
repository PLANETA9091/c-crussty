# absorb #10 ZEROALLOC ISOLATION s7194 (run 35470254981, head 1e4f8d6) — PROTOCOL v8-REGRESSION DUAL BAR

- PG-T1: travel_diet=OK, zero_alloc=OK, skip_store_bb=OK, region_steal=OK, bu_defer=OK, inside_cache=OK, flush_diet=OK, region_threads=OK, batch_collector=OK, NCDFE=0, pop=VALID, arm=COMPOSED, isolation=OK, guarded-marker=OK (ОБЯЗАТЕЛЕН: гонка закрыта s7193), strict-violated=False -> **PASS**
- PG-T2: threw=0, unexpected=0, s7180-class=0, TPS-поллов=5 -> **PASS**
- PG-T3: runner=6633963 (широкий банд 6000000..9500000: OK), median5=1.4
  DUAL BAR (v8-REGRESSION): normalized=-11.9%, absolute=-12.5% (бар: ОБЕ >= +10%; оси страхуют index-конфаунд в обе стороны)
  -> **< +10% хотя бы по одной оси** (нога валидна: zeroin COMPOSED sites:3 + threw=0) -> REFUTED под-лейна inside-blocks/fluid-scan (22.3% alloc, RECON-24) рычагом #10 -> свежий RECON следующего GC-под-лейна (jdk-collections/serde 25.5%) до под-лейнов >=5%
- PG-T4: young=140, Full=0 -> **PASS**
- PG-T5: park=117/1187 (9.9%; класс ~1.4%) -> PASS/N/A

## VERDICT: **LANE-OPEN**
