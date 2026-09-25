# RESEARCH-459-L05 — root-cause RED-ноги roar-2 (cmp458_roar @eb47e869, run 36149782264, −10.7% @8960455)

Автор: TASK-459-L05 (ЛАБ, закон 14b). Артефакт: `research/gc-recon-2026-09-19/round-round-458k-roaring-36149782264/` (+ дубль `round-roar458-2/` с collapsed+jar). Вопрос тика: где просадка — bloom на горячем пути? chain-walk дорожe? самотест overhead? Ответ: **ни то, ни другое, ни третье — treatment вообще не выполнялся**.

---

## 1. ГЛАВНЫЙ ВЕРДИКТ: PLACEBO-НОГА (seed-fail → hook dormant), RED-число — депресс-шум

### 1.1 Цепочка доставки (факт-цепочка с местами)
1. Диспатч agent-K (RESULT.json step-4): канон-инпуты + `lever_flag=cmp458_roar, lever_arg=1`.
2. Workflow `world-bench-parallel.yml` (на ветке @eb47e869): input `lever_flag` (строка 75) → env `LEVER_FLAG` (строка 240) → `bench/world3/run_world3.sh:459` `export CRUSSTY_LEVER_FLAG="${LEVER_FLAG:-}"`. Проводка ЦЕЛА.
3. Натив: CI собирает `libcrussty.so` из исходников рефа (workflow «Build engine runtime + module (Rust only)», строки 200-207) → `entity_index.rs` с eidx-нативами В КОМПИЛЯТЕ.
4. Manager gate: `entity_index_manager.rs@eb47e869` строки 76-82: `GATE_LEVER="cmp405_eindex"`, `GATE_LEVER2="cmp458_roar"`, `lever_matches()`. В stdout **НЕТ** строки `eindex: dormant (set CRUSSTY_LEVER_FLAG=...)` → **гейт ПРОЙДЕН → флаг ДОСТАВЛЕН** (здесь я опровергаю запись BOTTLENECK ×459 «спящий гейт — джава-блоб, фикс = ребилд блобов»: блоб живой, ребилд не нужен).
5. define_class + RegisterNatives: без ошибок-печатей → ОК. Probe: `eidxProbe` вернул PROBE_MAGIC (иначе была бы печать `probe magic 0x... != expected`) → класс инициализирован (`<clinit>` прошёл), ENABLED=true, нативы живы.
6. **`server-stdout.log:947`: `[crussty-plugin] eindex: seed failed (0) — hook stays dormant`; :948 `probe/seed/armed failed`.** Manager (`:470`) требует `seeded == 1`, seedAll вернул 0.
7. ARM-маркер v2-roar НЕ напечатан; `grep -ci roar server-stdout.log` = **0**; EntityLookup-retransform (4 redirect + 4 note) не выполнен → запросы пошли ванилью. Нога = носитель-конфиг без treatment (entity_compose ARMED chain [inside->rng->batch] 205458→205610 байт, region_threads ARMED rc 0/0/0/0, batch_collector/flush_diet/inside_cache/fluid_guard — банк-инпуты, как у якорей).

### 1.2 Почему seedAll()==0 (javap-контракт)
`javap -p -c` блоба `entityquery/build/.../EntityIndexOps.class` @eb47e869 (sha-блоб d95d1d7b, **major 65 = 0x41**, Java 21):
- `leverEnabled()`: `System.getenv("CRUSSTY_LEVER_FLAG")` → trim → equals `cmp405_eindex` | `cmp458_roar` (javap смещения 20-58).
- `seedAll()` возвращает 0 тремя путями: (а) offset 0-7 `getstatic ENABLED; ifne 8; iconst_0` — !ENABLED (исключено: гейт-печать отсутствовала, а Java-ENABLED и Rust-гейт читают один и тот же env); (б) `eidxFlush(...) < 0` (offset 323-330 и 355-362: `ifge` — любой отрицательный rc → `iconst_0`); (в) **Exception table 8-367 → Throwable → `iconst_0` (offset 368-370)** — java-исключение глотается молча.
- Натив `eidx_flush` = `load_ops → ops_from → apply_ops`; ERR только `ERR_STRUCT(-1)`: `BROKEN`/WSTATE-poison (статичны, seed первый writer → исключено), `id_insert`/`chunk_entry`/`slot_alloc` переполнение. Ёмкости: SHARDS=64, CHUNK_CAP=4096/шард (262k чанков), SLOT_CAP=8192/шард (512k), ID_CAP=8192/шард, PROBE_MAX=128; на seed ~12-18k сущностей → ~200-280/шард (загрузка ~3%) — переполнение невозможно. `apply_ops` возвращает 0 на успехе.
- ⇒ Ранжирование кандидатов: **(1) глотаемый Throwable в java-цикле seedAll** (getAllCopy()/entity-аксессоры на 12-18k живых сущностях во время инжекта) — наиболее вероятен, т.к. apply_ops структурно не может упасть на этих объёмах, а probe/clinit прошли; **(2) eidxFlush<0** — маловероятен, но не нулевой (load_ops OOM-класс). Точная нозология = офлайн-харнесс/инструментированный ре-ролл (см. §5).

### 1.3 Миф «спящий блоб» и ARM-парсер
- `patched-kernel.jar` артефакта sha256 `e2992d63…e87200` — **байт-в-байт равен** `round-396-a/patched-kernel.jar`; внутри НЕТ EntityIndexOps/roar-классов (unzip -l: только ваниль `SetRoarTarget`/`Roar` warden). Это НОРМАЛЬНО: ops-классы инжектятся агентом (`define_class` из блобов, вшитых в .so), jar — базовое ядро. Т.е. сам по себе «плоский jar» ≠ спящий гейт.
- **Найден ложный ARM-маркер**: absorb-словарь roar-2 содержит `ARM: {"cmp457_paldelta": "MARKER"}`, при этом stdout:8 `paletted: dormant (…lever_flag=cmp457_paldelta to enable)`. Парсер маркеров ловит ПОДСТРОКУ lever-id внутри dormant-подсказки → ARM-гейт G1 отравлен ложноположительным срабатыванием. Фикс: анкерить на полный ARMED-маркер (`v2-roar ARMED`, `ARMED shards=64 …`), запретить матч по голому id.

## 2. Откуда −10.7 числом (декомпозиция профилей)

### 2.1 TPS-мать
- polls first-of-window `[20.0, 1.7, 2.0, 2.4, 2.7, 2.8]` → median **2.4**; TPS_exp(8960455)=2.686 → norm **−10.7%** (модель банка v4: 2.6@8551924 / 2.2@6653417, slope 2.107e-7 it⁻¹).
- MSPT avg 378.77ms / min 321.6 / max 491.5; tick-behind=0; pop 156969 peak (VALID 140-165k), entity totals [148516, 150267, 150864], churn 3093 (2.1%) ACTIVE, item×102982, item_frame×2714, fake_players 4/4.

### 2.2 GC — единственная наблюдаемая «просадка» (roar-2 сам себя удавил STW-кластером)
| нога | runner | median | norm | STW total | Full | avg pause | max pause |
|---|---|---|---|---|---|---|---|
| chk-14 (монстр) | 8687055 | 3.2 | +21.7 | 17.66s | 9 | 129ms | 1955ms |
| chk-16 | 6951662 | 2.55 | +12.7 | 20.53s | 9 | 185ms | 3010ms |
| chk-11 (депресс) | 7040413 | 2.2 | −3.6 | 22.1s | 9 | 211ms | 2789ms |
| **roar-2** | **8960455** | **2.4** | **−10.7** | **23.55s** | **10** | **184ms** | **2277ms** |
| anchor-33 | 6973621 | 2.0 | −11.8 | 25.19s | 9 | 219ms | 2537ms |
(банк-справка: 18.8s / Full=7 / avg 162ms / max 2400ms)

- roar-2: 128 пауз (young 108 Allocation Failure + 6 CodeCache + 4 Metadata), **Full=10 > банк 7**; heap high-water **7712MB** → last-after 3686MB.
- Тяжёлые Full во время инжекта/соука: 735.3ms@41.6s, 970.8ms@51.1s, 1477.5ms@75.5s, **2249.4ms@172.0s, 2276.7ms@410.5s** — при MSPT ~379ms один такой STW = ~6 потерянных тиков; сумма Full-хвоста в окне ≈ 7.7s.
- Регрессия по кластеру (n=5): norm ≈ −4.96пп × (STW,с) + const; r ≈ −0.93. roar-2 предсказано ≈ −12.3 против факта −10.7 → **−10.7 объясняется STW-кластером co-ран сессии-B, вклад roar-механики = 0.00пп** (n=5 — не причинность, но вилка честная).
- CPU-подтверждение GC-хвоста: `PSCardTable::scavenge_contents_parallel` **2036** self (1.8%) + `OopOopIterateBoundedDispatch…narrowOop` **1113** (1.0%); GC-лейн-присутствие **6399 (5.67%)** от 112915.

### 2.3 CPU-лейны roar-2 (python-сумма по cpu-collapsed, 112915 сэмплов, presence)
| лейн | сэмплы | share | сравнение |
|---|---|---|---|
| items (ItemEntity.tick) | 32029 | 28.37% | = F3-сверка 28.37% ✓; на якорях 29.05-31.17% |
| fluid | 17180 | 15.21% | якоря 16.35-16.72% |
| nav_ai | 15033 | 13.31% | якоря 13.67-14.16% |
| inside | 11641 | 10.31% | якоря 10.47-12.01% |
| **broadphase (цель roar!)** | **11181** | **9.90%** | anchor-33 14.68%; chk-14 9.36% |
| gc-плейн | 6399 | 5.67% | — |
| synched-data | 3959 | 3.51% | — |
Лист-ранжирование топ-листьев roar-2 (self): PalettedContainer.get **5031 (4.5%)**, updateFluidHeightAndDoFluidPushing **3749 (3.3%)**, ChunkEntitySlices$EntityCollectionBySection.getEntities **2609 (2.3%)**, AABB.intersects **2566 (2.3%)**, ChunkEntitySlices.getEntities **2383 (2.1%)**, SynchedEntityData$DataItem.getValue **2242 (2.0%)**, vtable stub 2211, scavenge 2036, SimpleBitStorage.get 1911, VarHandle…getVolatile 1723. Broadphase-self = 2609+2383+EntityLookup.getHardCollidingEntities 725 = **5717 (5.06%)**.

### 2.4 Roar-фреймы: НОЛЬ
`grep -ci 'roar|EntityIndexOps|entity_index'` = **3** попадания в cpu-collapsed — все ложные (длинные стеки RegionTickOps/MinecraftServer с moonrise-`EntityIndex` в хвосте, не мост); в wall **0**, в alloc **0**; `bloom|Roaring` = **0** везде. Вывод по вопросам тика: **bloom-проверок на горячем пути нет (bloom вообще не вызывался), chain-walk не дорожe (walk не начинался), самотест overhead ноль (selfTest сидит в flushQuery, который не вызывался)**.

### 2.5 Wall-профиль (55-80% окна) — не диагностичен
61212 сэмплов: libc.so.6 **49164 (80.3%)** + clock_nanosleep **4765 (7.8%)** + read/accept/epoll_wait по 1201-1223 — сон/парковка потоков; kernel-лейны ≤1.5%. wall-collapsed в этом харнессе НЕ несёт lane-сигнала (у chk-14 тоже: broadphase 0.20%) — атрибуция «грубой wall» в absorb дала `{}` (пусто) именно поэтому. Диагностика только по cpu/alloc.

## 3. Сравнение с монстр-ногой chkmono457-14 (что отличает монстра)
1. **Механика живая**: chk-14 @d73758aARMed EntityGoalQueryOps snapshotQuery **3246 (3.1% self)** + InsideSnapOps.serve4 **2012 (1.9%)** + NavPlaneOps.handle **1327 (1.3%)** — treatment-фреймы ВИДНЫ в профиле; roar-2 treatment-фреймов 0.
2. **items-лейн убит**: chk-14 items 0.00% (−31.17пп) vs roar-2 28.37% — монстр переносит item-тик в регион-плейн, roar-2 — ваниль.
3. **broadphase**: chk-14 9.36% (его snapshotQuery и срезал walk) vs roar-2 9.90% — roar-2 сидит в(anchor-класс) без собственного среза.
4. **GC-экономика**: chk-14 STW 17.66s/Full=9/avg 129ms vs roar-2 23.55s/10/184ms — монстр +5.9s STW ДЕШЕВЛЕ; корреляция norm↔STW выше — паритет по GC был важнее скорости кода.
5. **inside_volatile**: chk-14 РОСТ 15.36% (+3.35) — известный сдвиг нагрузки; у roar-2 inside 10.31% — anchor-класс.

## 4. CAPTURE-МАТЕМ (закон 13a: потолок-числа ДО ре-ролла)
- Лейн broadphase на roar-конфиге: presence **9.90%** (11181), self **5.06%** (5717). Потолок (100% захват, пересчёт lane→norm по chk-14 эмпирике 1пп lane ≈ 1.26пп norm): **+12.5пп norm** — САМ ПО СЕБЕ СУБ-БАР (<+20).
- Реалистичный захват: мост отвечает count-exact на 4 query-типа + vanillaReplica-фолбэк по ERR_RANGE; chain-walk короче section-скана на AABB-heavy фикстуре (item×103k) — оценка захвата **40-60% self** → **Δ-прогноз +2..+4пп от broadphase + вторичный эффект pre-gate на nav_ai 13.31% (getEntities-каллеры) +1..+2пп** → **нога-прогноз +4..+7пп**.
- До пары ≥+20 обязана компоноваться: + P31 INSIDE-BATCH (+5-8пп, LEDGER) + P32+P36 SNAP-sidecar (+1.5-2.5пп) → суммарный носитель 12-17пп → пара на якоре ≤−3. (честно: даже вся композиция упирается в потолок ~+25пп; без P31 roar-семья пары не даёт).
- REFUTED-граница: если instrument-ре-ролл покажет захват <30% self при живом ARM — потолок 9.9×1.26×0.3 ≈ +3.7пп → рычаг закрывается как суб-бар-некомпозируемый (запись в LEDGER обязательна).

## 5. Гипотеза-фикс для ре-ролла (что чинить, приоритет)
1. **PRIMARY (доставка)**: инструментировать seed-путь: (а) в manager после `seeded != 1` — `describe_exception(env)` + печатать rc каждого `eidxFlush` батча (java: новый diagnostic-метод или раз-лог в seedAll через stderr-хук); (б) в EntityIndexOps.seedAll не глотать Throwable молча — печать класса исключения в ERR-канал (пассивная диагностика, поведение не меняет). Прогноз: один инструмент-ран назовёт кандидата (Throwable-класс или ERR_STRUCT-код).
2. **HARDENING**: seed-толерантность — `seeded != 1 && seeded != 0-empty-world` неправильно; правильный фикс: seedAll возвращает число засеянных уровней (≥1 при успехе), 0 только на реальной ошибке; + опциональный re-seed после `POPULATION INJECT DONE` (зеркало сойдётся note-потоком, но чистый re-seed убирает окно рассинхрона 14:54:07→конец инжекта).
3. **ARM-ГЕЙТ G1**: парсер маркеров — матч только полного ARMED-маркера (строка `ARMED shards=64 …` / `v2-roar`), не подстроки lever-id (ложный paldelta-MARKER из dormant-строки, §1.3).
4. **FAIL-FAST диспатча**: pre-bench assert в workflow: если lever_flag непуст, а через N сек после boot в stdout нет `eindex.*ARMED|<lever>.*ARMED` — нога падает ДО soak (экономия 8 мин CI-слота и band-лотереи).
5. Блоб/джар НЕ трогать: контракты верифицированы javap (§1.2), натив собирается из рефа (§1.1.3) — «ребилд блобов» (BOTTLENECK ×459) не нужен.

## 6. Preregistered гейты ре-ролла (G1-G6)
- **G1 ARM/эффект**: stdout содержит полный `eindex … ARMED shards=64 … retransform rc=0/0` И эффект-маркер (`v2-roar`/flushQuery-счётчик >0); AIOOBE=0; NCDFE=0 (T1).
- **G2 lockstep**: selfTest chain_len-vs-walked + sec-consistency + bloom-FN=0 каждые 100 flushQuery; любое false → fail-closed ERR_STRUCT → DISARM-маркер (не краш).
- **G3 GC**: young/Full-метрики в гейтах: Full ≤ 9, STW ≤ 19s, max-pause ≤ 2400ms — иначе нога депресс-кластера (не вердикт, ре-ролл в другое окно).
- **G4 популяция-паритет**: 140-165k, churn ACTIVE, fake 4/4, spawnable 289.
- **G5 TPS-бар**: norm ≥ +8пп против банка v4 на band 6.0-9.5M; pair-матем от anchor-класса того же окна.
- **G6 disarm**: broken=true → ваниль бит-в-байт (vanillaReplica), threw=0.

## 7. Внешние источники (≥3, что взято)
1. **Moonrise (spottedleaf, chunk-system)** — EntityLookup/ChunkEntitySlices$EntityCollectionBySection: per-section entity-массивы и getAllCopy() — семантика данных, которую roar зеркалит; взят контракт `sectionY = clamp(blockY>>4)` и seed-идемпотентность.
2. **C2ME (parallel entity ticking / sectioned slices)** — дисциплина fail-closed при структурных нарушениях индекса и «vanilla fallback по ошибке» — взята модель ERR_STRUCT/ERR_RANGE → vanillaReplica.
3. **Pufferfish (DAB/simulation-cache)** — прецедент occupancy-прегейтов «чанк без сущностей = нулевая работа»; roar-4KB bloom pre-gate = тот же класс рычага, но бит-в-байт-безопасный (insert-before-publish, FN-impossible).
4. **Lithium (Entity tracking / AI grazing)** — принцип «точный ответ + superset-гейт дешевле строгого скана» (relaxed bb_overlaps в count-плейне) — взято в count_chunk pre-gate.

## 8. Ответы на вопросы тика (коротко)
- **Bloom-проверки на горячем пути?** Нет: bloom-фреймов 0 сэмплов (§2.4); bloom вызывался бы только из flushQuery, который не достигнут.
- **Chain-walk дорожe?** Не измерено — walk не выполнялся; априорная маржа walk-vs-section-скан уже в capture-матем (§4).
- **Самотест overhead?** Ноль: SELFTEST_EVERY=100 flushQuery, flushQuery=0 вызовов.
- **Где просадка?** 100% доставочная (seed-fail → placebo) + STW-кластер co-рана (Full=10, 23.55s, два STW >2.2s в окне) — см. §2.2-2.3.
- **Вердикт-гипотеза**: PLACEBO/DELIVERY-FAIL, не потолок; ре-ролл с фиксами §5 по гейтам §6; потолок лейна при полном успехе +12.5пп → компоновка с P31/P32+P36 обязательна (§4).
