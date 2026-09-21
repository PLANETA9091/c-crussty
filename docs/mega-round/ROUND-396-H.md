# ROUND-396-H — vector `items_oss`: порт Lithium item_entity_merging в crussty-bridge (1.21.10)

Task ID: TASK-396-H · Agent: mega-round-agent-h · Флаг: `CRUSSTY_LEVER_FLAG=items_oss` (`CRUSSTY_LEVER_ARG=1`)

## ТОП-1 ботлнек (базлайн-карта TASK-394, банк v4, fp=4, seed 42, 150k pop)

- `items/ItemEntity.tick` 31.17% java (36051/115655); `broadphase getEntities` 15.66%;
  `fastutil` 8.54%; `java_util` 7.01%. ~70% популяции — item-сущности; каждый item КАЖДЫЙ тик
  `mergeWithNeighbours()` → `Level.getEntitiesOfClass(ItemEntity, AABB.inflate(...), pred)`.
- javap ядра (`mergeWithNeighbours()V`, purpur-1.21.10): полный скан секций вокруг бокса +
  материализация `ArrayList` кандидатов + цикл `tryToMerge` — каждый тик, для каждого item,
  без какого-либо abort/limit. Paper-патчи merge-радиуса (`spigotConfig.itemMerge`,
  `onlyMergeItemsHorizontally`, `fixItemsMergingThroughWalls`) УЖЕ в ядре — upstream Paper
  дальнейшей оптимизации запроса не имеет (mergeWithNeighbours в Paper main == эта же форма).

## ВЫБРАННЫЙ АПСТРИМ-ПАТЧ: Lithium `item_entity_merging` (experimental mixin)

Репозиторий: CaffeineMC/lithium (canonical slug после редиректа repositories/224949995), ветка `develop`.

1. **ItemEntityMixin (the redirect)** — <https://raw.githubusercontent.com/CaffeineMC/lithium/develop/common/src/main/java/net/caffeinemc/mods/lithium/mixin/experimental/entity/item_entity_merging/ItemEntityMixin.java>
   `@Redirect(method = "mergeWithNeighbours()V", at = @At(value="INVOKE", target="Lnet/minecraft/world/level/Level;getEntitiesOfClass(...)Ljava/util/List;"))`
   → `consumeItemEntitiesForMerge(...)`: вместо generic-запроса — прямая итерация секций
   (`forEachAccessibleNonEmptySection`) с per-type списками секций и **abortable lazy-консумером**.
2. **ItemEntityLazyIterationConsumer** — <https://raw.githubusercontent.com/CaffeineMC/lithium/develop/common/src/main/java/net/caffeinemc/mods/lithium/common/entity/item/ItemEntityLazyIterationConsumer.java>
   Сухой прогон (dry-run) слияний без их применения: `predictReceivedItemCount` (точная математика
   переноса `min(min(max,64)-target, source)`), накопление `adjustedStackCount` и
   **`Continuation.ABORT` как только стек полон** (`adjustedStackCount >= stack.getMaxStackSize()`).
3. **Конфиг-описание (package-info.java)** — <https://raw.githubusercontent.com/CaffeineMC/lithium/develop/common/src/main/java/net/caffeinemc/mods/lithium/mixin/experimental/entity/item_entity_merging/package-info.java>
   «Optimize item entity merging by categorizing item entities by item type and only attempting to
   merge with the same type. Categorizing by stack size allows skipping merge attempts of full item
   entities or two more than half full item entities.»
4. **ItemEntityList (категоризация по типу+компонентам, UPGRADE_THRESHOLD=10)** —
   <https://raw.githubusercontent.com/CaffeineMC/lithium/develop/common/src/main/java/net/caffeinemc/mods/lithium/common/entity/item/ItemEntityList.java>

### Что портировано (1:1 механика, адаптер под Moonrise)

| Lithium (Fabric mixin) | crussty-порт (bridge, runtime byte-patch) |
|---|---|
| `@Redirect` вызова `getEntitiesOfClass` в `mergeWithNeighbours()V` | whole-body retarget `ItemEntity.mergeWithNeighbours()V` → `ItemMergeOps.mergeWithNeighbours(ItemEntity)` (cplug_sdk ReplaceBody, паттерн fluid_guard) |
| итерация секций `forEachAccessibleNonEmptySection` + per-type raw-списки | Moonrise `EntityLookup.getEntities(Class, null, AABB, list, null, LIMIT)` — **тот же** region-walk (±2 блока, тот же порядок), per-class индекс `entitiesByClass`, **limited** сбор (правда `getEntitiesLimited`: null-pred safe, `list.size()>=limit → true` → walk EARLY-RETURN — bytecode-верифицировано) |
| lazy-консумер + `predictReceivedItemCount` + early ABORT | тот же dry-run в `ItemMergeOps.query()`: предикат (`other != this && isMergable`) → wall-clip (если конфиг) → добавление → предсказание переноса → `ABORT` при `adjusted <= 0 || adjusted >= maxStackSize` |
| vanilla-остаток цикла (isMergable-recheck, clip, tryToMerge, events) | сохранён байт-в-байт: приватные `isMergable`/`tryToMerge` вызываются через кэшированные `MethodHandle` (`privateLookupIn` — same unnamed module); цикл = транскрипт javap |

### Паритет и отклонения (все — как в апстриме)

- **Порядок кандидатов**: prefix точного vanilla-порядка (тот же Moonrise region-walk; limited-сбор
  — префикс того же `EntityCollectionBySection`-обхода).
- **DOC-DEV #1 (апстрим-цитата)**: early-abort при `adjusted >= maxStackSize` — у Lithium
  идентичное условие (`if (this.adjustedStackCount <= 0 || this.adjustedStackCount >= this.stack.getMaxStackSize()) return Continuation.ABORT;`). Для кастомных max-stack > 64 (моды) реверс-слияния
  за порогом пропускаются — точно как в апстриме (экспериментальный mixin, выключен по умолчанию
  у Lithium; у нас — за флагом `items_oss`).
- **DOC-DEV #2 (адаптер)**: `QUERY_LIMIT=64` на весь walk; если после лимита стек НЕ полон →
  консервативный fallback в точный vanilla-запрос `level.getEntitiesOfClass` (паритет по
  построению; триггерится только в смешанных кучах >64 пересекающихся item'ов).
- Предикат/клип/tryToMerge/события — vanilla без изменений (MethodHandle → приватные методы).

## Потолок ≥ +10% (обоснование)

- Лейны-экспозиция (базлайн): merge-запрос и его инра = broadphase 15.66% + fastutil 8.54% +
  java_util 7.01% ≈ **31.2% java** + merge-доля внутри items 31.17%.
- Механика убирает: полную материализацию списка на каждый item-тик, generic type-test,
  скан хвоста после заполнения стека (ранний abort), повторный `tryToMerge`-цикл по всему списку.
- Консервативная конверсия 30–50% от 31.2% → **+9…+15% TPS**; цель амбиции ≥+15% при
  homogeneous-кучах (bench: однотипные дропы → abort после первого слайса).
- Апстрим-обоснование класса механики: Lithium — эталонный entity-lookup-оптимизатор (FPS/TPS-фокус),
  DAB-подобные tick-диеты не требуются: поведение merging НЕ меняется (кроме цитированных DOC-DEV).

## Отклонённые кандидаты (проверены, пруфы)

- **Pufferfish DAB** (`patches/server/0016-Dynamic-Activation-of-Brain.patch`, ver/1.20 —
  <https://github.com/pufferfish-gg/Pufferfish/blob/ver/1.20/patches/server/0016-Dynamic-Activation-of-Brain.patch>):
  «Dynamic Activation of **Brain**» — дебафф tick-rate МОЗГОВ мобов; ItemEntity не покрывает
  (items без Brain) + менял бы частоту тиков. REJECTED.
- **Leaves** (LeavesMC/Leaves master, paper-patches/features полный листинг): item-merge/tick
  оптимизационных патчей нет (0016-Old-ender-dragon…, protocol/fakeplayer и пр.). REJECTED.
- **Pufferfish ver/1.21**: только 3 feature-патча (branding/mob-spawning/shapeless) — items нет.
- **Paper**: mergeWithNeighbours в Paper main == форма ядра (изменений нет — цитата javap ядра).
