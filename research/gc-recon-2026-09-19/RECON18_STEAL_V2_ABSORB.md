# RECON-18 - absorb STEAL v2 gate lega s7177 (run 35455926384, head ecdd484)

- PG-V2: bu_defer=1 ARMED, region_steal=1, BU-DEFER composed marker, NCDFE=0, pop VALID -> **PASS**
- PG-V2a: threw=0, unexpected=0, sendBlockUpdated-NPE=0, alloc-collapsed=5972690B (crash-leg had EMPTY), TPS-поллов=6 (crash-leg 2) -> **PASS**
- PG-V2b: DONE-park 16/1174 = 1.4% (база 121/901 = 13.4%, гейт ≤40) -> **PASS**
- PG-V2c: TPS линий=6 median5=1.40 (гейт ≥1.98, банк 1.80+10%) -> **FAIL**
  - серия: [18.2, 1.1, 1.3, 1.4, 2.1, 2.2]
- PG-V2d: young=166 (≤174) Full=0 -> **PASS**
- ВЕРДИКТ: **REFUTED-by-TPS — буст <10%** (median5=1.40 < 1.98); rollback region_steal=0 + bu_defer=0, лейн НЕ закрыт (v7) → RECON-17 того же лейна
