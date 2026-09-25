# RESEARCH-459-L06 — paldelta-климб: P32+P36 SNAP sidecar+epoch поверх cmp457_paldelta (закон 13/14, v18.3)

Агент: TASK-459-L06 · слот L06 · носитель cmp457_paldelta @29876077 (в master 0d147876, CERT-FIX блобы) · leg-3 = +8.3 @6729996 (суб-бар → CLIMB, закон 13b).

## 0. Отправная точка (банк ×459)
- paldelta-3: median 2.4 TPS @6729996, TPS_exp(v4)=2.216, norm = 2.4/2.216−1 = **+8.30%**, NCDFE=0, ARM {"cmp457_paldelta":"ARMED"} (ABSORB leg-3, run 36150335881).
- Пара ≥+20 требует якорь norm ≤ −11.7 возле 6729996 — в банке таких нет (ближайший a4 −0.8@6737702, Δ7706) → суб-бар не берётся парой сегодня, нужен климб ноги.

## 1. Декомпозиция paletted/java_util лейнов (числа сэмплов, лист-ранжирование)

### leg-3 (round-round-458a-paldelta-3-36150335881, CPU self 104552 сэмплов, full-bridge, BENCH-4, pop 150225..153565)
Лист-ранжирование (self-сэмплы / % от 104552):
1. `PalettedContainer.get` — 3145 / 3.0% (fast-path + miss-dispatch демукса)
2. `PalettedContainerOps.get` — 2352 / 2.2% (slow-path демукса: бит-декод + палитра + счётчик)
3. `InsideSnapOps.serve4` — 1960 / 1.9% (inside-snap serve, V4)
4. `InsideBlockOps.gate` — 877 / 0.8% (ретаргет-вход)
5. `java/util/concurrent/ConcurrentHashMap.get` — 935 / 0.9% (SNAPS + прочие CHM)
6. `LevelChunk.getFluidState` — 1080 / 1.0%
7. `java/lang/invoke/VarHandleReferences$FieldInstanceReadOnly.getVolatile` — 1205 / 1.2%
8. `SimpleBitStorage.get` — НЕТ в top-40 (корт <573 / <0.5%) — эффект демукса!
9. JDK collections бакет — 7140 / 6.8% (HashMap.getNode 1282/1.2%, ConcurrentLong2ReferenceChainedHashTable.getNode 1234/1.2%, LongOpenHashSet.add 862/0.8%, ArrayList.isEmpty 573/0.5% → top-40 покрывает 4886/7140 = 68% бакета)
10. Итог paletted-get лейн leg-3: 3145+2352+(<573) ≈ 5.2-5.7% CPU.

### Ваниль-профили (для базлайна демукса; cpu-collapsed пуржнуты — числа из BOTTLENECKS_3, wall-листья сверены по wall-collapsed.txt)
- anchor458-33: PalettedContainer.get 5384/4.7% + SimpleBitStorage.get 1602/1.4% + readPalette 1067/0.9% = **7.0%**; JDK collections 6361/5.5%; HashMap.getNode 1759/1.5%.
- chkmono457-14 (монстр +21.7): 4151/4.0% + 1083/1.1% + 749/0.7% = **5.8%**; HashMap.getNode 1459/1.4%; CHM.get 992/1.0%; JDK collections 7330/7.1%. Wall-листья: PalettedContainer.get 169, LevelChunk.getFluidState 169, LevelChunk.getBlockState 125, SimpleBitStorage.get 25.
- chkmono457-16: 3952/3.8% + 791/0.8% + 685/0.7% = 5.3%; chkmono457-11 (депресс, Full=9): 4179/4.0% + 820/0.8% + 760/0.7% = 5.5%; JDK collections 8031/7.7%.
- ВЫВОД (кросс-ран оговорка): демукс срезал SimpleBitStorage.get с 0.8-1.4% до <0.5% и держит суммарный лейн 5.2-5.7% против 5.8-7.0% ванили; остаток = fast-path prologue (3.0%) + slow-path (2.2%) — stride-экономика (4096 медленных чтений на материалайз) съедает часть выигрыша на write-активных секциях.

## 2. javap-контракты (round-396-a/patched-kernel.jar = канон javac-cp + tracked блобы master)
1. `PalettedContainer.get(int)`: `getfield data` (volatile) → `invokeinterface BitStorage.get` (itable-диспатч!) → `readPalette`; readPalette = `checkcast FastPaletteData` → `moonrise$getPalette()[raw]` (aaaload, moonrise fast-palette) else readPaletteSlow.
2. `SimpleBitStorage.get(int)` (final): `magic*i >> 20` (индекс слова) + `(magic*i & 1048575)*mulBits >> 20` (сдвиг) → `data[q] >>> r & mask` — Moonrise-стиль packed-декод (уже оптимизирован; добивать тут нечего — лейн убит демуксом).
3. `PalettedContainerOps.get(PalettedContainer,int)` (блоб paletted/build): ваниль-эквивалент body + `crusstyMiss++` + epoch-гейт `epoch<2` со stride-масками `sipush 4095/16383` + `tryMaterialize` — javap байт-в-байт совпадает с src/classfile.rs контрактом (публикация snap→snapGen, volatile store 1/2).
4. `InsideSnapOps.serve4` (блоб entityinside/build): 2 гейт-сайта `getfield Snap.builtAtGen:J` + `getfield Snap.gen:J` volatile-long сравнения (офсеты 388/393 и 420/425 serve-пути) — ЦЕЛЬ P36; `SNAPS.get(sec)` + `register(sec)` per-claim — ЦЕЛЬ P32.
5. CERT-FIX жив-гейт: `javap -v` ценз 10/10 tracked блобов (colpush/entitygoalquery/entityinside/goalops/mobai/mobpush/queryplane/randomtick/sense/sscan) содержат Utf8 `cmp457_paldelta` (2-3 вхождения на класс); build_430b_blobs.sh ре-ран на HEAD: `flat==nested gates: OK` (gate_fe ×11, вкл. InsideSnapOps$Snap) + indy raw-byte gate; svorta-архив agent-N: 10/10 EQUAL (RESULT: 10/10 LEVER-ONLY → после дособора 10/10 EQUAL); git-tracked блобы байт-идентичны (worktree чист).

## 3. Офлайн-эксперимент (закон 16: lockstep/харнесс где возможно)
- Reflection-smoke против round-396-a jar + paletted/build + entityinside/build + fastutil.jar: `PalettedContainerOps.selfTest()=true`, `LIVE=0`, `InsideSnapOps.SNAPS = ConcurrentHashMap size=0` — машина демукса loadable, cap-математика валидна (полный PalettedContainer-инстанс требует полного minecraft-cp — офлайн-оракул ограничен, отмечено честно).
- Бит-в-байт оракул для ног (preregister G2): serve4 hit ДОЛЖЕН возвращать ТОТ ЖЕ BlockState-референс, что ваниль `sec.states.get(packed)`/readPalette-слот; secWrite-бамп gen mid-read → miss → vanilla continuation. Прецедент-механика уже в коде (ref-compare old!=new в secWrite ⇒ snapshot exact).

## 4. CAPTURE-МАТЕМ (lane% × захват% = Δ; потолок = lane% × 100%)
Адресуемый лейн P32+P36 на leg-3:
- serve4 self 1.9% — P36 (epoch fast-gate: builtAtGen==gen 2×volatile-read → один plain long-tag compare) + P32 (flat-реестр вместо CHM-хопа) легально съедают 40-60% self → 0.76-1.14 пп.
- CHM.get 0.9% глобально; SNAPS-доля ≤50% (0.45 пп — остальное heap-map/roar чужие) → P32 захват 60-80% → 0.27-0.36 пп.
- **Δ-прогноз P32+P36 = +1.0..+1.5 пп** (ledger-оценка +1.5-2.5 пп была по inside_volatile 12-16.6% плоскости RED-ног; на leg-3 экономике лейн меньше). Потолок пары лейнов (100% захват) = **+2.35 пп**.
- VarHandle getVolatile 1.2% НЕ трогаем (P33 отдельный рычаг: demotion Acquire→Plain, +0.6-1.2 пп).
- 1 пп norm @6729996 ≈ +0.022 TPS median — Δ сопоставим с полл-шумом ⇒ вердикт по маркерам-эффекту + min-of-3, не по одиночному median (честная оговорка).

## 5. Потолок ПАРЫ (какой якорь нужен) — лестница климба
| Шаг | прирост | нога (norm) | якорь для пары ≥+20 (Δ≤50k от 6729996) |
|---|---|---|---|
| leg-3 сейчас | — | +8.3 | ≤ −11.7 — НЕТ в банке |
| +P32+P36 | +1.0-1.5 | 9.3-9.8 | ≤ −9.7..−10.7 — НЕТ |
| +P31 INSIDE-BATCH (климб-кандидат #1) | +5-8 | 14.3-17.8 | ≤ −5.2..−2.2 — НЕТ возле 6.7M |
| +P33 VarHandle demotion | +0.6-1.2 | 14.9-19.0 | ≤ −4.1..−1.0 — a4 −0.8 покрывает только ≥19.2 |
| +P34 квант-гейт K-тик-покой | +2-4 | **16.9-23.0** | ≤ −3.1..+3.0 — a4 −0.8@6737702 (Δ7706) и окно-C хвост [6761670,6779996] anchor ≤−1.3 покрывают ногу ≥18.7 |
- ОТВЕТ «какой якорь нужен»: пара paldelta-климба берётся якорем **norm ≤ −0.8..−1.3 в [6729996,6779996]** (a4-класс или свежий ролл в окне-C хвосте); критический порог ноги **≥ +18.7-19.2**. P32+P36 — НЕ pair-maker (+1.0-1.5 пп); pair-maker = P31 (+5-8) + P34 (+2-4) как swing-vote; без P34 нога 17.4-19.0 = пограничная лотерея.
- Конкуренция окон: a4/окно-E принадлежат chk-19-климбу (E: anchor ≤−7.2 для ноги 12.8) — наш порог ≤−1.3 СЛАБЕЕ chk-19-требования ⇒ paldelta-климб пairs-friendly к тому же якорному пулу (один ролл закрывает обе миссии, если anchor ≤−7.2).

## 6. Спящие блобы — lesson (обязательный гейт вайринга)
Факт ×459: roar-2 DELIVERY-FAIL (ARM-маркера нет, код на eb47e869 есть) = джава-блоб спит; paldelta-1/2 — то же. Сегодняшняя верификация CERT-FIX (см. §2.5): блобы ЖИВЫ (10/10 flag-census + gate_fe OK + svorta 10/10 EQUAL + selfTest=true). ПРАВИЛО для P32+P36-вайринга: правка только SOURCES = placebo (×425/×458-F1) — каждое изменение обязано пройти `scripts/build_430b_blobs.sh` (rebлоid + gate_fe flat==nested) ДО диспатча + javap-ценз флага на пересобранных блобах + строка `flat==nested gates: OK` в логе ноги.

## 7. Внешние источники (≥3, по памяти — что взято)
1. **Moonrise (Spottedleaf)** — fast_palette (FastPaletteData.moonrise$getPalette) + packed SimpleBitStorage (magic/mulBits) + collision/entity-lookup rewrite: в ядре УЖЕ есть (javap §2.1-2.2) ⇒ палитра/бит-декод не лейн; остаток = уровень контейнера (volatile data + itable) — ровно то, что снимает demux.
2. **Lithium (CaffeineMC)** — flattened palette / flat value-table (mixins world.chunk.palette) и collision fast-paths: взята идея плоского `vals[demux[i]]` (наш demux vals = таблица слотов палитры, Lithium-стиль).
3. **C2ME (ishland)** — concurrency chunk-data access / snapshot-публикация: взята seqlock-архитектура (odd/even write epoch, re-validate после построения) — наш crusstyGen-протокол = та же дисциплина.
4. **Paper/Pufferfish** — single-call-site retarget вместо wholesale-rewrite (inside-block optimization эпохи 1.19-1.21) + lane-массивы per-claim (V4 serve4): подтверждает P32/P36 механику «кэш на границе вызова», а неTraversal-замена (flat_traversal запрещён законом 5).

## 8. PREREGISTERED ГЕЙТЫ G1-G6 (для климб-ноги P32+P36 поверх paldelta)
- **G1 ARM/эффект-маркеры**: ARM-строка содержит {"cmp457_paldelta":"ARMED"} + V4-inside ARM; ЭФФЕКТ ДО пурджа: serve4-hit-каунтер >0, flat-registry lookups >0, epoch-gate passes >0, `paletted: builds=` строка присутствует; NOT-A-BENCH маркер; retargeted sites:N>0 + DATA-PLAN (snap population >0 — канон ×425 DATA-ПЛАН-гейт).
- **G2 lockstep бит-в-байт**: офлайн-harness secSnap-serve == vanilla identity (референс BlockState) на 4096-секциях; mid-read secWrite-бамп → miss→vanilla; AIOOBE=0 (biomes-sw 201d9d66 whitelist исключений).
- **G3 young/Full GC**: Full ≤9 (leg-3 факт), total STW ≤ 19226ms +10%, young-паузы в классе leg-3 (94 + CodeCache 5).
- **G4 популяция-паритет**: entity totals 148289..153565 (churn ~3.5% ACTIVE), alive-check 4/4, spawnable-chunks 289.
- **G5 TPS-бар vs банк v4**: runner band [6.0M,9.5M] (ре-ролл ≤2); norm = median/TPS_exp(runner)−1; гипотеза-дельта прегист: Δ ≥ +1.0пп vs paldelta-3 ⇒ нога ≥ +9.3; pair-target ≥ +18.7 (полный стек).
- **G6 fail-closed disarm**: любой Throwable → vanilla-хвост (serve4 fail-dominant); threw=1 = REFUTED; NCDFE=0 на T1 (EARLY-define EntityGoalQueryOps канон); items 0.00-гейт на серт-ногах.

## 9. Вердикт-числа L06
- Lane P32+P36 на leg-3: **2.35%** (serve4 1.9 + SNAPS-CHM 0.45) × захват 40-60% → **Δ +1.0..+1.5 пп**; потолок лейна **+2.35 пп**.
- Paletted-get лейн: демукс уже срезал 5.8-7.0% → 5.2-5.7% (SimpleBitStorage 1.1-1.4% → <0.5%); дальнейший резерв — stride-перетюн (4096→2048 на dense-секциях), гипотеза на отдельный диспатч.
- Потолок ПАРЫ: нога ≥ **+18.7-19.2** ← P31(+5-8) + P32+P36(+1.0-1.5) + P33(+0.6-1.2) + P34(+2-4) = суммарно +8.6-14.7 → нога 16.9-23.0; якорь: **norm ≤ −1.3..−0.8 в [6729996,6779996]** (окно-C хвост / a4-класс).
- CERT-FIX блобы: живы, javap 10/10 (flag-census + gate_fe OK + svorta EQUAL).
