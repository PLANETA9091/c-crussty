# AG-342 work — волна-522: canary-5 RED классификация + band-gate drift-2 фикс

## Классификация canary-5 пары (curl + job-logs, токен)
- **run-36832349428** (seed 351515): completed **failure**, job 110271621317, умер на **step-3
  "Runner calibration band gate (pair-hunter fast-fail, S7-96d)"** за 44с. Steps 4-6 (JDK,
  BENCH-V2, Report gate) — **skipped**. Артефактов 0. `runner_cpu_index=7073387 band=[10000000,13500000]`
  → `OUTSIDE band — fast-fail (pairing discard)` → exit 1.
- **run-36832346586** (seed 351601): completed **failure**, job 110271611498, тот же шаг за 36с.
  `runner_cpu_index=6977844` → fast-fail. Артефактов 0.
- ref обоих = master bc7e8722 (band-gate v2 merge подтверждён в логе чекаута).

## ВЕРДИКТ (главное)
**Canary-5 RED ×2 = НЕ #16a и НЕ #16b — bench ни разу не стартовал.** Это **#16c-2:
band-gate stale ВТОРОГО поколения** — рекалибровка AG-318 (pool 10.2-12.5M, floor 10.0M)
устарела за часы: текущий пул меряет **6.98-7.07M** (−35%). Оба инварианта волны
(#16a marked=0 / #16b POI-crash) на пуле волны-522 **не проверены** — fast-fail их маскирует.

## Санкционированный канал диспатча: ядро баги
Гейт живёт на **inputs** `cpu_band_min/max` (пустые = гейт off) — дефолты в обоих ymls
(bench-v2.yml:41, world-bench-parallel.yml:152) = 10.0M → **все дефолт-диспатчи волны-522
умирают на гейте**. Координаторский canary-payload передал явное окно 10.0/13.5M — тоже мимо.

## Фикс (ветка swarm-522-342, коммит bf9078c3, tree 3240)
- `cpu_band_min` default 10000000 → **6500000** в bench-v2.yml + world-bench-parallel.yml
  (покрывает наблюдённые 6977844/7073387 + запас; max 13500000 не тронут; pairing-дисциплина
  сохранена; per-dispatch override жив). Комментарии в yml несут улики-руниды.
- Пайплайн канона: worktree --no-checkout → sparse .github/workflows → **read-tree origin/master**
  (локальный master c3bca145 — diverged от bc7e8722, read-tree master был бы миной!) →
  ls-files 3240 → diff --stat 2 файла → commit bf9078c3 → ls-tree HEAD 3240 → push.

## Вериф-нога (DISP)
- **run-36847865424** (bench-v2, ref swarm-522-342, seed 522298 ∈ gate 522001..522299,
  явное окно 6.5M/13.5M, canon radius 1136 / 300s / xmx 10G / dims all-3).
- 1 POST (лимит ≤2 соблюдён, rl-remaining 3921). Concurrency group = ref+seed+radius →
  sibling-cancel невозможен.
- Ожидание: гейт пропускает (~7.0M) → первая bench-v2 телеметрия волны-522 (#16a/#16b
  статус на новом пуле — раньше её не было).
- Скрипт: scripts/dispatch_342_bandfix.py (payload-аудит: живые inputs
  radius_blocks/run_seconds/seed/server_xmx/bench_dims/cpu_band_min/cpu_band_max).

## Инфра-наблюдения
- df: 100% (ENOSPC на index.lock) → чужой процесс освободил 1.1G → 96-98% на момент пуша.
  Тяжёлые: /home/z/wt522-302 795M, /home/z/wt522-322 763M (full-checkout-утечки, не мои —
  координатору на purge), /tmp/ag322_p42 873M.
- Локальный master (c3bca145) diverged от origin/master (bc7e8722) — canon-ветвление
  вести от origin/master.

## norm_v5-фид (калибровка)
| run | idx | вердикт |
|---|---|---|
| 36832349428 | 7073387 | GATE-FASTFAIL (не-S) |
| 36832346586 | 6977844 | GATE-FASTFAIL (не-S) |
| 36847865424 | TBD | верификация floor 6.5M |
