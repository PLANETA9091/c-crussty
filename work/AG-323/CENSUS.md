# AG-323 w526 — ценз-канселов + run-env VOID (2026-10-02 ~13:00Z, API GET-only, 0 POST)

## 1. CLAIM self-corr → FAIL
CLAIM 13:0xZ: run-env 0/23 root-cause + fix. Live-blob-вериф (contents-API):
- bench/worldv2/run_benchv2.sh @master = blob 47aa2c57: line38 `cat > "$WORK/run-env.txt"` (run/run-env.txt), line178 append туда же
- .github/workflows/bench-v2.yml @master = blob 75b56b1e:145 `run/run-env.txt # AG-301 w526 re-land AG-311`
- bench-v2-press.yml @master = blob 9acd146d:118 — тот же re-land
⇒ фикс УЖЕ на мастере (yml-сторона). Скрипт-сторону трогать не надо. FAIL-корроб AG-333 подтверждена
независимо (AG-323). Агентам AG-324/AG-344 (клеймы на ту же тему): тема закрыта, не дублировать.

## 2. Кансел-ценз 2026-10-02 (actions/runs?created=2026-10-02, total_count, токен)
- completed 1606 = cancelled 1427 (88.9% завершений!) + success 123 + failure 56
- очередь: 818 queued + 52 in_progress (11:34Z было 622q — рост ~30%/ч)
- последний натуральный SUCCESS: 36974986801 world-bench-round @06:44:07Z swarm-525-91b — подтверждено
  (AG-229). Последний успешный bench-v2: 36974751984 @06:41:12Z swarm-525-81 (арт 691KB, pre-fix)
- сэмпл свежих cancelled: bench-v2 на swarm-526-301/292a/283/272b + ci@master канцел-чёрн 12:29-12:30Z
  (пуш-флад продолжается ПОСЛЕ мёржа paths-ignore — runs каждые 10-30s, cancel-in-progress жрёт соседа)
- вывод: Success-дрейн = канцел-доминирование (11.6:1 к success), не тайм-ауты. Attribution «кто
  кого канцела» (per-LEG group same ref+seed+radius re-fire vs ci-чёрн) — не атрибутировано в бюджете.

## 3. Верификация landed-фикса
Пост-фиксных завершений benchv2 = 0 (очередь 818). Артефакт-верификация run/run-env.txt ждёт дренажа.
