# RECON-13c — Фазовая гистограмма (JFR) + атрибуция burst-курсора — TASK-326, 2026-09-19 ~16:5x +08

База: s7165 (10G, 150k, банк v3) — recon.jfr (78,686 ObjectAllocationSample / 70,898 ExecutionSample, span 473s) + alloc-collapsed (30,516 ap-сэмплов). Инструменты: scripts/bench4_recon/jfr/{JfrAlloc,JfrExec,JfrThread}.java, scripts/bench4_recon/recon13c_{gc_buckets,burst_attribution,full_stacks}.py.

## 1. Фазовая гистограмма ObjectAllocationSample (30s-бакеты, JFR-веса)
- t=0-30s: [B=2.97GB — инъекция чанков (expected).
- t=30-360s (steady): 17-25GB/бакет; доминанты AABB+Vec3+BlockPos-семья (travel-physics мобов) ~80% весов; [B ≈ 0.1-9MB/бакет (чанк-parse в JFR-канале НЕ виден — см. слепоту).
- **t=390-420s burst: total 40.9GB/бакет**: BlockPos$6=19GB (×10 от нормы), AbstractList$RandomAccessSpliterator=8.8GB (×1000 от нормы), AABB=10.5GB. Потоки: Server thread 11.15GB + crussty-region-worker-2 8.8GB (JfrThread) = region-тиковый путь, не отдельный поток.
- t=450-473s (финальный save/shutdown): [I=3.76GB, Object[]=2.24GB, [B=0.83GB, [J=0.88GB — NBT-write-профиль; entity-классы отсутствуют.

## 2. Атрибуция burst-класса (ап-стэки, полные, 451 сэмпл / 50 стэков)
**BlockPos$6 = итератор BlockPos.betweenCornersInDirection (lambda$8), вызыватель BlockGetter.forEachBlockIntersectedBetween ← Entity.checkInsideBlocks ← Entity.applyEffectsFromBlocks ← {ItemEntity.tick, LivingEntity.aiStep (Zombie/Sheep/Spider/Creeper/Skeleton/Chicken/Animal...)} ← RegionTickOps.tickBucket (Server thread + region-workers).**
Механика: на каждый тик каждая сущность проходит checkInsideBlocks (ванильные блоки-эффекты: magma/cactus/berry/...) → на каждый вызов создаётся итератор BlockPos$6 + BlockPos$MutableBlockPos (курсор); при 150k сущностей = массовый мелкообъектный churn каждый тик. Путь НЕ покрыт ни alloc_diet (#2), ни inside_cache (#3 — мемоизирует дискавери эффект-блоков, но не убирает курсорный обход), ни zero_alloc (#10 — только fluid-push тела).

## 3. Слепая зона JFR OAS (системная, зафиксировать)
jdk.ObjectAllocationSample (weight-сэмплер) атрибутирует TLAB-накопленный вес «крупным» объектам потока → мелкообъектный churn (DataResult, малые [B, курсоры) невидим/недосчитан: DataResult 19.4% ap-сэмплов в JFR = 0; parse-[B burst отсутствует в JFR-бакетах steady при ap 33.38%.
**Решение по источникам истины: ap-collapsed (стэки+интервал-сэмплы) = классы/вызыватели/объёмы; JFR = время/потоки/фазы; JFR-классные веса НЕ использовать для ТОПа.**
Следствие для TASK-325 (RECON-13b): parse 33.38% ap-окна — реален для окна, но окно ap не синхронизировано с JFR-осями (ap.log без таймстемпов) → периодика parse (steady vs burst) остаётся открытой; НЕ блокер для кэша декодированных chunk-data по ревизии (устраняет повторы при любой периодике; хит/мисс доказывается runtime-счётчиком гейт-лега).

## 4. Новые под-лейны / рычаги (атака сверху вниз, ap-база)
1. **checkInsideBlocks-cursor churn** (новый, из этого RECONа; ap ~1.5% сэмплов + JFR-бурст-доминация в 390-420s; каждый тик × 150k): рычаг = zero-alloc итератор (thread-local reusable cursor в forEachBlockIntersectedBetween) ИЛИ memo "без блоков-эффектов в кубе" с event-driven dirty от инъекции/изменений чанка; парити: выдача блоков побитовая (тот же порядок обхода) — риск низкий.
2. chunk-parse diet (кэш декодированных chunk-data по ревизии) — из RECON-13b, 33.38% ap-окна; требуется runtime-доказательство повторов (счётчик parse-per-rev в гейт-леге).
3. navigation/pathfinding 6.58% — node-pool v2 (критерий young-GC частота) — из RECON-13.

## 5. NEXT (тип+1)
Реализация #1 (zero-alloc betweenCornersInDirection-курсор, lever #11 CRUSSTY_ZERO_CURSOR) + прегистер-гейты: PG1 аллок-сэмплы BlockPos$6/[$6.<init>] ↓≥80% в ap-окне; PG2 TPS last-5 медиана ≥ диаг-база 2.0; PG3 remset dirty p50 не хуже ±1%; PG4 NCDFE=0/CRASH-FREE; A/B min-of-2. Параллельно RECON-13d: parse-повтор счётчик (пассивный диаг в гейт-леге #2).
