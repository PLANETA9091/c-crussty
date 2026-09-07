## SESSION wave-2 — TASK-12 done

**Task ID: TASK-12-sub (wave-2, proc-1)** · Agent: agent-7625532f (TASK-12-sub) · 2026-09-08

Work Log:
- Двойная доставка обнаружена ДО push: docs/BATCH_ADOPTION_MATRIX.md (TASK-12-w1, d02fbc2) уже в origin/master. По прецеденту TASK-03/TASK-11 объединение: оба документа сохранены, мой — компаньон docs/BATCH_ADOPTION_MATRIX_wave2.md с перекрёстными ссылками; расхождений в цифрах нет (одна каноничная база).
- База: P500_REPORT.md @ HEAD ≡ P500_REPORT_v2.md байт-в-байт (diff) — один каноничный перегон 2026-09-08 (3baa0f7): 49 групп / 129 кернелов / 70 пар / 0 CRASH. Stem-pairing воспроизведён офлайн (70 пар, 1 unpaired, 9 multi-pair — точное совпадение с агрегатором).
- Floor-инвентарь по ПАРАМ: 13 групп / 17 old↔alt пар / 32 кернела ≤225ns (в полосе 35-90ns строго — 6 кернелов: g35, g39, g40; нижний якорь g42 34.6ns). Все 17 ratio — parity (0.94-1.05) = «сидят на полу».
- Грейды H/M/L по сигнатурам JNI_EXPORTS.manifest: H=4 кернела (g9 (III[J)I, g35 ([D[I[I[II[J)I — плоские примитив-арены), M=26 (объектные [Lobj;/String слоты), L=2 (g42 <40ns — амортизировать нечего). Сверка с wave-1 (g42 у них HIGH как калибровочный) — обе линзы описаны.
- Модель: T(K)=m+C/K, C=35-90ns, m=3-9ns → T(64)=3.5/7.0/10.4, T(256)=3.1/6.2/9.4, T(4096)=3.0/6.0/9.0 ns. Добавлена M-поправка: m_M = m + 6ns×(ref-слоты) — воспроизводит errata TASK-10 (~1.3-3x для ref-тяжёлых); H-пары дают 11-19x @K=256.
- Top-10 wire-кандидатов (по риск-скорректированной экономии @K=256): g9 113.6ns (19.2x), g32 91.3 (4.8x), g30×3 84.9-86.0 (3.8x), g18 85.4 (4.5x), g31 84.5-84.8 (3.3x), g35 74.2 (11.3x), g39 69.5 (4.8x). DO-NOT-WIRE: 4 масштабно-инвариантные регрессии (5.70/4.54/2.35/1.78) + L-исключение g42.
- ms/tick: параметрическая модель f×(T1−T(K)); при f=1k → 0.065-0.094 ms/tick, f=10k → 0.65-0.94, f=100k → 6.5-9.4. Сверилась с wave-1 S1/S2/E — согласование в пределах ~±50%. Честный вывод: на референсных нагрузках ≤0.1% тика; ≥1 ms/tick требует ≥12k-40k вызовов/тик (JFR-профиль обязателен до вайринга).
- План вайринга через kernel_policy::decide_id(): пререквизит TASK-24 (8 allocs/call в batch_api.rs ≈ пол на K=16), гейт на build+serve, ERR_POLICY_DENIED, промоция в PROVEN_WINS только по BatchFloorBench A/B, тесты (a)-(d), порядок шейпов A′→D→refArgs.
- НОВЫЕ инконсистентности: (1) PROVEN_WINS evidence протух против перегона-2026-09-08 — 3 из 7 «P500 WIN» больше не воспроизводятся (PluginLoadingAllocation 1.55x/1.53x → parity 0.993/0.996; AquiferSurfaceSampling 1.15x → 0.917 parity; blend-cache 244x → 316x) — DO_NOT_WIRE синхронизирован в e5c4fad, PROVEN_WINS нет; (2) baseline.tsv самоссылочен (все drifts ±0.0% — гейт TASK-14 осмыслен только со СЛЕДУЮЩего прогона); (3) P500_REPORT.md ≡ v2 (один датасет); (4) UTC vs UTC+8 в датах; (5) batch_table.rs ids 0-11 — все µs-масштаба, ноль floor-покрытия.

Stage Summary:
- Документ: docs/BATCH_ADOPTION_MATRIX_wave2.md (компаньон wave-1 d02fbc2; union сохранён) — коммит docs(TASK-12) в c-crussty, хэш в CLAIMS.
- Топ-кандидат: PaperNativeDensityAp2MinMaxFill oldSummary→newSummary — 19.2x @K=256 (113.6ns/op, grade H), следующий RangeChoice 11.3x (74.2ns).
- Грейды: H 4 / M 26 / L 2 кернела (13 групп / 17 пар ≤225ns).
- Главный инконсистент-файндинг: PROVEN_WINS evidence stale (3 из 7 WIN-вердиктов не воспроизводятся на каноничном перегоне 2026-09-08) — нужен evidence-sync коммит (не делал: analysis only).
