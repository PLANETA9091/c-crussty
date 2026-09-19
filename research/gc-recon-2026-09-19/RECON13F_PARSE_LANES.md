# RECON-13F — декомпозиция ТОП-1 chunk-parse лейна на под-лейны — 2026-09-19 ~18:3x +08 (TASK-329, офлайн при летящем s7169)

Инструмент: `scripts/bench4_recon/recon13f_parse_diet.py` (лейн-фильтр ChunkDataLoadTask/GenericDataLoadTask/SerializableChunkData.parse/ChunkSerializer; атрибуция глубочайшим фреймом право-на-лево; кросс-ран). Источники: alloc-collapsed.txt s7168 (свежий, банк v3 + parse_diag) и s7165 (диаг-база).

## Главный факт: доля лейна ЗАВИСИТ ОТ ОКНА
- s7165 (окно захватило chunk-load burst): parse-лейн = **45.04%** total; 13,745/30,516.
- s7168 (окно спокойное): parse-лейн = **8.70%** total; 944/10,850.
- Вывод: лейн реален (>=5% в ОБОИХ окнах), но его вес = периодика burst'а перезагрузки чанков. Периодику решает census s7169 (в полёте): repeat_share — сколько из parse'ов это ПОВТОРНАЯ декодировка того же чанка.

## Под-лейны (доля от total аллок-байтов)
| под-лейн | s7165 (burst) | s7168 (спокойн.) | глубочайшие фреймы |
|---|---|---|---|
| **codec-машинерия** (DataResult/MapDecoder/StringConcat/ResourceLocation.read) | **19.06%** | 3.43% | DataResult$Success.success ← ResourceLocation.read / NbtOps.getMap / MapDecoder.compressedDecode; StringConcatHelper.newArray (Unsafe.allocateUninitializedArray); StringLatin1.substring |
| **paletted-decode** (ShortArrayList.grow — распаковка палитровых контейнеров) | **13.98%** | 3.26% | ShortArrays.forceCapacity ← ShortArrayList.grow ← ShortArrayList.add ← SimpleBitStorage… (1,728+786+316 сэмплов — ОДИН и тот же chain роста) |
| **other-parse** (String-churn + dataconverter gson) | **7.62%** | 1.52% | StringConcatHelper; StringLatin1.newString←substring; gson JsonReader ← dataconverter V4290 (WalkerUtils.convertList) |
| chunk-struct (ShortList/LevelChunkSection.recalcBlockCounts) | 2.55% | 0.30% | Short2ShortOpenHashMap.<init>/rehash ← ShortList ← LevelChunkSection |
| nbt-io (NbtOps.getMap, CompoundTag) | 1.82% | 0.18% | NbtOps$1 ← NbtOps.getMap ← MapDecoder.compressedDecode |
| io-stream | 0.01% | 0.00% | — |

## Ветвление рычага (прегенерировано, «ТОП-1 ОБЯЗАН УПАСТЬ»)
1. **Census repeat_share >= 30%** → GO lever #12 **decode-cache по ревизии**: кэш декодированной структуры SerializableChunkData ключом (chunkKey, deep-hash тега) + copy-on-reuse глубоким клоном (бит-точность: на хите кодек-машинерия не вызывается вовсе; клон = array-clone уровень, убивает codec 19.06% + nbt-io + большую часть other-parse на повторах). Точка внедрения: call-site SerializableChunkData.parse в GenericDataLoadTask/ChunkDataLoadTask (retarget-машинерия TASK-327).
2. **repeat_share < 10%** → cache REFUTED, лейн НЕ ЗАКРЫТ → крупнейший под-лейн = **codec-машинерия 19.06%** → рычаг L-A **schema-specialized decoder**: бит-точный рукописный декодер ChunkData-схемы вместо MapDecoder.compressedDecode (убирает DataResult/Pair/String-churn полностью). Резерв: L-B **paletted-diet** (pre-sized распаковка вместо ShortArrayList.grow — 13.98%, узко и легко верифицируется) — ТОЛЬКО после L-A/L12.
3. **10–30%** → расширенный RECON периодики (census per-phase из s7169: span/30s-бакеты повторов).

## NEXT
- absorb s7169 (run 35437243128) готовым absorb_s7168.py → вердикт ветки 1/2/3.
- Ветке 2 требуется свежий RECON вызывателей codec-машинерии (какие поля схемы дают ResourceLocation.read/getMap-доминанту) — recon13f уже даёт топ-стэки для прицеливания.
