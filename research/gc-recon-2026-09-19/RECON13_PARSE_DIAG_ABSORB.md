# RECON-13d — absorb переписи chunk-parse лега s7168 (run 35435848509, head 1fc46c3)

- PG-D0: **FAIL** — chunk-parse-diag.txt отсутствует/без SUMMARY (доставка диага дефектна; вердикт по кэшу НЕВОЗМОЖЕН)
- PG-D1 доставка: pop VALID; NCDFE=0; OOM=нет; parse_diag patch=ARMED -> PASS
- PG-D2: TPS last-5 медиана=1.4 (база s7165=2.0, серия n=6: [18.9, 1.1, 1.2, 1.5, 1.4, 1.9]) -> FAIL
- PG-D3: N/A — remset.log в артефакте отсутствует (remset-диаг не включён в диспатч s7168; дефект пегистера TASK-327, не сервера)
- ВЕРДИКТ: **ДЕЛИВЕРИ-ДЕФЕКТ** — перепись не доставлена; чинить доставку (CRUSSTY_PARSE_DIAG_FILE/upload), повторный диаг-лег; кэш-решение отложено
- GC-фон: young=155 full=0 суммарно=19.5s max=179ms (база s7165: young=161 full=0 20.3s max=189ms)
