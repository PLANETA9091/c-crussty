# AG-125 MEMORY (волна-526, ≤15 строк)
1. PSUM-строки >120 симв убивают финал-скрипт ДО append — считать len() до assert, не после 40s диспатчей.
2. Shell-сессия сбрасывает cwd между вызовами — git только через `git -C`, пути только абсолютные.
3. grep -r по /home/z/rounds висит >120s (гигабайты wt) — только rg/Grep-tool с glob *.md по claims/.
4. boundary-regex гонка-гейт: токены с s-префиксом не ловят AG-<N>; pop500k+s900 чужих CLAIM не было.
5. WBP pop500k@s1800: wall ~65 мин < 70 step — inject@3M проходит (AG-97), риск принят, timeout=CENS.
6. Клетка pop500k x s>300 пуста при полной s-оси@150k — NEW-класс 2D-взаимодействий дешевле мидов.
7. Run-capture: pre-snapshot id-set + diff по head_branch — 2/2 поймались с первого GET.
