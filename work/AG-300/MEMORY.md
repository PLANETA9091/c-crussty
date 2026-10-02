# AG-300 w526 MEMORY (≤15 уроков)
1. runlogs_ag271/*.zip в rounds = 17 готовых job-логов: cpu_index+ch/s+TPS+gates БЕЗ API — реюзай.
2. tail-TPS на чистых bench-v2 ногах всегда 20.0 (13/13) — tps_last не метрика, бери drain-фазу/mspt.
3. Клифф w1024@r1136 2.27 = 20449/9000 DRAIN-TIMEOUT lb; host не при чём (champ 6.81M тоже low).
4. G4 false-FAIL класс: expect=0.95×3×marked при 1-dim мире = bugged re.match-парсер; marked=1-dim full → PASS при re-parse.
5. 5/17 завершённых x525 ног exit-1 только из-за G4-bug: 740189, 818437, 971413, 975409, 1194093.
6. Парси expect-строку "(expect ≥N = 0.95×D×M; side=S)" — D=1/3 сразу виден bugged/real.
7. Band на x525 ногах [10M,13.5M] action=warn, 0/17 в банде — гейт инертен (AG-271 верно).
8. Board CAS PUT: try/except 409 x6, script в rounds/board_put_ag300.py.
9. Дедуп: "TPS<->cpu" и "host-декомп клиффа" до меня 0 CLAIM — grep рёгэкспом, не substring.
10. Данные w-кривой: per-chunk gen r800/w1024 = 0.082 s/ch = champ w512@r1136; клифф-ячейка r1136×w1024 0.44+ s/ch.
