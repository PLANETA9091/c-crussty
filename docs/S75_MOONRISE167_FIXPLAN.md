# S75 FIX-PLAN — Moonrise #167 parity-debt в crussty-ядре (EntityCollectionBySection query-box)

ROUND-468 / S75 / ветка `round-468-s75-mr167-parity` (base c1196321 = origin/master).
Вердикт-лейн: **RESEARCH/parity (закон 4)** — это НЕ перф-рычаг, TPS-дельта ≈ 0 (матем ниже).

## 1. Долг подтверждён javap на блобах (RESEARCH ≥5 чисел)

Носитель: `/home/z/tools/patched-kernel.jar` (Purpur 1.21.10-2535, 0a2dc04, build 2025-12-10
18:34:22 -0800, jar 29,386,794 B). Класс `ca.spottedleaf.moonrise.patches.chunk_system.level.entity.
ChunkEntitySlices$EntityCollectionBySection` (5,168 B, major 65).

1. `getEntities(...)V`: pc30 `ldc2_w 2.0d` → `dsub` (minY − **2**), pc52 `ldc2_w 2.0d` → `dadd` (maxY + **2**).
2. `getEntitiesLimited(...)V` (лимитный lane, пуш-стопы/merge-caps): pc31 `2.0d`+`dsub`, pc53 `2.0d`+`dadd`. Итого **4 сайта** `2.0d` на класс (grep javap == 4).
3. Ваниль-референс в ТОМ ЖЕ jar: `EntitySectionStorage.forEachAccessibleNonEmptySection(AABB,…)`: X = ±2.0 (`posToSectionCoord`), **Y: minY − 4.0 (`ldc2_w 4.0d`) … maxY + `dconst_0` (+0.0)** — ваниль **−4/+0**, асимметрия подтверждена байткодом (совпадает с mcsrc 1.21 EntitySectionStorage#L37 из PR #188).
4. X/Z-паритет OK: Moonrise `EntityLookup.getEntities` = `floor(minX)−2>>4 … floor(maxX)+2>>4` = ваниль точно (`floor(x−2.0) == floor(x)−2` для целого 2.0). Долг ТОЛЬКО по оси Y.
5. Caller-плоскость: **7** query-оверлоадов `ChunkEntitySlices` (`getHardCollidingEntities` + 6 `getEntities`) все делегируют в 2 метода ECBS; носители = **4** коллекции (`allEntities`, `hardCollidingEntities`, `entitiesByClass`, `entitiesByType`). Т.е. пистоны/лодки/шалкеры (hard-collide), таргетинг, пуш — ВСЕ через битые константы.
6. Upstream: **issue #167** (Tuinity/Moonrise, created 2026-02-25, MC 1.21.11, Moonrise 0.9.0-beta.4): «expands getentities search box by 2 blocks up and 2 blocks down instead of 0 blocks up and 4 blocks down as in vanilla» + репро «пистон не толкает гаста из нижнего субчанка». Фикс — **PR #188** «Match vanilla vertical entity retrieval in chunk sections», **merged 2026-08-07T01:55:22Z**, merge **d71d640a**, diff **+6/−4, 1 файл**, оба метода 1:1 как у меня.
7. Ядро старше фикса на **240 дней** (2025-12-10 build vs 2026-08-07 merge) — «#188 не доехал» подтверждено и датой, и байткодом (pre-gate: 4×2.0d, 0×4.0d, 0×0.0d).

## 2. Патч-механика (уже сделана, артефакты в ветке)

`mr167/Mr167Patch.java` (ASM 9.7.1 core, size-preserving):
- `LDC2_W 2.0; dsub(0x67)` → `LDC2_W 4.0` (minY −4.0), `LDC2_W 2.0; dadd(0x63)` → `LDC2_W 0.0` (maxY +0.0) — 4 сайта, name+descriptor-pin (канон S43 symbol-pin, НЕ offset).
- Delay-slot visitor: любой чужой `ldc2_w 2.0` вне dsub/dadd-паттерна → refuse (kernel-drift guard).
- LDC2_W→LDC2_W = 3 байта → **pc-layout идентичен** (getEntities ветка `if_icmpgt 209`, getEntitiesLimited `223` — совпадают с оригиналом), StackMapTable/maxs переиспользуются verbatim (ClassWriter(0)). Файл 5168→5177 B (+9 = только CP-repack: добавлены CONSTANT_Double 4.0/0.0).
- Load-check OK: `Class.forName(…, false, URLClassLoader[patched-dir, kernel.jar])`.

Гейты: `scripts/mr167_javap_gate.sh`
- `pre`: DEBT = ровно 4×`2.0d`, 0×`4.0d/0.0d` → GREEN на текущем ядре.
- `post`: 2×(`4.0d`+`dsub`), 2×(`0.0d`+`dadd`), pc-layout 209/223, 0×`2.0d` → GREEN (прогнан, вывод в §4).
- exit 2 = «ALREADY-FIXED upstream» (если ядро когда-нибудь ребазится на Moonrise ≥ d71d640a — рычаг mr167 сносить, не патчить).
- One-shot: `mr167/build_and_patch.sh` (pre-gate → extract → patch → install → post-gate → load-check). Эталонный патченный класс: `mr167/ChunkEntitySlices$EntityCollectionBySection.mr167.patched.class`.

## 3. Рантайм-фикс-план (след. шаг, 0 Rust-дельт НЕ получится — нужен ASM-hook)

Вариант A (канон, рекомендую): cplug ClassFileLoadHook-трансформ — новый модуль по образцу `src/chunk_sched.rs` (Retargeted-путь) / `src/classfile.rs` (append-only CP): при загрузке `ChunkEntitySlices$EntityCollectionBySection` применить ту же 4-сайтовую констант-своп-логику (one-delay visitor) перед define. Lever `cmp468_mr167`, STRICT-arm из rust (0-дельта стиль S22/S96).
Вариант B: whole-class blob redefine (прецедент `SingleUserAreaMapOps` в `src/area_map.rs`) — требует byte-exact пересборки Moonrise-исходника против ТОЧНОЙ версии ядра; хрупко к любому bump ядра → A предпочтительнее.

Порядок:
1. ветка += ASM-трансформ в cplug + selfTest-маркер `mr167: query-box -4.0/+0.0 ARMED` в javap-логе рана;
2. гейт в CI-лог: pre-gate (4×2.0d) → post-arm javap dump (2×4.0d+dsub, 2×0.0d+dadd) — как check_blobs_sync, но против живого рана;
3. паритет-харнесс на fixtures (`tests/fixtures/ChunkEntitySlices_real.class` уже в репо): синтетика — entity pos.y = minY−3.0, bbox height 4.0 (гаст-класс), query box [minY, minY+1]: до фикса MISS, после FIND; симметрично phantom-band сверху (pos ≤ maxY+2, bbox dips in) — до фикса FIND, после (по ванили) MISS. Оба направления обязаны совпасть с ванильным `EntitySectionStorage`;
4. диспатч банк-EXACT ваниль (инпуты §SWARM_SPEC.3) + parity_validator.py на паре логов до вердикта; NCDFE=0, selfTest==true, threw=0;
5. прогноз norm: **0 ± A/A-шум** (скан-спан Δ ∈ {−1, 0, +1} секций на запрос: у Moonrise суммарное расширение тоже 4 блока, просто сдвинуто на −2 → стоимость фикс-а ≈ 0). Вердикт-лейн = закон-4 паритет, НЕ ≥+20пп.

## 4. Прогон-протокол (сделано в этой сессии)

```
$ bash scripts/mr167_javap_gate.sh pre
kernel /home/z/tools/patched-kernel.jar: ldc2_w 2.0d=4, 4.0d=0, 0.0d=0
GATE pre: DEBT CONFIRMED (4x 2.0d sites = Moonrise #167 present)

$ java -cp asm-9.7.1.jar:. Mr167Patch <ECBS> patched.class   # 4 sites, 5168->5177 (CP repack)
$ bash scripts/mr167_javap_gate.sh post mr167/patched
GATE post: FIXED = vanilla (-4.0/+0.0), pc-layout preserved, loadable

$ javap patched: pc30 ldc2_w 4.0d + dsub (minY−4.0); pc52 ldc2_w 0.0d + dadd (maxY+0.0) — оба метода
```

Урок в-строю: первый прогон дал ИНВЕРТИРОВАННЫЕ константы (minY+0/maxY+4) — javap-гейт post с проверкой «4.0d→dsub / 0.0d→dadd по соседству» поймал это ДО ранта; причина: dadd=0x63, dsub=0x67 (не наоборот). Гейт теперь инвариантен к такого класса ошибкам.
