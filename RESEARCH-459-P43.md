# RESEARCH-459-P43 — Brain flat-memory registry (ID-P43, moonshot-карточка round-458p-ideas)

Агент: TASK-459-68 (WILD, закон 11 тика-459, v18.2; финал = {run id, ветка+SHA, вердикт-число}).
Ветка: `round-459-p43` от `origin/master` @ `0d147876` (sparse-worktree: src, scripts, Cargo.toml, Cargo.lock).

## 1. Идея и механика (карточка ID-P43 → план)

**Реестр Memories: `MemoryModuleType.ordinal` → флет-массив значений; snapshot БЕЗ Map-обхода на пути
чтения; Map = источник истины, флет = read-зеркало после мутаций тика.**

Развитие двух банковских паттернов:
- **brainhook (F2, TASK-249/S7-114)** — flat-snapshot lens по `availableBehaviorsByPriority` с
  fingerprint-пробами и WeakHashMap-identity-кешем (`randomtick/src/BrainOps.java`, 502 LOC);
- **goal_selector iter-2 (TASK-422-B)** — «флет-фастпат: единственное отличие от ванили — устройство
  данных», плоские проходы вместо итераторных обходов.

### 1.1 javap ground truth (purpur-1.21.10, `research/f2-brainiter-2026-09-17/Brain.cfdump.txt`)

- ctor `@5-8`: `memories = Maps.newHashMap()` — plain HashMap, ключ = MemoryModuleType (не LinkedHashMap!).
- Горячие читатели (каждый = `Map.get` → `checkcast Optional` → `INDY Optional.map`-цепочка):
  `getMemory` @262 (len 45, IllegalStateException-гвард для незарегистрированного типа),
  `getMemoryInternal` @283 (len 32), `checkMemory` @339 (len 59), `hasMemoryValue` @184,
  `getTimeUntilExpiry` @299, `isMemoryValue` @321 (композит hasMemoryValue+getMemory+filter).
- Мутационные сайты — ТОЛЬКО `setMemoryInternal` @234 (`Map.put`/`Map.remove` через eraseMemory);
  growth-only инвариант BrainOps-скана здесь не нужен: карта живая, мутации за тик — норма.

### 1.2 Схема зеркала (parity-канон карточки)

1. **Мутации не трогаются**: `setMemoryInternal`/`eraseMemory` живут в ванильном HashMap (Map = истина).
2. **Post-mutation rebuild**: после мутаций тика (один bulk-вызов на brain, точка врезки — хвост
   `Brain.tick`-фазы мутаций / рядом с tick2-плоскостью) зеркало перестраивается ОДНИМ Map-обходом:
   `flat[type.ordinal()] = Optional<ExpirableValue>` (тот же ref, клонов нет). Платим обход 1 раз
   за тик после мутаций вместо hash-прохода на каждом чтении.
3. **Чтения** (getMemory/getMemoryInternal/checkMemory/hasMemoryValue) body-swap'ятся в
   `flat[ordinal]` — O(1) array load, ноль hash/equals, ноль indy-цепочки.
4. **Fingerprint (все O(1), горячий путь)**: P1 `live == map.size()`; P2 identity-проба карты;
   P3 точечная parity-проба против свежего rebuild — oracle-only.
5. **DISARM-ЛАТЧ**: любое расхождение (в т.ч. ординал вне капа, null-ключ, коллизия) → односторонний
   disarm → чтения навсегда vanilla `Map.get` (strict-off до перезапуска). Tie-break isEmpty-порядка
   ванили не нарушается: флет читает ТЕ ЖЕ значения по ТЕМ ЖЕ ключам.

### 1.3 Сайты (будущие iter-2, scaffold фиксирует контракт)

| сайт | класс | тип врезки | фаза |
|---|---|---|---|
| getMemory/getMemoryInternal/checkMemory | Brain | static body-swap → BrainFlatOps | чтение (hot) |
| hasMemoryValue/isMemoryValue | Brain | композиция над checkMemory/getFlat | чтение |
| rebuild-точка | Brain.tick-хвост | 1 вызов/тик/brain | post-mutation |

## 2. Внешние источники (≥2, проверены live 200)

1. **Pufferfish DAB** — https://docs.pufferfish.host/setup/pufferfish-fork-configuration :
   «DAB is an optimization that reduces the frequency of brain ticks. Brain ticks are very intensive» —
   канон-источник карточки (q11_brain.json из /tmp/458p выветрился, заменён живой страницей): brain-лейн
   признаётся интенсивным самим Pufferfish, дебаунс тиков = их рычаг; наш рычаг — устройство данных
   внутри того же лейна (закон-композиция: DAB снижает частоту, P43 — цену оставшихся тиков).
2. **Pufferfish optimization guide** — https://docs.pufferfish.host/optimization/pufferfish-server-optimization-guide :
   DAB = «decreases how frequently complex AI tick» — подтверждение адресуемого лейна.
3. **Lithium** — https://github.com/CaffeineMC/lithium-fabric : прецедент оптимизаций AI-плоскости
   изменением структур данных (не семантики) — банк-канон goal_selector iter-2.
4. **R. Startin, 5 Java Mundane Performance Tricks** — https://richardstartin.github.io/posts/5-java-mundane-performance-tricks :
   бенчмарки Enum.ordinal/EnumSet-итерации — база «ordinal → флет-слот» как cheapest-ключа реестра
   (array index vs Map.getNode hash-walk).

## 3. Δ-арифметика (числа карточки + банка)

- Лейн: brain-хвост **1-1.5пп** CPU (карточка; согласуется с банком: brain 1.4% в CLAIMS ×426,
  task168 sizing F2-ядра 0.9-1.5% MSPT в javadoc BrainOps, serverAiStep subtree 9.73% в mobs_ai.rs).
- Захват: **25-35%** лейна — доля Map.get+indy-цепочек в brain-хвосте, снимаемая array-load'ом
  (оценка карточки; 4 горячих читателя + getMemories() на пути сохранений/целей).
- **Прогноз Δ = 1-1.5пп × 25-35% = +0.3-0.5пп** (суб-бар волны; кандидат в композит по закону 7
  с tick2/goalops-семьёй — тот же лейн, тот же носитель).

## 4. Scaffold этого тика (что именно легло в ветку)

| файл | роль |
|---|---|
| `src/brain_flat_registry.rs` | rust-носитель: контракт слотов (REGISTRY_CAP=512), модель Map/зеркала/мутаций, fingerprint P1/P2/P3, DISARM-латч (Synced→Diverged→Disarmed, терминальный), fail-closed чтение; 6 unit-тестов фиксируют parity-инварианты ДО define_class (KernelLoaderSim-канон). `register()` — STRICT-eq `cmp459_p43`, БЕЗ байт-хуков (закон 11: vanilla bytes не трогаются до оракула) |
| `brainflat/net/minecraft/world/entity/ai/flat/BrainOps.java` | java-мост stub: rebuildMirror (ОДИН Map-обход → flat по ordinal), getFlat (флет-чтение с disarm-фолбэком в Map), selfTestRegistry (oracle ДО arm: put/erase-партии + инъекция фантома + флет-чтения == vanilla Map.get), mirrorMatches. Компилируется БЕЗ kernel classpath (только java.util); FQCN `net.minecraft.world.entity.ai.flat.BrainOps` — без клэша с живым `net.minecraft.world.entity.ai.BrainOps` |
| `src/lib.rs` | wiring: `mod brain_flat_registry;` + `brain_flat_registry::register();` рядом с brainhook (dormant при пустом/чужом флаге) |
| `RESEARCH-459-P43.md` | этот документ |

### 4.1 Верификация stub'а (этот тик, локально)

- `cargo check` — чисто (0 errors; 52 pre-existing warnings — unreachable-pattern банк lever-гейтов).
- `cargo test --lib brain_flat` — **6/6 PASS** (0.02s): parity после put/erase-партий, инъекция
  фантома детектируется, disarm-латч терминален (идемпотентен), ординал вне капа = расхождение,
  size/identity-пробы, isEmpty tie-break = пустое зеркало.
- java 21 source-launcher (`java P43Run.java`, jdk.compiler in-JRE): stub компилируется и
  **`selfTestRegistry=true`** — оракул жив ещё до define_class (KernelLoaderSim-канон: модель
  прогоняется до попадания в kernel loader). Формальный javac-пин `--release 8` — iter-2.

## 5. NCDFE-канон (обязательный раздел)

Уроки RC7 (TASK-430-A), ×422-placebo, ×432-b DELIVERY-FAIL, TASK-437-A — все учтены в контрактах:

1. **define_class** BrainFlatOps в loader Brain'а — ТОЛЬКО после фактической загрузки Brain
   (brainhook::activate poll-луп, 180s deadline), **всегда init=false** (lazy): ранний
   `Class.forName(init=true)` отравляет registries → NCDFE в main → SEEN_DONE=0.
2. **SelfTest ДО arm** (TASK-437-A паттерн): `selfTestRegistry()==true` в kernel loader до READY-флипа;
   любой throwable = fail-closed (lane vanilla), как tick2_selftest в brainhook.rs.
3. **Nested-классов нет** (stub плоский) — порядок define не критичен (урок BrainOps$IdKey/Snapshot
   lazy-resolution снят: определять внутренние первыми не требуется).
4. **Ординалы — истина kernel loader'а** (Enum.ordinal мостом в момент rebuild); rust-CAP=512 — guard,
   не истина; ординал вне капа = расхождение → disarm (не attempt-scale).
5. CI-гейты следующего тика: javac-гейт `--release 8` (мajor 52, канон area-map-пина) +
   `grep NCDFE=0` + `grep AIOOBE=0` + ARM/EFFECT-маркеры (`brainflat: DISARMED` — grep≤1).

## 6. Риски (канон карточки «зеркал-расхождение — disarm»)

- **Расхождение зеркала и Map** (внешние писатели в Map, мутации вне учтённой фазы) — главный риск;
  гейт: fingerprint P1/P2 на горячем пути + P3 oracle-only + односторонний disarm (unit-тест
  `divergence_detected_and_disarmed_forever`).
- **Датапак-рост реестра** (ординалы за кап): не scale, а disarm + vanilla-путь (P43 iter-2 решает,
  растить кап или лочить реестр на старте).
- **Клэш имени** с живым F2 BrainOps при define в один loader — устранён пакетом `...ai.flat`.
- **Порядок итерации** не зависит от зеркала (чтения по ключу) — порядок-риски F2 здесь не наследуются.
- **Стик-точка rebuild**: врезка post-mutation должна стоять ПОСЛЕ всех мутаций тика; поиск точки —
  javap-работа iter-2 (кандидат: хвост Brain.tick / соседство tickEachRunningBehavior).

## 7. Next (iter-2, вне этого тика)

1. `scripts/build_brainflat_ops.sh` (--release 8) + committed .class + include_bytes в rust-носитель.
2. javap-снятие точек rebuild + body-swap патчи чтений (`classfile::patch_brain_memory_*`).
3. Oracle-харнес в kernel loader (selfTestRegistry) + STRICT-gate cmp459_p43 на носитель.
4. A/B по канону: min-of-3, пара Δ≤50k, окно ≤±5пп; вердикт только по EFFECT-маркеру.
