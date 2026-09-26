# RESEARCH-458-I — SWAR/SIMD batch-AABB broadphase + monotone SAP push-хвоста
TASK-458-I (тик-458, 22:08 +08 2026-09-25). Вектор: ID-H01 (SWAR batch-AABB) + ID-H02
(монотонный SAP push-хвоста) с доски ROUND-457 (RESEARCH-457-H.md, agent-H). Носитель:
ветка round-458i-swar от origin/master (96cc2704 = полный cert-stack: ins4⊕senseins⊕
chunk-comp⊕paldelta). Lever: **cmp458_swar** (STRICT eq, union-widen поверх cmp457_paldelta).
Цель: broadphase-лейн 8.8-10.9% CPU.

## 0. РАЗВЕДКА КОДА (факты, не мнения)

1. **Под cert-stack пуш-плоскость мертва → лейн = ваниль.** `mobs_soa::eqsnap_mode()`
   включает cmp457_paldelta (и cmp436_ins4/cmp451_senseins) → per-entity `mobUpsert`
   аппендит dirty-строку в пер-потоковый DeltaShard. Но `entity_query::enabled_flag_is_eqsnap()`
   (ШИРИНА ТОЛЬКО cmp411_eqsnap|v3|cvs|bq|brain) НЕ включает эти флаги →
   `drain_eqsnap_shards()` из eq_epoch НЕ вызывается. Шарды насыщаются за
   16×8192 = 131072 upsert'а (~1-2 тика при 47k, <1 тика при 150k population_target),
   дальше каждый mobUpsert = ERR_RANGE → per-call ваниль; колонки плоскости нулевые →
   eq-цепи пустые (rc=0, «cold tables») → pushCandidates=false → 100% vanillaFill.
   ВЫВОД: lane «broadphase 8.8%» на серт-ногах = ванильный EntityLookup.getEntities,
   а SoA-инфраструктура спит. Ремонт кормления = часть вектора (свой feed в нашем
   bulk-JNI), чужие флаги не трогаем (закон 4: STRICT-eq изоляция).
2. **Лестница push под серт-стеком**: MobPushOps.pushables → upsertSelf (per-entity JNI
   mobUpsert → шард) → (K4||EQSNAP) EntityGoalQueryOps.pushCandidates (java bucket-walk
   по замороженным колонкам, rect=box±MARGIN 8.0, дедуп ≤64 бакетов, x/z-prune, точный
   live-AABB+predicate хвост) → vanillaFill. MARGIN=8.0 документирован как «≫ tick-дрейфа
   (~1-2 блока)» — т.е. покрывает 2×дрейф (кандидат И запрашивающий оба дрейфуют).
3. **Прецедент второго bulk-JNI в EPOCH_LOCK-окне**: senseArena (TASK-419-B) зовётся
   java из maybeEpoch СРАЗУ после eqEpoch, «суммарно 2 bulk-перехода плоскости на тик;
   per-entity JNI по-прежнему отсутствует». Наш swarEpoch = третий в том же окне,
   один на подсистему (закон 6).
4. **NCDFE-канон** (5ecd841a/097def9d): новая live-ветка в MobPushOps.pushables,
   резолвящая EntityGoalQueryOps, требует EARLY-define ДО первой пуша:
   entity_query::define_bridge_once (anchor LivingEntity) + mobs_manager publish-гейт
   lever_matches()==enabled() → union-widen флага в flag_enabled закрывает гонку
   структурно (arm AFTER define).
5. **boxFor(e)** (TASK-419-A, MobPushOps): 0-JNI id-регистрация — готовый механизм
   для фид-режима без per-entity JNI.

## 1. ИНТЕРНЕТ-РИСЁРЧ (curl, 2026-09-25)

- **box2d.org «SIMD for Collision» (Erin Catto, Jul 18 2026)** — «wide SIMD»:
  обрабатывать несколько work-unit'ов одним регистром; «For SIMD to work well the
  data needs to be in structure of arrays format (SoA)»; замеры convex pile
  (PEEL-порт): scalar 40706ms → SSE2 17337ms (>2×) → AVX2-Lite 15762ms на 1 потоке;
  «In Box2D I have AVX2 intrinsics as well, but there were a surprising number of
  users without AVX2 capable CPUs» → обязан иметь scalar-fallback (риск −15% с доски
  подтверждается: fallback обязателен, но не фатален). Прямой маппинг: наши 8
  кандидатов окна = 8 work-units, SoA-колонки плоскости уже есть.
- **Wikipedia «Sweep and prune»** (I-COLLIDE, Baraff/Lin/Canny lineage): сортировка по
  оси с минимальным разбросом дисперсии; инкрементальная поддержка почти-отсортированного
  массива (insertion-sort pass) = O(n + инверсий) на тик при малых смещениях —
  монотонный push-хвост ID-H02. Теорема SAP: x-пересечение интервалов необходимо
  для AABB-пересечения → окно по отсортированному xmin — суперсет.
- **NVIDIA GPU Gems 3 ch.33 «Broad-Phase Collision Detection with CUDA»** (ссылка с
  доски умерла 404, контент известен по тексту RESEARCH-457-H): sort-and-sweep с
  подавлением дубликатов пар; у нас пары не нужны — нужен per-query список кандидатов,
  поэтому бинпоиск-окно вместо полного sweep-прохода пар.
- **docs.rs geo-index** (Hilbert R-tree bulk-load) — рассмотрен и ОТЛОЖЕН (H03 уходит
  agent-J; статический R-tree на мутящуюся популяцию каждый тик дороже SAP).

## 2. §БЕЗУМИЕ — 3 идеи (закон 11: ≥3, ≥1 до ноги)

### [ID-I01] SWAR-CSR push-плоскость: monotone-SAP окно + 8-лановый AVX2 intersect + CSR-выдача одним bulk-JNI/тик — GO (нога)
- **Суть**: на эпохе (один bulk JNI `swarEpoch` в EPOCH_LOCK-окне, ДО eqEpoch):
  (а) поддержать монотонно отсортированный по xmin порядок живых id (insertion-pass,
  self-heal full-sort при деградации, O(n) sortedness-валидация);
  (б) для каждого живого id i: бинпоиск окна [xmin_i − hw_i − MARGIN − WPAD .. xmax_i]
  в порядке (WPAD = 2×RADIUS_GATE+1e-6 — soundness-пад на строгий strict-тест
  AABB.intersects и fp-округление ширины); (в) 8-лановый wide-SIMD (AVX2
  runtime-detect, 2×__m256d на ось, movemask-битмаска) intersect центров/радиусов
  по x и z из SoA-колонок (y НЕ пруним — паритет pushCandidates: вертикальный дрейф
  не ограничен маржой по x/z); (г) CSR-выдача (off[id]..off[id+1] → row[]; ovf[id]
  пер-строковый пер-энтити fallback-флаг) в java-массивы одним pinned-critical
  копированием. Java: swarPushables = ТОЧНЫЙ строгий хвост (level, other≠entity,
  live AABB.intersects, pushableBy) по строке — STRICT-режим хвоста не меняется;
  лестница: swarRow → pushCandidates (без изменений) → vanillaFill. NaN → кандидат
  не пересекается (NaN-compare=false в AVX2 и скаляре — fail-closed в суперсет).
- **Почему ≥+20 потенциал**: закрывает ЦЕЛИКОМ java-перечисление (дедуп бакетов
  O(b²)+chain-walk+prune = основа lane 8.8-10.9%) + ПЕРВЫЙ на серт-стеке живой
  SoA-candidate-путь; захват 40-70% lane = +3.5-7.5пп к ноге; пара с якорем +12-13
  (a26) при удаче ≥+20.
- **Риск**: плотные кластеры (фермы) → окна > ROW_MAX → per-entity ovf-fallback
  (деградация локальная, не глобальная); глобальный ROW_CAP → ERR_RANGE тика
  (= текущее мастер-поведение, fail-open безопасен). AVX2-отсутствие → скаляр
  (−15%, box2d-прецедент).
- **Parity-план**: superset-оракул в tests (coarse-модель против CSR-строк),
  sortedness-валидация SAP, MARGIN=8.0 паритет pushCandidates, self-heal сортировки.
  GO.

### [ID-I02] Zero-JNI upsert-feed: java-scratch колоночная кормление вместо per-entity mobUpsert JNI — GO (в той же ноге, тот же bulk)
- **Суть**: под cmp458_swar upsertSelf НЕ зовёт JNI: пишет (cx,cy,cz,hw,hh) в
  персистентные java FEED_D[id*5..]/FEED_I[id]=tick (0 JNI, boxFor-идентификация),
  swarEpoch применяет строки fresh≥t−1 к плоским колонкам (WLOCK+VERSION-брэкет,
  shape colpush_plane_refresh) и строит строки — ОДИН bulk-JNI обслуживает и кормление,
  и перечисление. 150k per-entity JNI-переходов/тик → 0. Заодно чинит stale-column
  разрыв (§0.1) для НАШЕГО флага: колонки снова живые, ai/sscan/eq read-views
  получают консистентное состояние, eq-цепи строятся из СВЕЖИХ колонок
  (swarEpoch ДО eqEpoch в окне).
- **Почему ≥+20**: JNI-переход ~50-100ns × 150k/тик ≈ 8-15ms/тик ≈ 2-4пп CPU —
  аддитивно к ID-I01; суммарный потолок ноги +6-11пп.
- **Риск**: гонка feed-чтения vs воркеры-писатели = контракт «нетик = не-дельта»
  colpush-прецедента (≤1-тик ghost, документирован); свежее-нужно честно маркировать.
- **Parity-план**: fresh-гейт в rust-тесте; graveyard sweep (mobRemove) не тронут;
  oversized/broken ветки upsertSelf без изменений. GO.

### [ID-I03] Unified query epoch: один swarEpoch обслуживает push+goal-query+sense (три CSR-выхода одной проходки) — PARK
- **Суть безумия**: goal-query rect (±8.0, окно >64 бакетов запрещено) и sense-арена
  можно вычислять той же проходкой SAP-окон (rect-боксы отличны от self-боксов —
  нужен второй список rect-запросов java→rust без JNI через scratch-очередь).
- **Почему PARK**: окна goal-query в 2-4× шире push-окон → объём CSR ×4+ при том же
  ROW_CAP; sense-арена уже O(rows) дешёвая; выигрыш = 2 JNI-перехода/тик (~мкс) —
  несоизмеримо с риском паритета. Переоценка после ноги (если JNI-переходы станут
  топ-лейном).

## 3. ПЛАН НОГИ (имплементация)
1. scripts/add_swar_gates_458.py — union-widen cmp458_swar во все гейт-листы с
   якорем cmp457_paldelta (rust Some/Ok/eq-паттерны + java equals-листы + ColpushOps
   FLAG-константа + BrainOps TICK2_FLAGS) — механика add_paldelta_gates_457.py.
2. src/mobs_soa.rs: swar_mode() STRICT-eq + SAP-статика + feed+sap+csr core + AVX2/scalar
   8-лановый intersect + selfTest-оракулы (superset/monotone/feed).
3. src/entity_query.rs: natives swar_probe/swar_epoch в RegisterNatives-таблицу
   (sig (II[D[I[I[I[I)I), публикация-лесенка java, ARM/EFFECT-маркеры, тест
   flag_enabled(cmp458_swar).
4. Java: EntityGoalQueryOps (natives + SWAR-флаг + maybeEpoch-хук + reader-аксессоры),
   MobPushOps (SWAR-флаг + feed-массивы + upsertSelf 0-JNI-ветка + swarPushables
   строгий хвост). STRICT-хвост (коллект/предикат/ваниль-fill) байт-в-байт.
5. scripts/build_458i_blobs.sh (механика build_432b_blobs.sh: один javac-pass
   --release 21, full cp kernel+fastutil+paper+adventure, nested+flat установка,
   flat==nested gate) + check_blobs_sync.sh.
6. cargo check --lib зелёный → cargo test (оракулы) → commit+push каждый шаг →
   диспатч ноги world-bench-parallel.yml (inputs канон ×457, lever_flag=cmp458_swar,
   lever_arg=1) через git/refs API.
