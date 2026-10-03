# SPEC: SAME-BOOT A/B harness, AG-361 w527 (branch swarm-527-361)
## Коммиты (API-only, contents-PUT)
- 296f9b7f41 .github/workflows/bench-v2-sameboot.yml
- 7f940efe61 bench/worldv2/run_benchv2_sameboot.sh
- e1ca3a0af5 bench/worldv2/report_sameboot_ab.py
База: master 7d5182a910e8586d52fdc088d8e403837f3642fe (tree 4743 файлов, чек PASS).

## Контракт
- Входы yml: run_seconds/leg (def 1800), ab_null (def 1), leg_b_vars (def ''),
  r/seed/xmx/dims/dgw/dcp как bench-v2. Timeout job 330 мин (GH hard 360):
  2 ноги x (boot ~2 + pregen ~28 + run_seconds + report) — run_seconds<=5400 ок.
- Leg B env-дельта: wrapper экспортирует K=V из AB_LEG_B_VARS ДО второго вызова
  run_benchv2.sh; leg A наследует базовый env шага. AB_NULL=1 игнорирует дельту.
- run_benchv2.sh НЕ модифицирован (мастер-канон неприкосновенен); re-entry через
  BENCH_WORK=runA/runB. Артефакты: BENCHV2_AB.md, BENCHV2_LEG_{A,B}.md,
  run{A,B}/server/{BENCHV2.md,server-stdout.log,run-env.txt,*.sparkprofile}.
- Парсинг BENCHV2.md (report_benchv2.py форматы): ch/s '**X.XX**', 'MSPT: idle≈X,
  sustain-median≈Y', 'TPS samples ... last=Z'. D = (B-A)/A*100.
- Вердикты: AB-NULL PASS<=10%/WARN<=25%/FAIL>25% (|D(ch/s)| и |D(mspt_med)|,
  обе ноги G4 PASS + NCDFE=0 обязательны для PASS); AB-LEV = REPORT.
- Гейт job-а: VERDICT: FAIL = красный job (x519 dud-gate канон).

## WBP-порт (фаза-2, lever-пары fd/ic pop50k/150k — recipe)
run_world3.sh (WBP) читает levers из env, экспортируемых yml-шагом
(gc_tune/inside_cache/flush_diet/region_threads/batch_collector/... —
inputs L60-130 world-bench-parallel.yml). Порт = копия wrapper с
run_world3.sh вместо run_benchv2.sh + AB_LEG_B_VARS='FLUSH_DIET=0' пример;
concurrency-группа bench-wbp-sameboot-*; ПЕРЕД портом — canary AB_NULL=1.
Цель: закрыть fd Δ-13.3% загадку (AG-204/212: сигнал = шум? same-boot решит).

## Ограничения/риски (честно)
- Внутри-VM boot-to-boot drift не измерен до canary — бары 10/25 прereg, не факт.
- Второй pregen на той же FS: диск-кэш теплее — если D(ch/s) сдвинут вверх
  систематически, AB-LEV читать по знаку консервативно (медленная нога = первая).
  Решение фазы-2: чередование A/B порядка (leg_swap) на 2-й и 3-й паре.
- Пины CDN (purpur/terralith/...) перечитываются в каждой ноге: если CDN
  ротирует между ногами — G-пины уронят ногу B честно (fail-closed, не другая
  версия тихо).
