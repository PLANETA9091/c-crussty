# C20 (480) — alloc-элимина: точечный сайт ≥2пп capture? (javap/cargo-декомпозиция collide-move)

Ветка round-480-c20-alloc1 от **686f2258** (×479 MAIN-консолидация), docs-only, **0 код-дельт**, 0 диспатчей.
База: Л-477-C37.1 (4 профиля), Л212-ALLOC-CENSUS (S24), Л-479-A8 (young-стена slope ×95), Л-474-C48 (travel_diet RECON-21).
Fresh-ценз тика: alloc-collapsed ×479-ноги **a4s2** (load-фаза, 28,584 сэмпла) + **a4s3** (steady-state, 3,823 сэмпла), скрипт c37_alloc_lane_census.py (пере-прогон, метод идентичен C37) → `c20_census_out.txt`.

## 1. Fresh ×479 alloc-карта (сравнение с C37 ×477)

| лейн | a4s3 (steady) %tot | a4s2 (load) %tot | Δnorm_max@100%-kill (2.9пп×s) |
|---|---|---|---|
| **collide-move** | **41.80** (C37: 38.45/41.30 — воспроизведено, рост 0×) | 5.19 | **+1.21пп** / +0.15пп |
| chunkio-nbt | 0.34 | **52.52** | +0.01 / +1.52пп (фазовость C37-канон ×3) |
| spark-self+watchdog | 23.02 | 3.01 | stand-оверлей, не game |
| entity | 14.13 | 1.90 | +0.41пп |
| jvm-self | 5.68 | 24.25 | +0.16пп |
| collision (cmp401 rust-плейн) | 1.02 | 0.13 | +0.03пп (уже закрыт C02 −1.00пп) |

Модель Л212 (gc.log-ось, независимая): Σyoung-STW 1362ms/60s = 2.27% wall × 0.78 server → потолок ПОЛНОГО элимина всего young-аллока **+1.8пп**; collide-move 41.8% → ≤**+0.95пп** wall-эквивалент. Обе модели сходятся: лейн целиком < +2пп.

## 2. javap-декомпозиция tick-пути (точечные сайты; fixtures Entity_real/CollisionUtil.class, javap JDK21; §1-стеки a4s3)

Entity.collide(Vec3) 325-op: new ArrayList ×4 (@63/@72/@131/@377), new Vec3 ×1 (@452 — продукт, материализуется по семантике), invokestatic CollisionUtil.{getEntityHardCollisions,getCollisionsForBlocksOrWorldBorder,performCollisions}; collideBoundingBox: collectColliders→ImmutableList$Builder; collideWithShapes: FloatArraySet@0; Entity.move: ClipContext + Entity$Movement (продукт) + VehicleBlockCollisionEvent (редкий).
CollisionUtil: getCollisionsForBlocksOrWorldBorder → new LazyEntityCollisionContext + MutableBlockPos (закон-5 collision-scratch/inside-зоны); offsetList → **OffsetDoubleList**@13 (закон-5); collideY(VoxelShape,AABB,d) = **0 new** (moonrise CachedShapeData-путь уже безаллокационный); performCollisions → 1 new Vec3 (продукт).
TravelDietOps (мастер, DORMANT — canon `travel_diet: 0`, yml :236 «input dropped TASK-395»): уже реализует легальную точечную элиминау non-escaping temps collide (4×ArrayList + ε-AABB + FloatArraySet + getInputVector scale + travelInFluid ×2 Vec3) — Л-474-C48.1/.2 javap 4/4 term-exact, гейт-риск снят.

**Топ-точечные сайты a4s3 (Δnorm_max = 2.9пп × s @100%-kill, k=1, survivor=1 — щедрый верх):**

| сайт | s | Δnorm_max | легальность |
|---|---|---|---|
| EntityDimensions.makeBoundingBox→AABB | 4.16% | **+0.121пп** | skip_store #13-SBB DORMANT (stored boundingBox алиасится hard-colliding ×2714) = закон-5 |
| Vec3.subtract/add | 2.54% | +0.074пп | продукт/мутация — закон-4 движение |
| FluidState.getAABB | 2.35% | +0.068пп | **fluid-зона — граница запрета (закон 5)** |
| AABB.collidedAlongVector-цепи | 1.80% | +0.052пп | zero_alloc #10 DORMANT = закон-5 |
| Vec3.scale/multiply (fluidPushing) | 1.39% | +0.040пп | fluid-зона |
| AABB.inflate/deflate | 1.31% | +0.038пп | skip-store зона |
| FlowingFluid.getFlow | 1.20% | +0.035пп | fluid-зона |
| ImmutableCollections$ListItr | 1.15% | +0.033пп | iterable-дети G1-запрет-банк |
| **тип Vec3 весь** | 16.19% | +0.47пп | продукты setDeltaMovement/travel — закон 4 |
| **тип AABB весь** | 13.73% | +0.40пп | skip_store закон-5 |
| ArrayList весь (частично TD-temps) | 1.55% | +0.045пп | travel_diet-subset легален |

## 3. Capture-матем ≥2пп (пререгистрация тика)

- Бар точечного сайта: s ≥ 2.0/2.9 = **69.0%** всех alloc-сэмплов в ОДНОМ сайте. Максимум профиля: 4.16% (makeBoundingBox) → **дефицит ×16.6**; топ-1 стек вне collide-move 0.89%. Сайта ≥2пп НЕ существует (лейн ШИРОКИЙ: ~сотни стеков, топ-1 4.16%).
- Лейн collide-move ЦЕЛИКОМ: +1.21пп [модель 2.9пп] / +0.95пп [gc.log-модель Л212] < merge-гейт +2пп (дефицит ×1.65) ≪ pair +20 (×16.5); его 100%-вырез = reuse-пул/серез семантики движения = **alloc_diet закон-5 ЗАПРЕТ** (Л-477-C37.1).
- Потолок ВСЕЙ alloc-плоскости: +2.9пп (C37) / +1.8пп (Л212) < +20 (дефицит ≥17.1пп конструктивен) — даже суперблейн не даёт закона 18.
- Единственный некомпрометированный легальный точечный сайт = travel_diet-temps (уже в мастере, DORMANT): арм TRAVEL_DIET=1 цепочка ≤10.07% alloc (s7177) → **≤+0.29пп**, temps-доля ≈ +0.10..+0.15пп — суб-шум канона Л71 (young-only +0.10-0.28пп), STW-эффект ≈ −0.2..−0.3s/300s (лестницу 202k 26.30s→≤23.0 не переворачивает; flip требует full-plane kill = закон-5).
- STW-канон G6/Л125: вердикт-ось alloc-плоскости = только GC-дебт (MB/s, young-pause); TPS-конверсия запрещена.

## 4. Вердикт: **REFUTED_CENS** (точечный сайт ≥2пп недостижим: max-легал +0.15пп, max-сайт +0.12пп, лейн +1.21пп < +2пп)

- **0 диспатчей** (placebo-запрет, прецедент Л-477-C37.1: суб-бар нога = бесплатный band-сжигатель), **0 код-дельт**; cargo check --lib **0 err**, cargo test **373/373 PASS** (0 failed) — стражи дерева 686f2258 зелёные, фикс не требуется.
- arm-ветка-логика (для протокола): если бы гипотеза ≥2пп была GO и сайт collision-класса → lever cmp401_collide-класс (cmp466_c98ai union уже несёт); travel-класс → TRAVEL_DIET=1 env (не lever_flag); НЕ collision → канал без диспатча.
- Парити-LOCK: RNG/семантика движения не тронуты (0 Java/rust дельт); javap flat==nested N/A (новых блобов нет); NCDFE/AIOOBE преги N/A (0 фикс).

## 5. Reopen-условия (C37-канон, без изменений + fresh-числа)

Structural-лейн ≥70% young-alloc (смена load-профиля — генерация в окне: a4s2 chunkio-nbt 52.52% это load-фаза, steady 0.34%) ИЛИ рост collide-move ≥2× (стабилен ×2 тика: 38.45/41.30 → 41.80). Vec3/AABB-масса = 19b-heap-wall трек (Л187/Л198 interning), не GC-частота.
