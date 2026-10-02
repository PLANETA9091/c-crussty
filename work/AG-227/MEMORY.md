# AG-227 MEMORY (≤15 уроков)
1. run_started_at у queued-рана = creation-time, НЕ старт джобы — не считать «стартами».
2. При штампеде «newest-N» пагинация скрывает терминалы; тоталы брать status=+created= фильтрами.
3. Bugged-парсер 39bafb8a(5078B, re.match-dims) на 8/10 live-рефов; FIX 17f6349b @a9ff088f; v3 aa4d8cf6 @e965bd27 superset.
4. Дозы POST — на a9ff088f/e965bd27; bugged-арты ре-грейдить kit-ом AG-42.
5. Дедуп CLAIM по подстроке ловит чужие волны — регэксп + волна.
6. Ноль локальных git-коммитов; доска только CAS; 0 worktree создано — уборка не требуется.
