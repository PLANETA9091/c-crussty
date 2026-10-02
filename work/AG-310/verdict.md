# AG-310 w526 — w-cliff host-confound census (0 POST, n=4, method AG-271: runner_cpu_index из job-LOGs)
Hypothesis (AG-271 OBSERVED): w1024@r1136 ch/s-клифф 2.27 может быть host-конфаундом (нога на low-host 6.43M).

Данные (cpu_index извлечён из bench-v2 job-логов, band-gate echo):
| leg | run | cfg | ch/s | cpu_index |
| cliff | 36971063771 | w1024@r1136 | <=2.27 (cap-trunc DRAIN-TO) | 6.43M low |
| control | 36971189248 | w512@r1136 | 11.69 | 6.81M low |
| alive1 | 36971390335 | w1024@r800 | 12.25 | 7.16M mid |
| alive2 | 36971397141 | w1024@r800 | 15.18 | 8.94M high |

Вердикт: host-confound REFUTED.
- Control w512@r1136 на том же r=1136 и equally-low host (6.81M, Δhost 6% от 6.43M) даёт 11.69 ch/s.
- Host-shoulder модель AG-271 (<8M=10.6 vs >=8M=14.2) даёт максимум +33%; наблюдаемый разрыв >=5.15x (11.69/2.27) — на 3.9x за пределами host-эффекта.
- Значит клифф живёт в w x r interaction (gen-window душит только большой r при большом окне), host модулирует ±33% внутри.
- Следствие: r-бисект AG-221 (r960/r1024 @w1024, legal s3000/dcp1500) — правильная ось; хост-матчинг/ре-роллы хостов для клиффа НЕ нужны, экономия пула.

Побочный FACT: band-gate [10.0M,13.5M] action=warn инертен — 4/4 моих ног (6.43-8.94M) вне гейта,
кумулятивно 21/21 с n=17 AG-271; пул целиком ниже гейта. Re-cal гейта на живой пул = отдельный код-фикс.

Уроки: cpu_index восстановим из ЛЮБЫХ bench-v2 job-логов (echo-строка банд-гейта) — 0-POST метод дешёв и универсален.
