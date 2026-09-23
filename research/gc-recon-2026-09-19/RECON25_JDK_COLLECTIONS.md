# RECON-25 — дрилл jdk-collections/serde до домен-клиентов (>=5%)

## s7189: run-s7189-traveldiet-v2a

Alloc-ось: 9491 сэмплов; бакет jdk-collections/serde: **2329 = 24.5% alloc-оси** (RECON-24 дал 25.5% на s7189).

### ЧТО (leaf-типы бакета, топ-8; порог 5% ОСИ = 475 сэмплов)

| тип | сэмплы | % оси | % бакета | >=5% оси |
|---|---|---|---|---|
| long[] | 401 | 4.2% | 17.2% |  |
| java/lang/Object[] | 194 | 2.0% | 8.3% |  |
| java/util/ArrayList | 172 | 1.8% | 7.4% |  |
| com/google/common/collect/Iterators$ArrayItr | 159 | 1.7% | 6.8% |  |
| 0x00007eff6d9ed260 | 117 | 1.2% | 5.0% |  |
| 0x00007eff6d9ff7c8 | 70 | 0.7% | 3.0% |  |
| int[] | 68 | 0.7% | 2.9% |  |
| byte[] | 68 | 0.7% | 2.9% |  |

### ДЛЯ ЧЕГО (домен-клиент, deepest-client-wins)

| домен | сэмплы | % оси | % бакета | >=5% оси |
|---|---|---|---|---|
| inside-scan-visitset | 684 | 7.2% | 29.4% | ДА |
| pathfinding/ai | 218 | 2.3% | 9.4% |  |
| other:iterator | 131 | 1.4% | 5.6% |  |
| other:<init> | 97 | 1.0% | 4.2% |  |
| other:getInstance | 90 | 0.9% | 3.9% |  |
| other:moonrise$getHardCollidingEntities | 79 | 0.8% | 3.4% |  |
| other:lambda$tick$4 | 66 | 0.7% | 2.8% |  |
| other:getEntities | 65 | 0.7% | 2.8% |  |
| spark/jmx-monitor | 64 | 0.7% | 2.7% |  |
| other:tags | 62 | 0.7% | 2.7% |  |
| other:findEligibleGroups | 51 | 0.5% | 2.2% |  |
| chunk-serde/persistence | 48 | 0.5% | 2.1% |  |

### Ключевые call-sites топ-доменов (лист-1 фрейм, топ-3)

- **inside-scan-visitset** (7.2% оси): `it/unimi/dsi/fastutil/longs/LongOpenHashSet.<init>` 393; `java/lang/invoke/DirectMethodHandle.allocateInstance` 117; `net/minecraft/world/level/BlockGetter.forEachBlockIntersectedBetween` 61
- **pathfinding/ai** (2.3% оси): `java/lang/invoke/DirectMethodHandle.allocateInstance` 75; `java/util/EnumMap$EntrySet.iterator` 33; `java/util/EnumSet.noneOf` 25
- **other:iterator** (1.4% оси): `com/google/common/collect/Iterators.forArrayWithPosition` 131
- **other:<init>** (1.0% оси): `java/util/IdentityHashMap.init` 34; `java/util/Optional.of` 27; `net/minecraft/util/context/ContextMap$Builder.<init>` 11

## s7194: run-s7194-zeroalloc-v1

Alloc-ось: 8726 сэмплов; бакет jdk-collections/serde: **2658 = 30.5% alloc-оси** (RECON-24 дал 25.5% на s7189).

### ЧТО (leaf-типы бакета, топ-8; порог 5% ОСИ = 436 сэмплов)

| тип | сэмплы | % оси | % бакета | >=5% оси |
|---|---|---|---|---|
| long[] | 438 | 5.0% | 16.5% | ДА |
| java/util/ArrayList | 305 | 3.5% | 11.5% |  |
| java/lang/Object[] | 231 | 2.6% | 8.7% |  |
| com/google/common/collect/Iterators$ArrayItr | 154 | 1.8% | 5.8% |  |
| 0x00007f39d59d8870 | 122 | 1.4% | 4.6% |  |
| 0x00007f39d59e7188 | 75 | 0.9% | 2.8% |  |
| int[] | 73 | 0.8% | 2.7% |  |
| it/unimi/dsi/fastutil/longs/LongOpenHashSet | 73 | 0.8% | 2.7% |  |

### ДЛЯ ЧЕГО (домен-клиент, deepest-client-wins)

| домен | сэмплы | % оси | % бакета | >=5% оси |
|---|---|---|---|---|
| inside-scan-visitset | 723 | 8.3% | 27.2% | ДА |
| pathfinding/ai | 217 | 2.5% | 8.2% |  |
| other:collide | 185 | 2.1% | 7.0% |  |
| other:iterator | 129 | 1.5% | 4.9% |  |
| other:<init> | 86 | 1.0% | 3.2% |  |
| other:tags | 83 | 1.0% | 3.1% |  |
| spark/jmx-monitor | 72 | 0.8% | 2.7% |  |
| other:lambda$tick$4 | 72 | 0.8% | 2.7% |  |
| other:getInstance | 72 | 0.8% | 2.7% |  |
| other:moonrise$getHardCollidingEntities | 68 | 0.8% | 2.6% |  |
| other:getEntities | 62 | 0.7% | 2.3% |  |
| other:betweenClosed | 56 | 0.6% | 2.1% |  |

### Ключевые call-sites топ-доменов (лист-1 фрейм, топ-3)

- **inside-scan-visitset** (8.3% оси): `it/unimi/dsi/fastutil/longs/LongOpenHashSet.<init>` 421; `java/lang/invoke/DirectMethodHandle.allocateInstance` 122; `net/minecraft/world/level/BlockGetter.forEachBlockIntersectedBetween` 73
- **pathfinding/ai** (2.5% оси): `java/lang/invoke/DirectMethodHandle.allocateInstance` 77; `java/util/EnumMap$EntrySet.iterator` 33; `java/util/EnumSet.noneOf` 30
- **other:collide** (2.1% оси): `net/minecraft/world/entity/Entity.collide` 141; `java/util/Arrays.copyOf` 44
- **other:iterator** (1.5% оси): `com/google/common/collect/Iterators.forArrayWithPosition` 129

## Кросс-раннер стабильность

- бакет jdk-collections/serde: s7189 24.5% vs s7194 30.5% alloc-оси

| домен | s7189 % оси | s7194 % оси | стабилен |
|---|---|---|---|
| inside-scan-visitset | 7.2% | 8.3% | ДА |
| pathfinding/ai | 2.3% | 2.5% | ДА |
| other:collide | 0.4% | 2.1% | ДА |
| other:iterator | 1.4% | 1.5% | ДА |
| other:<init> | 1.0% | 1.0% | ДА |
| other:tags | 0.7% | 1.0% | ДА |
| other:getInstance | 0.9% | 0.8% | ДА |
| other:moonrise$getHardCollidingEntities | 0.8% | 0.8% | ДА |
| other:lambda$tick$4 | 0.7% | 0.8% | ДА |
| spark/jmx-monitor | 0.7% | 0.8% | ДА |
| other:getEntities | 0.7% | 0.7% | ДА |
| other:betweenClosed | 0.0% | 0.6% | ДА |

## ВЫВОДЫ (прегистер следующего рычага)

1. Крупнейший домен-клиент >=5% alloc-оси = следующий рычаг ТОП-1 GC-семейства
   (архитектурный: кэш/мемоизация/батчинг/O(n)->O(1); структурные аллокации
   только — урок C2 scalar-replacement лейна #14).
2. spark/jmx-мониторинг в доменах — НЕ игровой код: при выборе рычага
   вычитается из рассмотрения (не парити-релевантен).
3. Кросс-раннер стабильность доменов обязательна (пул раннеров 1.9x
   дневная дисперсия, GOAL x30-ADD/x31).

