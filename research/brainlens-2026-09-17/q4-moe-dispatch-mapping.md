### **ЗАДАЧА 4 — ПЕРЕНОС ПАТТЕРНОВ DeepSeek-инференса на mob-AI диспатч**

---

### **1. Анализ оригинала (Brain.tick кластер)**
- **Состав**: `Mob.serverAiStep` (5.21%) → `Brain.tick` (6.85%) → `startEachNonRunningBehavior` (4.66%) + `tickEachRunningBehavior` (1.41%) + `tickSensors` (0.76%).
- **Ключевые узкие места**:
  - **LinkedHashMap-итерация**: `startEachNonRunningBehavior` сканирует все behaviors (O(n)), order = insertion order (parity-critical).
  - **Мегаморфизм**: `itable stub` (0.49%) — динамический диспетчинг сенсоров/поведений.
  - **Аллокации**: 11.3% всего tick-alloc в кластере (вероятно, `HashMap` для behaviors/sensors).

---

### **2. Изоморфизмы и их реализация**

#### **(1) MoE fine-grained routing: Precomputed Routing Table**
- **Оригинал**: `startEachNonRunningBehavior` итерирует `LinkedHashMap` behaviors, проверяет `canStart()` для каждого.
- **Изоморфизм**: 
  - **Precomputed routing table**: Снапшот behaviors (ID → `canStart`-статус) один раз per-tick на мир, а не per-mob.
  - **Батч-диспатч**: Мобы обрабатываются группами по общему набору активных behaviors.
- **Rust-рычаг**:
  - Снапшот: `Vec<(BehaviorId, bool)>` для каждого мира, обновляется в `Brain` при изменении behaviors.
  - Диспатч: Параллельный итератор по мобам с фильтрацией по снапшоту.
- **Прогноз MSPT**:
  - Оригинал: `4.66%` из `80.9ms` = **3.77ms**.
  - Оптимизация: Убрать O(n) LinkedHashMap-итерацию → **~2.0ms** (**47%↓**).
- **Parity-риск**: Низкий. Порядок старта behaviors = порядок в снапшоте (копия LinkedHashMap).
- **Kill-criteria**: Изменение `canStart` без обновления снапшота → устаревшая таблица.

#### **(2) Shared Expert: Global Memory-Check Pass**
- **Оригинал**: Каждый behavior/sensor выполняет `memory.check()` (аллокация/доступ к `Long2ObjectHashMap`).
- **Изоморфизм**:
  - **Shared pass**: Глобальный скан всех мобов для обновления memory-состояния один раз per-tick.
  - **Batched memory updates**: Накопить изменения в Rust, синхронизировать с JVM раз per-tick.
- **Rust-рычаг**:
  - `HashMap<WorldId, Vec<(MobId, MemoryUpdate)>>` для накопления изменений.
  - JNI-вызов `Brain.updateMemoryBatch()` с массовым обновлением.
- **Прогноз MSPT**:
  - Оригинал: `Long2ObjectOpenHashMap.find` (0.10%) + аллокации (11.3%) → **~9.5ms**.
  - Оптимизация: Убрать перебор мобов → **~7.0ms** (**26%↓**).
- **Parity-риск**: Средний. Side effects от `memory.check()` могут нарушаться при батчинге.
- **Kill-criteria**: Несовпадение state после батч-апдейта (e.g., кэш-промахи).

#### **(3) MTP Speculative: Speculative Start with Rollback**
- **Оригинал**: `canStart()` синхронный, с side effects (e.g., consume item).
- **Изоморфизм**:
  - **Speculative start**: Выполнить `canStart()` без side effects, если верно — зафиксировать, иначе откат.
  - **Rollback**: Хранить состояние до проверки, восстанавливать при failure.
- **Rust-рычаг**:
  - Копировать state (e.g., `inventory ItemStack`) в Rust до проверки.
  - JNI-вызов `canStartSpeculative()`, при успехе — `commit()`, иначе `rollback()`.
- **Прогноз MSPT**:
  - Оригинал: `startEachNonRunningBehavior` (4.66%) → **3.77ms**.
  - Оптимизация: Убрать 50% false-start checks → **~2.8ms** (**26%↓**).
- **Parity-риск**: Высокий. Side effects (e.g., particle effects) могут не откатиться.
- **Kill-criteria**: Несовпадение state после rollback (e.g., предмет не вернулся).

#### **(4) Paged/Block-Wise State: Behavior Bitmap**
- **Оригинал**: `Brain.behaviors` — `LinkedHashMap<Behavior, Status>`.
- **Изоморфизм**:
  - **Bitmap status**: Битовая карта для статусов behaviors (e.g., 1 байт per behavior).
  - **Paged access**: Группы behaviors по 64 (long), быстрые битовые операции.
- **Rust-рычаг**:
  - `Vec<u64>` для статусов, индекс = behavior ID.
  - `startEachNonRunningBehavior` → итерация по bitmap (бит & 1 = running).
- **Прогноз MSPT**:
  - Оригинал: `LinkedHashMap.sequencedKeySet` (0.11%) + `HashMap.getNode` (0.23%) → **0.27ms**.
  - Оптимизация: Убрать HashMap → **~0.15ms** (**44%↓**).
- **Parity-риск**: Низкий. Порядок behaviors = порядок в bitmap (фиксированный).
- **Kill-criteria**: Изменение размера behaviors → bitmap реаллокация.

#### **(5) DualPipe Overlap: AI-Chunk Phase Overlap**
- **Оригинал**: `Mob.serverAiStep` → `Brain.tick` → chunk system (9.8%).
- **Изоморфизм**:
  - **Overlap**: Запуск `tickSensors` параллельно с chunk GC (Paper тик-луп).
  - **Dependency**: `tickSensors` зависит от chunk data → синхронизация после.
- **Rust-рычаг**:
  - Разделить `Brain.tick` на `tickSensors` (Rust) + `tickBehaviors` (JVM).
  - `tickSensors` выполняется в pre-chunk-phase, `tickBehaviors` — post.
- **Прогноз MSPT**:
  - Оригинал: `tickSensors` (0.76%) + `chunk system` (9.8%) → **8.5ms**.
  - Оптимизация: Параллелизм → **~6.0ms** (**29%↓**).
- **Parity-риск**: Высокий. Несинхронизированный доступ к chunk data → corruption.
- **Kill-criteria**: Desync chunk read/write (e.g., блоки меняются тиком).

---

### **3. Ранжирование по эффективности и безопасности**
| Изоморфизм               | Прогноз MSPT↓ | Parity-риск | Kill-criteria | Рекомендация |
|--------------------------|---------------|-------------|--------------|--------------|
| (1) Precomputed Routing   | 47%↓ (3.77→2.0) | Низкий      | Снапшот устарел | **P0** (высокий приоритет) |
| (4) Paged State Bitmap    | 44%↓ (0.27→0.15) | Низкий      | Реаллокация    | **P1** (низкий риск) |
| (2) Shared Expert         | 26%↓ (9.5→7.0)  | Средний     | Side effects   | **P2** (требует тестов) |
| (3) MTP Speculative       | 26%↓ (3.77→2.8) | Высокий     | Rollback fail  | **P3** (опасно) |
| (5) DualPipe Overlap      | 29%↓ (8.5→6.0)  | Высокий     | Chunk corruption| **P3** (Paper-зависимо) |

---

### **4. Итоговая стратегия**
1. **P0**: Реализовать **Precomputed Routing Table** (Rust снапшот + батч-диспатч). 
   - Ожидаемый gain: **~1.77ms MSPT** (3.77→2.0), parity-safe.
2. **P1**: Добавить **Paged State Bitmap** для behaviors. 
   - Дополнительно: **~0.12ms MSPT** (0.27→0.15).
3. **P2**: Протестировать **Shared Expert** на subset мобов (e.g., только зомби). 
   - Если side effects可控 → масштабировать.
4. **P3/P4**: Исключить MTP и DualPipe из-за high parity-риска.

**Общий прогноз**: **~1.9ms MSPT reduction** (2.3% от 80.9ms), прирост TPS до **~14.1**.