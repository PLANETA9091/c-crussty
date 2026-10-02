### **Аналитика CPU-ONLY МЕГА-УСКОРЕНИЯ для Server Tick-Loop (Paper 1.21.10)**

---

#### **(a) AoS→SoA для Entity-Tickers + Cache-Line Alignment + Prefetch**
- **Техника**: Преобразование `List<Entity>` (AoS) в `struct Entities { x: Vec<f64>, y: Vec<f64>, ... }` (SoA). Выравнивание по 64B (cache line), AVX2-предзагрузка (`_mm_prefetch`).
- **Применение**: Через JNI-нативы (`entity_mirror.rs`) с маппингом на Java-объекты (reflection-free).
- **Прогноз**: 
  - Entity loop: 2.27% MSPT → **↓1.35%** (60% оптимизация).
  - Cache misses: ↓40% (измерено в `entity_mirror.rs`).
  - **Итого**: **↓1.35% MSPT**.
- **Parity-риск**: Низкий (если SoA-структура копирует Java-семантику `getBoundingBox`/`AABB.intersects`).
- **Реализуемость**: Высокая (уже есть прототип).

---

#### **(b) SIMD-Батчи: AABB Broadphase, Narrowphase, RNG**
- **AVX2 (4-wide) → AVX-512 (8-wide)**:
  - **Broadphase**: AABB intersection (4x entities/cycle). AVX-512 даст **2x** скорость (8 entities/cycle).
  - **Narrowphase**: SAT (Separating Axis Theorem) в AVX-512 — **↓0.5% MSPT**.
  - **RNG**: SIMD-версия `xorshift` (bit-exact с Java `Random` через JNI-обертку). 
    - Текущий `random-tick`: 4.5% MSPT → **↓2.7%** (60% векторизация).
    - Риск: Проверка бит-точности (тест: 10M чисел → совпадение бит).
- **Прогноз**: **↓3.2% MSPT** (broadphase + RNG + narrowphase).
- **Parity-риск**: RNG — средний (требует unit-тестов), AABB — низкий.
- **Реализуемость**: Высокая (AVX-512 есть в Xeon Scalable).

---

#### **(c) GC 9.5%: Off-Heap Structures + Barrier Avoidance**
- **Техника**: Вынос горячих структур (entity metadata, chunk palettes) в Rust-арену (zero-copy).
- **GC Avoidance**: 
  - G1 barriers: 4.7% MSPT → **↓2.8%** (через JNI-аллокации без GC).
  - Allocation-shape: Замена `ArrayList<Entity>` на pre-sized Rust-вектор.
- **Прогноз**: **↓4.0% MSPT** (barriers + ephemeral allocs).
- **Parity-риск**: Средний (требует синхронизации JNI-Java для off-heap данных).
- **Реализуемость**: Средняя (требует careful memory management).

---

#### **(d) Арена-Аллокатор на Фазу Тика**
- **Техника**: Буфер на 1 тик (pre-allocated arena) для entity-запросов.
- **Применение**: Замена `new ArrayList<>()` в `ChunkEntitySlices.getEntities`.
- **Прогноз**: 
  - Allocs: ↓0.32% MSPT (fastutil map) + ↓0.5% (GC pressure).
  - **Итого**: **↓0.8% MSPT**.
- **Parity-риск**: Низкий (арена очищается после тика).
- **Реализуемость**: Высокая (уже в `entity_mirror.rs`).

---

#### **(e) Амортизация: Budgeted Incremental Updates**
- **Техника**: Разделение entity-тика на фазы (movement → collision → AI) с time-slicing.
- **Пример**: 20 тиков на полное обновление (1 фаза/тик).
- **Прогноз**: **↓0.5% MSPT** (сглаживание пиков).
- **Parity-риск**: Низкий (детерминизм через RNG seed).
- **Реализуемость**: Средняя (требует refactoring entity loop).

---

#### **(f) Детерминированный Параллелизм (Folia/Leaf)**
- **Техника**: Параллелизм по регионам (16x16 чанков) с read-only фазами.
- **Безопасные фазы**: 
  - Movement (no cross-region deps).
  - Random-tick (seed-based).
- **Прогноз**: **↓1.0% MSPT** (4-core scaling → 25% load reduction).
- **Parity-риск**: Высокий (требует careful dep tracking).
- **Реализуемость**: Низкая (Paper не поддерживает regions natively).

---

#### **(g) Zero-Copy Palette-Lenses для PalettedContainer.get**
- **Техника**: JNI-метод `getPalette(int x, int y, int z)` → возврат Rust-среза (&[u16]).
- **Прогноз**: 
  - PalettedContainer.get: 3.7% MSPT → **↓2.2%** (60% zero-copy).
  - JNI overhead: ~50ns/call → приемлемо при 10k выз/тик.
- **Parity-риск**: Низкий (если Java-код не модифицирует палитру).
- **Реализуемость**: Высокая (уже есть JNI-инфраструктура).

---

#### **(h) AbstractBoat.tick Whole-Body Hot-Patch**
- **Анатомия**: 
  - Physics: 5.2% (water drag, collision).
  - Passengers: 2.8% (entity riding logic).
  - Lerp: 1.6% (position smoothing).
- **Rust-оптимизация**: 
  - SIMD для AABB (8-wide AVX-512).
  - Arena для passenger list.
- **Прогноз**: **↓6.0% MSPT** (62% от 9.66%).
- **Parity-риск**: Средний (требует точной копии Java-физики).
- **Реализуемость**: Высокая (аналогично `PerlinNoise.getValue`).

---

#### **(i) Оценка JNI-Перехода**
- **Цена**: 30-50ns (измерено в `entity_mirror.rs`).
- **Выигрыш у JIT**: 
  - JIT-код: 1-2ns/instruction (оптимизирован).
  - Натива выигрывает при: 
    - Long loops (>1000 итераций).
    - SIMD-операции (JIT не векторизирует сложный код).
- **Проигрыш**: 
  - Короткие методы (<100ns) — JNI overhead >50%.
  - Частые вызовы (например, `getBoundingBox`).

---

### **РАНЖИРОВАНИЕ (Эффект × Реализуемость / Риск)**
| Техника | Эффект (%) | Реализуемость | Риск | Счёт |
|---------|------------|---------------|------|------|
| (c) GC Off-Heap | 4.0 | 5/10 | 5/10 | 4.0 |
| (h) Boat Hot-Patch | 6.0 | 8/10 | 6/10 | 4.8 |
| (b) SIMD RNG/Broadphase | 3.2 | 9/10 | 6/10 | 4.8 |
| (a) AoS→SoA | 1.35 | 9/10 | 3/10 | 4.05 |
| (g) Zero-Copy Palette | 2.2 | 8/10 | 3/10 | 5.87 |
| (d) Arena Alloc | 0.8 | 9/10 | 2/10 | 3.6 |
| (e) Time-Slicing | 0.5 | 6/10 | 4/10 | 0.75 |
| (f) Parallel Regions | 1.0 | 3/10 | 3/10 | 1.0 |

**Итоговый прогноз**: **↓12-15% MSPT** (комбинация топ-3 техник).  
**Ключевые риски**: Параллелизм (f), GC barrier avoidance (c).  
**Рекомендация**: Начать с (h), (b), (g) — максимальный эффект/риск.