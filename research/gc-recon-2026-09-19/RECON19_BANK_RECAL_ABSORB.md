# RECON-19 - absorb BANK RE-CAL lega s7178 (run 35457901949, head df6b3c4)

- PG-RC1: region_steal=0✓, bu_defer=0✓, inside_cache=1✓, flush_diet=1✓, region_threads=4✓, batch_collector=1✓, NCDFE=0, pop=VALID, ARMED-маркер=есть -> **PASS**
- PG-RC2: runner_cpu_index = 8493973 (s7169=8566450, s7177=6874223; пара валидна при ±5%)
- PG-RC3: TPS линий=3 median5=None (якорь v3-RECAL; ожидание 1.4-1.8); серия: [1.4, 1.4, 1.4]
- PG-RC4: young=142 (≤174) Full=0 -> **PASS**
- PG-RC5: **N/A** — wall-сэмплов нет
- ВЕРДИКТ: **ЯКОРЬ v3-RECAL ЗАПИСАН** (median5=None, runner=8493973, park None/None); NEXT: DISPATCH s7179 (region_steal=1 + bu_defer=1) → парный вердикт: нормализованная дельта ≥ +10% (обе ноги ±5% runner) = GREEN banking v4, иначе лейн открыт с рычагом #14 (TRAVEL-ALLOC-DIET)
