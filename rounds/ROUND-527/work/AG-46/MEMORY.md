# AG-46 MEMORY (<=15 уроков)
1. contents-GET у 1-100MB файлов отдаёт content:"" enc:none — b64decode без проверки = clobber.
2. git/blobs/{sha} читает до 100MB — единственный безопасный GET большой доски.
3. v1 guard AG-333 закрывал только урезание; dup-гонка и >1MB-wall остались.
4. Idempotent-dedup делает retry-409 и штампед безвредными: no-op = PASS.
5. post-verify exact-once (count==1) ловит и потерю, и дубль сразу.
6. Self-test drill'ы: floor-stub/long-line/newline/fallback-fp/fallback-wall/dedup — 5/5.
7. blobcheck = live byte-eq contents vs blob — репетиция пути без PUT.
8. Ад-хок board_put_agN.py — источник всех 3 классов каскада; ходить только через guard.
9. Worktree-канон: --no-checkout + read-tree -mu + sparse scripts = 30MB, ls-tree >=3200.
10. v1 live-fire на 561KB PASS — инструмент жив, его просто не используют; канон-тексты на доске.
