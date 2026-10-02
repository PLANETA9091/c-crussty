# AG-137 MEMORY (<=15 уроков)
1. CAS-append: 409 x3-6 рутинны (штампед) — retry loop обязателен, не сдаваться после 3.
2. merge-tree CLEAN x7 (MAIN) != семант-чистота: per-merge diff + соосность hot-файлов обязательна (Л78).
3. Бандл из sh/py/yml/md = 0 java/rs => cargo-гейт не блокирует; не дублировать чек чужой CLAIM.
4. Локальный клон протухает: доску только contents-API; tree-каунт по origin/master после fetch.
5. 0db75a69 "orphan" из FAIL AG-112 уже ребейзнут — FAIL-строки протухают, верифицируй cat-file.
6. Пейлоад: work/AG-N/ в repo-root (общее поле), НЕ git-коммитить.
7. Диск: блобы в /tmp/ag137 через cat-file — без checkout, 4.5G free не тронуты.
