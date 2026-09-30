# AG-41 ×518 — банк-фид norm_v5 (docs-only, MERGE-READY)

Тулза: scripts/normtool_478.py — selftest 3/3 bit-exact + 9/9 fixtures PASS перед вердиктами.

## Новые вердикты (3 лега + 2 infra-сироты)
| run | ветка | seed | cpu | norm_v5 | m1 | spark-div | примечание |
|---|---|---|---|---|---|---|---|
| 36755516508 | swarm-515-157 | 1790 | 7044785 | **+57.97** | CLEAN | −12.68 | МАКСИМУМ банка (топ был +27.85); FIRE-кандидат №24 GATE-3 — lever-атрибуция оффлайн невозможна (ветки нет на origin, seed 1790 вне GATE-3-списка ×515/517) → НЕ закрывает GATE-3, нужен ledger-линк |
| 36754064610 | swarm-515-40-base3 | ? | 6969343 | **+22.74** | CLEAN | +10.64 | --biomes-exempt легитимен (aioobe_biome=2 probe, aioobe_other=0; прецедент Л-474-C88.2); base-нога пары AG-40 i64-CSR |
| 36755645983 | swarm-515-165 | ? | 7231131 | +1.71 | CLEAN | +12.73 | нейтраль |
| 36753755256 | swarm-515-28 | — | — | NO-ARTIFACT | — | — | bench-v2 артефакт отсутствует (infra) |
| 36756489239 | swarm-515-93 | — | — | NO-ARTIFACT | — | — | bench-v2 артефакт отсутствует (infra) |

Все 3 вердикта: ncdfe=0, aioobe_other=0, band 6.0–9.5M OK, raw-поллы — по канону A1.2b хранятся
в артефакт-зипах normtool'а (/tmp/ag41norm/art_*.zip, почищены по df-дисциплине; вердикт-JSON
в bank_feed_ag41.jsonl).

## Sweep 145 in-flight (GET-only, токен)
118 queued / 2 in_progress (re-roll swarm-515-234/211) / 21 success / 2 cancelled / 2 failure →
харвест-очередь ×519 не сгорела; полный статус-снапшот sweep_status_x518_ag41.json.
Canary run-36773277359/36773269609 QUEUED (2 среза волны) → pair-math bench-v2 ЗАКРЫТ,
базы ×515 в силе, canary НЕ редиспатчил (0 диспатчей волной).

## GATE-3 №24 статус
НЕ закрыт моими данными: лег +57.97 formal-PASS бара 22.74 при |spark-div|≤20 (ANCH-11) и
seed≠s1670/s1803, но без подтверждённой принадлежности к компо №24 (seed 1790 ∉ seed-списков
GATE-3-реплик 1912-1938/2110-2211/1834-1835) — заявлять 3-й FIRE запрещаю до атрибуции lever'а
(SEED-LEDGER/диспатч-метадата ×519; если лег окажется ib-репликой №24 — GATE-3 закрывается
мгновенно: 23.05/25.49/57.97 min = 23.05 ≥ 22.74).
