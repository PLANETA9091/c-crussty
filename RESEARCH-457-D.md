# RESEARCH-457-D — TASK-457-D (agent-d, C1 SIMD-noise, закон-8 ось; base ea1ccfa1 = origin/master)

## §1 ИНТЕРНЕТ-РИСЁРЧ (z-ai web_search CLI, 3 запроса, 2026-09-25 21:4x +08)

1. **Java Vector API (jdk.incubator.vector, JEP 508 10th incubator)**: Vector API с AVX-512
   легитимно 10-16× быстрее скаляра (javacodegeeks 2026-03); ARM-порт ускоряется через
   SVE/NEON backend (community.arm.com 2026-10). В JDK 21 пакет ЕСТЬ, но требует
   `--add-modules jdk.incubator.vector` на РАНТАЙМЕ. Для нашего ядра это значит: blob,
   определённый JVMTI-define'ом в kernel classloader, должен резолвить incubator-модуль —
   не гарантировано ⇒ NCDFE-риск того же класса, что ×456-отравление.
2. **SIMD Perlin/шумы (Rust wide / практика)**: SIMD-векторизация перлина даёт заметный
   выигрыш только на МАССОВОЙ генерации (procedural-пайплайны); на подаче редкими срезами
   выигрыш съедается переупаковкой. Подтверждает форму «один bulk-вызов на срез» (закон 6).
3. **Minecraft worldgen noise профили**: community-профили серверов — noise-ось заметна
   только на генерации новых чанков (worldgen phase), на прегенерированных мирах ≈0 —
   совпадает с локальным RC1 (§2).

Артефакт интернет-действия: joml-1.10.7.jar скачан с Maven Central (repo1.maven.org)
и установлен в /home/z/tools (восстановление тулчейна после дискового пурджа — javac
блобов agent-d зависел от org.joml.Vector3f).

## §2 ЛОКАЛЬНЫЙ РИСЁРЧ

- **NOISEFILL_ROOTCAUSE.md (TASK-421-C, ×457/agent-a copy)**:
  RC1 — GEN-ось инертна на преген-фикстуре: `worldgen/noise (kernel) = 42 сэмпла = 0.0%`
  soak-CPU; forceload 9216 чанков = PARSE, не worldgen.
  RC2 — parse-бурст BOOT-only, поллы его пропускают.
  RC6 — стабильная составляющая chunk-плоскости = GC-debt relief ≈ +6-8пп.
- **Мастер уже несёт ДОРМАНТНУЮ batch noise-плоскость** (TASK-108/419-421 heritage):
  `src/noise_fill.rs` — whole-body swap `DensityFunctions$Noise.fillArray` +
  `DensityFunctions$ShiftNoise.fillArray` + `NoiseChunk$NoiseInterpolator.interpFillArray`
  → `NormalNoiseBatchOps` (record через ВАНИЛЬНЫЙ fillAllDirectly + ОДИН native crossing
  nativeFillScaledPositions/nativeFillShiftA/nativeFillShiftB, jni_table.rs:282-291,
  бинарная поверхность native/libpaper_native_jni.so). Parity-by-construction +
  raw-bits selfTest (гейт: «bench refuses armed runs on SELFTEST FAIL»).
  Почему дормант: kernel-policy двухключевое правило — fill-family НЕ в PROVEN_WINS
  (decide → KeepJava при дефолтном Strict), env-ключ CRUSSTY_NATIVE_NOISE_FILL на бенче
  не ставится.
- **improved_noise.rs**: PaperNativeImprovedNoise (nativeNoise/nativeBuildHandle/
  nativeFreeHandle) = `live` в PROVEN_WINS, но env-гейт CRUSSTY_NATIVE_IMPROVED_NOISE
  выключен по умолчанию.
- **Профили ×457**: топ-лейны items/nav/broad/fluid/inside — noise-лейна нет (согласовано
  с RC1). Серт-носитель cmp450_chunk (chunk-comp ×455, master f44a831e) несёт
  ins4⊕senseins⊕chunk4-send⊕chunk5-encode⊕chunkparse⊕noise-GEN(partial: perlin live).

## §3 БЕЗУМИЕ (закон 11a — идея/почему ≥+20/риск/parity-план/вердикт)

### C1a — LEVER-SCOPED ARM дормантной bulk-JNI noise-плоскости — **GO (доведена до ноги)**
- Идея: не писать новый SIMD-код, а ВЕРНУТЬ В СТРОЙ уже сертифицированную по parity
  подсистему: lever `cmp457_noisesimd` (a) добавлен во ВСЕ гейты серт-юниона
  (STRICT-OR: 22 rust-файла + 10 java-блобов, механика fe7e7120/c25782b4), (b) армит
  fill-family (kernel_policy.rs `lever_scoped_allow` — УЗКО: только 3 (class,kernel)
  ключа × только мой lever; Strict-режим не тронут, пустой lever = vanilla бит-в-байт),
  (c) армит improved-noise native-плоскость. Закон 6: подсистема целиком = вся
  fillArray-генерация выбранного слоя → один bulk-JNI на срез.
- Почему может дать ≥+20: сама noise-ось — НЕТ (RC1), честно; ≥+20 берётся экономикой
  серт-носителя (монстр-охота юниона: семейства chunkmono/poi дают ноги −0.8..+16.9,
  сертные носители 25-50% монстров). Noise-плоскость = закон-8 лег + effect-маркеры.
- Риск: GEN-классы могут вообще не загрузиться на прегене ⇒ нет noise ARM-маркеров
  (документирую как RC1-подтверждение, не delivery-fail). SelfTest-FAIL = refused arm
  (плоскость молча падает в ваниль, нога продолжает нести юнион).
- Parity-план: raw-bits selfTest + STRICT-OR + флаг-гейт + kernel-policy аудитор
  (audit_wire пишет каждый wire).
- Вердикт: **GO** — leg-1 на round-457d-noisesimd.

### C1b — Java Vector API (jdk.incubator.vector) векторизация октав NormalNoise — **PARK**
- Почему: JEP 508 — реально 10-16×/AVX-512, НО incubator-модуль должен резолвиться в
  kernel loader'е (JVMTI-define путь) — NCDFE-риск ×456-класса; и RC1: на фикстуре
  эффект ≈0. Разворот 4-6ч > бюджета. PARK до раунда с GEN-фазой в бенче (idea bank).

### C1c — Rust-SIMD рерайт нативных noise-кернелов (wide/simd crate) — **PARK**
- Нативная поверхность = предсобранный libpaper_native_jni.so (исходников .so в дереве
  нет) — рерайт = восстановление нативного тулчейна + перекат JNI-манифеста, вне
  бюджета тика. Парити-план был бы тот же (raw-bits selfTest). PARK (idea bank ×458).

### C1d — noise prefab-кэш (chunkPos,seed)→density — **PARK (запрет-смежное)**
- Мемо-механика на GEN-слое: (а) бессмысленна на прегене (RC1), (б) мемо-плоскости
  флейворно пересекаются с запрещённым классом fluid_dirty-мемо (закон 5). PARK.

## §4 Δ-ПРОГНОЗ (закон 9, ДО имплементации; записан на BOARD до ноги)
- Marginal noise-плоскости на фикстуре: **≈0.0-0.3пп** (RC1 0.0% soak-CPU; boot worldgen
  tail negligible). Ноль религиозных ожиданий — это НЕ молитва, это зафиксированный ценз.
- Нога (носитель юниона + noise-плоскость): ожидание = распределение серт-носителей
  (−0.8..+18.7 norm); целевой монстр ≥+18 → пары ≥+20 с депресс-якорями пула (−0.8@6737702,
  −1.8@6843586, …) Δ≤50k, min-of-3.
- Гейты вердикта: T1 NCDFE=0, AIOOBE=0, selfTest==true, threw=0, band 6.0-9.5M,
  ARM (юнион-плоскости; noise-ARM = бонус-свидетельство оси), items 0.00.

## §5 ИМПЛЕМЕНТАЦИЯ (факт)
- scripts_457d/union_widen_457d.py + pass2: cmp457_noisesimd в 22 rust-гейта (включая
  noise_fill lever-union) + Ok/Some marker-arms + 10 java-блобов (.equals-цепи,
  string-first equals, FLAG_457D/CARRIER_UNION_457D raw-cp иглы) + check_blobs_sync.sh
  иглы (×10).
- kernel_policy.rs: `lever_scoped_allow` (fill-family × lever) перед default KeepJava.
- improved_noise.rs: lever-гейт += cmp457_noisesimd (PROVEN_WINS live-ключи уже есть).
- Блобы: ОДИН javac-pass (урок ×93) 9 ops-классов (colpush/mobpush/mobai/sscan/
  entityinside/sense/goalops/entitygoalquery/queryplane) cp=kernel-396a+fastutil+paper+
  adventure+joml; chunkparse/chunksend пересобраны своими скриптами; install nested+flat;
  `check_blobs_sync.sh: ALL IN SYNC` (needles cmp457_noisesimd ×10, javap flat==nested).
- cargo check --lib 0 err (118 pre-existing warnings); cargo test 329/0+1ign = серт-база.
- joml-1.10.7.jar возвращён в /home/z/tools (Maven Central) — дисковый пурдж съел jar,
  без него блобы entityinside не собираются (java-урок ×457: тулчейн-jar'ы не пурджить).
