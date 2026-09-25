# RESEARCH-459-P21 — parse-cache widen: biomes+light (ID-P21, WILD закон 11, TASK-459-60)

Агент: TASK-459-60 (WILD-агент закона 11, тик-459). Ось: chunk-parse/serialization
(закон 8, player-visible chunk-loading) — 33.38% burst-alloc в reload-окне 240-300s
(RECON-13b/13f: codec machinery 19.06% + paletted-decode 13.98%). Развитие
cmp420_chunk2 (blocks section-cache) ⊕ cmp434_chunkpl (biomes-parse cache, TASK-424-C).

---

## 1. Механика (javap ground truth, tests/fixtures/SerializableChunkData.class,
sha256-идентичен round-396-a patched-kernel.jar; purpur 1.21.10 + Starlight/Moonrise)

`SerializableChunkData.parse(LevelHeightAccessor, PalettedContainerFactory, CompoundTag)`
декодирует секции через invokedynamic-сайты:

| сайт | indy → bootstrap | impl | статус в мастере |
|---|---|---|---|
| `ldc "block_states"` | indy#4 → bootstrap#4 | `lambda$parse$5(Codec,ChunkPos,int,CompoundTag)PalettedContainer` | ЗАПАТЧЕН → `ChunkParseOps.parseSection` (template-cache, twin-fallback) |
| `ldc "biomes"` | indy#6 → bootstrap#6 | `lambda$parse$7` — ТОТ ЖЕ канонический дескриптор | ЗАПАТЧЕН → `ChunkParseOps.parseBiomesSection` (mirror-cache, TASK-424-C) |
| `ldc "BlockLight"` + `ldc "SkyLight"` (2 сайта) | indy#8 → bootstrap#8 | `REF_newInvokeSpecial DataLayer."<init>":([B)V` — Function `byte[]→DataLayer` | **НЕ ПАТЧИЛСЯ — ваниль-путь БЕЗ кэша (цель P21)** |

BootstrapMethods#8 (javap -v fixture): impl-хендл = конструктор `DataLayer([B)`,
instantiated type `([B)Lnet/minecraft/world/level/chunk/DataLayer;`. Свет-декод в parse:

```
aload 30; ldc "BlockLight"; invokevirtual CompoundTag.getByteArray(String)Optional
invokedynamic #8 apply:()Function          // byte[] -> new DataLayer(...)
invokevirtual Optional.map(Function); aconst_null; Optional.orElse(null); checkcast DataLayer
```

Свет-декод — БЕЗ codec/DataResult, но: (а) каждый световой слой = новый DataLayer-объект
per секцию per слой (burst 24+ объектов/чанк); (б) гомогенный преген-свет (пустые секции:
`starlight.*_light_state` 0/1 + full-empty nibble) даёт массивы-дубликаты — тот же паттерн
~90% hit-rate, что biome-секции прегена. DataLayer javap (kernel jar): `protected byte[] data`,
`DataLayer(byte[])` — конструктор АЛИАСИТ переданный массив (шаблон обязан хранить защитную
копию и раздавать клоны), `getData()` public, `copy()` public.

### Что уже есть (мастер) и что добавляет P21

1. **LIGHT-плоскость (новое)**: retarget bootstrap#8 impl-хендла с
   `REF_newInvokeSpecial DataLayer.<init>([B)V` на
   `REF_invokeStatic ChunkParseOps.parseLight([B)DataLayer` — call-site descriptor и
   instantiated type НЕ меняются (stack shape контракт), меняется только impl-хендл.
   `parseLight` = cache-first: HIT → `new DataLayer(template.clone())` (контент бит-в-бит);
   MISS → `new DataLayer(bytes)` — ТОЧНО ванильный хендл (twin-fallback по конструкции:
   ретаргеченный инди был чистым конструктором).
2. **P21-ключ для light**: byte[] не может быть CHM-ключом (equals = identity) → ключ
   `((long)len<<32) | (Arrays.hashCode(b)&0xFFFFFFFFL)` + ПОЛНАЯ `Arrays.equals`
   верификация на каждом HIT (коллизия не может отдать чужой слой — «тот же
   структурно-equal ключ» в терминах карточки; риск-строка карточки «tag-hash коллизии →
   ключ (codec-identity, tagHash, len)» закрыта).
3. **Twin-fallback быстрый хвост (biomes/blocks MISS)**: `vanillaReplica` в мастере
   резолвит `getMethod("parse", ...)`/`promotePartial`/`getOrThrow` REFLECTION НА КАЖДЫЙ
   MISS-вызов — P21 кэширует резолв в volatile-поля (паттерн уже есть в `invokeTwin`),
   паритет семантики не меняется (тот же reflective call, минус getMethod-поиск).
4. **Онлайн-самотест 1/100**: light-MISS каждые 100 проверяет инвариант
   `Arrays.equals(new DataLayer(bytes).getData(), bytes)` (доказательство, что конструктор
   не трансформирует вход — предусловие корректности template-copy) → маркер PASS.

## 2. Внешние источники (≥2, механика подтверждена)

1. **Mojang/DataFixerUpper (канонический репо)** — `MapDecoder.compressedDecode`:
   на КАЖДЫЙ декод секции аллоцируются `DataResult.error(()->...)` лямбда+объект,
   `new ArrayList<>()` entries, анонимный `MapLike`, `KeyCompressor` — это и есть
   19.06% codec-machinery burst, который пропускает template.copy() HIT-путь.
   URL: https://github.com/Mojang/DataFixerUpper ,
   https://raw.githubusercontent.com/Mojang/DataFixerUpper/master/src/main/java/com/mojang/serialization/MapDecoder.java
2. **Documented DataFixerUpper API (kvverti)** — DataResult «Represents either a
   successful operation, or a partial operation with an error message»; partial-машина
   (`promotePartial`+`getOrThrow`) — обязательный хвост ванильного section-decode,
   сохранён в twin/replica-пути. URL:
   https://kvverti.github.io/Documented-DataFixerUpper/snapshot/com/mojang/serialization/package-summary.html
3. **C2ME (Concurrent Chunk Management Engine)** — соседняя ось той же плоскости
   (chunk gen/IO/loading): C2ME параллелит parse-работу, P21 УДАЛЯЕТ повторную работу
   (гомогенный преген декодирует одинаковые секции многократно). Независимое
   подтверждение, что parse-плоскость — признанный CPU/alloc-бюджет chunk-loading.
   URL: https://github.com/RelativityMC/C2ME-fabric , https://modrinth.com/mod/c2me-fabric
4. **Lithium wiki (CaffeineMC)** — прецедент кэша гомогенных biome-данных:
   «an optimized cache will be used for storing noise data used in biome sampling» —
   та же дисциплина (кэш поверх детерминированного декода, значение бит-в-бит).
   URL: https://github.com/CaffeineMC/lithium-fabric/wiki
5. **minecraft.wiki DataFixerUpper** — обзор codec/DataResult роли в сериализации
   Minecraft. URL: https://minecraft.wiki/w/DataFixerUpper

## 3. Parity-план (закон 4)

* **LIGHT HIT**: контент = `template.clone()` — template снят защитной копией с массива,
  чей ванильный декод дал бы байт-в-байт тот же DataLayer (полное Arrays.equals
  докажывает равенство на каждом HIT); выдаваемый слой НИКОГДА не алиасит шаблон и
  caller-массив (ваниль алиасит tag-массив — паритет утечки идентичности не требуется:
  tag после parse отбрасывается, loadStarlightLightData клонирует через
  MixinWorkarounds.clone перед SWMRNibbleArray).
* **LIGHT MISS**: `new DataLayer(bytes)` — байткод-эквивалент ретаргеченного
  newInvokeSpecial (twin-fallback = сам ванильный хендл).
* **Ключ без коллизий**: probe = (len,hash) 8-байт long; cert = полная Arrays.equals.
  Коллизия хеша → просто MISS/пере-put (не неверный слой).
* **NCDFE-канон (EARLY-define)**: ChunkParseOps определён в kernel-loader ДО flip READY
  (активатор не менялся) → первое выполнение ретаргеченного indy не может дать
  NoClassDefFoundError. Сайт indy, УЖЕ разрешённый до retransform, остаётся на ванильном
  конструкторе (CallSite кэш) → плоскость инертна до следующей загрузки класса —
  fail-inert, не fail-broken.
* **Гейт**: lever_flag STRICT-OR master ∪ {cmp459_p21}; пустой/чужой флаг → хуки не
  регистрируются, класс байт-в-байт ваниль.
* **Retarget guards**: ровно 1 bootstrap-аргумент с newInvokeSpecial DataLayer([B)V
  (дрифт → dormant); ровно 2 indy-сайта ссылаются на bootstrap#8 в fixture (BlockLight +
  SkyLight); anti-placebo: sites>0 иначе disarm.

## 4. Δ-прогноз (числа, закон 11)

* Biomes-плоскость (мастер) уже снимает codec-прогон на ~90% гомогенных biome-секций
  прегена; P21 добавляет: light-слои (каждый 2-й/3-й слой пустого преген-чанка —
  дубликат full-empty nibble) + дешёвый probe + быстрый MISS-хвост.
* Ожидаемый эффект на reload-сценах: **+1-2пп** (карточка), механизм: light-плоскость
  убирает DataLayer+клон-бурст и P21-ключ убирает deep-probe стоимость на light;
  GC-debt relief (DataResult/ArrayList/MapLike мусор на biomes-MISS ↓, light-объекты ↓).
* Инертно на преген-soak (GEN-ось), carrier-член chunk-оси — живой на reload-сценах.

## 5. Риски

| риск | митигация |
|---|---|
| tag-hash коллизия отдаёт чужой слой | полная Arrays.equals верификация на каждом HIT (probe ≠ cert) |
| DataLayer(byte[]) алиасит вход | template = защитный клон; HIT раздаёт `template.clone()`; MISS — ваниль |
| indy-сайт разрешён до retransform → плоскость инертна | fail-inert (ванильный конструктор жив); статистика lightHits в stats() покажет инертность |
| ChunkParseOps не определён к первому indy-выполнению | NCDFE-канон: define+init ДО READY (существующий активатор, ретраи) |
| kernel drift (третий light-сайт / другой ctor) | retarget guard: ровно 1 bootstrap-аргумент DataLayer([B)V, sites==2 на fixture, иначе dormant |
| CP переполнение | pool overflow guard (next > u16::MAX-16 → Err → dormant) |

## 6. Файлы

| файл | Δ |
|---|---|
| `chunkparse/net/minecraft/world/level/chunk/storage/ChunkParseOps.java` | +parseLight, LIGHT_CACHE (P21-ключ), кэш reflective-хендлов replica, carrier cmp459_p21, stats |
| `chunkparse/build/.../ChunkParseOps.class` | rebuild (scripts/build_chunkparse_ops.sh) |
| `src/classfile.rs` | +TAG_METHODHANDLE/Pool::method_handle, +retarget_light_bootstrap_to_static, closure+parseLight, fixture-гейты |
| `src/chunk_parse.rs` | 3-я ступень retarget (light) в activate(), гейт cmp459_p21, маркеры, delivery-тесты |
