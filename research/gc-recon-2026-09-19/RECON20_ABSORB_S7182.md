# absorb #14 TRAVEL-ALLOC-DIET v1 s7182 (run 35461477587, head 0abaaaf) — DUAL BAR

- PG-T1: zero_alloc=BAD:False, skip_store_bb=BAD:False, region_steal=OK, bu_defer=OK, inside_cache=OK, flush_diet=OK, region_threads=OK, batch_collector=OK, NCDFE=0, pop=VALID -> **FAIL**
- PG-T2: threw=0, unexpected=0, s7180-class=0, TPS-поллов=6 -> **PASS**
- PG-T3: runner=6680195 (якорь 8493973, класс ±5%: ВНЕ КЛАССА), median5=1.6
  -> **ВНЕ КЛАССА ПАРЫ** — лег невалиден для пары (банд-гейт должен был отсеять; проверить inputs cpu_band_min/max)
- PG-T4: young=158, Full=0 -> **PASS**
- PG-T5: park=153/1182 (12.9%; класс ~1.4%) -> PASS/N/A

## VERDICT: **INVALID-PAIRING**
