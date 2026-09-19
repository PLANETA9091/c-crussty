# RECON-11 — G1/GC-фаза (свежий ТОП-1): декомпозиция драйверов, свип кандидатов #13

Тик 13:08 +08 2026-09-19 (TASK-316). База: банковый профиль v3 (run 35399980345; cpu-collapsed 128124, alloc-collapsed 10263, gc.log) — урок №6; скрипты research/gc-recon-2026-09-19/{recon11_g1_drivers.py, recon11_alloc_attribution.py}, raw: RECON11_raw.txt.

## 1. Декомпозиция G1-фазы (31.0% self-CPU)

| категория | self-CPU | механика |
|---|---|---|
| **card-set/remset (кросс-регионные записи)** | **16.39%** | G1CardSet.add_card 3.34 + add_to_howl 1.52 + add_to_container 1.25 + G1ScanCardClosure 1.26 + refine_card_concurrently 1.54 + G1RemSet/UpdateBuffer — обслуживание remembered-set: mutator пишет ссылку в СТАРЫЙ объект → карта → concurrent refine → card-set. Драйвер = ЧИСЛО old→young записей (не байты!) |
| **oop-scan живого графа** | **10.70%** | OopOopIterate (ConcurrentRefine 4.28 + CMOop 2.42 + RebuildRemSet 1.35 + ObjArray 1.07/0.97 + bounded 0.85) + oopDesc::size 4.01 — обход живых объектов (live ~3.9G к концу рана) в STW/concurrent-фазах |
| queue/evac | 1.69% | копирование выживших |
| rebuild/scrub | 0.70% | RebuildRSAndScrub |
| WallClock::signalHandler | 0.945% | артефакт профайлера (signal handler), не реальный G1 |
| прочие | ~0.6% | — |

gc.log: 50 Normal-young за [1..462]s → **интервал 9.41s**; паузы n=145 (вкл. Mixed) медиана 156.2ms, p90 176.7ms, max 190ms, суммарно 20.6s STW; eden-заполнение ~2.3-2.4G/9.4s ≈ **250 MB/s аллок-рейт**; live-сет к концу ~3.9G (150k сущностей + 9216 чанков + структуры).

## 2. Аллок-база (урок №7: сер-стеки = 0.0% — окно чистое)

ТОП типов: **AABB 20.19% + Vec3 19.89% (= 40.08%, сходится с prereg)**, BlockPos-семья ~15.7% (BlockPos 6.03 + Mutable 5.47 + BlockPos$6 3.62 + …), long[] 4.77, char[] 4.48, byte[] 4.11, ArrayList 2.82, Object[] 2.79, LazyEntityCollisionContext 1.40, Entity$$Lambda 1.19.

Мульти-лейбл атрибуция по деревьям (завышает пересечениями, см. RECON11_raw.txt): inside/checkInside — крупнейший аллокатор для ВСЕХ основных семей (AABB 1051/2413, Vec3 797/2533, BlockPos 663/1922, long[] 460/746).

## 3. Внутри-лейн: полная декомпозиция (свежая)

Лейн (inclusively, деревья checkInsideBlocks/applyEffectsFromBlocks/InsideBlockOps/forEachBlockIntersectedBetween) = **11.10% CPU**, из них:
- InsideBlockOps.gate+replay (банked inside_cache, статик-гейт deltaMovement==0 && pos bit-equal) = **0.59%**
- **ванильное тело discovery+эффекты = 10.51%** — статик-гейт почти не дентил лейн: в X150K сущности почти всегда имеют ненулевой deltaMovement (гравитация/фрикцион/блуждание), hit-rate низок
- Листья лейна: AABB.<init> 0.493, LongOpenHashSet.add 0.414 (TraverseOps-LongTable), BatchCollector.flushStep 0.400 (инфра v3), BlockPos$6.computeNext+<init> 0.604, vtable 0.311, applyEffectsFromBlocks 0.301, lambda$checkInsideBlocks$2 0.256, forEachBlockIntersectedBetween 0.251, ArrayList.isEmpty 0.241, Mth.floor 0.229, PalettedContainer.get 0.211, Sets$1$1 0.200

**javap-контракт discovery (entity_full.txt)**: checkInsideBlocks(List<Movement>, collector) 106 юнитов → per-step checkInsideBlocks(Vec3 from, Vec3 to, collector, LongSet, int) 61 юнит (makeBoundingBox по шагу, Mth-floor диапазон) → lambda$checkInsideBlocks$2 161 юнит — per-block visitor: BlockState + VoxelShape + **collidedWithShapeMovingFrom(from,to) + collidedWithFluid** + StepBasedCollector.visit.

## 4. Вердикт по кандидатам #13 (свип, все честно закрыты)

**(a) LESSON-8-COMPLIANT редирект крупных не-инлайнибельных аллок-тел → PAPER-REFUTED**: крупные тела (Entity.move 560 юнитов) держат alloc-долю move = 5.7% семьи AABB+Vec3 ≈ 2.3% давления → потолок ниже 2-3% класса + урок №8 (граница редиректа теряет инлайн — move сейчас инлайнится в caller'ы: inclusive Entity.move = 0.00%). updateFluid уже скаляризован как нейтральный (прецедент #10) — повторять нечего.

**(b) INSIDE-DIRTY-BOUNDARY (dirty-flag по пересечению границ блоков + генерация секций) → INFEASIBLE-BY-PARITY**: discovery = функция СВЁРНУТОГО ПУТИ (per-step segment-тест collidedWithShapeMovingFrom(from→to) с суб-блочной чувствительностью форм: полслибы/0.25-высоты меняют результат при том же floor-диапазоне), а не функция состояния. Парити-безопасный skip-условие = бит-равенство from/to = статик — УЖЕ БАНKED (inside_cache, 0.59% gate). Расширение на движущихся = парити-дефект по построению. Урок fluid_dirty повторён с уточнением механики.

**(c) DEMUX-V2 residual (getFluidState/getBlockStateFinal вне demux-пути) → PAPER-REFUTED**: residual-семья блочных чтений вне demux ≈ 1.2-1.5% CPU (getFluidState 0.568 + getBlockStateFinal 0.645 + часть readPalette 0.682/SimpleBitStorage 0.636) → ниже 2-3% класса; demux уже банked на главном пути get().

**(d) G1-френдли предвыделение (объект-пул collision-листов) → PAPER-REFUTED**: темпы, умирающие в eden, НЕ создают card-dirty (young→young записи не грязнят карты — барьер G1 пропускает young-цели); пул = старый объект → записи в него = old→old… но пул сам добавляет живые объекты (oop-scan +) и не сокращает НИ ОДНОГО old→young стора (сторы делают СЕТТЕРЫ полей сущностей, не темпы). Механически не попадает в драйвер card-set.

**(e) Скаляризация live-полей (deltaMovement/boundingBox в double-слоты Entity) → PAPER-REFUTED (read/write-асимметрия)**: getDeltaMovement()/getBoundingBox() читаются на порядок чаще записи; синтез Vec3/AABB на каждое чтение ПОВЫШАЕТ аллок; кэш-инвалидация возвращает сторы. Плюс shape-изменение горячего Entity.class (field-append) = риск вне выгоды.

## 5. Гипотеза эры: store-firehose card-set (НЕГЕРМЕТИЗИРОВАНО — требует инструмента)

Card-set 16.39% драйвится ЧИСЛОМ old→young записей. Оценка объёма: ~110-150k setDeltaMovement + ~50-100k setBoundingBox + sync + списки ≈ **200-300k old→young store/тик ≈ 4-6M/s**. КЛЮЧЕВОЙ блокер атрибуции: сеттеры инлайнены C2 → в alloc-стеках их фреймы ОТСУТСТВУЮТ (прямое измерение по стекам даёт 72+93 сэмпла = 1.6% — это АРТЕФАКТ инлайна, зеркальный урок №8); честная оценка через producer-цепочки (travel-physics = 13.87% аллока). Если entity-поля дают 40-70% всех old→young записей, **value-equal store-skip** (skip сета при бит-равенстве нового и старого значения — иммутабельные Vec3/AABB, идентичность ненаблюдаема) → card-set −20-40% ≈ 3-6.5% CPU → выше 2-3% класса. НО: доля неизвестна = нужен ИНСТРУМЕНТ, не вера.

## 6. NEXT (тип+1): ИНСТРУМЕНТИРОВАННЫЙ RECON-лег (CI-бут, без поведения)

Рычаг #13 SKIP-STORE-DIET получает GO/NO-GO через измерение: (1) диагностический `-Xlog:gc+remset*`/`gc+refine` в workflow (ИНСТРУМЕНТ, не config-win: не меняет поведение/GC-политику, только логирование) → пер-tick частота dirty-card и распределение по регионам; (2) кросс-чек: async-profiler alloc-стеки с учётом инлайн-границ (producer-атрибуция travel/aiStep вместо сеттеров); (3) опционально JFR OldObjectSample. Порог GO: измеренная доля entity-полей в old→young ≥ 40% И model-потолок skip ≥ 2-3% wall; NO-GO → paper-REFUTED #13 без леги. Vanilla-parity-скептицизм для skip: grep identity-использований getBoundingBox/getDeltaMovement в kernel + lockstep-оракул ≥1M при GO. S7-108 чист; диспатчей за тик 0.

## 7. Свежий ТОП после RECON-11 (три оси, урок №7)

| место | лейн | CPU | аллок | статус |
|---|---|---|---|---|
| 1 | GC/G1-фаза | 31.0% self (card-set 16.4 + oop-scan 10.7) | драйвер store-firehose | ИНСТРУМЕНТ-ГЕЙТ (#13) |
| 2 | fluid-push | 10.3-11.0% | 6.6% | REFUTED (#10) |
| 3 | inside-pipeline | 11.1% incl. (ваниль 10.5%) | ~40% семьи AABB+Vec3 | статик-гейт банked; dirty-расширение INFEASIBLE-BY-PARITY |
| 4 | travel-physics | 9.15% | 13.87% | paper-REFUTED (#11) |
| 5 | broadphase | ~2.1% | — | 2×REFUTED, ПАРК |
| 6 | tracker | 2.08% | — | <5% |
