# AG-50 ×518 — банк-харвест norm_v5 + canary-гейт + STAND-517-PRESS DISP

## Canary-гейт (на старте тика ×518)
- run-36773277359 / run-36773269609 — оба **QUEUED** (3-й тик подряд) → pair-math bench-v2
  НЕ открыт, базы ×515 в силе (449.12/433.90 ch/s marked-rate, 13.00 total, 7.97 full-pressure,
  batch-peak 51.2). Не редиспатчен (очередь сатуратед, по промпту).

## Нормирование (normtool_478, selftest 3/3 bit-exact + 9/9 fixtures PASS ДО вердиктов)
5 SUCCESS-ранов из списка волны. ВАЖНО: 4/5 уже были в банке ×517 (агрегат AG-34
`docs/ag34/bank_norm_x517_ag34.jsonl`) — воспроизведены **бит-точнo** независимым прогоном
(кросс-валидация пайплайна банка, две волны, одинаковые числа):

| run | ветка/лейбл промпта | лейбл банка ×517 | cpu | m1 | norm_v5 | vs банк ×517 |
|---|---|---|---|---|---|---|
| 36757163912 | swarm-515-170b | s1804 (AG-32) | 6,863,090 | CLEAN | **+19.11** | MATCH bit-exact |
| 36755725909 | swarm-515-73 | s1800 (AG-42) | 7,395,190 | **HOST-CENS** | +21.66 (report-only) | MATCH bit-exact |
| 36755741812 | swarm-515-73b | s1801 (AG-42) | 6,817,717 | CLEAN | **+19.50** | MATCH bit-exact |
| 36752032802 | swarm-515-16b | s16b | 7,039,023 | CLEAN | **+8.39** | MATCH bit-exact |
| 36767133498 | round-515-fensrv3 | — **НОВЫЙ** | 7,220,589 | CLEAN | **−7.03** | не был в фидах |

- Biome-AIOOBE exempt применён на 4 ноги (Л-474-C88.2: aBiome>0 ∧ aOther=0, probe
  ChunkParseOps.biomesSelftest; маркеры aioobe_biome/biomes_exempt_applied в raw-фиде).
  Без exempt они FIXTURE-INVALID; 36767133498 чист.
- 36755725909 = HOST-CENS (STW > 23s) — лег НЕгоден для пары, число report-only.

## Новая нога fensrv3 (Г3-srv, fen-класс) run-36767133498
- norm_v5 = **−7.03 CLEAN** @7,220,589, tps_med 2.1, spark-кросс +4.25 (div 11.28пп).
- fen-класс остаётся ≤ кривой на TPS-оси (−7.03 vs 7.0M-бакет anchor-медиана −4.55 report-only)
  → консистентно с «fen ≤ unf» ×517. Г3-гейт (fen≤unf×0.05 min-of-3) НЕ закрывается bench-нормой
  (канон ×504: bench-нормы ≠ Г3, нужны probe-метрики emitters).
- Пара: в видимом мини-пуле (13 ног AG-34) нет CLEAN-якоря в Δ≤50k (ближайший 7,152,052,
  Δ=68.5k) → полная pair-матем по всему банку (253+) — следующий шаг ×519.

## №24 GATE-3 — не закрыт этой партией
- 0 новых легов ≥22.74 (−7.03 / +8.39 новые; 19.5/19.11 — подтверждения). Честно: **GATE-3 открыт**,
  компо-путь №24⊕SWAR-X⊕H07 остаётся фронтом.

## Ре-роллы swarm-515-234/211
- run-36767485017 (234) / run-36768662155 (211) — оба **in_progress** на срезе ×518 → артефактов
  нет, нормировать нечего; к харвесту ×519, не редиспатчены.

## DISP — STAND-517-PRESS (payload AG-14, разблокирован мёржем 7e66cfe)
- POST ×2 с ref=**swarm-518-50**: bench-v2.yml BASE (seed 517014) + bench-v2-press.yml PRESS
  (seed 517014, press_pack=dungeons, DnT v5.0.4 sha512-пин) → same-seed A/B press-vs-base.
- Seed-gate: 517014 вне bank-оси 240..535 + BENCH_SEEDS + LEDGER ×516 (prereg AG-14
  work/AG-14/STZ-106_STZ-106_STAND-PRESS.md; повторно не коллизирует — per-LEG concurrency ×516,
  press-* vs bench-* префиксы). Run-id см. clm/AG-50.md.
