# PREREG 478-B5 — chk-19 3-й якорь (закон 14a/16, гейты BANK_V5_FREEZE §2 v5-FROZEN)

Тик ×478, база master 92443914 (дрейф от 2700571f docs-only+dispatch-скрипты, 0 Java/Rust).

## CLaim-гейты (заморожены ДО минирования, x464 — не двигать)
- Носитель: chk-19, leg_v5 = **+16.11** (лег +12.8@6733439, эра ×460).
- Порог якоря: **norm_v5 ≤ −3.89** (≡ pair = 16.11 − norm ≥ +20), окно **GLOB 6.0–9.5M**.
- Существующие легальные пары: **22.5→23.2 (a22 −9.7@6729244)** и **22.6→23.3 (a53 −9.8@6737567)** = 2/3;
  нужен ≥1 новый ваниль-якорь с norm ≤ −3.89 → **PAIR min-of-3 = min(23.2, 23.3, 16.11−norm) ≥ +20**.

## Диспатч (3 ваниль-якоря, 0 код-дельт)
- Алиасы **round-478-b5-a1..a3** @master 92443914, lever_flag/lever_arg ПУСТО (master default = vanilla).
- Canon x466-C98 ЯВНЫМ JSON (урок C66-C72/C73): 640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G/band[6000000,9500000].
- Runs: a1 36367088466 / a2 36367092915 / a3 36367098236 (01:43:47Z).

## Absorb-гейты (A1-метод, Л-478-A1.1 канон)
1. cpu_index ТОЛЬКО из run-env.txt (Л195); band 6.0–9.5M иначе BAND-DEAD (fast-fail/discard, не вердикт).
2. polls-median = медиана TPS-поллов "TPS from last 5s" < 15.0 (канон C55); norm_v5 = 100·(med/tps_exp_v5(cpu)−1),
   tps_exp_v5 = линейная интерполяция узлов §2 (6.5M→2.1252 … 9.0M→2.6280); <6.5M — линейная экстраполяция (канон c42).
   Robust-вариант (не вердикт-носитель): c42-Л201-локальный узел [6.9,7.2]M=2.1293.
3. HOST-ценз M1: STW_total > 23.0s ИЛИ avg_pause > 200ms (gc.log completion-строки, без gc,phases) → **HOST-CENSORED = не-якорь**
   (в-точка deep-host-класса в банк §3.3 VALID, в пары НЕ идёт).
4. Ваниль-валидность: lever_flag=∅ в run-env; NCDFE=0; AIOOBE=0; FIXTURE-VALIDITY VALID (BOTTLENECKS_3.md).
5. Вердикты: anchor_norm ≤ −3.89 ∧ CLEAN ∧ vanilla-valid → **[478-B5] PAIR {min-of-3, маржи}**; иначе MISS-NORM/MISS-WINDOW;
   runs > 15 мин → вердикт **DISPATCHED run-id** (абсорб след. тиком).
6. Все ваниль-VALID в-точки (включая ценз-класс по §3.3) идут в банк; пороги §2 НЕ пересматривать (BANK_V5_FREEZE §5).
