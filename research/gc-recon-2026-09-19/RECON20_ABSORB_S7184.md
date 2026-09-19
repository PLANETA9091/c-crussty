# absorb SLOW-CLASS BANK s7184 (run 35461477587, head 0abaaaf)

- PG-R1: zero_alloc=OK, skip_store_bb=OK, region_steal=OK, bu_defer=OK, inside_cache=OK, flush_diet=OK, region_threads=OK, batch_collector=OK, NCDFE=0, pop=VALID -> **FAIL**
- PG-R2: threw=0, unexpected=0, NPE=0, TPS-поллов=6 -> **PASS**
- PG-R3: runner=6680195 (банд 6500000..7200000: В БАНДЕ), median5=1.6
  -> **ANCHOR-SLOW ЗАПИСАН: median5=1.6 @ 6680195** — пары #14 дальше в медленном классе (ДВОЙНОЙ БАР vs этот якорь)
- PG-R4: young=158, Full=0 -> **PASS**
- PG-R5: park N/A tolerated (AP-PID дефект)

## VERDICT: **ANCHOR-RECORDED**
## ANCHOR-SLOW: median5=1.6 @ runner=6680195
