# RECON-13 — декомпозиция entity-аллокаций (/home/z/c-crussty/research/gc-recon-2026-09-19/run-s7165-recon-diag/alloc-collapsed.txt)

- всего аллок-семплов: 30,516; entity-семья (глубочайший фрейм): 10,748 = 35.22%

| под-лейн | семплов | % всего | % семьи |
|---|---|---|---|
| entity-tick-core | 6,527 | 21.39% | 60.7% |
| navigation/pathfinding | 2,008 | 6.58% | 18.7% |
| entity-lambda | 1,824 | 5.98% | 17.0% |
| collision-context | 184 | 0.60% | 1.7% |
| goals/behavior-selector | 105 | 0.34% | 1.0% |
| brain/sensing/behavior | 65 | 0.21% | 0.6% |
| look/attack-control | 18 | 0.06% | 0.2% |
| spawn | 14 | 0.05% | 0.1% |
| entity-data/sync | 3 | 0.01% | 0.0% |

| топ-25 листьев аллокации | % всего |
|---|---|
| short[]_[k] | 10.30% |
| byte[]_[i] | 7.33% |
| net.minecraft.world.phys.Vec3_[i] | 7.29% |
| com.mojang.serialization.DataResult$Success_[i] | 7.17% |
| net.minecraft.world.phys.AABB_[i] | 6.74% |
| java.lang.Object[]_[i] | 5.31% |
| byte[]_[k] | 4.34% |
| net.minecraft.core.BlockPos_[i] | 4.34% |
| java.lang.String_[i] | 3.83% |
| short[]_[i] | 3.40% |
| long[]_[k] | 2.70% |
| java.lang.Object[]_[k] | 2.35% |
| com.mojang.datafixers.util.Pair_[i] | 2.16% |
| it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap_[i] | 1.74% |
| java.util.Optional_[i] | 1.67% |
| net.minecraft.core.BlockPos$MutableBlockPos_[i] | 1.63% |
| long[]_[i] | 1.60% |
| char[]_[k] | 1.58% |
| java.util.ArrayList_[i] | 1.21% |
| net.minecraft.core.BlockPos$6_[i] | 1.16% |
| it.unimi.dsi.fastutil.objects.Object2ObjectArrayMap_[i] | 0.91% |
| ca.spottedleaf.dataconverter.types.nbt.NBTMapType_[i] | 0.89% |
| net.minecraft.resources.ResourceLocation_[i] | 0.78% |
| it.unimi.dsi.fastutil.objects.Object2ObjectOpenHashMap$FastEntryIterator_[i] | 0.70% |
| java.util.ImmutableCollections$ListItr_[i] | 0.59% |

## Приложение (тик 16:08): кодек-сериализация чанков — новый аллок-гигант вне entity-семьи
- DataResult-связанные стэки (MapDecoder/codec-механика): 5,923 семпла = **19.4% всех аллок-байтов**; видимый серверный вызыватель: **SerializableChunkData.parse** (парсинг/загрузка данных чанков) + MapDecoder.compressedDecode (84.6% глубины) — NBT→объект churn (short[]/byte[]/String/Pair листья = те самые 10.3+7.3+3.8+2.2% в топ-листьях)
- Под-лейны entity-семьи (аттрибуция глубочайшим фреймом, 10G-база s7165): **entity-tick-core 21.39%** (внутри = travel-physics Vec3/AABB, уже известная физика движения), **navigation/pathfinding 6.58% = ВАЛИДНЫЙ лейн ≥5%** (Node/Path/NodeEvaluator аллокации — кандидаты атак: node-pool уже PAPER-REFUTED для card-set, НО для аллок-давления пул переоткрывается с новым критерием: young-аллокации → young-GC частота, карты не трогаем), entity-lambda 5.98%, collision-context 0.60%, goals 0.34%, brain 0.21%
- Следующий RECON-шаг (тип+1): глубже в SerializableChunkData.parse — ЧТО вызывает parse в установившемся тике (периодическая перезагрузка чанков? region-cache eviction? flush_diet side-effect?) и attribution window (S7-134: alloc-профиль = окно 80-100% рана)
