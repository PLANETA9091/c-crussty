# RECON-14 — absorb INSIDE-DIET gate лега s7174 (run 35448586519, head 6dae474a9288)

- PG-ID2: pop VALID, NCDFE=0, bridges=есть, stage=composed, chain=ARMED, region_threads=ARMED, batch_collector=defined, OOM=нет -> **PASS**
- PG-ID3: TPS lines=5 median5=1.60 (гейт ≥1.60 -> PASS); дельта vs банка 1.80 = -11.1% (LEVER-BOOST: <10% — лейн НЕ закрыт (v7))
- PG-ID4a: inside-alloc лейн 16.48% (база 31.18%, samples=10294) дроп=+47.2% (гейт ≥8% -> PASS, ожидание 10..25%)
- PG-ID4b: young=123 (гейт ≤154), Full=0 -> **PASS**
- PG-ID4c: inside-CPU лейн 3.02% (база 9.20%) дроп=+67.2% (гейт ≥3% -> PASS)
- CRASH-FREE: Entity threw exception=1 (≤5) -> **PASS**
- ИТОГ гейтов: PG-ID2=PASS, PG-ID3=PASS, PG-ID4a=PASS, PG-ID4b=PASS, PG-ID4c=PASS, CRASH-FREE=PASS
- **ВЕРДИКТ: гейты PASS — банкинг v4 = v3 + inside_diet=1 (harm-floor OK); НО буст <10% → по v7 лейн НЕ закрыт: v2 (walk-транскрипция) — следующий рычаг**
