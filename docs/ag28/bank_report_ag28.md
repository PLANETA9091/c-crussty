# AG-28 wave-518 — банк-фид: норм fensrv3 (Г3-srv fen-side, орфан ×515)

## Что сделано
- Статус-харвест 48 in-flight ранов (мои R3-ноги компо + canary + 44 из очереди ×515/516):
  **48/48 QUEUED, 0 terminal** (HARVEST_518.json) — конжестия раннеров >25ч, редиспатч запрещён.
- Найден 1 НОВЫЙ терминальный SUCCESS вне снапшотов: **run-36767133498** (fensrv3,
  branch round-515-fensrv3, world-bench-parallel, SUCCESS 2026-09-30T21:42:43Z;
  в снапшоте ag32/ag50 от 20:44Z ещё queued).
- Нормирован канон-нормтулом normtool_478 --biomes-exempt; selftest ПЕРЕД вердиктом
  3/3 bit-exact + 9/9 fixtures PASS. Артефакт-zip 114М в /tmp, удалён после парса.

## Норма (полная строка: norm_fensrv3_36767133498.jsonl)
| поле | значение |
|---|---|
| run_id | 36767133498 (fen-сторона Г3-srv, seed-орфан ×515) |
| verdict | NORM-COMPUTED, m1_state=CLEAN (STW 22.14s ≤23.0; young_avg 126.9ms ≤200) |
| cpu_index | 7 220 589 — in_band [6.0, 9.5]M ✓ |
| tps_med (C55) | 2.1 (polls_valid [1.6,1.9,2.1,2.5,2.6], n=5; pre-inject 18.5 отсечён) |
| tps_exp_v5 | 2.2587 |
| **norm_v5** | **−7.03** |
| spark-кросс | tps_avg 2.3548, norm_spark +4.25, divergence 11.28пп (<20, не ANCH-11) |
| гейты | fixture_valid ✓, ncdfе=0, aioobe=0/0, exempt_applied=false |

## Флаги честности
- Full GC n=9 (депресс-кластер-фактор из LEDGER: Full=9 душит ноги) — CLEAN формально,
  но STW 22.14s в 0.9s от ценза и max_pause 2853ms → точка банка с пометкой full_n=9.
- Пара НЕ закрывается: (а) орфан ×515 — unf-контрапарт того же бута отсутствует;
  (б) Г3-srv кросс-бут REFUTED ×517 (потолок 0.77пп capture, флор ±11–15пп, AG-50);
  (в) одна сторона. Ценность = банк-в-точка 7.22M по правилу BANK_V5_FREEZE §3.

## Артефакты
- work/AG-28/: HARVEST_518.json (48 статусов), harvest_518.py, norm_fensrv3_36767133498.jsonl
- Ветка swarm-518-28 → docs/ag28/{bank_feed_ag28.jsonl, bank_report_ag28.md}
