# GOLDEN_EXEC_PLAN_443.md — TASK-443, golden-слот 02:08 +08 (2026-09-25)

Точный порядок исполнения тика 02:08 +08. Диспатчер: `/home/z/c-crussty/scripts/golden_443.py`
(шаблон golden_439.py — механика 1:1; banked TASK-443-C, 14:26 +08 2026-09-24; py_compile OK;
dry-run OK — 24 строки плана, 0 SHA-DRIFT, 0 диспатчей).
**Мандат владельца 2026-09-23: ≥+20% pair-stable ОБЯЗАТЕЛЬНО** — батч = сертификационный дизайн
(min-of-3 пары для всех живых рычагов). Арсенал 7 рычагов: ins4d / ins4 / chunk4 / sscan2 /
pdemux / chk3 / MEGA(optional).

## 0. ПРЕФЛАЙТ (перед диспатчем, ≤5 мин)

1. `df -h /home/z` — диск <90% (иначе чистка tmp/worktrees; RESULT → PRESERVED).
2. Токен жив: `git -C /home/z/c-crussty ls-remote origin master` (ожидание `c9060b1f…`).
3. Ре-верификация баз (если master ушёл вперёд после prep — НЕ блокер для якорей, но
   нога-ша должна совпасть): ожидаемые ша —
   | base | sha | lever |
   |---|---|---|
   | master | c9060b1f (tick-442 FINAL) | '' (якоря) |
   | round-442-b-ins4d | d9d1fb30 | cmp440_ins4d |
   | round-436-b-ins6 | 07078007 | cmp436_ins4 |
   | round-438-c-chunk4b | c5fe0251 | cmp437_chunk4 |
   | round-437-a-sscan2 | 3f3b111f | cmp436_sscan2 |
   | round-437-b-pdemux | 48362768 (ФИКС STRICT-OR — обязателен) | cmp436_pdemux |
   | round-436-c-chunk3 | 7afe6d17 | cmp435_chunk3 |
   | round-443-mega | **OPTIONAL** (передаётся агентом-B; пин ставить в `EXPECTED_SHA["round-443-mega"]`) | cmp443_mega |
4. **MEGA OPTIONAL-пин (рекомендуется до запуска)**: если агент-B уже запушил `round-443-mega` —
   взять sha из `ls-remote origin round-443-mega` и вписать 8-символьный префикс в
   `EXPECTED_SHA["round-443-mega"]` (сейчас `None`). Без пина: ветка есть → диспатч как есть
   (живой sha пишется в лог), ветки нет/ша≠пин → обе mega-ноги SKIP с предупреждением, НЕ аборта.
5. Скрипт сам абортирует при SHA MISMATCH любой из 7 обязательных баз (preflight в main());
   OPTIONAL-база на аборта не влияет — только предупреждение и -2 ноги.
6. Санити: `python3 -m py_compile scripts/golden_443.py && python3 scripts/golden_443.py --dry-run`
   → 24 строки, 0 SHA-DRIFT на обязательных базах, финальная строка «диспатчей 0».

## 1. ДИСПАТЧ (сразу на старте тика, ~2 мин)

```bash
cd /home/z/c-crussty
python3 scripts/golden_443.py --dry-run   # финальный санити: 24 строки, все sha OK, 0 диспатчей
python3 scripts/golden_443.py             # РЕАЛ: preflight 7 ша + optional-mega → dispatch, sleep 4
```

Батч 24 запуска, интерлив в 3 трети по 8, **якоря ПЕРВЫМИ и ПОСЛЕДНИМИ в каждой трети**
(якорная плотность по всему окну 02:2x-03:3x ~70 мин; раны ~18-25 мин параллельно):
- ТРЕТЬЯ 1: anchor-1 → ins4d-1 → anchor-2 → ins4-1 → chunk4-1 → anchor-3 → ss-1 → anchor-4
- ТРЕТЬЯ 2: anchor-5 → ins4d-2 → anchor-6 → pd-1 → chunk4-2 → anchor-7 → ins4-2 → anchor-8
- ТРЕТЬЯ 3: anchor-9 → ins4d-3 → anchor-10 → chk3-1 → mega-1 → anchor-11 → mega-2 → anchor-12

Итого: 12 якорей @master lever='' + 3 ins4d + 2 ins4 + 2 chunk4 + 1 ss + 1 pd + 1 chk3 +
2 MEGA(OPTIONAL). Без travel_diet/fluid_dirty_ledger, без concurrency-гвардов (проверено эрой ×434).
Банк INPUTS РОВНО: radius 640, seconds 300, fake_players 4, fluid_guard 1, gc_tune 3, inside_cache 1,
flush_diet 1, fluid_dirty 0, fluid_bitmask 0, region_threads 4, batch_collector 1, inside_bitmask 0,
skip_store_bb 0, region_steal 0, bu_defer 0, population_target 150000, population_seed 42,
server_xmx 10G, server_xms 4G, cpu_band_min 6000000, cpu_band_max 9500000 + lever_flag per-leg,
lever_arg 1.

**Дизайн двухзонной лотереи**: total_chunks — лотерея рана (депресс-зона ~6.3-6.9M мигрирует по
позиции/времени: ×437 6.6-6.8M → ×438 6.66/6.856/8.6M → ×442 обе зоны живы 6.46-8.90M). 12 якорей
первым/последним в каждой трети = максимальная плотность по ВСЕМУ слоту → ОБЕ зоны получают
якорей-партнёров Δ≤50k. ins4d-нога в каждой трети = честный min-of-3 для линии-рекордсмена
(3.30 +23.4 / 3.20 +22.2).

Ветки `round-443g-*` создаются при запуске (на origin отсутствуют, prep 14:26; ветки батча 13:08
`round-443-*` НЕ трогаются). **Скрипт запускать РОВНО ОДИН РАЗ за слот** — повторный запуск
переиспользует существующие round-443g-* ветки (ensure_branch) и даст дубли-диспатчи.

## 2. ПОЛЛ (каждые 240с)

```bash
TOKEN=$(git -C /home/z/c-crussty remote get-url origin | sed -n 's|https://[^:]*:\([^@]*\)@github.com/.*|\1|p')
curl -s -H "Authorization: Bearer $TOKEN" \
  "https://api.github.com/repos/PLANETA9091/c-crussty/actions/workflows/world-bench-parallel.yml/runs?per_page=60" \
  | jq -r '.workflow_runs[] | select(.head_branch|startswith("round-443g-")) |
           [.id, .head_branch, .status, .conclusion, .created_at] | @tsv'
```

- Прогрев + бенч ≈ 18-25 мин/ран; полный батч 24 ≈ 70-90 мин волнами по третям.
- Каждый rc!=0/threw=1 — НЕ абсорбить вслепую: сначала классификация (шаг 3).

## 3. ABSORB списком + МАРКЕР-КАПЧЕР (server-stdout НЕ purge до ARM-чеков!)

Порядок на каждую ногу (уроки ×437/×438: маркеры теряются purged stdout):
1. Забрать артефакт `world3-bench` (server-stdout.log, run-env.txt, BOTTLENECKS_3.md).
2. **СНАЧАЛА маркер-капчер из server-stdout.log** (ARM читать по stdout, НЕ по run-env — урок ×438-B):
   - ins4d (cmp440_ins4d): selfTest=true, goal-query ARMED, inside_snap, V4 FLIPPED, epoch ok, lifetime-heap;
   - ins4 (cmp436_ins4): selfTest + ARMED + inside_snap;
   - chunk4 (cmp437_chunk4): ARMED + queryplane awake (chunk-send snapshot);
   - ss (cmp436_sscan2): `[crussty-plugin] sscan: ARMED`, selfTest=true ДО arm, despawn
     `Mob.checkDespawn` + spawn `NaturalSpawner` retransform rc=0, EFFECT first-gate, spawn epoch ok,
     bulk JNI epoch mobSlots=…, 0 per-call JNI;
   - pd (cmp436_pdemux): `PALETTED-DEMUX ARMED` + `PATCHED 30967→31521 bytes`, first-gate/EFFECT.
     Без ARMED-маркера нога = дормант → вердикт НЕ читать;
   - chk3 (cmp435_chunk3): cmp435_chunk3 ARMED (queryplane awake) + chunk-parse + collide-batch;
   - mega (cmp443_mega): маркеры по доке ветки round-443-mega (агент-B) — без ARM/EFFECT-маркеров
     нога = дормант → вердикт НЕ читать.
3. **Только после извлечения маркеров** — purge server-stdout (per-absorb purge discipline).
4. threw=1: классификация по стеку. Ваниль-артефакт LecternBlockEntity (validateBlockState @
   forceload-sweep) ≠ lever-фейл (класс ×116/×438; также goat_horn instrument decode,
   random_sequences salt, watchdog syncLoad thread-dump).
5. AIOOBE=0 и NCDFE=0 проверить на КАЖДОЙ ноге (NCDFE — урок inside2: selfTest ДО BRIDGE_READY).
6. Извлечь: TPS, total_chunks (band), профиль lanes, shed не-TPS.

## 4. ПАР-ВЕРДИКТЫ / ВЕРДИКТ-ГЕЙТЫ (канон v2 / ×443)

Гейт-лист (все обязательны для мерджа победителя):
1. **Якорный спред окна**: якоря norm, выкинуть депресс-выбросы (norm < −2); если разброс
   оставшихся ex-outlier >±5пп — окно браковано → вердикты НЕ читать → перенос на следующий
   слот 02:08 (батч переигрывается новым скриптом-препом, shas ре-верифицировать).
2. **Депресс-гейт**: якорь norm ≥ −2 иначе якорь в депрессии — пары читать ТОЛЬКО против живых якорей.
3. **BAND 6.0-9.5M**: нога вне полосы = BAND-DISCARD → ре-ролл ≤2 (новая ветка round-443g-<leg>-r2).
4. **Пара**: leg против ближайшего живого якоря **Δ≤50k** по total_chunks; эффект = norm к медиане
   живых якорей.
5. **ARM-маркеры stdout** (п.3) — без ARM вердикт не читается; **AIOOBE=0; NCDFE=0; selfTest
   (true ДО arm); threw=0** (после классификации п.3.4).
6. **min-of-3**: мердж-кандидат требует 3 здоровые ноги семейства (ре-роллы засчитываются) с
   **медианой pair-эффекта ≥+20%** — мандат владельца 2026-09-23.
7. Спец-вердикты: ins4d — удержание рекорда 3.30 +23.4/3.20 +22.2 в golden = главный кандидат;
   chunk4 — GREEN ×6 эры, нужна пара Δ≤50k; pdemux/sscan2 — ARM-зелёный + pair; chk3 — повтор
   пары +16.0пп; MEGA — только при ARM-маркерах по доке агента-B.

## 5. МЕРДЖ (если есть победитель — НЕМЕДЛЕННО по гейтам)

```bash
git -C /home/z/c-crussty checkout master && git -C /home/z/c-crussty pull origin master
git -C /home/z/c-crussty merge --no-ff origin/round-443g-<WIN> -m "TASK-443 golden: <lever> +X% min-of-3"
git -C /home/z/c-crussty push origin master
```

Порог: min-of-3 медиана ≥+20% pair при ARM-пруфе маркерами и всех гейтах п.4.1-4.5.
Нет победителя → NO MERGE honest (канон v2) — мандат НЕ выполнен фальсификацией гейтов.

## 6. ФИНАЛ ТИКА (вне зависимости от вердикта)

1. GOAL (increment), CLAIMS — секция TASK-443 учёт (не трогать другие секции).
2. worklog append (шаблон ---/Task ID/Agent/Task/Work Log/Stage Summary) → dev-logs.
3. `git push` c-crussty + my-project. RESULT.json заполнить вердиктами (write-through).
