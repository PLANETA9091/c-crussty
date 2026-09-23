# RESEARCH-A-436 — PLAN-Б: запасной kernel-leaf на случай закрытия PALETTED-DEMUX/wgen3

Агент: TASK-436-A (tick-436, Job 406609, trace cron-agent-loop-202609240647).
Ветка: round-436-a-wgen4 @062baea(+gates). Вход: работа tick-434/435 (worklog),
RESULT-a.json ×435, BOTTLENECK-435, CLAIMS TASK-436, RESEARCH-G (C, ×435),
RESEARCH-A-k3/k4, entity-query-2026-09-18/DESIGN.md (S7-140), профили
BOTTLENECKS_3.md раундов 433/435 + ABSORB lane-данные ×434/435.

## 0. Контекст (почему план-Б)

wgen3 (cmp434_wgen3 = wgen set ⊕ PALETTED-DEMUX на базе ccefix 513c4835 @062baea):
×434 холодное окно (свой якорь −18.2, депресс-гейт, вердикта нет), ×435 ноги
−8.1/−10.6 norm, пара a-w3-1↔p3 = −12.8%. Два окна подряд негатив/нуль.
Мандат: финальная попытка в ЗОЛОТОМ окне (02:2x-03:3x +08, слот тика 02:08) —
диспетч banked (dispatch_436a.py, 2 якоря @master + 3 ноги w4 @round-436-a-wgen4).
Если снова ≤0 — вектор закрывается (CLAIMS TASK-436 п.3), слот волны A
освобождается под новую подсистему закона 6. Этот документ = план-Б того слота.

Профильная правда PALETTED-DEMUX (legs w3-1/w3-2 vs anchor-a, одно окно ×435):
lane paletted 7.06% → 5.76% (−1.3пп: fast-path get 3.0% + Ops.get slow 2.2% vs
vanilla get 4.5% + readPalette 1.0% + SimpleBitStorage.get 1.5%). Демукс работает
как спроектирован (~+1.3пп), но НЕ является источником негатива; негатив несёт
стоимость полного стека wgen-плоскости в сдутых окнах (EntityGoalQueryOps.snapshotQuery
7.5% self — собственная цена goalquery-плоскости; nav_ai lane −8.2пп, items −31пп =
сдвиг пирога, не потери). RESEARCH-G §2: «DEMUX REFUTED-BY-ECONOMICS соло
(§148-149), субстрат верифицирован для combo» — консистентно: демукс живёт только
в юнионе с wgen-плоскостью.

## 1. Листы на НОСИТЕЛЕ (master e05994c = композит mobsoa⊕inside, pair-cert +27.3)

Свежий якорный профиль (round-435a-anchor-a, master, 115196 сэмплов, окно ×435)
+ lane-карта композит-носителя (ROUND-435 BOTTLENECK: items 0.00 / nav_ai 3.78 /
broadphase 10.21 / inside_volatile 15.91). Атакуемые JVM-Java листы по убыванию:

| # | лист | self | статус атаки |
|---|---|---|---|
| 1 | PalettedContainer.get (+readPalette+SimpleBitStorage) | ~7.0% | **ТЕКУЩИЙ ВЕКТОР** (PALETTED-DEMUX в wgen3, золотой шот) |
| 2 | ChunkEntitySlices.getEntities + $EntityCollectionBySection.getEntities | 3.9% | S7-140 стена для median-exact; K3-pivot (goalquery) уже в wgen set |
| 3 | Entity.updateFluidHeightAndDoFluidPushing (+getFluidState 0.9) | 4.0% | закон-5 мертвец (fluid_bitmask REFUTED, fluid_dirty REFUTED) |
| 4 | SynchedEntityData.getValue+getItem (+VarHandle 1.3) | 2.8%+ | S7-140 §5 ЗАКРЫТ (volatile неизбежен) |
| 5 | AABB.intersects 2.1 + CollisionUtil.getCollisions... 1.3 | 3.4% | S7-140 §4 закрыт (микро ≤0.6 поверх DEMUX) |
| 6 | inside_volatile ЛЕЙН (checkInsideBlocks subtree) | 15.9% | ЖИВАЯ inside2-плоскость (B-линия, cmp432_inside2/cmp435_inside3) — ГЛАВНЫЙ кандидат лесенки, не слот A |
| 7 | внутри-наши Ops-цены (InsideBlockOps.gate 1.1, InsideSnapOps.serve, snapshotQuery 7.5 под флагом) | ~3-9% | диета собственных плоскостей (см. §3 зафикс-кандидат) |

Вывод: после закрытия wgen3 ни один «свежий» крупный JVM-Java лист не имеет
легальной атаки вне стен S7-140/закон-5. Два владельческих кандидата (CLAIMS
п.49: «despawn+spawn сканы или entity-query слой») — это плоскости ПОВЕРХ
проверенного SoA-субстрата, а не сырые листы: их адресуемая база — оставшаяся
колоночная работа моб-цикла (despawn/spawn сканы) и порядок-независимые
query-сайты. Это законный закон-6 путь: подсистема целиком на живом субстрате.

## 2. ПЛАН-Б МАТРИЦА (лист → подсистема → STRICT-OR флаг)

### Б1 (ПЕРВИЧНЫЙ): SSCAN2 — despawn+spawn scan plane, закон-6 подсистема целиком
- Лист: моб-цикл сканы (per-tick despawn-check по всем мобам + spawn-плотностные
  сканы), разбросаны по entities/mobs kernel (23-35% бакет) и phase mob spawning;
  адресуемая база на носителе оценивается по lane-методике ABSORB (nav_ai остаток
  3.78% + despawn-части items/mobs-бакета).
- Субстрат: mobs_soa SoA-плоскость (пронесена мержем ×433, pair-cert +27.3) —
  DOD-колонки + per-tick bulk-JNI прецедент.
- Подсистема ЦЕЛИКОМ (закон 6): mobs_sscan.rs MobScanOps УЖЕ ПОЛНА и дормантна
  (sscanProbe/sscanEpoch natives, lib.rs:597 activation, блобы собраны и
  ALL IN SYNC: sscan/build/MobScanOps.class 6177B) — legacy STRICT eq
  «cmp406_sscan». План: (а) STRICT-OR член `cmp436_sscan2` в гейты
  mobs_sscan.rs + mobs_manager.rs + mobs_grid.rs (+ natives-needles), (б)
  run_world3.sh arming case, (в) расширение sscanEpoch до spawn-полуплоскости
  (spawn-плотностные колонки: per-category caps, mo§-окрестности — единый
  bulk-JNI, ноль per-call JNI), (г) блобы build_430b + check_blobs_sync,
  (д) cargo test needle.
- Флаг: `cmp436_sscan2` (STRICT-OR, пустой/чужой = ваниль, закон 4).
- Прегист: ARM + ЭФФЕКТ (Retargeted>0, sscanEpoch serving) + DATA-PLAN
  (sscan-колонки>0) + AIOOBE=0 + NCDFE=0 + selfTest + band + pair Δ≤50k +
  здоровое/золотое окно. Бенч свой: 2-3 ноги + 2-3 якоря интерлив.
- Риск: legacy-плоскость никогда не сертифицирована живьём (эра 406 дормант) —
  поэтому Б1 идёт ПЕРВЫМ в своём окне с якорями, Δ<+20% → цикл закона 3
  (новый рисёрч → доработка → бенч).

### Б2 (ВТОРИЧНЫЙ, условный): EQW — entity-query слой K4 (widening)
- Лист: ChunkEntitySlices.getEntities семейство 3.9% + периодические goal-query
  сайты вне текущих 2 ретаргетов (LookAt/Avoid/nearest классы).
- Подсистема: EntityGoalQueryOps ретаргет-лист расширение на
  порядок-НЕзависимые потребители (nearest-pick класс, документированный delta
  класс banked); S7-140 стену НЕ нарушаем (push-порядок/item-merge herd
  исключены).
- Флаг: `cmp436_eqw` (STRICT-OR в entity_query.rs + EntityGoalQueryOps gate).
- ПРЕДУСЛОВИЕ: декомпозиция золотого вердикта wgen3 — если шот провалится из-за
  собственной цены snapshotQuery (7.5% self на w3-ногах), сначала ДИЕТА плоскости
  (sampled-счётчики/колонки per B-урок V3 «per-thread sampled bump»), затем
  widening. Без диеты widening умножает ту же цену.

### Б3 (ПОСТОЯННЫЙ): A-слот = якорное покрытие живых линий
- Лесенка проекта: inside_volatile 15.9% (B-линия inside2-full-plane) —
  главный кандидат следующего мержа; A-слот держит 2-3 якоря @master в каждом
  золотом батче (pair-лотерея: канон big-interleaved, P(пара) ~3-6%/якорь-нога).
- Плюс chunkpl (C-линия, +22.1/+17.7 norm, сильнейшая живая без пары) —
  якорная поддержка 6.60-6.75M зоны (CLAIMS TASK-436).

### НЕ-кандидаты (честно, анти-плацебо)
- DATA-PLAN/SynchedEntityData — закрыт S7-140 §5 (volatile неизбежен).
- fluid-семейство — закон-5 мертвецы (fluid_bitmask/fluid_dirty/fluid_free).
- broadphase/AABB/коллизии — закрыты S7-140 §4 (микро поверх DEMUX).
- chunk-parse новые стадии — RESEARCH-G §2/§3: профиль-dead/рефьютед/плацебо;
  подсистема завершена на гранулярности закона 6.
- С4/CDS template persistence (RESEARCH-G §5a) — закон-8 ось (boot-Done),
  НЕ TPS-MSPT мандат; парк на chunk-ось.

## 3. Зафикс-кандидат PALETTED-DEMUX (документирован, НЕ вошёл в золотой шот)

Write-path guard (patch-tool + rust-зеркало classfile.rs, двойная имплементация):
(а) prologue: инлайн `if (crusstySnapGen != 0) Ops.onMutateStart(this)` вместо
безусловного static-call (семантика бит-в-бит, минус static-call для каждого
write по снапшот-меньшим контейнерам); (б) blacklist-skip: при
`crusstyEpoch >= MAX_BUILDS_PER_CONTAINER(=2)` пропустить пару volatile gen++
(prologue+epilogue) — write-hot секции (жидкости/апдейты) перестают платить
2 volatile write за мутацию навсегда после бана; корректность: snapGen==0
навсегда → gate никогда не проходит → slow path.
НЕ поехал в золотой шот: (1) требует синхронного изменения зеркала
src/classfile.rs (закон 6 подсистема целиком) + парити-харнес ре-ран
(asm-джарья найдены, kernel round-396-a на месте; banks libraries dir purged —
восстанавливаемо), (2) загрязнил бы вердикт последней попытки wgen3 (если шот
провалится — демукс или фикс?). Кандидат на следующий цикл ПОСЛЕ золотого
вердикта (если вектор жив) или в Б1-цикл (если закрыт).

## 4. Дисциплина цикла (закон 3) для Б-слота

Б1: имплементация подсистемы целиком → свой бенч (2-3 ноги + 2-3 якоря,
big-interleaved, золотое/здоровое окно) → Δ<+20% → НОВЫЙ рисёрч (декомпозиция
ABSORB-лейнов Б1-ног) → доработка → снова бенч — до ≥+20% pair-stable или
исчерпания бюджетной команды. Каждый шаг commit+push + RESULT.json write-through.
