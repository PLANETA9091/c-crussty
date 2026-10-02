# S7-134b — ПЕРВЫЙ ИСТИННЫЙ ALLOC-ЦЕНЗ X150K (run 35275967738, master 662738e, 2026-09-18)

## Гейты абсорба (preregistered в dispatch_s7134.py)
1. ap.log чист: 4× «Profiling started», 0 ошибок — **PASS** (впервые: 4 сессии cpu/wall/alloc/flamegraph реально сменились)
2. alloc-листья = аллокационные сайты (long[]/AABB/Vec3/char[]/<init>), 0 G1 oop-closures/C2-фреймов (grep = 0) — **PASS**
3. Байтовая оценка S7-133b («сотни GB/60s») — **REFUTED честно**: истинный чёрн 25.6GB/60s = 427MB/s = **21.4MB/тик** (gc.log: 14 young GC в окне × ~245 eden-регионов × 8MB); оценка S7-133b завышена ~15× (считала эвакуацию выживших, не eden-потребление). Инфраструктура валидна: интервал = 25.6GB/7369 сэмплов = 3.56MB/сэмпл, внутренне согласовано.
4. wall ≠ cpu — **PASS** (wall-стеки содержат waiters, отсутствующие в cpu: WatchdogThread.run, Reference$ReferenceHandler.run, DedicatedServer$1.run, PrioritisedQueueExecutorThread ×3; файлы 1.4MB vs 23MB)
5. Fixture: INJECT DONE 150000/150000 (105118ms), FIXTURE-VALIDITY: VALID, alive 4/4 — **PASS**

## Ранжирование чёрна по байтам (25.6GB/60s = 21.4MB/тик, young-gen)
| подсистема | доля | ~GB/60s | ключевые сайты |
|---|---|---|---|
| movement/collision-геометрия | 43.9% | 11.2 | **Vec3.add 9.5% (топ-1 сайта ценза)** через Entity.collidedWithFluid→collidedWithShapeMovingFrom→AABB.collidedAlongVector; AABB makeBoundingBox 5.8% (EntityDimensions), AABB.inflate 3.9% (Zombie.aiStep и др.), Fluid.getAABB 3.0%, Vec3.multiply 2.4%, AABB.getCenter 1.7%, AABB.move 1.1%, Vec3.relative 1.0% |
| inside-blocks | 22.1% | 5.7 | **LongOpenHashSet long[] 6.1%** (checkInsideBlocks→forEachBlockIntersectedBetween — dedup-множество НА КАЖДУЮ сущность НА КАЖДЫЙ тик), BlockPos$6 corner-lambda 5.2% (betweenCornersInDirection), **InsideBlockEffectApplier.flushStep→Arrays.copyOf 4.6%**, ListItr 1.7%, MutableBlockPos 1.2%, Entity$$Lambda 1.4%, List.of 1.1% |
| JVM/other | 17.3% | 4.4 | вкл. CgroupUtil BufferedReader 6.3% (JVM-внутренний cgroup-поллинг — вне досягаемости серверного кода, JVM-флаги запрещены) |
| fluid | 5.5% | 1.4 | FlowingFluid.getFlow 1.6%, FluidPushGuardHook.slow MutableBlockPos 1.0% |
| entity-tick прочее | 4.9% | 1.3 | |
| chunk/palette | 3.3% | 0.8 | |
| AI/Brain | 2.5% | 0.6 | |

## Кросс-связка с CPU (тот же ран, свежий мастер-профиль)
- GC-лейн совокупно ~24% (G1 oop-итераторы/remset/cardset — старый рычаг S7-133b)
- **Топ-1 kernel-функция по использованию = PalettedContainer.get 3.1%** (+readPalette 0.8%) — питается ТЕМИ ЖЕ путями: getBlockState из checkInsideBlocks (пересекаемые блоки) и updateFluidHeightAndDoFluidPushing (4 угла × сущность × тик)
- FluidPushGuardHook.slow 1.6% (собственный guard-лейн), AABB.intersects 1.5%, ChunkEntitySlices.getEntities 2.5% сумм

## Очередь рычагов S7-135+ (вердикт по байтам+CPU)
1. **INSIDE-BLOCKS/FLUID per-entity мемоизация** (event-driven dirty-flag — инструментарий владельца): сущность с Δpos=0 и неизменной ревизией секции пропускает checkInsideBlocks/updateFluidHeight геометрию ⇒ бьёт 22.1% чёрна + часть 43.9% + долю топ-1 CPU-функции PalettedContainer.get. Ключ кэша обязан включать ревизию block-state секции (fluid flow меняет состояния без движения сущности). Механика — кэш-гвард (прецедент fluid_guard TASK-80, median-exact parity).
2. **collidedWithFluid геометрия** (Vec3.add 9.5%): STEP-0 javap-контракт AABB.collidedAlongVector — читаются ли промежуточные Vec3 (лens/батч против переиспользования).
3. **InsideBlockEffectApplier.flushStep** (Arrays.copyOf 4.6%): rotating pool — субстрат EntityQueryOps готов.
4. CgroupUtil-чёрн (6.3%) — JVM-внутренний, недостижим без JVM-флагов (запрещены) — помечен не-целью.

Evidence: alloc-collapsed.txt (2.56MB, 1588 стеков, 7369 сэмплов), gc.log, raw_census_top25.txt, BOTTLENECKS_3.md (v3-отчёт: ALLOC-единицы = BYTES-семантика сэмплов, интервал выведен).
