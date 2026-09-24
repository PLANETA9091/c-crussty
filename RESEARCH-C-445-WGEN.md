# RESEARCH-C-445-WGEN — TASK-445-C (tick-445, 2026-09-24, base master 8eded686)

Мандат владельца 2026-09-23: ≥+20% pair-stable; закон 8 R5 («оптимизация шума в генерации
мира — чанки медленно грузятся»). Ось TASK-445-C = WGEN NOISE-FILL (GEN/worldgen), НЕ
chunk-send (chunk5 едет у 444-B) и НЕ спавн-сканы (sscan2 certified).

Правило цикла (бриф): javap-ценз/профайл-ценз ДОЛЖЕН доказать hot path ДО имплементации;
если лейн <2% в дискретных замерах — честный LOW-POTENTIAL вердикт с числами и НЕТ
имплементации (валидный финиш-этап цикла).

## 1. javap-ценз (vanilla hot path, purpur-1.21.10 kernel, javap -p -c)

GEN-стек чанковой генерации (единственные места, где крутится noise math):

```
NoiseBasedChunkGenerator.fillFromNoise(Blender,RandomState,StructureManager,ChunkAccess)
  -> CompletableFuture.supplyAsync(lambda$fillFromNoise$11)
  -> ChunkAccess.getSectionIndex loop (y-cells) + NoiseChunk (createNoiseChunk)
  -> NoiseChunk$NoiseInterpolator (slice0/slice1 cell-сэмплы, fillSlice)
  -> DensityFunctions / PerlinNoise.getValue(DDDDDZ)D   <-- ОКТАВНЫЙ ЦИКЛ (тело = цикл)
  -> ImprovedNoise.noise(DDDDD)D -> sampleAndLerp(int,int,int,DDDD) + gradDot
  (BlendedNoise -> PerlinNoise.createLegacyForBlendedNoise для legacy-interpol)
```

javap-факты (классы в booted patched kernel, 2025-12-11 build):
- `NoiseBasedChunkGenerator.fillFromNoise` — единственная точка входа density-заливки;
  вызывает supplyAsync с лямбдой лезущей по секциям (getSectionIndex/getSection/acquire).
- `NoiseChunk$NoiseInterpolator` — интерполятор со слайсами `double[][] slice0/slice1`,
  бэтч-семплинг на клетках уже структурный (ваниль сам бэтчит по клеткам, не по вокселям).
- `PerlinNoise.getValue(DDDDDZ)D` — октавный цикл `noiseLevels[]` (ImprovedNoise[]).
- `ImprovedNoise` — `noise/noiseWithDerivative/sampleAndLerp/gradDot`, поле `byte[] p`.
- ВЫВОД ЦЕНЗА: hot path существует и локализован (fillFromNoise→NoiseChunk→PerlinNoise→
  ImprovedNoise.sample); мост класса «буфер координат → один JNI → готовые плотности»
  технически возможен. НО это ПОДМЕНА горячего тела — она имеет смысл только если
  тело реально греется на фикстуре. Смотри §2.

ВАЖНО (прецедент): перлин-мост уже СУЩЕСТВУЕТ в продукте — `src/perlin_noise.rs` +
`PerlinNoiseNativeOps` (TASK-73/74): whole-body swap октавного цикла, parity 0/20000
bit-exact, wall −12.3% на worldgen-микробенче, ON BY DEFAULT с TASK-148; рядом
`src/improved_noise.rs` (ImprovedNoise hot-patch, CRUSSTY_NATIVE_IMPROVED_NOISE) и
`src/noise_fill.rs` (batch NormalNoiseBatchOps, CRUSSTY_NATIVE_NOISE_FILL). Т.е.
«Rust-крейт noise-плоскости + bulk-API» — это не новая подсистема, а УЖЕ зашитый
продуктовый мост, у которого просто нет работы на этой фикстуре (§2).

## 2. профайл-ценз: GEN-лейн в соак-окне = 0.0% (17 ранов ×3 раунда)

Мандат-порог: <2% = LOW-POTENTIAL. Замеры по cpu-collapsed/BOTTLENECKS_3 абсорб-дир
(эра ×436c/×442b/×444a, та же фикстура MineShield-3 Min, world_sha256 afb3a0b3…):

| ран (run-файл) | total samples | worldgen/noise (kernel) | доля |
|---|---|---|---|
| 444a anchor-9  | 116978 | 23 | 0.0% |
| 444a anchor-11 | —      | 42 | 0.0% |
| 444a chunk4-3  | —      | 22 | 0.0% |
| 444a ins4-3    | —      | 34 | 0.0% |
| 444a ins4d-4   | —      | 21 | 0.0% |
| 444a mega-1/2/3r | —    | 25/22/31 | 0.0% |
| 442b ins4-5r2  | —      | 34 | 0.0% |
| 442b ins4-6r2  | 104704 | 20 | 0.0% |
| 442b ins4d-1r2 | 107350 | 34 | 0.0% |
| 442b ins4d-2   | 103596 | 19 | 0.0% |
| 436c anchor-1  | 115997 | 31 | 0.0% |
| 436c anchor-2  | 116691 | 37 | 0.0% |
| 436c chk3-1/2/3 | —     | 35/23/27 | 0.0% |

Дрейлл моих собственных разборов collapsed (вне bucket-скрипта):
- `PerlinNoise* / ImprovedNoise* / NoiseChunk* / NoiseInterpolator* / fillFromNoise*`
  сэмплов = **0** во всех доступных файлах (ins4-6r2, ins4d-1r2, ins4d-2, 436c-a1/a2,
  436c-chk3-1). Единственные «Noise»-фреймы = `ChunkAccess.getNoiseBiome →
  PalettedContainer.get` (палитрный ЛУК биома по уже загруженному чанку, от
  NaturalSpawner.getRoughBiome / BiomeManager.getBiome) и ОДИН сэмпл
  `NoiseBasedChunkGenerator.getSeaLevel` (константный геттер от tickPrecipitation).
  Это НЕ noise math — это tick-время, пользующееся СГЕНЕРИРОВАННЫМ объектом.
- тик-фазы (BOTTLENECKS_3 anchor-9): entity tick 79.2%, chunk tick 1.7%, mob spawning
  0.1%, random tick 0.3% — GEN-фазы В СОАКЕ НЕТ ВООБЩЕ (нет и фазы «worldgen» в
  phase-сплите, она отсутствует как класс).

## 3. Механика нуля (root-cause, RC1 ×421 подтверждён на текущей эре)

- Фикстура бенча = ПРЕГЕНЕРИРОВАННЫЙ мир MineShield-3 (workflow качает зип мира,
  `run_world3.sh` — forceload 36 команд = 9216 чанков). Forceload чанков с диска =
  PARSE сохранённых чанков (PalettedContainer.read и т.д.), НЕ worldgen:
  `fillFromNoise` не вызывается НИ РАЗУ за соак.
- Boot-фаза (~14-16s до Done) тоже не генерирует: Done наступает на pregen-мире;
  population-инжект (150k, seed 42) — спавн сущностей, не noise.
- TPS-поллы (6 шт, каждые 60s) стартуют ПОСЛЕ boot+forceload+inject → даже если бы
  GEN греал в boot, в метрике (медиана 6 поллов) его нет. RC2 ×421: parse-бурст тоже
  не перекрывается поллами.
- История линии согласуется: wgen-l1/l2/l3 (×420-423) нестабильны из-за RC1/RC4;
  wgen3 ×434-435 = негатив ×2 окна; wgen4 ×436 golden-shot закрыл линию (план-Б ушёл
  в sscan2); NOISEFILL_ROOTCAUSE.md RC1 «worldgen/noise = 42 сэмпла = 0.0% soak-CPU».

## 4. Потолок по лейнам (мандатная математика)

- Лейн = 0.0% (0 сэмплов noise math из ~104-117k; 19-42 «worldgen/noise» = биом-палитра
  + getSeaLevel, не PerlinNoise).
- Потолок дельты TPS от ИДЕАЛЬНОГО (нулевой стоимости) noise-стека = доля лейна =
  **0.0%** (даже минус собственная цена обвязки → ожидаемо ≤0, как wgen3/wgen4).
- Барьер мандата ≥+20% pair-stable: разрыв 0.0% → 20% = бесконечное отношение;
  никакая оптимизация octave-стека (SIMD-бэтчинг, кэш градиентов, сокращение
  бокс-муллирования) не может дать ≥+20% на лейне с нулевой работой.
- Единственный теоретический носитель = НОВАЯ фикстура с генеративным миром
  (реальная загрузка чанков у игроков) — это смена бенч-контракта, не подсистема,
  и на текущем CI-фикстуре она невоспроизводима (мир предгенерится владельцем).

## 5. ВЕРДИКТ: LOW-POTENTIAL (0.00% ≪ 2% порога) — НЕТ имплементации, НЕТ диспатча

- javap-ценз: hot path доказан и задокументирован (§1) — но он МЁРТВ на фикстуре.
- профайл-ценз: 17 ранов ×3 раунда = 0.0% (§2). Порог мандата 2% превышён на два
  порядка в минус.
- Имплементация Rust noise-подсистемы НЕ выполняется (по брифу: «честный
  LOW-POTENTIAL вердикт с числами и НЕТ имплементации, это валидный финиш-этап
  цикла»). Cargo/java-гейты не применимы (код не пишется). Ноги round-445-c-wgen-1/2
  НЕ диспатчатся (диспатч разрешён только при доказанном потенциале).
- Существующие мосты (perlin_noise.rs/improved_noise.rs/noise_fill.rs) остаются как
  есть: они bit-exact, default-on для PerlinNoise, и НЕ являются источником регрессии
  (0.0% лейн = нет и цены). Их трогать = риск без апсайда.

## 6. Следующий виток (меню закона 6, замеры на vanilla-якорях 436c, 116k сэмплов)

| вектор меню | лейн (anchor-1 / anchor-2) | носитель | комментарий |
|---|---|---|---|
| **entity-query слой** (остаток getEntities/AABB/EntitySectionStorage вне sense-плоскости) | **8.31% / 7.80%** | sense-семья живая (cmp438_sense/sensemega, пара +4.2) | ЕДИНСТВЕННЫЙ пункт меню с реальным хедрумом; SubSystem = bulk-query плоскость на SoA-субстрате |
| despawn-сканы расширение | 0.34% / 0.38% (checkDespawn) | sscan2 уже certified | лейн суб-процентный — LOW-POTENTIAL |
| POI | 0.06% / 0.06% (PoiPage/Manager) | — | мёртвый лейн на фикстуре |

Рекомендация следующего витка цикла: entity-query слой (доказанный субстрат sense-семьи,
остаток ~8% = самый крупный не-закрытый не-taken лейн после items 29.7% (агент-B) и
broadphase 15.6% (агент-A)).
