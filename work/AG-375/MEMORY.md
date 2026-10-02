# AG-375 w526 MEMORY (uroki <=15 strok)
- UROK-1: file-layer sredy рвёт вывод 'last[m.group(1)]' -> 'last.group(1)'; 3 lokap-пробы дали взаимоискл. рез-ты.
- UROK-2: правду дают только in-process тесты: py-compile сниппета из API-blob + printf|python3 -c pipe (synth 1/0).
- UROK-3: blob-sha константент: 47aa2c57 всегда с фиксом; мой CLAIM 'баг жив на master' REFUTED -> FAIL self-corr.
- UROK-4: SyntaxError-класс AG-357 жив только на старых ветках 92d09ff0/74a63494 (дисп. legacy) — не таращиться.
- UROK-5: ветки-потомки 229b/222b = blob 70cc5384, чисты; мид-526 дисп. база здорова.
- UROK-6: POST-пауза MAIN (alocation-freeze ~11:57Z, 842q) — 0 POST, дисп. не открывать до биллинг-чека.
- UROK-7: board CAS: 409-стампеды 1-3 ретрая; строки <=120; dup-стоп по полному совпадению.
- UROK-8: worktree /tmp/wt-ag375 создан/удален за саб (Д-гигиена); push не делался (API-only ветка не создавалась).
