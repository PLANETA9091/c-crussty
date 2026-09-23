# PROFILE-C — TASK-421-C (тик 08:08 +08, база 2d23f45, закон 8 ось chunk/worldgen)

Источники: round-ch420a/b/c RUN-артефакты (@85f74b8 — код носителя, полные
BOTTLENECKS_3 живы), RECON-13b/13f/29/35/36, свежие якоря-421 (@2d23f45 ваниль).

## Лейн chunk/worldgen на носителе (CPU-окно soak, ch420b BOTTLENECKS_3, 116,621 сэмпл)

| лейн/фрейм | сэмплы | доля | статус |
|---|---|---|---|
| chunk system (kernel) | 9,819 | **8.4%** | ось-цель |
| — PalettedContainer.get | 4,533 | 3.9% | REFUTED-зона (RECON-35: потребители fluid/collision/inside, ванильная семантика) |
| — SimpleBitStorage.get | 1,520 | 1.3% | там же |
| — readPalette | 1,096 | 0.9% | там же |
| — LevelChunk.getFluidState | 1,086 | 0.9% | fluid-плоскость (закрыта ×4) |
| — LevelChunk.getBlockStateFinal | 930 | 0.8% | там же |
| — ServerChunkCache.getChunkNow | 799 | 0.7% | микро |
| phase: chunk tick | 2,068 | 1.8% | микро-зона |
| phase: chunk system (off-main) | 1,357 | 1.2% | микро |
| moonrise/paper patches | 10,107 | 8.7% | broadphase-семья (colpush уже в носителе) |
| network (kernel) | 3,256 | 2.8% | трекер+отправка ~2.0-2.2% (RECON-36) |
| — ServerEntity.sendChanges | 826 | 0.7% | трекер |
| — ChunkMap$TrackedEntity.moonrise$tick | 612 | 0.5% | трекер |
| **worldgen/noise (kernel)** | **42** | **0.0%** | **GEN-ось инертна на прегенерированной фикстуре** |

## GEN-ось / parse-ось (вне soak-CPU)

- parse-бурст: 16,384 миссов за ~2-3 сек на forceload (boot), RECON-13b: 33.38%
  alloc-байт burst-окна / 8.70% steady; codec 19.06% + paletted 13.98% (burst).
- hit-rate parse-кэша: 63/64/64% (ch420a/b/c), evicted=0 @cap 16384 — структурное плато.
- noise-fill: патчи ×3 целей (Noise/ShiftNoise/NoiseInterpolator) ARMED, soak-работы 0.
- TPS-поллы ch420b: [17.8 boot, 1.7, 2.0, 2.3, 2.6, 2.7] — рампа = выгорание 103k items.
- MSPT avg 397.55ms; boot Done 15.61s; population inject 56,344ms; 9216 forceloaded.

## Якоря-421 (ваниль @2d23f45)

| run | ветка | статус |
|---|---|---|
| 35800920567 | round-421-anchora | in flight (диспатч 00:11Z) |
| 35800926548 | round-421-anchorb | in flight |
| 35800931828 | round-421-anchorc | in flight |

Банк-ожидание ~2.75/2.20/2.10 (якоря-420 @41456af); парность по runner_cpu_index,
band 6.0-9.5M. Свежие числа якорей будут дописаны по завершении (см. ниже).

## Вердикт профиля для вектора C

Soak-представление оси chunk: block-read семья 7.5% (REFUTED, RECON-35) + chunk tick
1.8% + off-main 1.2% + трекер/отправка 2.0-2.2%. Потолок soak-среза оси ≈ +8-12% TPS —
меньше мандата. Единственный механизм, которым ось давала стабильные +6-8пп
(кросс-чек cp420 vs mg-ноги) — GC-debt relief от parse-кэша в population/ранние поллы.
→ Вектор: стабилизация гейтов (cmp421_chunk STRICT-OR ×3 сайта) + расширение
GC-debt relief (biomes-сайт кэша) + единственный soak-живой срез оси (chunk-send
serialization, revision-keyed) — см. NOISEFILL_ROOTCAUSE.md §4.
