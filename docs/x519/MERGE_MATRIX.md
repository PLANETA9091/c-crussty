# AG-117 ×519 — Infra-Convergence Audit (81 origin-веток swarm-519-*)
Метод: git ls-remote (полный рефлект origin, fetch-refspec клона = только master!), merge-base/integrity sweep, hunk-дифы, локальный repro, job-log canary.

## 🚨 БЛОКЕР-1: canary-фиксы НЕ дойдут координатору через первый fetch
Клон-рефспек origin = `+refs/heads/master` ТОЛЬКО → `git branch -r` видел 35/81 веток. Полный ls-remote = **81**. Координатору фетчить явно: `git fetch origin 'refs/heads/swarm-519-*:refs/remotes/origin/swarm-519-*'`.

## 🚨 БЛОКЕР-2: 3 plumbing-жертвы (×518-болезнь рецидив) — НЕ мержить целиком
| ветка | удалено строк | файлов | 
|---|---|---|
| swarm-519-10 (82de8b28) | 3 179 996 | 3223 |
| swarm-519-25 (493d9d65) | 3 156 264 | 3095 |
| swarm-519-62 (ca021bc7, ahead=2) | 3 179 996 | 3223 |
Мерж целиком = уничтожение master (прецедент 0b3dee2). Нужно: файл-спасение координатором или reject. Все 3 автор PLANETA9091, payload-файлы внутри есть.

## 🚨 БЛОКЕР-3: swarm-519-34 = ORPHAN (merge-base с master ПУСТ, корень a7de1cf7 «TASK-463-69a» — чужая эра)
P44-payload AG-34 (move_plane activate() wiring) на несвязанной истории → мерж потребует --allow-unrelated-histories. Нужен cherry-pick/rebase на 2f795c64.

## Пустые ветки (ahead=0, коммитов нет): 107, 333, 35, 55a, 83, 97 — 6 шт (claims без пуша).

## Конфликт-матрица харнесса (все на базе 2f795c64)
### run_benchv2.sh — 5 редакторов:
| ветка | что | чинит tectonic 3-space? |
|---|---|---|
| swarm-519-54 (f15d6c4f) | printf '%s  %s\n' — whitespace-инвариантный фикс | ✅ ДА |
| swarm-519-111 (123aa9e8) | 3→2 пробела echo + коммент; +DimForceloadPlugin.java | ✅ ДА |
| swarm-519-13 (cb3e3663) | press-порт AG-14 (dungeons pin) | ❌ НЕТ |
| swarm-519-3 (b5e7dda6) | press-порт v2 (на 876b3f45-фиксах) | ❌ НЕТ |
| swarm-519-38 (c2df42cd) | press-порт v3 | ❌ НЕТ |
→ -3/-13/-38 = ТРИ ДУБЛЯ одного lane (урок 3 ×518). ВЕРДИКТ repro: мерж любого из них БЕЗ -54/-111 оставляет canary RED (exit 42 до бута, детерминированно).

### bench-v2.yml — 3 редактора ОДНОЙ Report-gate зоны:
- -23 (32de818e): числовые dud-гейты (MARKED≥58272) + scripts/dud_gate.py
- -45 (51d225a0): canon-гейты marked>0 + G-DIM parse + per-dim/total/heartbeat
- -59 (35abd7f9): whitelist «green ⇔ VERDICT: VALID» + report_benchv2.py
→ 3-way conflict гарантирован; канон-гейты промпта (G-DIM per-dim≥19000 total≥60000 + HB≥60) полнее всего в -45; whitelist -59 = самый дрейф-устойчивый. Рекомендация: канонизировать -45+-59 (ручной конвердж) или -45 минимум.
### world-bench-parallel.yml: -16/-16b (b0720cda) — пара-алиасы, без взаимных конфликтов с yml-трио.

## РЕКОМЕНДОВАННЫЙ ПОРЯДОК МЁРЖА (координатору)
1. -54 (canary фикс, минимальный 1-файл) ИЛИ -111 (если брать и DimForceloadPlugin) — конфликт между собой: взять ОДИН (-54 предпочтительнее: printf-инвариант)
2. ОДИН press-порт: -13 (наименьший диф, 1 файл) или -3; -38/-3/-13 — дедуп
3. yml: -45 (+ вручную whitelist-строку из -59 при желании), -23 дуд_gate.py опционально
4. -10/-25/-62 — ТОЛЬКО файл-спасение; -34 — rebase; остальные 60+ веток чистые (del=0)
