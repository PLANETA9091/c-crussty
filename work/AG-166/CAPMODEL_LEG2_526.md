# AG-166 w526 — cap-модель leg-2 (независ. банк AG-4, HARVEST_AG4_W526.csv)
n=26 терминалов x525, 13 код-sha, метрика mspt_med/tps_med (медианы).
| res | mean_abs | max |
|---|---|---|
| все n=26 | 0.025 | 0.30 |
| под-cap (mspt>=50, n=3) | 0.217 | 0.30 |

- 23/26 терминалов сидят НА cap 20.0 при mspt_med 9.7-46.3 (меж-sha инвариантность cap).
- Pearson r(mspt_med,tps_med) = -0.807 (cap даёт жёсткую анти-связь).
- Под-cap signed resid: -0.30..-0.14 (tps_hat>tps — согласен с AG-199 n8: 0.13/0.42).
- Вывод: H2 cap-модели AG-199 подтверждена независимой leg-2 (x3.3 данных, tighter resid); leg-3 открыта.
- Caveat: mspt_med (AG-4) vs mspt_sus (AG-199) — разные агрегаты; cap верен для обоих.
