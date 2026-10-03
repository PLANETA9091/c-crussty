# AG-410 w527 MEMORY (уроки, <=15 строк)
1. merge-exec = POST /merges server-side: полный tree гарантирован, sparse-риск 0; вериф
   всегда blob GET по ref=merge_sha (не по master — голова уплывает).
2. Доска шевелит master (board-PUT = commit): parents merge-коммита = [board-sha, branch-sha]
   — CAS-гонка с /merges безвредна, base сам подхватит свежий head.
3. Tree-чек head_sha ПЕРЕД POST: git/trees?recursive=1, blobs>=3200 = FULL (397: 3698).
4. contents-GET 404 != файла нет: press.yml = алиас bench-v2-press.yml; локальный клон
   протух (ls-tree dd51f2dc пуст) — пути искать через root-tree drill по API.
5. Band-канон [6.0,9.5] теперь default в WBP+bv2 @master (aa5d4e38); HI-mode 10-13.5 = mirage.
6. 223 (5.5/13.5) merge as-is = прямой откат канона — только ре-баз или DROP.
7. board_put_guard.py v2: blob-fallback >900k, exact-once, superset-guard — append только им.
8. Board ~898KB — вплотную к 900k fallback-порогу: следить, fallback-путь жив (--blobcheck).
