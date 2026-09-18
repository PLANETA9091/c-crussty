# ABSORB S7-162 leg #1 (35395826385) — DUD: сервер убит watchdog'ом во время инжекта (средовой инцидент)

**Run:** 35395826385, head 2080aa7, started 21:15:26 UTC, conclusion **failure** (21:47 UTC).
**Config echo (пререг):** inside_cache=1 + flush_diet=1 + region_threads=4 + **batch_collector=1**
(v3-кандидат: compose-цепочка entity_compose + ensure-retired + INSTANCES-телеметрия), fp4/300s/150k/seed42/xmx10G, radius 640.

**Вердикт: лега #1 = БРАК (не A/B-сэмпл) — сервер умер на фазе инжекта популяции ДО
профайл-окна; все гейты неизмеримы. Прецедент S7-158 leg#3: брак исключается из выборки,
редиспатч леги #2 с теми же входами.**

## Таймлайн (server-stdout.log, все метки UTC)
- 21:24:44 старт сервера; 21:24:49 «Done (16.097s)».
- 21:25:00-21:25:20 — **watchdog-дампы каждые ~5с** на тике ForceLoadCommand.setChunkForced
  (синхронная загрузка 9216 чанков одним тиком). ВАЖНО: это ЧИСТО ВАНИЛЬНАЯ фаза без сущностей;
  компоуз-цепочка ещё НЕ заармлена (первый watchdog-дамп = строка 267 лога, ARMED-маркеры = строки
  984/999) → столл форслоада средовой (медленный раннер), НЕ эффект рычага. В 4 предыдущих легах
  watchdog ни разу не палил.
- ~21:25:1x — compose ARMED (`entity_compose: ARMED chain [inside->rng->batch], 205458 -> 205610
  bytes, retransform rc=0` + `region_threads: ARMED ... ChunkMap=0` + `batch_collector: defined`).
- 21:25:21 INJECT START (loadedChunks=**9954** из 10000; farmClusters=497 из 500 — форслоад
  завершился неполностью, ещё один симптом медленного раннера), единственная TPS-строка 19.1.
- 21:25:34-21:26:24 — дампы watchdog продолжаются (тик инжекта > порога).
- 21:26:25 финальный дамп; 21:26:26 **«Stopping server»** — watchdog hard-stop.
- Сценарий ждал маркер INJECT DONE до 900с («still injecting... waited=900s»), затем fixture-гейты
  workflow: INJECT DONE marker missing → exit 1 (gate 1a/1b/1c FAIL: spawnable/churn/alive — окно
  бенча никогда не началось).

## Профиль гибнущего сервера (cpu-collapsed.txt, 96604 сэмплов — покрывает только хвост инжекта + stopServer)
- **95.4% CPU = Reference2IntOpenHashMap.{find (48,005), shiftKeys (47,355)}** на путях
  moonrise-регистрации сущностей:
  - ~48k: `BenchPopulation.spawnItem → addFreshEntity → addNewEntity → entityStartLoaded →
    ReferenceList.add → putIfAbsent → find` (фаза инжекта);
  - ~43k: `WatchdogThread.run → stopServer → PlayerList.removeAll → ... → Entity.setRemoved →
    entityEndLoaded → ReferenceList.remove → removeInt → removeEntry → shiftKeys`
    (watchdog-нить закрывала сервер).
- Для сравнения: в базе leg#5 (профайл-окно 300с) вся ReferenceList-семья = 20 сэмплов (0.02%).
- КАЛИБРОВКА: в leg#5 фаза инжекта (150k добавлений за 56.1с = ~374µs/добавление) вообще НЕ
  входила в профайл-окно. Порядок per-add стоимости в гибнущей леге сопоставим с ОБЫЧНОЙ стоимостью
  инжекта (в базе 56-69с на инжект — инжект и в здоровых легах доминируется moonrise-картой
  идентичностей). ОТСЮДА: концентрацию в Reference2IntOpenHashMap НЕЛЬЗЯ атрибутировать рычагу —
  это нормальная стоимость фазы инжекта, впервые попавшая в профайл (из-за гибели сервера до окна).
- Young GC = 30 (worst 139ms) за короткий хвост; 0 NCDFE / 0 NPE / 0 uuid-dup.

## Что живо-верифицировано (инженерные маркеры PG2 — все ЗЕЛЁНЫЕ до гибели)
- `entity_compose: ARMED chain [inside->rng->batch], 205458 -> 205610 bytes, retransform rc=0`
  — единая цепочка armится на живом сервере;
- `region_threads: ARMED, retransform rc ServerLevel=0 EntityCallbacks=0 Level=0 ChunkMap=0
  (Entity via entity_compose)` — регион-хук цел;
- `batch_collector: defined ... kernel loader` — мост определён;
- 0 NCDFE, 0 крэшей движка, паритет не нарушен (тик-луп просто не дошёл до измеримого окна).
- Телеметрия INSTANCES не успела (печатается каждые 600 тиков ретаргеченного forEach — тик-луп
  сделал единицы тиков).

## Корневая причина (составная)
1. **Медленный раннер** (средовой фактор, доказан): форслоад-тик >60с на ЧИСТО ванильной фазе
   без сущностей и без наших патчей → watchdog. В предыдущих 4+ легах порог никогда не брался.
   Форслоад завершился неполностью (9954/10000 чанков).
2. **Каскад watchdog'а**: дампы каждые 5с сами жгут CPU (известный covariate e2e-скриптов),
   затем hard-stop сервера на тике инжекта. Спиготовский watchdog — кофейник харнесса, не движка.

## Харденинг харнесса (этот тик, bench/world3/run_world3.sh)
`spigot.yml: settings.timeout-time: 86400` перед бутом (monitor-only; тики измеримого окна
0.4-1.2с — на два порядка ниже капа). Watchdog-килл = ковариата харнесса, не ванильное поведение;
e2e-прецедент учёта watchdog-fire covariate (bench/e2e/run_worldgen_ab.sh:26).

## Следствия для протокола
- Лега #1 из выборки S7-162 исключается. Редиспатч леги #2 с теми же пререг-входами
  (dispatch_s7162.py) на этом же тике; absorb — следующий тик по тем же гейтам PG2/PG3/PG4'''/CRASH-FREE.
- Инженерные маркеры leg#1 засчитываются как ЖИВОЕ подтверждение compose-цепочки (частичное
  покрытие PG2; на леге #2 перепроверяются полным набором с телеметрией).
- Открытое наблюдение для будущих ранаундов (НЕ рычаг, НЕ регресс): фаза инжекта доминируется
  moonrise ReferenceList/Reference2IntOpenHashMap (~300-400µs/регистрацию) — потенциальный
  архитектурный материал ОТДЕЛЬНОГО рекона, если когда-нибудь станет профиля-фазой (сейчас вне
  измеримого окна и вне ТОП).
