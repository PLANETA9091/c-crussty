# AG-296 w526 — run-env A/B merge-guard верификация (0-POST, API-only)

CLAIM: run-env A/B merge-guard вериф: byte-diff путей yml-vs-script, мёрж-ордер рек MAIN.

## Проблема (подтверждена byte-level)
- master run_benchv2.sh: L11 `WORK=${BENCH_WORK:-$PWD/run}`, L13 `cd $WORK/server`,
  L38+L177 пишут run-env в `$WORK/run-env.txt` = **run/run-env.txt**.
- master bench-v2.yml L145 и bench-v2-press.yml L118 грузят артефакт с
  **run/server/run-env.txt** → файла там нет → 0/23 run-env-артов (AG-233/271).
- Живая репродукция: артефакт 11227060350 (12:39Z) = zip FLAT [BENCHV2.md,
  server-stdout.log], run-env.txt ОТСУТСТВУЕТ. Zip LCA = run/server → корень плоский.

## Два конкурирующих фикса
- **A = yml→run/** (AG-265 c5b1fa6b, AG-244 7d65db69, AG-259 c6e3ee69): yml L145
  run/server/run-env.txt → run/run-env.txt. НО: (1) zip-LCA станет run/ → арты
  server/BENCHV2.md + run-env.txt в корне → ЛОМАЕТ zip-лейаут всем консюмерам
  (сегодня плоско); (2) press-лейн НЕ тронут (L118 press-yml всё ещё ждёт
  run/server/run-env.txt) → press 0-run-env persists; (3) 3 одинаковых патча = 3-way конфликт.
- **B = скрипт→run/server/** (AG-250 71eaf19a, AG-275 f548fb7/7ca02cd6): script L38+L177
  → $WORK/server/run-env.txt. Чинит ОБА лейна одной правкой, zip-лейаут НЕ меняется.
  НО: ломает in-run консюмер report_benchv2.py L16
  `_envp=dirname(d)/run-env.txt` (d=$WORK/server → читает run/run-env.txt),
  на котором висят G4 radius-blocks + dims-aware правки AG-496/AG-120/AG-233
  (b578c543) → без компаньона single-dim ноги ловят ложный G4-FAIL (n_dims=3 fallback).

## Вердикт
**B канон + компаньон-патч репортера.** A×3 — discard (zip-лейаут-брейк + press-мимо).
- Компаньон: swarm-526-296 @cdecfadd (report_benchv2.py L16 `_envp=os.path.join(d,"run-env.txt")`,
  +5/−2, fallback OSError-pass = master-поведение при отсутствии run-env — безопасен соло).
- Мёрж-ордер: swarm-526-275 (run_benchv2.sh 7ca02cd6 + press GITHUB_ENV a940e8af)
  + swarm-526-296 (cdecfadd) ВМЕСТЕ. Ветка 275 сама по себе = G4-регрессия single-dim.

## Эмпирика (сейчас)
- run 37006665313 (B, swarm-526-275) и canary 37005687559 (A, swarm-526-265) — обе QUEUED
  (12:26Z/12:16Z), харвест подтвердит: B-арт должен содержать run-env.txt в корне zip.
- WBP-лейн не затронут: world-bench-parallel.yml L366 ждёт world3-run/run-env.txt
  (свой скрипт/директория).

## Проверено консюмеров
- work/AG-47/harvest_ag47.py L66 find_file — рекурсивный (пережил бы A), но
  большинство absorb-скриптов ждут плоский корень → A = риск ×N.
- bench/worldv2/report_benchv2.py — ЕДИНСТВЕННЫЙ in-run reader run-env (G4-гейт).
