# RECON-24 — аллок-атрибуция GC-семейства (свежий ТОП-1)

Run: `run-s7189-traveldiet-v2a` (банк v3 + travel_diet=1, свежая валидная нога).
Alloc-окно ЖИВОЕ: **13144 сэмплов**, листов-типов 268 (AP-PID дефект на cpu-окно не влияет).

## ЧТО аллокируется (leaf-типы, топ-12)

| тип | сэмплы | % alloc-оси |
|---|---|---|
| net/minecraft/world/phys/Vec3 | 1919 | 14.6% |
| net/minecraft/world/phys/AABB | 1900 | 14.5% |
| byte[] | 1475 | 11.2% |
| java/lang/Object[] | 824 | 6.3% |
| java/lang/String | 549 | 4.2% |
| char[] | 535 | 4.1% |
| net/minecraft/core/BlockPos$MutableBlockPos | 506 | 3.8% |
| net/minecraft/core/BlockPos | 480 | 3.7% |
| long[] | 463 | 3.5% |
| short[] | 331 | 2.5% |
| net/minecraft/core/BlockPos$6 | 320 | 2.4% |
| com/mojang/serialization/DataResult$Success | 230 | 1.7% |

## КТО аллокирует (deepest-owner, бакеты взаимоисключающие)

| подсистема | сэмплы | % alloc-оси | >=5% |
|---|---|---|---|
| jdk-collections/serde | 3348 | 25.5% | ДА |
| inside-blocks/fluid-scan | 2934 | 22.3% | ДА |
| jvm-core | 2829 | 21.5% | ДА |
| travel/movement | 1452 | 11.0% | ДА |
| chunk-system | 843 | 6.4% | ДА |
| kernel-infra/ops | 783 | 6.0% | ДА |
| pathfinding/ai | 393 | 3.0% |  |
| moonrise/paper | 254 | 1.9% |  |
| block/random-tick | 242 | 1.8% |  |
| tracking/sync | 45 | 0.3% |  |
| entity-mgmt/spawn | 15 | 0.1% |  |
| other | 6 | 0.0% |  |

## Топ-3 подсистемы: их доминирующие типы

- **jdk-collections/serde** (25.5%): byte[] 13%, long[] 12%, short[] 10%, it/unimi/dsi/fastutil/objects/Object2ObjectOpenHashMap 6%
- **inside-blocks/fluid-scan** (22.3%): net/minecraft/world/phys/Vec3 41%, net/minecraft/world/phys/AABB 34%, net/minecraft/core/BlockPos$MutableBlockPos 11%, net/minecraft/core/BlockPos$6 11%
- **jvm-core** (21.5%): byte[] 32%, java/lang/Object[] 29%, java/lang/String 19%, char[] 16%

## GC-бюджет (gc.log того же лега)

- пауз: 316 (young 316 / full 0), сумма 19559 ms = 6.5% 300s-окна TPS

## ВЫВОДЫ (прегистер выбора рычага после финала лейна #14)

1. Под-лейны ≥5% alloc-оси: jdk-collections/serde 25.5%, inside-blocks/fluid-scan 22.3%, jvm-core 21.5% — каждый кандидат на архитектурный рычаг (кэш/батчинг/O(n)->O(1)).
2. Связка осей: cutting под-лейна X% alloc-оси режет пропорционально young-evacuation и oop-barriers (12.2% scene-CPU) — двойной эффект на TPS.
3. Парити-ограничение: любые scalar-replacement-манёвры обязаны учитывать урок лейна #14 (C2 сам замещает темпорари — аткаться только на СТРУКТУРНЫЕ аллокации: контейнеры/строки/сериализацию, не на скалярные Vec3/AABB продукты).
