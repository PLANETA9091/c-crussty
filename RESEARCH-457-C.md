# RESEARCH-457-C — ценз ActivationRange+NaturalSpawner (§1) и пивот в R2 entity-query слой (§2)

## 0. Постановка (закон 3)
Носитель spawn-вектора round-456a-spawn @01688ad2 (despawn sscan-плейн, ноги +14.0/+7.5).
Мандат: GO/NO-GO ценз подсистемы ActivationRange+NaturalSpawner ЦЕЛИКОМ как ×456-A;
NO-GO → пивот в другую подсистему закона 6 на новом носителе cmp457_*.

## 1. ЦЕНЗ: NO-GO подтверждён по файлам (не переделывая миграцию ×456-A)
Проверка выводов §6 RESEARCH-456-A по дереву и истории:
1. **Слайсы в носителе ОТСУТСТВУЮТ**: `git ls-tree -r 01688ad2 | grep -iE "spawner|activation"` — пусто;
   в дереве единственный spawn-осевой артефакт = despawn-плейн sscan/MobScanOps (+ build_sscan_ops.sh).
   Merge fe7e7120 --stat: не трогает НИ ОДНОГО Spawner/Activation-файла (java-блобы: colpush,
   entitygoalquery, entityinside, goalops, mobai, mobpush, queryplane, BrainOps, sense, sscan).
2. **Измерения корня вектора 75233c2b подтверждены** (RESEARCH-455-A.md §1, javap/профили ×452-×454):
   despawn 0.31-0.35% + NaturalSpawner 0.50-0.94% + ActivationRange 0.62-0.83% (ванильные анкоры
   0.33-0.35/0.50-0.54/0.62-0.73; diet-нога 3.00TPS 0.35/0.94/0.83). НЕТРОНУТЫЙ остаток оси
   (despawn уже в носителе) = **+1.12…+1.77пп АБСОЛЮТНЫЙ МАКСИМУМ** (100% захват без
   субаддитивности — недостижим; канон субаддитивности ×3: юнионы едят 40-60% суммы).
3. **Математика бара**: собственный потолок семьи носителя +14.0 (max из 4 ног: −6.3/+2.0/+14.0/+7.5).
   Идеальный потолок ноги 14.0+1.77 = **+15.77 < +18**. Порог ЛЮБОЙ пары ≥+20 при депресс-гейте
   norm≥−2 фиксирован: leg ≥ +18.0 (лучший депресс-якорь банка ×456 a10 −2.0@6865200). Пара-потолок
   15.77+2.0 = **+17.77 < +20**. Никакое расширение якорного пула порог не опускает.
4. **Реализация незаконна в бюджете** (закон 4/6): NaturalSpawner bit-exact требует миграции
   getNoiseBiome + blockstate-коллизии + entity-obstruction в Rust; ActivationRange имеет
   java-побочки (activatedTick/isTemporarilyActive) + per-entity JNI; обе 2-4ч > остатка цикла.

**ВЕРДИКТ §1: NO-GO** (числа ценза ×456-A подтверждены по файлам) → ПИВОТ по закону 3.

## 2. ПИВОТ: подсистема ЦЕЛИКОМ = R2 entity-query слой (остаток broadphase-лейна)
### 2.1 Выбор из меню закона 6 (по остаточному лейну на серт-ногах мастера f44a831e)
Лейн-таблица серт-ноги (spawn456-1, +14.0, лейн идентичен сертным; vanilla a1×456 для базы):
- items 0.00 (ins4 спит-armed) — недоступен (закрыт сертом)
- fluid 15.84% — ЗАКРЫТ каноном (gsel/fluid/mega не воскревать)
- **inside_volatile 16.75-17.9%** — (б) в меню, НО: единственный законный механизм = bulk-JNI
  без bitmask; bit-exact трансформация requires Rust-side blockstate-семантики (предикаты
  BlockState/StepBasedCollector) — прецедент InsideDietOps v2 DDA-транспис diverged 21/350k
  REJECTED; cmp435_inside3 углубление ВРЕДНОЕ ×2 (−16/−17.6, ROUND436_BOTTLENECK) —
  **NO-GO пивота в (б) в бюджете** (2-4ч+парити-риск > цикла).
- **broadphase 9.55-10.9%** — (а) R2 entity-query слой: остаток после eqsnap-плоскости
  (snapshotQuery 2.7% = собственная стоимость плейна, НЕ захватываемая) распадается на
  ChunkEntitySlices.getEntities 0.9% + EntityLookup.getHardCollidingEntities 0.7% +
  AABB.intersects 0.7% (пер-кандидатные фильтры) + Level.getEntitiesOfClass non-goal сайты —
  **движительно-коллизионный entity-query срез ~1.6-2.5% wall = ЦЕЛЬ ПИВОТА**.
- (в) pathfinder ~3.2% остаток — слаб (мандат: не брать без рисёрча) — отклонён.

### 2.2 Состав носителя cmp457_eqsnap2 (STRICT-OR композиция, закон 6)
Носитель = master 1838ae1d (cert-юнион ins4⊕senseins⊕chunk, серты +20.0/+21.9/+20.0)
⊕ **queryplane-подсистема ЦЕЛИКОМ** (cmp417_bq, TASK-412-B/417-C, DORMANT на мастере —
собственная дельта рычага):
1. `Level.getEntitiesOfClass(Class,AABB,Predicate)` whole-body redirect → QueryPlaneOps.
   getEntitiesOfClass: Player.class fast-path через authoritative level.players() (O(players)),
   все остальные классы = ванильная реплика тела бит-в-бит.
2. `Level.moonrise$getHardCollidingEntities(Entity,AABB,List,Predicate)` whole-body redirect →
   QueryPlaneOps.getHardCollidingEntities: empty-list fast path при HARD_ADDS.get()==0
   (монотонный probe-счётчик на ChunkEntitySlices.addEntity при moonrise$isHardColliding()==true —
   лодка/шалкер/армор-стенд в сцене ⇒ перманентный фолбэк в ваниль = парити-гвардия точная);
   двигательно-коллизионный срез Entity.collide 1.02% + Level.noCollision 0.92% (meganav-профиль).
⊕ eqsnap goal-query плейн (entity_query.rs, cmp453_diet-armed на серт-стеке; расширяется моим
флагом для EARLY-define/NCDFE-канона) ⊕ ПОЛНЫЙ cert-стек (fe7e7120-механика union-widen).

Закон 6: queryplane добавляет 0 JNI (fast paths чисто-java); eqsnap = ОДИН bulk-JNI/тик
(eqEpoch). Per-entity JNI отсутствует. Пустой флаг = ваниль бит-в-байт (все гейты STRICT).

### 2.3 NCDFE-канон ×456 (урок тика — ОБЯЗАТЕЛЬНО)
Рейс arm/define EntityGoalQueryOps @ MobPushOps.pushables:467. Перенос фикс-паттерна:
`cmp457_eqsnap2` добавлен в `entity_query::flag_enabled` (PURE production gate, x452-урок
no-mirror-drift) → EARLY define моста EntityGoalQueryOps (ensure_bridge_early → define_bridge_once
в раннем arm-хуке активационного воркера + mobs_manager publish-gate) ДО первого диспатча
pushables; mirror-drift иглы = вербатим-списки (x451/x452/x454 канон), одинаковые добавки на
rust-гейтах и java flag-lists СИНХРОННО. T1 NCDFE=0 до вердикта.

### 2.4 Прогноз и критерии (честно)
Δ-прогноз: захват 60-80% среза 1.6-2.5% = **+1.0-2.0пп** над семьёй cert-юниона (медиана ног
семьи ~+9-14, хвосты +16-19.7). Порог пары ≥+20 = leg ≥+18.0 — достижим только хвостовым
розыгрышем; реалистичный исход = GREEN-ноги суб-бара → BANK (ноги+пары в канон, вектор жив).
ВЕРДИКТ-ГЕЙТЫ: NCDFE=0 T1, ARM cmp457_eqsnap2 + EFFECT (player/hard fast-path first-hit),
AIOOBE=0, selfTest==true (QueryPlaneOps.selfTest + eqProbe), items 0.00, nav ~3.2, broad↓,
band 6.0-9.5M ре-ролл ≤2, dep-gate norm≥−2, threw=1 = REFUTED.
Пары: leg_norm − anchor_norm, Δ≤50k pair-fresh, банк ×456 15 шт (a1 +4.4@8935474 …
a34 +5.9@6520174; депресс-хвост a10 −2.0@6865200, a6×453 −0.8@7211960, a8×456 −0.7@6731200).

## 3. План имплементации
1. worktree round-457c-eqsnap2 от origin/master 1838ae1d ✓
2. union-widen cmp457_eqsnap2 (адаптация add_spawn_gates_455.py: rust Ok/Some/eq + java
   flag-lists + FLAG_LABEL) — ВСЕ гейты cert-стека + entity_query + queryplane; синхронно.
3. Блобы: пересборка ВСЕХ изменённых .java (javac --release 21, cp=kernel round-396-a +
   fastutil + paper-api + adventure — НЕ round-j2b), javap flat==nested, check_blobs_sync.
4. cargo check --lib + cargo test (CARGO_TARGET_DIR=<tmp>, target УДАЛИТЬ сразу).
5. RESULT.json write-through; диспатч leg-1 world-bench-parallel.yml (канон-инпуты,
   lever_flag=cmp457_eqsnap2, lever_arg=1); полл → абсорб → вердикт → пары; цикл закона 3.

## 4. ABSORB leg-1 eqsnap2-1 (run 36134758772, @9d71b461, window @6619122, TASK-457-C2 restart-absorb)

Доставка: run success, PARITY/LOW **+2.6 norm** @6619122, MSPT avg 399.41ms (TPS polls [19.2, 1.8, 2.1, 2.0, 2.6, 2.4]), GC 105 pauses (5 Full, avg 118ms), heap hi-water 8086MB. Wall-профиль вырожден (libc sleep 81.8% — TPS-голод фикстуры), лейны снимались по CPU self-time (канон агрегации BOTTLENECKS_3).

### 4.1 Маркеры ARM/EFFECT (все канон-гейты)
- NCDFE=0 (0× NoClassDefFoundError за ран); `entity_query: bridge EARLY define ok (anchor=NearestAttackableTargetGoal)` — EARLY-define канон ×456 держит, push-lane NCDFE window closed.
- `cmp412_b2p1: ARMED queryplane` (Level.getEntitiesOfClass + moonrise$getHardCollidingEntities whole-body redirects ⊕ ChunkEntitySlices.addEntity hard-probe; retransform rc Level=0 Slices=0) + `hook serve Level 98310 bytes (composed: getEntitiesOfClass sites:1, hardColliding sites:1)` + `PATCHED Slices.addEntity (22225→22381, moonrise$isHardColliding → isHardCollidingProbe + 2nop)`.
- EFFECT: `first gate hit: players fast path (0 candidates)` — player fast-path жив (O(4) players()), hard-colliding empty fast-path: HARD_ADDS=0 только на arm-времени; после популяции item_frame×2714 (hard-colliding) → probe>0 → перманентный ваниль-фолбэк (парити-гвардия точная = capture 0 на этом фикстуре).
- eqsnap-плейн: `ARMED soa=flat-arrays seqlock=global-version` + `epoch ok tick=9/10 mobSlots=274 players=4` + `sense epoch ok tick=94 senseSlots=38540` — bulk-JNI 1/тик жив.
- Latches/fail-dominant: 0. AIOOBE=1 = cmp420_chunk2 biomes selftest FAIL — pre-existing fail-closed green marker (канон 201d9d66), не lever.

### 4.2 Лейны были→стало (CPU self-time; «было» = серт-нога spawn456-1 §2.1)
| лейн | было | стало | capture |
|---|---|---|---|
| items | 0.00 (ins4 спит-armed) | 0.00 ваниль; serve4 1.9% own-cost — гвард держит | — |
| nav | ~3.2 остаток pathfinder | NavPlaneOps.handle 1.3% own-cost (серт-плейн) | — |
| eqsnap own | snapshotQuery 2.7% | 3.0% (serving senseSlots=38540) | — |
| broad: ChunkEntitySlices.getEntities | 0.9% | **0.9%** | **0** |
| broad: EntityLookup.getHardCollidingEntities | 0.7% | **0.7%** | **0** |
| broad: AABB.intersects | 0.7% | **0.9%** | **0** |
| broad: getEntitiesOfClass non-goal сайты | в срезе 1.6-2.5% | QueryPlaneOps body = 3 сэмпла (<0.01%) — сайтов в top-40 НЕТ | **0** |

### 4.3 Пары leg-1 ↔ якорный пул ×457 (Δ≤50k)
| якорь | norm@idx | Δ | пара |
|---|---|---|---|
| a12 | +5.0@6606571 | 2551 | **−2.4** |
| a32 | +0.7@6579335 | 39787 | **+1.9** |

Обе суб-бар. Монстр-нога ≥+18 — НЕТ. Пара ≥+20 — НЕТ.

### 4.4 ВЕРДИКТ: REFUTED_CENS + BANK (захвата НЕТ, leg-2 не диспатчена)
Ценз-гипотеза «движительно-коллизионный entity-query срез 1.6-2.5% capturable 60-80%» на фикстуре 150k REFUTED лейнами: broad-лейны НЕ упали (0.9/0.7/0.9 были→стало, capture=0/2.3%), +2.6 norm = window-шум, не эффект. Механизмы delivery-чистые, но плоскость на фикстуре пуста: player-запросы редки/пусты (0 candidates), hard-colliding fast-path парити-заблокирован 2714 item_frames (гвард работает как спроектирован), non-goal query-сайты <0.01%. Расширение плоскости (другие классы / targeting-слайс) захватило бы <0.6-1.3% — суб-шум, leg-2 экономически мертва (правило цикла). НОСИТЕЛЬ round-457c-eqsnap2 @9d71b461 остаётся на ветке — доставка валидна для будущих композиций: (а) H03 interval-tree targeting R3 (BLACKBOARD ID-H03: «ноги eqsnap2 + это → пара ≥+20»), (б) сценарии с реальным player-трафиком (players fast-path) и fixture без hard-colliding-массы (hard fast-path). Банк: 2 пары суб-бар (−2.4/+1.9) + чистая NCDFE-доставка в канон.
