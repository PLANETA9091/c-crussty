# RESEARCH-C-k4 — TASK-411-C (cron tick-411, 12:08+08), K4 решение по свежему профилю

## ШАГ 0 — посимвольный профиль ЦЕЛЕВОГО лейна (до кода)

Источники (collapsed-cpu, method = stack-weighted attribution to DEEPEST lane frame,
regex CollisionUtil|getEntities|EntityLookup|getCollisionsForBlocksOrWorldBorder|
performCollisions|EntitySectionStorage|EntityCollectionBySection|EntitySection|ChunkEntitySlices):

| сцена | total | lane | lane/wall | скрипт |
|---|---|---|---|---|
| round-round410anchora (ваниль-якорь 410, run 35673607127) | 115385 | 18083 | 15.67% | /home/z/k4work/k4_decomp.py, k4_entries.py, k4_buckets.py |
| round-round410multi3 (multi-сцена, run 35673637169) | 110026 | lane 14.31% | engine 7119 = 6.47% wall | те же |
| round-round410ck3l (k3 нога cmp410_eindexq, run 35679297810) | 116376 | 17174 | 14.76% | те же |

### engine = ChunkEntitySlices.getEntities + EntityCollectionBySection.getEntities (anchora)

- EntityCollectionBySection.getEntities 31.28% lane / ChunkEntitySlices.getEntities 16.42% lane
  ⇒ engine 8630 сэмплов = **7.48% wall** (47.7% lane — подтверждает k3 48.6%).
- CollisionUtil.getCollisionsForBlocksOrWorldBorder 5510 = 4.78% wall (Entity.collide 61% / noCollision 39%).
- вход EntityLookup.getEntities(AABB) обслуживает 8002 = 44.25% lane.

### КЛЮЧЕВОЕ (новое vs k3): разложение engine по входам (anchora, ваниль)

| вход | доля engine | wall | что это |
|---|---|---|---|
| Level.getEntities ← getPushableEntities ← **LivingEntity.pushEntities** (91.2% бакета) | 50.0% | 3.74% | push-перечисление |
| getEntitiesOfClass: NearestAttackableTargetGoal.findTarget 65% + AvoidEntityGoal.canUse 9% + ItemEntity-pickup из Mob.aiStep 26% | 41.0% | 3.07% | goal-target + item-pickup |
| getHardCollidingEntities (ItemEntity.noCollision 67% + Entity.collide 33%) | 8.9% | 0.67% | hard-collision |
| прочее (boats/ActivationRange/minecart/pressure-plate) | ~0.1% | ~0.01% | — |

### ck3l (нога k3, флаг cmp410_eindexq АРМИРОВАН) — engine 8348 = 7.17% wall:

- 43.8% engine (3659) = `pushEntities → MobPushOps.pushables → **MobPushOps.vanillaFill** → EntityLookup.getEntities` — push-плоскость УПАЛА В ВАНИЛЬ;
- 33.6%+ (2807) = `findTarget/AvoidEntityGoal.canUse → EntityGoalQueryOps.entitiesOfClassGate → **Level.getEntitiesOfClass (vanilla)**` — goal-мост УПАЛ В ВАНИЛЬ;
- ItemEntity-pickup 480 (5.7%) — Mob.aiStep @109 `getEntitiesOfClass(ItemEntity.class, bb.inflate, )` javap round-396-a: класс НЕ Mob/Player-assignable — не обслуживается индексом (и не должен);
- getHardCollidingEntities 707 (8.5%), boats/Activation 429 (5.1%).

## КОРНЕВАЯ ПРИЧИНА PARITY-серии (server-stdout.log round410ck3l, 02:30:26-27)

```
cmp410_eindexq: epoch ok tick=12 rows=2 linked=2
cmp401_soa: oversized bounding radius 1.1875 on entity.minecraft.camel — lever reverted to vanilla
```

**ХРОНИКА (grep 'oversized bounding radius' по ВСЕМ artifact-директориям round-round405..411, verbatim):**

| нога | сущность | r_eff |
|---|---|---|
| 406dleg3, 406dleg4, 406dleg6, 406eleg1, 409eleg2, 409eleg3b, 409multi2, 410ck3l, 410multi3 | entity.minecraft.camel | 1.1875 |
| 409eleg2b | entity.minecraft.iron_golem | 1.35 |
| 409multi1 | entity.minecraft.warden | 1.45 |

Fired в 11/11 SoA-armed ног эры 150k (все артефакты на диске); в ногах без SoA-плоскости (якоря, cleg*, bl2, ak5) warn отсутствует — хроника полная. Java-гейт `MobPushOps.upsertSelf`: r_eff = max(hw,hh) > 1.0 ⇒ GLOBAL `oversized=true` ⇒ planeReady()==false ⇒ (1) push-плоскость vanillaFill навсегда, (2) eqEpoch-популяция мертва (ck3l: 'epoch ok tick=12 rows=2' → через тик camel → конвейер ваниль) — с тика ~12-13 из ~690.

### SCOPE (честно, не раздувая — верифицировано по ABSORB.md/BOTTLENECKS_3.md каждой ноги)

- **Вердикты эры НЕ перевернуты**: multi/comp-ноги GREEN несли ЖИВЫЕ плоскости — multi3: items 31.17%→**0.00%** (aibatch жив), nav_ai 14.16%→**7.54%** (sscan жив), TPS 3.0; 406d +19.9pp = items-несомая; eleg2b/3b = sscan-несомая (cmp406_sscan epoch-ok маркеры стоят ПОСЛЕ oversized-warn, эффект nav_ai реальный). Дизарм в них = мёртвый САБ-план (push), занижающий потолок, но не носитель вердикта.
- **Что дизарм ДЕЙСТВИТЕЛЬНО испортил**: (a) 410ck3l PARITY — goal-query мост cmp410_eindexq мерялся с МЁРТВОЙ популяцией (rows=2 → oversized через тик) ⇒ отрицательный вердикт по классу query-снапшота = АРТЕФАКТ, класс не опровергнут; (b) push-плоскость SoA ни разу в эре не измерена armed (все её атрибуции — ваниль); (c) cmp410_eindexq-цепь (цепочка наследования популяции) — гейт-дефект КОПИРОВАЛСЯ на неё через общий upsertSelf.
- Вывод: root-cause = хронический ГЛОБАЛЬНЫЙ ДИЗАРМ плоскости (не портящий вердикты композитной лестницы, но обнуляющий потолок push/goal-query лейна и невалидирующий ck3l-вердикт). Ремонт НЕ требует пересмотра ранее закрытых GREEN.

## РЕШЕНИЕ K4: вариант (a) — ремонт + доводка section-resident read-only снапшота (флаг cmp411_k4soa)

Выбрано по профилю: engine 7.2% wall состоит из push-enum (3.2pp) + goal-запросов (2.4pp), оба класса обслуживаются
ОДНИМ механизмом k3-снапшота (chunk-cell chains + frozen-column prune + живой exact-фильтр java). Ремонт = разблокировка
уже спроектированных плоскостей, а не новый класс мостов.

### K4-изменения (все под СТРОГО новым флагом `cmp411_k4soa`; cmp410_eindexq/cmp401_soa сохраняют точное прежнее поведение):

1. **Радиус-ремонт населения**: MobPushOps.upsertSelf гейт r_eff 1.0 → **2.0** (max в популяции бенча: camel 1.1875,
   iron_golem 1.35; warden 1.45 в multi — покрывает); src/mobs_soa.rs mob_query окно pad ±1 → **±2** ячейки
   (soundness: центр пересекающего AABB ∈ box.grow(hw,hh) ⇒ floor(q0−hw) ≥ floor(q0)−2 при hw ≤ 2.0);
   MAX_SPAN 32 сохраняется. eqEpoch-прун не меняется: MARGIN 8.0 ≥ radius 2.0 + tick-дрейф.
2. **Push-лейн из снапшота (0 per-query JNI)**: MobPushOps.pushables под cmp411_k4soa СНАЧАЛА пытается
   `EntityGoalQueryOps.pushCandidates(level, entity, box, pushableBy(entity))` — тот же chain-скан, что
   entitiesOfClassGate, но: предикат pushableBy, фильтр other != entity, БЕЗ cls-гейта (универс = SoA-популяция
   LivingEntity — ТОТ ЖЕ принятый контракт round-401, что и mobQuery: не-living pushables (boats) не в универсе —
   унаследованная документированная дельта, не новая); **дедуп ячеек прямоугольника ДО прохода цепей** (прямоугольник
   push-коробки ≤ ~4×4 ячеек чанка — попарная дедупликуляция ≤256 сравнений): двойной проход одной цепи дал бы
   дубликат-кандидата ⇒ двойной doPush (для nearest-пика дубликаты безвредны, для push tail — НЕТ).
   Fallback-лестница: snapshot null → легаси mobQuery (JNI per-call, остаётся) → vanillaFill.
3. Мотивация против легаси mobQuery как primary: per-call JNI + глобальный seqlock-ретрай скан под 150k upserts/тик
   (region_threads 4) = 1.5-2.5µs/запрос оценка vs ~0.4µs ванильного engine на разреженной популяции бенча;
   снапшот-путь = чистые java-чтения (k3-механика), 1 bulk-JNI/тик уже платится eqEpoch'ом.
4. **Не обслуживается** (и не должно): ItemEntity-pickup (класс не из универса), getHardCollidingEntities
   (универс hard-colliders = boats/shulkers — не в SoA), block-collision (CollisionUtil — меню (b) ОТКЛОНЕНО:
   per-move stateful вердикты нельзя батчить 1 bulk-JNI/тик без изменения ванильного порядка, block-data недоступна
   Rust-плоскости, bit-exact voxel-оракул вне бюджета тика; в свежем профиле 4.78% wall — принимаем).

### javap ground truth (ПОЛНЫЙ kernel-jar round-396-a patched-kernel.jar, метод d05c930):

- `NearestAttackableTargetGoal.findTarget` @52: invokevirtual `Level.getEntitiesOfClass(Class,AABB,Predicate)List` — 1 сайт;
- `AvoidEntityGoal.canUse` @47: тот же дескриптор — 1 сайт (k3-ретаргеты без изменений);
- `LivingEntity.pushEntities` @75: invokevirtual `Level.getPushableEntities(Entity,AABB)List` — 1 сайт (сайт
  MobPushOps.pushables уже стоит с round-401; под новым флагом внутри бриджа выбирается снапшот-путь);
- `Mob.aiStep` @109: `Level.getEntitiesOfClass(Class,AABB)List` = **ItemEntity**-pickup — целевой сайт НЕ подходит
  (универс), задокументировано как сознательный отказ.

### Ожидаемый эффект (оценка): engine −(3.2+2.4)pp wall потолок ≈ −5.5pp; реалистично −2…−4pp wall ⇒ broadphase lane
15.7 → ~11-13%; TPS против пула-411 (2.1-2.4) — цель ≥2.35 при прокраске EFFECT-маркеров и grep oversized = 0.

## Чек-лист гейта cmp411_k4soa (STRICT eq, пустой = ваниль бит-в-байт)

rust: entity_query.rs enabled(), mobs_soa.rs lever_mode(), mobs_manager.rs java_gate_matches()+ARM-ветка;
java: MobPushOps.leverEnabled(), EntityGoalQueryOps.leverEnabled(); баня-фикстуры k3 не трогаются (новых
ретаргет-сайтов нет); ARM-маркер: "cmp411_k4soa: ARMED soa-population+push-snapshot", EFFECT-маркер: первый
snapshot-serve push + epoch ok rows>1000; AIOOBE grep = 0; oversized grep = 0.
