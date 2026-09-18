# ABSORB S7-158 leg #3 (35376530777) — DUD: RegionTickOps DORMANT (fail-closed)

**Run:** 35376530777, head fae2233, started 17:48:46 UTC, SUCCESS 18:10 UTC.
**Config echo:** inside_cache=1 + flush_diet=1 + **region_threads=4** (fluid_free/dirty/demux/alloc=0).
**Вердикт: leg БРАК (не A/B-сэмпл REGION-THREADS) — мост не активирован, тик шёл ванильно.**

## Корневая причина (строковый证据 stdout)
```
[crussty-plugin] region_threads: computed patches (ServerLevel 142812 -> 142952 bytes Retargeted { sites: 1 };
  EntityCallbacks 12348 -> 12504 bytes add=Retargeted { sites: 1 } remove=Retargeted { sites: 1 }; ...)
[crussty-plugin] region_threads: Entity strict site-count violated (NotFound), hook stays dormant
```
S7-158d-хук UUID-сидирования зарегистрирован на **MTH_CLASS = "net/minecraft/util/Mth"** вместо
`net/minecraft/world/entity/Entity` (копипаст-наследие имени переменной `mth`). Mth проходит
probe-проверки патчера (`createInsecureUUID` есть в его пуле — там ОБЪЯВЛЕНИЕ метода), но не
содержит `<init>` с дескриптором Entity-конструктора → `retarget_invokestatic` вернул NotFound →
строгий fail-closed усыпил ВЕСЬ region_threads-хук ДО READY-флипа и ретрансформов. Патчи
ServerLevel/EntityCallbacks/Level/ChunkMap были вычислены и закэшированы, но никогда не сервились.

**Защита сработала штатно:** 0 NCDFE, 0 крэшей, 0 tracker-NPE, 0 uuid-dup (параллельного тика
не было вообще), vanilla-parity не нарушен — измерен чистый inside_cache+flush_diet профиль.

## Числа leg #3 (= ваниль-базовый профиль CUMULATIVE, не REGION-THREADS)
- total 57880 samples (мало — короткие очереди профайлера); TPS crawl 0.7-0.9 плоский, медиана 0.80
- young_gc=140 (база 118, +18.6% — шум/варианс базы; 140 = то же значение, что в S7-153 leg)
- entity-phase 52.38% CPU: residual 38.3% фазы, broadphase/collision 21.2%, fluid 17.8%,
  movement 8.5%, AI 8.0% (прим.: недо-RECON — классификатор first-match жёстче, чем S7-155)
- RegionTickOps lane = 0 сэмплов, worker offload = 0% — доказательство отсутствия активации
- TrackerTickOps/RngOps live-сэмплов 0 (хук спал)

## ТОП пожирателей leg #3 (методика владельца «ТОП-ПОЖИРАТЕЛЬ → ∞», ось a: CPU)
1. entity-tick фаза 51.98% (ванильный одиночный тик)
2. GC/JIT-native 39.25%
3. main-tick-other 3.10%; tracker 2.57%; chunk workers 1.28%
Оси б/в: young GC 140 пауз, total 16.8s (мертвые 175.4ms worst); TPS 0.8 = MSPT ~1250ms.
→ ТОП-1 остаётся entity-фаза; REGION-THREADS = атака на неё (экономика доказана leg #2 +66.7%).

## Фикс S7-159 (этот тик)
`src/region_threads.rs`: MTH_CLASS("net/minecraft/util/Mth") → **ENTITY_CLASS
("net/minecraft/world/entity/Entity")** + честные переименования mth→ent (хук, Target, activate,
ретрансформ-список, лог-строки). Патчер `patch_region_rng_entity` не менялся (был корректен).
Композиция с inside_cache сохранена: dispatch_bytes подаёт хуку Entity байты ПОСЛЕ inside_cache
ретаргета; скан сайта Mth.createInsecureUUID по имени — сдвиги пула не мешают.
Верификация: cargo suite 147/0/1; RegionThreads harness OFFLINE PASS (структура/wiring/dormant+
parallel; S7-158b sweep + S7-158d UUID-регрессы целы). Живое доказательство — leg #4 stdout:
появление `pristine sighting net/minecraft/world/entity/Entity` + строка
`region_threads: ARMED, retransform rc ServerLevel=0 EntityCallbacks=0 Level=0 ChunkMap=0 Entity=0`.

## Следствия для протокола
- leg #3 = брак, из min-of-2 исключается. leg #4 = min-of-2 сэмпл №1 (свежий preregister,
  тот же гейт-набор PG2/PG3/PG4 + 0 инцидентов). leg #5 = сэмпл №2 → банкование при полном PASS.
- Watchdog гигиены leg #3: BOTTLENECKS_3.md сгенерирован, артефакт полный (28MB), job завершился
  сам за ~22 мин — фиксы S7-158a (bounded console ops) работают.
