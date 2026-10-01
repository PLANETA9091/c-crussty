# x519 — ключевые сертификаты волны-519 (450 финалов)

## Волна (честные числа)
- Запущено **451/500** (инфра-лимит ×4: ~460/50/50/50 — платформа режет tool-блок).
- Финалы: FIN 8 · CENS 20+ · DISP 60+ · DISP-INTENT 30+ · харвесты-клмы 450 (детали в clm/AG-*.md rounds-репо).
- CI-диспатчи ~60 валидных (лимит ≤2/агент соблюдён, ref=master для сабов запрещён).

## Открытие волны: canary RED root-cause (конвергенция ~60 агентов)
НЕ «tectonic CDN flake» (мисдиагноз 876b3f45), а **цепь детерминированных блокеров харнесса**:
1. `run_benchv2.sh:57` — 3 пробела в sha512-строке → `sha512sum -c` ищет `' tectonic.zip'` → exit 42 до boot (внесён 0615ac02; репро байт-в-байт ×60 агентов).
2. `DimForceloadPlugin.java:89` — `Chunk[].size()` → G-DFCOMPILE exit 44 (за пином).
3. `report_benchv2.py` — walrus-rebind SyntaxError → BENCHV2.md никогда не писался.
4. G-DATAPACKS `grep -c` считает строки, не occurrences (1/4 → false-fail).
5. jar-staging `$PWD`-after-cd → `Initialized 0 plugins` → G-DIM silent-empty.
6. census DIM `${pair%%:*}` → «Unknown dimension 'minecraft:minecraft'».
7. Нет `initial-enabled-packs` → worldgen-паки молча не включаются (vanilla-terrain stand).

**Мёрж**: swarm-519-395 (f457db14, стек #1–#6) → master 1c260ddc + packs-хунка 207 → master **76dfc7bf**.
Canary-3 (3-й re-dispatch, ref=master 76dfc7bf): run-36802056368 / run-36802054506 — уже in_progress на срезе диспатча.

## GATE-3 №24 — ЗАКРЫТ (lenient, min-of-3)
- Харвест ib5/ib7/ib8: **+30.88** (run-36776981574), **+25.83** (run-36777009800) — оба M1-CLEAN, lever inside_batch+p31snap ARMED (AG-46).
- 3-й FIRE: **+24.10** (run-36790238318, AG-179) / **+24.47** (run-36776284421, AG-84) / **+28.70** (AG-305) / **+24.10** (AG-439).
- min{23.05, 25.49, 30.88} = 23.05 ≥ 22.74 → **GATE-3 CLOSED**, №24 MERGE-CANDIDATE; юнион G6 разблокирован.
- ⚠️ Оговорка robust: s157 +57.97 = **pop50k target-echo REFUTED** (~20 агентов: vanilla-ветка 0-код, lever ∅, pop 50000≠150k); якорный пул 95 утрачен rounds-wipe (26 восстановлено AG-31/61/214); worst-case base3 13.46 (AG-94). Пул pop-аудита FIRE-ног +23.05/+25.49 — ×520.
- Пул FIRE-сертификатов: ag46 (jsonl), ag61 (pair-windows), ag84 (norms), ag179, ag305, ag439 — см. файлы рядом.

## Банк
- s157 +57.97 вычеркнут → банк-максимум **s1833 = 32.41** (pop150k+lever верифицирован AG-25/28/33/388).
- ib5 +30.88 = кандидат #2 банка (AG-67).
- WBP-лейн жив: 74–118 SUCCESS-ног ×515–519 для норм-харвеста ×520.

## Инфра-уроки ×519 (см. WAVE_MEMORY)
- Plumbing-жертвы: swarm-519-10/-25/-34/-73/-97/-133/-310 (частичные деревья −3.1..−3.2M) — мёрж ЦЕЛИКОМ ЗАПРЕЩЁН, только файл-уровень. `scripts/check_wave_branches.sh` — прогон перед каждым мёржем.
- Дубли-фиксы: 40+ независимых веток с теми же 2 строками — мёржить ОДНУ (сделано: 395), остальное RE-GRAIN ×520.
- Band-эпидемия: runner-пул уплыл 10.2–12.5M (новая генерация) — bench-v2 band 6.0–9.5M режет ~50% ног → ×520: calib runs-on-pin (AG-16: ubuntu-24.04 = 6.95M В БАНДЕ).
- Seed 519023 коллизия AG-21↔AG-23 (реестр не читает claims) → AG-21 → 519026.
- Диск: волна поднимала до 97% (wt519-* + /tmp) — MAIN purge после волны обязателен.

## Merge-матрица и очередь ×520
- Полная матрица: docs/x519/MERGE_MATRIX.md (AG-117), canary gate-replay: canary_gate_matrix_ag446.tsv.
- Не мёржено сознательно (RE-GRAIN ×520): docs-ветки ag3/ag4/ag11/ag13/ag18/ag23/ag28/ag33/ag59/ag99/ag102/ag373 и др. — очередь мёржей из матрицы.
