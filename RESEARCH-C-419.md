# RESEARCH-C-419 — chunk-pipeline (закон 8), TASK-419-C — 2026-09-23 ~03:0x +08

База: master fbb06e3 (первый мерж эпохи, cvs⊕queryplane +36.2%). Ветка round-419-c-chunk.

## STEP-0 профиль (подтверждение таргета)

Источники: round-bqa absorb (тик-417, run 35764266923 @ carrier fbb06e3-линии, извлечён из git
истории 430ffe9 — collapsed-файлы вычищены из HEAD хигиеной 4497be0, восстановлены в /tmp) +
RECON-13b/13c/13f канон + BOTTLENECKS_3 bqa.

- CPU (bqa, 105,902 сэмпла): chunk system (kernel) = **8.1%**; топ-1 лист всего профиля
  `PalettedContainer.get` = 3.9%; `SimpleBitStorage.get` 1.3%; `readPalette` 0.7%; фазы:
  chunk tick 2.5% + chunk system off-main 1.2%.
- ALLOC (bqa ap-окно): chunk_system_other = **11.40%**, codec-машинерия (DataResult/MapDecoder/
  ResourceLocation) = **6.16%**; SerializableChunkData-путь в окно не попал (burst 240-300s).
- RECON-13b (s7165, burst-окно 80-100% ap): **parse-путь = 33.38% ВСЕХ alloc-байтов**
  (45.04% в s7165 burst vs 8.70% в спокойном s7168 — RECON-13f).
- RECON-13f декомпозиция parse-лейна (burst): **codec-машинерия 19.06%** (DataResult$Success ←
  ResourceLocation.read / NbtOps.getMap / MapDecoder.compressedDecode; StringConcatHelper;
  StringLatin1.substring), **paletted-decode 13.98%** (ShortArrayList.grow ← SimpleBitStorage),
  other-parse 7.62% (String churn + dataconverter gson), chunk-struct 2.55%, nbt-io 1.82%.
- ВАЖНО (RECON-13c): BlockPos$6/RandomAccessSpliterator/AABB burst = checkInsideBlocks-cursor
  churn (ось inside, не моя); parse-[B в JFR невидим (OAS-слепота) — истина = ap-collapsed.

ВЫВОД: таргет подтверждён — **chunk-parse decode-путь** (SerializableChunkData.parse →
per-section codec.parse → PalettedContainer codec machinery). Максимальный chunk-лейн.

## Формат / механизмы (kernel jar javap, purpur 1.21.10, round-396-a fixture)

- `SerializableChunkData.parse(LevelHeightAccessor, PalettedContainerFactory, CompoundTag)`:
  секции-цикл → для каждой секции `block_states` CompoundTag → `invokedynamic #4:apply(Codec,
  ChunkPos, I)` = **lambda$parse$5** (bootstrap-таблица подтверждает: indy#4 → lambda$parse$5,
  indy#6 → lambda$parse$7 — biomes). Тело лямбды: `codec.parse(NbtOps.INSTANCE, tag)
  .promotePartial(→logErrors).getOrThrow(→ new ChunkReadException(String))`.
- `PalettedContainer.codecRW` → RecordCodecBuilder (lambda$codec$3: palette =
  qualityCodec.orElsePartial(default).listOf().fieldOf("palette"); data =
  Codec.LONG_STREAM.lenientOptionalFieldOf("data")) → PackedData(palette, Optional<LongStream>,
  bitsPerEntry=-1) → `unpack(Strategy, PackedData, default, presets)` → SimpleBitStorage/
  ZeroBitStorage/HashMapPalette/reencodeContents.
- NBT: block_states = {palette: ListTag<CompoundTag{Name,Properties}>, data: LongArrayTag}
  (NbtOps.createLongList → LongArrayTag; hashCode = Arrays.hashCode — value-based ✓).
- ВСЕ примитивные теги — records (value hashCode ✓); CompoundTag.hashCode = Map.hashCode,
  ListTag.hashCode = List.hashCode → **tag.hashCode() структурный**; equals глубокий ✓.
- `PalettedContainerFactory.blockStatesContainerCodec/biomeContainerRWCodec` — ПРЕДВЫЧИСЛЕННЫЕ
  поля record → один и тот же codec-объект между вызовами (anti-xray presets == null → тот же
  инстанс на все Y; presets != null → НОВЫЙ codec на секцию → codec-ref в ключе автоматически
  отключает хиты).
- `PalettedContainer.copy()` = глубокая копия (Data.copy(): storage+palette; strategy+presetValues
  сохраняются) — vanilla production API.

## Дизайн lever cmp419_chunk (STRICT-OR, пустой флаг = ваниль)

**Плоскость 1 — chunk-parse section-cache (новый мост ChunkParseOps + src/chunk_parse.rs):**
- Redirect ТОЛЬКО `lambda$parse$5` (blocks, главный churn) → `ChunkParseOps.parseSection` —
  static→static, тот же дескриптор (redirect_static_method_body_to_static, контракт
  zero_cursor/TASK-330). `lambda$parse$7` (biomes) остаётся ванильным twin'ом.
- `parseSection(codec, pos, y, tag)`: key = (codec-ref identity, tag hashCode структурный,
  equals глубокий) → HIT: `template.copy()` (машинерия codec/palette/SimpleBitStorage не
  вызывается вовсе); MISS: reflect-invoke ПРИСТИННОГО twin'а lambda$parse$7 = точное ванильное
  тело (логи/ошибки/DataResult — parity-by-construction) → сохранить template.copy().
- Безопасность: template никогда не мутирует (только copy() наружу); synchronized HashMap + CAP
  1024 + clear() на переполнении (~8-16MB worst case на 10G heap); codec-ref в ключе = хиты
  автоматически OFF при anti-xray presets; ThreadLocal depth-гвард (twin рекурсия невозможна —
  он не патчится; гвард для гипотетического мисконфига → реплика через MethodHandles, never-path).
- SELFTEST на боте: первые 3 уникальные секции — декод twin'ом повторно + сравнение 4096 get()
  identity + bitsPerEntry → маркер PASS в stdout (effect-маркер для чек-листа).
- per-chunk JNI = 0 (плоскость Java-side; JNI-дисциплина закона 6 удовлетворена: 0 ≤ 1).

**Плоскость 2 — noise-fill мост (TASK-108 legacy, dormант в дереве):** noise_fill.rs гейт
расширен STRICT-OR: {CRUSSTY_NATIVE_NOISE_FILL=1} ∪ {lever_flag==cmp419_chunk}; arming под
cmp419_chunk включает CRUSSTY_KERNEL_POLICY=off в run_world3.sh (документированный A/B
override двухключевого правила — noiseFillArrayWholeBody не в PROVEN_WINS). Rust-нативные
батчи (G-ABI-2, jni_table rows 282-291) уже в закрытом .so из release-ассета. GEN-ось закона 8.

## Parity-контракт (закон 4)
- Пустой lever_flag → хуки НЕ регистрируются, классы байт-в-байт ванильные (dormant-invisible).
- HIT: копия контейнера, который vanilla произвела из РАВНОГО тега тем же codec-инстансом
  (value-семантика Data.copy) — бит-в-байт.
- MISS/twin: неизменённые ванильные байты (реальный vanilla метод).
- Ошибки/частичные декоды/лог: ровно ванильные (через twin), на повторах лог не дублируется
  (задокументированный residual — лог-шум не геймплей).

## Эффект-каналы (честная оценка)
- burst-окно (240-300s): parse ≈ 33-45% alloc; hit-rate секций высок (внутри чанка 24 секции
  повторяют слои камня/воздуха; биом-палитры крошечные) → young-GC объём в burst ↓ оценочно
  25-40% → GC total (24.4s/473s, RECON-26: GC ≈ 33% CPU native-VM) ↓ → TPS +небольшие %.
- parse CPU (worker-контеншн на 4 vCPU) ↓ на codec-машинерию.
- worldgen steady ≈0% → noise-fill в bench нейтрален (видимая игроку GEN-ось, закон 8).
- Реалистичный соло-потолок оси: single-digit %. Цикл закона 3 продолжается композицией
  (law 7) — ноги передаются в PHASE-3 композицию волны-419.

## Инструменты/файлы
- bridge: chunkparse/net/minecraft/world/level/chunk/storage/ChunkParseOps.java (+ stub
  com/mojang/serialization/Codec ТОЛЬКО для javac-дескриптора — в рантайме реальный класс);
- build: scripts/build_chunkparse_ops.sh (javac --release 21 -cp round-396-a kernel jar + fastutil);
- rust: src/chunk_parse.rs (zero_cursor паттерн) + classfile::chunkparse_resolution_closure;
- флаги: CRUSSTY_LEVER_FLAG=cmp419_chunk (STRICT), env-дубликаты не добавлены кроме noise STRICT-OR.
