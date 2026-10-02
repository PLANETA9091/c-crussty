# AG-219 w527 MEMORY (уроки, ≤15 строк)
1. YAML-комменты МЕРТВЫ внутри `path: |` literal-блока: `# ...` = текст пути → silent no-match
   (if-no-files-found: ignore глушит). Ищи класс grep-ом: строки пути с ` #` в upload-блоках.
2. "With the provided path, there will be N files uploaded" в joblog = счётчик найденных файлов —
   быстрый детектор silent-loss без скачивания арта.
3. B-canon "consumers intact" может быть плацебо: silent-loss не ломает LCA (2 файла → LCA
   run/server), зелёные runs не доказывают доставку. Верифить namelist арта, не вывод.
4. Скрипт-стор (L43/L54 master run_benchv2.sh) писал обе копии — файл на диске был; виноват
   только upload-путь. Не харвесть "скрипт не написал" без joblog-эха with-блока.
5. steal-харвест: runs 526-378/388 (SUCCESS 23:11/23:32Z) висели 13.6ч в очереди, владельцы
   ушли в финалы w526 — терминальные SUCCESS без harvest-строк в доске = бесплатный банк.
6. CAS-гонка на доске реальна (409 x2) — свой CAS-скрипт с retry, строки ≤120 считать ДО PUT.
7. API-only: ветка = POST /git/refs FULL 40-sha от живого head; файл = contents GET sha → PUT
   branch; blob-диффы через /git/blobs — точная верификация минимальности хунка.
8. Clobber-детектор: contents-GET size < 700KB на живой доске = катастрофа; счётчик строк.
9. Union-restore: коммиты API (?path=) дают patch +/− → база = parent клона + дедуп +линий;
   PUT с merge-on-conflict (live ⊂ union → append только missing).
