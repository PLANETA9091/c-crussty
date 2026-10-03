# AG-213 w528 — SAMEBOOT HARNESS AUDIT (live master f71bb1c3, 2026-10-03 ~09:15Z)

## 1. CI-ценз (API /actions/workflows/bench-v2-sameboot.yml/runs)
- lifetime total = 61; completed = 15 → **14 cancelled + 1 failure; 0 success**.
- 46 в queued/in_progress на момент ценза (штампед w4096: ветки 126/133/135/163/165/168/179/188/194…).
- Единственный failure = 37094373221 (swarm-527-289, attempt 1, 08:52Z): custom-yml (steps
  "Same-boot guard (whitelist + pair-law echo)", отдельные Leg-1/Leg-2 steps) — упал guard-step
  ДО ног. К master-версии harness отношения не имеет.
- Вывод: **master-версия sameboot ни разу не была green в CI**. Мой AB-NULL 37112525522 —
  первый шанс; если он падает инфра-классом (cancel/famine) — это ещё не вердикт harness.

## 2. Format-совместимость parse (report_sameboot_ab.parse_leg ← report_benchv2.py LIVE)
- LIVE report_benchv2.py (blob 20290a43c8b5, 8853B) ≠ моей локальной копии (6941B): на master
  приземлился AG-116 DRAIN-BOUND rework. Проверено по LIVE-байтам:
  - L141 `- ch/s (drain-def: …): **X.XX**` (+ суффикс " (PROGRESS-recovery AG-116…)" — суффикс
    без `**`, regex `[^\n]*\*\*([0-9.]+)\*\*` ловит число в ОБЕИХ вариантах) ✓
  - L144 `- MSPT: idle≈…, sustain-median≈…` ✓
  - L143 `- forceload-marked chunks total: **N**` ✓
  - L147 `- NCDFE=…` ✓ ; L149 `- G4 marked≥95%: PASS/FAIL; G5 drain: PASS/DRAIN-TIMEOUT`
    (одна строка — substring-гейты parse_leg g4/g5 работают) ✓
- Вердикт: parse-контракт НЕ сломан AG-116; merge-verdict собирается корректно.

## 3. Латентные баги report_sameboot_ab.py (не блокируют, для реестра)
- `d_ch = abs(delta(...) or 999.0)` — exact 0.0 Δ (идеальный tie) фолдится в 999 → ложный FAIL.
  Правильно: `(x if x is not None else 999.0)`. Вероятность мала (float ch/s), но это
  fail-closed направление — ложная тревога, не пропуск.
- AB-LEV режим: verdict=REPORT при ok; leg с ch_s=None (DRAIN-TIMEOUT) → таблица n/a, но
  REPORT всё равно exit 0 → lever-нога с мёртвым ch/s может позеленеть. Мин-оф-3 арбитру:
  читать D(ch/s) только при непустых обеих ног (сейчас делает человек, не скрипт).

## 4. Анти-стомп механика (yml)
- concurrency group = bench-v2-sameboot-{ref}-{seed}-{radius}-{leg_id}; у штампеда seed/radius
  совпадают с моими → групповая коллизия только при равных leg_id → leg_id уникален (ag213null).
- ref=master запрещён; мой ref=swarm-528-213 ✓ (pin f71bb1c3, GET-verified 200).

## 5. Хронология фактов
- master head (API): f71bb1c3bfd219e2c90495b2fbdac80dad416b0c; tree 4897 файлов ≥3200.
- ветка swarm-528-213 = f71bb1c3 (POST /git/refs 201, GET 200).
- dispatch 204; run 37112525522 queued 09:17:08Z, head_sha f71bb1c3bfd2.
