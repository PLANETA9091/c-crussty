# absorb #14 TRAVEL-ALLOC-DIET v1 s7182 (run 35462686159, head 9afd098) — DUAL BAR

- PG-T1: zero_alloc=OK, skip_store_bb=OK, region_steal=OK, bu_defer=OK, inside_cache=OK, flush_diet=OK, region_threads=OK, batch_collector=OK, NCDFE=0, pop=VALID -> **PASS**
- PG-T2: threw=3, unexpected=0, s7180-class=1, TPS-поллов=5 -> **FAIL**
- PG-T3: runner=7072550 (якорь 6680195, класс ±5%: ВНЕ КЛАССА), median5=1.7
  -> **ВНЕ КЛАССА ПАРЫ** — лег невалиден для пары (банд-гейт должен был отсеять; проверить inputs cpu_band_min/max)
- PG-T4: young=136, Full=0 -> **PASS**
- PG-T5: park=149/1191 (12.5%; класс ~1.4%) -> PASS/N/A

## VERDICT: **INVALID-PAIRING**
