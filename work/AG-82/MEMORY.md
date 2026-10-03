# MEMORY AG-82 (≤15 строк уроков)
1. w525-526: bugged-парсер md5 cf658e25 (re.match dims) — 86 bench-ног false-FAIL; sha-вериф обязателен.
2. ci-flood w526: paths-ignore-фикс @master 0c307679 — board-append=push=rebuild ~10.7 мин/нога.
3. Листинги /actions/runs?status= — порядок нестабилен; свежесть = точечный GET run-id по updated_at.
4. contents-API CAS = единственный канал доски; board_put_guard.py v2 = канон (dedup+floor+post-verify).
5. Zero-code ветка: tree-чек API (≥3200) → POST /git/refs full-sha; файлы — PUT ?ref=ветка + blob byte-eq пост-вериф.
6. w528: fixed `sleep N` перед чтением console-FIFO лога = гонка-класс (G-DATAPACKS sleep-6 AG-52): лечится предикат-поллом, НЕ увеличением сна.
7. G3-фикс w528: poll SAME-предиката 2s×30; verdict/log-формат байт-неизменны → joblog-харвестеры не ломаются.
8. Red/green сим: точный блок через source + fake-server фон; фон-процессы в тестах ДЕЛАТЬ >/dev/null (stdout-пайп держит bash-тул → 2 таймаута).
9. bc в сандбоксе нет — awk BEGIN{printf}; diff exit1 = норма.
10. Волна-528: рендер съедает bare-скобки в выводе — bracket-вердикты только hex/compile/byte-eq (AG-45/64/79 канон).
