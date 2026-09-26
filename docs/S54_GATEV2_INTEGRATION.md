# S54 / ROUND-468 — gate_v2 → merge-канон: интеграция, сухой прогон на мастере, док-канон МЕРЖ №11

Автор: суб-агент S54 (плоский рой v20). Ветка артефакта: **round-468-s54-gatev2** (vendor gate-скриптов поверх
origin/master c1196321 + этот док). Мастер не тронут (0 код-дельт; коммит собран plumbing-ом, без чекаута).

## 1. Сухой прогон merge_safety_gate_v2 на мастере (0-дельта)

Стенд: sparse-зеркало master c1196321 (1006 tracked из 2902: research 669M + bench 84.5M исключены — диск бокса
100%/0 free, канон C77.4; ВСЕ 178 *.sh и ВСЕ 51 блоб *Ops.class-плоскости materialized; cargo-deps полные).
Гейт: round-467-s36-gatev2@fc5df459, vendored без изменений. Тёплый таргет: GATE_CARGO_TARGET_DIR=/home/z/c-crussty/target.

**Вердикт: GREEN (FAIL=0 WARN=4), total 83.3s** — совпадает с selftest S36 на 04d58e6c (GREEN FAIL=0 WARN=4).
WARN=4 — известный legacy-плоскости noise/ major=52 вне мостов (tier-severity: вне-моста=WARN, не блокирует).

| фаза | ms (dry-run на мастере) | selftest S36 @04d58e6c | status |
|---|---|---|---|
| P1 cargo check (тёплый таргет) | **378** | 31 | OK 0 err |
| P2 blobs (check_blobs_sync) | **60291** | 30700 | OK ALL IN SYNC |
| P3 case-arm (bash -n 178 + scan) | **2025** | 751 | OK 0 FAIL / 0 WARN |
| P4 ncdfe (selftest 4 + 10 мостов + sweep 51) | **20467** | 5300 | OK ok=4/0, мосты 10/0, stale 4 (FAIL-тир 0) |
| P5 маркер-ценз | **99** | 193 | OK candidates=0 |
| total | **83300** | ~37000 | GREEN |

RESEARCH-числа (10): P1 378ms · P2 60291ms · P3 2025ms · P4 20467ms · P5 99ms · total 83300ms · --fast **2993ms**
(×27.8 к полному) · negative-selftest 3/3 PASS (81.4s) · STRICT_PLANE=1 → RED, exit=1, FAIL-тир 4 (4 legacy noise
major=52: tier-эскалация проводки подтверждена) · standalone-тайминги: check_blobs_sync 68.3s,
case_arm_scan 178 sh 1.24s, ncdfe --selftest 6.1s, ncdfe 10 мостов 12.0s.

Оверхед оркестрации: сумма фаз 83.26s ≈ total 83.3s → **<0.1s / ~0.1%** на сам гейт (чистая композиция).
Exit-проводка: GREEN→0, RED→1 (проверено живым прогоном STRICT_PLANE). Обёртка `-- CMD` (setsid+exec>file,
Л141-УРОК-1) проверена detached-спавном: лог-файл создаётся, pid отвязывается.

## 2. Связка с ncdfe_guard / case_arm_scan / check_blobs_sync — матрица дублей

Вывод: **гейт НЕ реимплементит ни один инструмент — только оркестрация + 3 новых значения.** Vendored-копии
byte-identical канону (пруф по blob-sha):

| фаза | инструмент | blob-sha (gatev2 = канон) | дубль? | что ДОБАВЛЯЕТ фаза сверх standalone |
|---|---|---|---|---|
| P1 | cargo check | n/a | было ручным шагом мержа | автоматизация + GATE_CARGO_TARGET_DIR (тёплый таргет, 378ms vs cold rebuild) |
| P2 | check_blobs_sync.sh | 32e7bb96 = master | НЕТ (вызов по subcontract) | ничего — единая точка вызова |
| P3 | case_arm_scan.py | e7087a96 = round-466-c77-delivery | НЕТ (вызов) | **+ bash -n всех 178 tracked *.sh** (синтаксис-ценз, standalone не делал) |
| P4 | ncdfe_guard.sh | 30d1b84d = round-466-c84-ncdfe | НЕТ (вызов ×2: --selftest + мосты) | **+ major-сweep плоскости 51 блоб** (major!=65; мост=FAIL/вне-моста=WARN/STRICT_PLANE=1→всё FAIL) — в standalone ncdfe_guard НЕТ |
| P4 | авто-локация мостов | — | — | по basename (10 имён), не хардкод путей (глубже M09 x467) |
| P5 | — | — | — | **новая фаза**: конфликт-маркеры <<</>>> + ^=======-пара + branch-glue origin/(round-N|master|HEAD) в rs/java; git grep-префильтр (99ms) |

Дедуп-правило для МЕРЖ: отдельные прогоны check_blobs_sync / case_arm_scan / ncdfe_guard ПЕРЕД гейтом не нужны
(P2/P3/P4 уже их вызывают; повторный прогон = +80s впустую). Исключение: rebuild блобов (build_456b_blobs_all.sh)
обязателен ДО гейта — гейт не пересобирает, только верифицирует.

Ловушка автолокации: MobPushOps живёт в 2 местах (mobpush/ и sscan/) — гейт берёт head-1 (mobpush), raw-скан даёт
11 файлов вместо 10. Не дефект (sscan-твин legacy), но при чистке sscan/ автолокация не должна потерять мост.

## 3. Док-канон МЕРЖ №11 (команды)

Пререквизиты (канон): пара ≥+20 min-of-3, Δcpu ≤50k, pair-fresh (якорь только из предыдущего МЕРЖа), NCDFE T1=0,
parity_validator 12/12, HOST-ценз STW>23s→re-roll. Кандидат 3-го якоря climb5-p32: окно runner [8734563,8834563],
порог anchor ≤+2.99 (есть aD1 +34.55, W10 +26.96 — добрать 1 пару).

```bash
# ===== МЕРЖ №11 — док-канон (gate v2 integrated), S54 / ROUND-468 =====
# 0) гейт-скрипты взять из ветки round-468-s54-gatev2 (vendor byte-identical c84/c77/master)
cd /home/z/c-crussty && git fetch origin
# 1) рабочее место (диск-канон C77.4: НЕ полный чекаут; сейчас диск 100% — только /dev/shm/sparse)
git worktree add --no-checkout /dev/shm/wt-m11 -b round-468-merge11 origin/master
cd /dev/shm/wt-m11 && git checkout   # (или git sparse-checkout при дисковой тесноте)
# 2) merge --no-ff кандидата (leg-ветка с ≥+20 pair, min-of-3)
git merge --no-ff --no-edit round-468-<leg>
# 3) rebuild блобов ДО гейта (канон ×466-урок-4: KERNEL_JAR фиксирован, не find-скан)
KERNEL_JAR=/home/z/tools/patched-kernel.jar bash scripts/build_456b_blobs_all.sh
# 4) ГЕЙТ v2 — 5 фаз одной командой; RED => стоп, ничего не пушим
GATE_CARGO_TARGET_DIR=/home/z/c-crussty/target \
  bash scripts/merge_safety_gate_v2.sh --check-only --json /dev/shm/gate_m11.json \
  || { echo "MERGE-11 ABORT: gate v2 RED"; exit 1; }
#    (P1 cargo 0 err + P2 blobs ALL IN SYNC уже покрыты — отдельные прогоны НЕ запускать)
# 5) push ТОЛЬКО merge-ветки; canary после (world-bench-parallel, банк-EXACT ваниль-инпуты)
git push origin round-468-merge11
# 6) негатив-контроль детекторов (раз в тик; обязан быть 3/3 PASS)
bash scripts/merge_safety_gate_v2.sh --selftest-negative
# Режимы: --fast 3.0s (итерации до готовности) · STRICT_PLANE=1 только для задачи
# «вся плоскость release 21» (на текущем мастере RED: 4 legacy noise major=52)
```

## 4. Ограничения сухого прогона (честно)

- Индекс зеркала 1006 файлов из 2902: тела research/ (669M) и bench/ (84.5M) не materialized (диск 100%).
  P3 sh-ценз 178/178 и P4 плоскость 51/51 — ПОЛНЫЕ; P5 git-grep-префильтр покрывал только materialized-множество
  (весь код + все .sh + docs; research/bench = логи/бинари, master = чистый LEDGER-коммит — риск маркеров там ~0).
- P1 cargo: 0-дельта на мастере с тёплым таргетом → 378ms fingerprint-hit; это замер интеграции, не холодной сборки.
- Норма-канон МЕРЖ №11 (пара ≥+20) — вне скоупа S54: гейт не меряет TPS, он мерит безопасность мержа.
