# RESEARCH-459-P45 — ID-P45 sendBlockUpdated navigatingMobs pre-gate (WILD-агент закона 11, тик-459)

Ветка: `round-459-p45` @ origin/master 0d147876. Идея ID-P45 (ПОХИЩЕНА у agent-K ID-H04
roaring-occupancy паттерн, RESEARCH-458-P.md): chunk→navigatingMobs флет-маппинг из
home-chunk mirror; block-update в чанке без навигирующих мобов → zero-work ДО батча.
Лейн: nav_ai block-update хвост 0.5-1пп. Карточка-прогноз: 0.5-1 × 40-60% → +0.3-0.6пп дёшево.
Скаффолд: `src/nav_chunk_pregate.rs` + java-стаб в
`entityinside/net/minecraft/server/level/NavPlaneOps.java`.

## Механика (law 6: buffer → ONE native call → ready outputs)

Носитель — navplane-плоскость (cmp405_navplane): `NavPlaneOps.handle` уже батчит
per-mob `shouldRecomputePath` в ОДИН `navDecide` JNI. ОСТАТОК проблемы: на каждый
block-update с дельтой collision-shape `handle` ВСЁ РАВНО итерирует весь
`level.navigatingMobs` (collect-проход: UNSAFE-риды path/delayed + заполнение
плоских массивов) даже когда ВАНИЛЬНОЕ РЕШЕНИЕ для всех мобов = false.

P45-прегейт закрывает остаток O(1)-гейтом ДО collect-прохода:

1. **Mirror (rust, `src/nav_chunk_pregate.rs`)**: глобальный флет-маппинг
   `chunk key (cx,cz) → AtomicI32 count` (64 шарда, fixed-BSS, zero-alloc hot path —
   дисциплина entity_index.rs). Count = число НАВИГИРУЮЩИХ мобов, чьё
   **decision-sphere extent** накрывает чанк.
2. **Decision-sphere extent**: ванильное решение (javap-verbatim, транскрипция в
   nav_plane.rs) — `distSqr(posCenter, mid) < remaining^2`, где
   `mid = ((node.x+mobX)/2, (node.y+mobY)/2, (node.z+mobZ)/2)`,
   `remaining = nodeCount - nextNodeIndex`. Т.е. block-update в точке P может
   дать recompute только если `P ∈ sphere(mid, remaining)`. Каждый нав. моб
   регистрирует консервативный ЧАНКОВЫЙ extent своей сферы:
   `floor((mid±(R+1))/16)` по обеим горизонталям (вертикаль игнорируется —
   гейт только 2D-флет, покрывающий ВСЕ y — это расширяет superset, не сужает).
3. **Регистрация (след-нога, java note-сайты)**: funnels set/unset path —
   `PathNavigation.recomputePath/createPath/moveTo` + add/remove в
   `navigatingMobs`; позиционные входы (mob xyz) буферизуются per-thread как в
   entity_index (zero JNI per note), дрены в ОДИН fused JNI. Anchor home-chunk
   — тот же mirror-вызов (mob position), откуда считаются extent-чанки.
4. **Гейт**: `NavPlaneOps.handle` после Shapes.joinIsNotEmpty-гейта:
   `rc = navPregate(pos.getX() >> 4, pos.getZ() >> 4)`; `rc == 0` (PREGATE_EMPTY)
   → return ДО collect-прохода (zero-work: ни итерации сета, ни UNSAFE-ридов,
   ни navDecide); `rc == 1` → ванильный collect+navDecide как сегодня;
   `rc < 0` (ERR) → one-shot disarm (`pregateOk=false`) → навсегда полный
   ванильный проход (ERR-ladder, mobs_soa дисциплина).
5. **Unbounded bypass**: сфера с `remaining > REGISTER_R_CAP` (=64, типичный
   ванильный wander ≪ этого) НЕ регистрируется — вместо этого инкрементируется
   `unbounded_live`; пока он > 0, гейт всегда возвращает MAYBE (гейт засыпает,
   parity сохранён). FN невозможен по построению: либо сфера зарегистрирована
   (extent ⊇ sphere ⇒ chunk-hit), либо глобальный bypass.

## Сайты (ретаргет/дефайн — следующая нога, javap-декомпиляция обязательна)

- Java: `NavPlaneOps.handle` — 4 строки прегейта после Shapes-гейта (стаб уже в
  исходнике; класс НЕ пересобран — include_bytes! носит старый байткод, см. риски).
- Rust: `src/nav_chunk_pregate.rs` — mirror + `nav_pregate` JNI +
  `register_pregate(env, cls)` (RegisterNatives на только-что-определённом
  NavPlaneOps, define-ДО-arm — NCDFE-канон), вызов wiring'а в
  entity_index_manager-стиле — следующая нога.
- Note-сайты: javap patched-kernel.jar на 6 funnels (recomputePath/createPath/
  moveTo + navigatingMobs add/remove) — следующий тик, prегист-гейт
  «Retargeted sites:N>0» обязателен до вектор-ног.

## Parity: пустой чанк = superset-гейт

Контракт ТОЧНО как entity_index count==0 (см. шапку entity_index.rs):
**count==0 ⇒ ванильный проход не имеет наблюдаемых эффектов**. Формально:
гейт возвращает EMPTY только если ни ОДИН живой нав. моб не имеет
decision-sphere, накрывающей чанк блока ⇒ для каждого моба
`distSqr >= remaining^2` ⇒ ванильный `shouldRecomputePath(pos) == false` для всех мобов
⇒ ванилла ничего не пересчитывает; пропущенный collect-проход не имеет
побочных эффектов (риды + транзиентный isUpdatingNavigations-флаг). CME-retry
ваниллы (пере-диспатч себя) при skipped-итерации недостижим и ненаблюдаем.

- **FN-невозможность по построению**: register = inc(counts) ДО publish моба в
  live-set; unregister = unpublish ДО dec. Читатель (один AtomicI32 load без
  seqlock — дешевле entity_index, в этом и смысл гейта) видит транзиентные
  ТОЛЬКО ложные «не пусто» (безвредный fallthrough = ваниль).
- **V1-ловушка (почему extent, а не голый home-chunk)**: моб в чанке A с путём
  В чанк B (endNode в B, remaining ≈ дистанция) даёт mid на полпути —
  block-update в B (пустом по home-chunk) ванильно РЕКОМПЬЮТИТ моба.
  Голый home-chunk-маппинг дал бы FN. Поэтому регистрируется СФЕРНЫЙ extent,
  а не хоум — home-chunk остаётся только anchor'ом note-вызова.
- **mirror-расхождение — самотест**: `self_test()` (каждые N flush, паттерн
  cmp458_roar selfTest) сверяет структурные инварианты: всякий занятый
  chunk-key имеет count >= 1, sum(counts) == live-registration-сумма, mob-таблица
  без сирот; расхождение → BROKEN latch → fail-closed vanilla навсегда.
- Пустой lever / чужой флаг: natives не регистрируются, java-стаб спит
  (`pregateOk == false` — ставится ТОЛЬКО rust-регистрацией), хендл не
  ретаргечен → ваниль бит-в-байт по построению (закон 4).

## Δ (нога-прогноз)

- Хвост: nav_ai block-update collect+decide 0.5-1пп wall (компо-носитель).
- Захват: 40-60% block-апдейтов попадают в чанки, не накрытые ни одной
  decision-сферой (150k-поп фиксатор: нав. мобы концентрируются у спавн/путей,
  region-worker апдейты размазаны) → снятие всего collect-прохода на них.
- Прогноз ноги: **+0.3-0.6пп** (карточка ID-P45), цена — O(1) atomic load на
  апдейт + регистрация по path-funnel (не по тику/мобу). Мера: world-bench
  pair vs якорного банка, prегист-гейт pregate-hit-rate в stdout
  (DATA-PLAN канон: ноль pregate-hit = плацебо-класс).

## Риски

1. **Класс не пересобран**: стаб NavPlaneOps.java правит исходник, но
   `entityinside/build/.../NavPlaneOps.class` (include_bytes! в nav_plane.rs)
   остался старым — арм до javac-пересборки невозможен; пересборка +
   javap-сверка = первый шаг следующей ноги (доставка 65 major, ZERO nested).
2. **NCDFE-канон**: define NavPlaneOps ДО RegisterNatives(navPregate);
   one-shot `pregateOk` guard — натив зовётся только после регистрации
   (ROOTCAUSE-NCDFE: define-после-arm = NCDFE в hot path). Unregistered
   navPregate недостижим из java по построению.
3. **Переполнение таблиц** (CHUNK_CAP 128k / MOB_CAP 8k / probe) → ERR_STRUCT
   → BROKEN latch → навсегда ваниль (не краш, не FN).
4. **Распухание extent**: remaining > 64 → bypass-флаг гасит гейт глобально
   (деградация до статуса-кво, НЕ до невалидности). Реальный ванильный
   navigation ≪ 64 узлов на wandered-путях — ожидаемо никогда.
5. **y-ось игнорируется** (2D-флет): расширяет superset (лишние MAYBE),
   FN не создаёт — вертикальные update'ы в том же XZ-чанке покрываются.

## Интернет-рисёрч (закон 11, >=2 источника)

- **Roaring bitmaps — индустриальный прецедент occupancy-прегейта**:
  pkg.go.dev/github.com/RoaringBitmap/roaring/v2: «Roaring bitmaps are used by
  several major systems such as Apache Lucene and derivative systems such as
  Solr and Elasticsearch, Apache Druid…» — bitmap-index pre-gate перед точным
  сканом = стандарт Lucene/Druid/Spark. Формат (array <=4096 / bitmap 64k
  контейнеры, Lemire et al.) подтверждает выбор «AtomicI32-каунт на чанк +
  открытое адресование» как флет-версию того же паттерна для нашей
  BSS/zero-alloc дисциплины (heap-контейнеры docs.rs/roaring по-прежнему
  отвергнуты — унаследовано от 458-K). https://docs.rs/roaring
- **Paper world-configuration — site-факт**: docs.papermc.io/paper/reference/
  world-configuration — секция misc (вкл. updatePathfindingOnBlockUpdate) —
  документированный Paper-конфиг того самого хвоста: при true ванилла на
  КАЖДЫЙ collision-дельта block-update итерирует navigatingMobs
  (поведение, воспроизведённое javap-verbatim в NavPlaneOps.handle) —
  источник актуальности хвоста и точка входа прегейта.
- **Broad-phase before narrow-phase — канон супerset-гейта**:
  developer.nvidia.com/gpugems/gpugems3/chapter-32-broad-phase-collision-
  detection-cuda: «The output of the broad phase is the potentially colliding
  set… In the narrow phase, collision tests are exact» — наш гейт = broad-фаза
  (консервативный чанковый extent), collect+navDecide = narrow-фаза (точная
  IEEE754-математика); false positives безвредны, false negatives невозможны.

## NCDFE-канон (чек-лист ноги)

define-до-arm; one-shot fail-closed guard (pregateOk / BROKEN); STRICT eq
lever `cmp459_p45` (пустой/чужой = ваниль бит-в-байт); самотест-инварианты
каждые N flush; ERR-лестница ERR_STRUCT(постоянно)/ERR_RANGE(на вызов) →
ваниль; prегист DATA-PLAN (pregate-hit-rate>0 в stdout) до вектор-ног;
javap-сверка пересобранного класса (major 65, zero nested) до арма.
