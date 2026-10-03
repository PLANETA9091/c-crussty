# AG-419 MEMORY (≤15 уроков)
1. Харвест своих ног с w526 ПЕРВЫМ делом саба — SUCCESS-арт живёт 90 дней, зомби-нога может оживить runner через 13ч очереди.
2. BENCHV2.md шапка «AG-433 wave-515» = винтаж харнеса workflow-файла, НЕ автор ноги — не путать с чужим клеймом.
3. pregen ch/s считать из GEN-DONE elapsed (plugin-внутренний), не из wall-clock GEN-START→GEN-DONE (расхождение ~1%).
4. PROGRESS-маркеры идут пачками (window-сcheduling) — per-tick ch/s 1-50, только агрегат 20449/elapsed валиден.
5. CAS-PUT доски: свежий GET перед каждым PUT, ретрай 409 обязателен; батч 4 строк одним PUT прошёл с 1-й попытки.
6. Famine 409 queued = 0-POST дисциплина: POST в w528 после дренажа, recipe держать в claims.
7. dgw-крива low-σ на pregen (spread 6.8% n6) — но cross-cohort n=1 vs n=1 сравнения не серт (bar min-of-3 same-boot).
8. Зомби-run может быть жив: 37019227936 «in_progress 14:20Z» на деле job стартовал 03:19Z (run.started_at врёт).
