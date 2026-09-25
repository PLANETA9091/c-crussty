# RESEARCH-459-P41 — Path-node neighbor cache (TASK-459-66, WILD, law-11)

Карточка: RESEARCH-458-P.md «ПАКЕТ-4 — NAV_AI 2.75-3.2 ОСТАТОК» [ID-P41].
Ветка: `round-459-p41` @ origin/master (0d147876). Worktree: /tmp/wt459-p41 (sparse src+scripts).

## 1. Механика (vanilla → кэш)

Vanilla (Paper 1.21.10, javap-канон src/nav_pool.rs / src/nav_plane.rs):
`WalkNodeEvaluator.getNeighbors(Node)` на каждом раскрытии A*-узла обходит
фиксированный набор направлений (4 латерали + диагонали при открытых обеих
смежных + вертикали) и на КАЖДОГО соседа дергает block-getter / findAcceptedNode.
Повторные запросы одного и того же узла случаются:
  (a) внутри ОДНОЙ генерации пути (узел раскрывается из нескольких предков);
  (b) между ретаргетами/перепланировками по тем же чанкам (mob tick →
  shouldRecomputePath → recomputePath), пока чанк-ревизия не менялась.

P41: NodeEvaluator-соседи кэшируются флет-таблицей `posKey(i64) → neighborMask(u8)`
(бит i = направление i возможно-проходимо). Хит — пропускаем re-traversal
блочных данных getNeighbors; **MISS = ваниль бит-в-байт**. Инвалидация — по
chunk-ревизии: secKey(x>>4,y>>4,z>>4) → rev; rev mismatch ⇒ entry мёртв (miss).

Паритет-контракт:
- **маска = SUPERSET соседей**: кэш может предложить соседа, которого ваниль в
  этот момент не дала бы (диагональ при закрытой латерали), но НЕ МОЖЕТ
  потерять ни одного ванильного соседа;
- accept-тест соседа (findAcceptedNode/getPathNodeType) остаётся ванильным —
  кэш режет только re-traversal, супермножество кандидатов отфильтруется
  ванильным acceptance ⇒ результат пути идентичен (это трюк lithium
  PathNodeCache: кэшируется lookup, не решение);
- **порядок обхода ванили при miss** — miss-ветка = дословный vanilla-код;
- **STRICT-off до оракула**: lever `cmp459_p41` STRICT-eq, под любым другим
  флагом класс не определяется и ретаргет не компонуется → ваниль по
  построению. Включение только после оракула 10k путей (path = vanilla bit-in-bit).

## 2. Сайты (wiring-план iter-2)

| Сайт | Роль |
|---|---|
| `navpath/net/minecraft/world/level/pathfinder/PathOps.java` (новый) | java-сторона таблицы: lookup/store/invalidateSection/selfTest, MISS=-1 |
| `src/nav_path_cache.rs` (новый) | rust-движок флет-таблицы + bulk-JNI `navNeighborBatch` (паттерн nav_plane ERR-ladder) |
| `src/nav_pool.rs` | пул узлов (createHash-канон posKey), соседний кэш строится рядом |
| `src/classfile.rs` / `src/region_threads.rs` | define-before-arm + compose-гейт `cmp459_p41` (паттерн NavPlaneOps) |

## 3. Web-источники (lithium nether pathfinding)

1. **Lithium (Modrinth)** — https://modrinth.com/mod/lithium — changelog-семейство
   «nether pathfinding optimizations»: ai.pathing-оптимизации пути (nether =
   густая среда с дорогими neighbor-вычислениями, кэш neighbor'ов даёт основной
   выигрыш именно там).
2. **Lithium GitHub — ai.pathing mixins** — https://github.com/CaffeineMC/lithium-fabric
   (develop): `common/.../mixin/ai/pathing/{WalkNodeEvaluatorMixin, FlyNodeEvaluatorMixin,
   PathNavigationRegionMixin, PathfindingContextMixin, BlockStateBaseMixin}.java` +
   `common/.../ai/pathing/{PathNodeCache, BlockStatePathingCache}.java`.
   PathNodeCache.isSectionSafeAsNeighbor: chunk-section кэш «free of dangers» —
   «caching layer to greatly accelerate neighbor danger checks when path-finding»;
   getNodeTypeFromNeighbors изолирует 3×3×3 соседей в пределах одного chunk
   column и кэширует PathType per BlockState (superset-подход: null = «не
   инициализировано/динамика» → fallback на ванильный пересчёт).
3. **Lithium wiki Configuration-File** —
   https://github.com/CaffeineMC/lithium-fabric/wiki/Configuration-File —
   «mixin.ai.pathing»: «a faster chunk cache will be used for accessing blocks
   while evaluating paths» (PathNavigationRegionMixin) — подтверждение, что
   кэширование блочных данных на время path-evaluation — канонический приём.

Вывод по parity: lithium кэширует per-BlockState PathType + per-section danger
(читаемые данные), но accept-логику не трогает; наш neighborMask — то же
дисциплинарное семейство (кэш read-плоскости, решение остаётся ванильным).

## 4. NCDFE-канон (обязателен)

- define-before-arm: PathOps.class определяется в kernel loader ДО регистрации
  ретаргета и ДО включения ENABLED-латча (паттерн cmp420_colpush: «selfTest не
  может отравиться NCDFE» — 0 lazy resolutions на armed-пути);
- JVM кэширует NCDFE per constant-pool entry навсегда (root-cause cv3-1
  35712182885: NCDFE ×3938 после первого броска) ⇒ недопустимо достигать
  PathOps через неразрешённые символы: только define→selfTest→arm;
- fail-closed: ERR/throwable/native-null → one-shot disarm latch → ветка java
  ванильна; empty lever ⇒ класс не определяется вовсе.

## 5. Δ (прогноз-число)

nav_ai остаток 2.75-3.2% wall (компо-сцены, BOTTLENECK-459) × доля hit
20-30% повторных neighbor-запросов ⇒ **+0.6-1.0пп** к ноге.
Оракул-гейт: ≥10k путей, маска ≡ ваниль-recompute (superset-сходимость),
CI world-bench парный (min-of-2/3, law-16).

## 6. Риски

- pathfinder-мутации (блок-апдейты, сундуки/двери, fluid) → stale-маска:
  гасится chunk-ревизией (pathTypesByPosCache-инвалидация в sendBlockUpdated —
  тот же сайт, что nav_plane cmp405: единая точка invalidate);
- рост таблицы: cap 2^17 слотов, overflow = miss-фоллбэк на ваниль (bounded);
- tie-break A* (heap-порядок при равных f) — кэш НЕ меняет порядок вставки
  соседей: маска разворачивается в направления в ванильном порядке обхода;
- диагональные супермножества: extra-кандидаты всегда проходят ванильный
  accept ⇒ worst case чуть больше discarded nodes при hit — мерится оракулом.

## 7. Scaffold (этот коммит)

`src/nav_path_cache.rs` (движок + JNI-контракт + тесты), `navpath/.../PathOps.java`
(java-зеркало таблицы, JDK-only, самопроверка selfTest), lib.rs: `pub mod nav_path_cache`.
include_bytes!/ретаргет/region_threads compose — iter-2 (после javap-съёмки
getNeighbors-контракта и оракула).
