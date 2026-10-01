# AG-446 — Динамический gate-replay фиксовых веток canary-харнесса (волна-519)
Метод: НЕ статика (bash -n/py_compile) + локальный replay sha512-гейта с фейковым pin-архивом
(GNU coreutils = ubuntu runner); 0 POST, 0 диспатчей, repo-код не тронут (чтение git show только).

## Эмпирический закон sha512sum -c (воспроизведён локально)
- ≤2 пробела hash→name: parse OK (1sp и 2sp оба валидны — stellarity-строка с 1sp НЕ баг)
- ≥3 пробела: имя парсится как ' tectonic.zip' → "FAILED open or read" rc=1 → FAIL=1 → exit 42 ДО boot

## Блокер-цепь master (2f795c64 / 876b3f45)
1. tectonic-строка L57 = 3 пробела → детерминированный PIN-GATE abort (run-36788080912/83370 RED) — подтверждено байтами + replay
2. report_benchv2.py НЕ КОМПИЛИРУЕТСЯ: SyntaxError walrus-rebind 'm' (L13/15) → даже при живом мире вердикт-пайплайн мёртв (0-dims дадл) — подтверждено py_compile
3. DimForceloadPlugin Chunk[].size() — компил-баг на master (clean в 426/207)

## Матрица origin-веток (gate_matrix.tsv): ONLY origin/swarm-519-207 = FULL-PASS
- 54: tectonic-фикс (printf) ✓, report_py ✗, dimforce ✗ → частичный
- 426: tectonic ✓, report_py ✗, dimforce ✓ → частичный
- 354: tectonic ✓, report_py ✗, dimforce ✗ → частичный
- 207: tectonic ✓ + report_py walrus/G3-pack-regex фиксы (py_compile PASS, семантика честная: distinct-pack count с 'data packs enabled'-фильтром) + dimforce ✓ + cpu-guard ✓ → **единственная origin-ветка, закрывающая всю офлайн-тестируемую цепь**
- 217/433: локальные, base ДО pin-gates (нет cpu-guard, нет pin-строк) → без rebase не мержибельны

## РЕКОМЕНДАЦИЯ координатору
Мёрж origin/swarm-519-207 в master → canary re-dispatch. Остаточный риск: boot-стейдж локально
не верифицируем (2-CPU правило); G-TECTONIC/DIM/HB гейты в Report gate — смотреть final_chunks>0.
