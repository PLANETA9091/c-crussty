# RESEARCH-459-P46 — TASK-459-71 (WILD, закон 11, тик-459, c-crussty v18.2 финал)

**Идея ID-P46** (карточка: RESEARCH-458-P.md @ origin/round-458p-ideas): PathNavigation
tick-deadband (stuck-check батч) — стак/timeout-детекция `PathNavigation.doStuckDetection`
(раз/тик/моб) → батч: тот же navmath-мост решает stuck-флаги ВСЕХ навигирующих мобов ОДНИМ
JNI. Входы (карточка): delay-счётчики, node-дистанции. Lever `cmp459_p46` STRICT eq.

## 1. Механика (javap-транскрипция — канон nav_plane.rs, снята с живого ядра этого тика)

Источник: `/home/z/tools/patched-kernel.jar` (purpur 1.21.10, Mojang-mapped, классы 2025-12-11),
`javap -p -c net/minecraft/world/entity/ai/navigation/PathNavigation` (1417 строк, снято
TASK-459-71 2026-09-26; дополнение к L08: PathNavigation.tick = 185 байткодов).

Поля-состояние (javap Field-лист): `tick`, `lastStuckCheck:I`, `lastStuckCheckPos:Vec3`,
`timeoutCachedNode:Vec3i`, `timeoutTimer:J`, `lastTimeoutCheck:J`, `timeoutLimit:D`,
`maxDistanceToWaypoint:F`, `hasDelayedRecomputation:Z`, `timeLastRecompute:J`, `isStuck:Z`.
Константы компилятор-инлайном: MAX_TIME_RECOMPUTE=20 (`ldc2_w 20l` @recomputePath 12),
STUCK_CHECK_INTERVAL=100 (`bipush 100` @doStuckDetection 9), STUCK_THRESHOLD_DISTANCE_FACTOR=0.25f
(`ldc 0.25f` @doStuckDetection 57). GroundPathNavigation НЕ переопределяет tick/followThePath/
doStuckDetection (javap Field-лист) — один кодовый путь на всех ground-мобов.

### 1.1 PathNavigation.tick() (offsets 0..185)

```
++this.tick                                             // 0-9
if (hasDelayedRecomputation) recomputePath()            // 10-20
if (!isDone()) {
  if (canUpdatePath()) followThePath()                  // 28-39
  else if (path != null && !path.isDone()) {            // 42-56
    tmp = getTempMobPos(); n = path.getNextEntityPos(mob)
    if (tmp.y > n.y && !mob.onGround()                  // 76-97 dcmpl
        && Mth.floor(tmp.x) == Mth.floor(n.x)           // 98-112
        && Mth.floor(tmp.z) == Mth.floor(n.z)) path.advance()  // 115-136
  }
  if (!isDone()) {
    n = path.getNextEntityPos(mob)
    moveControl.setWantedPosition(n.x, getGroundY(n), n.z, speedModifier)  // 146-182
  }
}
```

### 1.2 followThePath() (offsets 0..194) — advance-гейт (node-дистанции, вход карточки #2)

```
tmp = getTempMobPos()
maxDistanceToWaypoint = bbWidth > 0.75f ? bbWidth / 2.0f : 0.75f - bbWidth / 2.0f  // 5-45 fcmpl
node = path.getNextNodePos()
d0 = Math.abs(mob.getX() - ((double)node.getX() + 0.5d))   // 56-76 i2d,ldc2_w 0.5d
d1 = Math.abs(mob.getY() - (double)node.getY())            // 77-93: БЕЗ +0.5 по Y!
d2 = Math.abs(mob.getZ() - ((double)node.getZ() + 0.5d))   // 95-115
if (d0 < (double)maxDistanceToWaypoint && d2 < (double)maxDistanceToWaypoint && d1 < 1.0d)
  advance()                                                 // 117-150: dcmpg ×3, dconst_1 по Y
else if (canCutCorner(path.getNextNode().type)
         && shouldTargetNextNodeInDirection(tmp)) advance() // 152-189
doStuckDetection(tmp)                                       // 189-194
```

### 1.3 doStuckDetection(Vec3) (offsets 0..257) — ЦЕЛЬ P46: два решения, раз/тик/моб

**(a) stuck-check** — уже deadbanded (STUCK_CHECK_INTERVAL=100), входы = delay-счётчик +
дистанция-дрейф:

```
if (tick - lastStuckCheck > 100) {                          // 0-11 isub,if_icmple 107
  f = getSpeed() > 1.0f ? getSpeed() : getSpeed()*getSpeed() // 14-51 fcmpl,iflt (NaN→else-ветвь)
  f1 = f * 100.0f * 0.25f                                    // 52-60 fmul×2 лево-ассоц
  if (tmp.distanceToSqr(lastStuckCheckPos) < (double)(f1*f1)) // 61-73 fmul,f2d,dcmpg
    { isStuck = true; stop(); }                              // 77-85
  else isStuck = false;                                      // 89-91
  lastStuckCheck = tick; lastStuckCheckPos = tmp             // 94-104
}
```
`Vec3.distanceToSqr`: `d0=x-x0; d1=y-y0; d2=z-z0; return (d0*d0 + d1*d1) + d2*d2` (лево-ассоц).

**(b) timeout-check** — БЕЗ 100-тикового гейта: выполняется КАЖДЫЙ тик КАЖДОГО навигирующего
моба (offsets 107-257) — это горячая часть плоскости (sqrt в distanceTo, Vec3i.equals, lsub/ladd):

```
if (path != null && !path.isDone()) {                       // 107-121
  node = path.getNextNodePos()
  g = level.getGameTime()                                   // 132-139
  if (node.equals(timeoutCachedNode))                       // 140-148 Vec3i.equals
    timeoutTimer += g - lastTimeoutCheck                    // 151-166 lsub,ladd
  else {
    timeoutCachedNode = node
    d = tmp.distanceTo(Vec3.atBottomCenterOf(node))         // 174-185: sqrt(sqr) node-дистанция
    timeoutLimit = getSpeed() > 0.0f ? d / (double)getSpeed() * 20.0d : 0.0d
                                                            // 187-219 fcmpl,ifle; ddiv,dmul лево-ассоц
  }
  if (timeoutLimit > 0.0d && (double)timeoutTimer > timeoutLimit * 3.0d)
    timeoutPath()                                           // 222-249: dcmpl ×2,ifle;
                                                            // resetStuckTimeout+stop
  lastTimeoutCheck = g                                      // 252-254 (пишется ВСЕГДА)
}
```

`recomputePath()` (offsets 0..71) — третий deadband (MAX_TIME_RECOMPUTE=20):
`if (gameTime - timeLastRecompute > 20) { if (targetPos != null) { path = createPath(...);
timeLastRecompute = g; hasDelayedRecomputation = false; } else hasDelayedRecomputation = true; }`
— v1 НЕ трогает (A*-createPath не батчится; это зона P44/следующих фаз).

### 1.4 Плоскость батча (дизайн v1)

Collect-проход ПОРЯДКОМ ИТЕРАЦИИ navigatingMobs (канон nav_plane/NavPlaneOps.handle): плоские
массивы → ОДИН bulk-JNI `navStuckBatch` → apply-проход stop()/resetStuckTimeout()/запись полей
в том же порядке. Оба решения (stuck + timeout) — чистая арифметика над delay-счётчиками и
node-дистанциями, БЕЗ allocation и БЕЗ block-access (входы уже собраны java-сторы) → бит-в-бит
транскрипция §1.3. Пустой флаг / чужой lever → java-путь ванили (хук не вызывается), NCDFE-канон:
класс определяется целиком в kernel loader, ZERO nested/lambdas, selfTest до ARM.

## 2. Сайты и web-источники (≥2, raw-феч проверен)

1. **Lithium (modrinth, raw-fetched 2026-09-26)** — карточный сорс: «general-purpose
   optimization mod … improves a number of systems (game physics, **mob AI**, block ticking …)
   with the goal of not changing any vanilla mechanics» — parity-канон направления. URL:
   https://modrinth.com/mod/lithium
2. **CaffeineMC/lithium `mixin/entity/inactive_navigations` (raw-fetched, ветка develop)** —
   ПРЯМОЙ прецедент tick-deadband навигации: PathNavigationMixin (recomputePath/moveTo/stop
   → регистрация active/inactive в мире) + ServerLevel$EntityCallbacksMixin/MobMixin —
   неактивные навигации исключаются из тик-обхода (deadband за счёт активности, порядок сета
   сохранён). URL: https://github.com/CaffeineMC/lithium
   (файлы: .../inactive_navigations/PathNavigationMixin.java, MobMixin.java,
   ServerLevel$EntityCallbacksMixin.java)
3. **NeoForge javadoc-зеркало PathNavigation (26.2.x)** — независимый листинг полей
   hasDelayedRecomputation/isStuck/lastStuckCheck (согласуется с §1; raw-феч упёрся в rate-limit
   зеркала, снапшот полей из поискового кэша). URL:
   https://aldak.netlify.app/javadoc/26.2.x/net/minecraft/world/entity/ai/navigation/pathnavigation

## 3. Parity-план (порядок = ваниль-очередь)

* **Обход = порядок итерации navigatingMobs** — collect/apply в одном порядке; решения не
  переупорядочиваются, side-effects (stop()/timeoutPath()) применяются в порядке итерации.
* **NaN/fcmpl-семантика транскрибируется как в JVM** (fcmpl speed>1.0: NaN → ветвь speed*speed;
  fcmpl speed>0.0: NaN → timeoutLimit=0.0; dcmpg stuck-сравнение: NaN → CLEAR; dcmpl
  timeout-гейт: NaN → skip) — канон fcmpl_f32/fcmpg_f32 navmath_flat.rs.
* **Состояние-входы без пере-вычисления**: tick/lastStuckCheck/timeoutTimer/lastTimeoutCheck —
  java-снапшот полей (delay-счётчики), node/pos-дистанции — снапшоты координат; ядро НЕ читает
  мир (zero block-access) → нет рангов между батчем и ванилью.
* **Запись состояния только через java-apply**: lastStuckCheck/lastStuckCheckPos/
  timeoutCachedNode/timeoutTimer/lastTimeoutCheck/timeoutLimit/isStuck пишутся java-стороной
  в порядке collect (натив отдаёт out-флаги + новые значения счётчиков).
* **advance-гейт followThePath НЕ в v1**: он требует getNextNode().type/canCutCorner/
  shouldTargetNextNodeInDirection (canMoveDirectly — block-access) → advance остаётся ванильным;
  v1 = ТОЛЬКО doStuckDetection (a)+(b) — решения, достижимые из чистых входов (супerset-граница
  фиксируется, widen-2 — следующий лаб-лег).
* Латч-безопасность: lever-флаг STRICT eq `cmp459_p46`; ERR-ladder (ERR_STRUCT=-1, ERR_RANGE=-2,
  канон mobs_soa/navmath_flat) → one-shot disarm → decideJava-фолбэк бит-в-бит.

## 4. Δ-прогноз и capture-матем (числом)

* Лейн nav_ai: 9.8-14% wall компо-сцены (BOTTLENECK-405/406, канон nav_plane.rs); хвост
  PathNavigation.tick внутри лейна — doStuckDetection ≈ 257 байткодов, причём timeout-часть
  (107..257) выполняется каждый тик каждого навигирующего моба.
* Прогноз карточки: **+0.3-0.5пп** (меньше P44: move-плоскость шире stuck-плоскости).
  P46-оценка от лейна: батч амортизирует per-mob интерпретацию 2 решений в 1 JNI; при доле
  doStuckDetection ≈ 3-5% от nav_ai-лейна и захвате 60-80% арифметики ядра → 9.8-14% × 4% × 65%
  ≈ **+0.25-0.4пп** (монотонно, compo-зависимый) — согласуется с карточкой; prereg: **Δ
  +0.3-0.5пп**, refute-гейт G5 (эффект < 0.1пп при >30% навигирующих мобов → REFUTED_CENS).
* VERDICT-NUM скаффолда: WIRED-файлы + LOC ниже (§7), CI ниже.

## 5. Риски

1. **advance-гейт не в v1** (см. §3): супerset-граница уже шире «только stuck» — timeout-часть
   входит; риск неполного захвата — честно в prereg Δ. Widen-2: canCutCorner/shouldTarget-решение
   (block-access precollect как shapeMaxY в P44).
2. **getSpeed() зависит от внешних мутаций между collect и apply** (speed-баффы в том же тике):
   ваниль читает getSpeed() внутри решения; батч читает снапшот. Окно = 1 collect→apply сегмент
   тика, порядок итерации тот же; скорость мутируется только moveControl-сайтом тика же моба —
   риск приравнивается к канону P44 (speedModifier precollect). Гейт G4 (lockstep-харнес).
3. **stop() сайд-эффекты** (path=null, isStuck, onRemoved-события): apply вызывает ванильный
   stop() как есть — не реплицируется нативом (канон: «java применяет решения»).
4. **NCDFE-канон**: мост определяется ЦЕЛИКОМ в kernel loader, ZERO nested/lambdas, selfTest()Z
   до ARM, EARLY-define retry ×10 (уроки x452/entity_query cv3-1). Скаффолд-коммит класс нигде
   не компилируется и не определяется — ваниль бит-в-байт по построению.
5. **Гонка с nav_plane/P45 на navigatingMobs-обходе**: STRICT-OR disjoint по lever-флагам —
   на cmp459_p46 хук принадлежит ТОЛЬКО nav_deadband (nav_plane dormant на этом флаге).
6. **/tmp-инцидент этого тика**: worktree снесён конкурентным процессом до коммита (потеря
   скаффолда, восстановлен по памяти сессии) — урок commit-early: все артефакты коммитятся
   немедленно после записи (добавлено в гейты G1-протокола).

## 6. Preregistered гейты (закон 16)

* G1 ARM: stdout «[crussty-plugin] cmp459_p46: ARMED nav tick-deadband stuck batch».
* G2 эффект: счётчики batches/stuckStops/timeoutStops > 0 при навигирующих мобах.
* G3 order-parity: apply в порядке collect (лог сэмплов mob-id), бит-в-бит против ваниль-трассы.
* G4 lockstep: оракул-харнес doStuckDetection-транскрипции vs живая JVM (fcmpl-края: speed=NaN,
  1.0, 0.0; lastStuckCheck краевые tick-lastStuckCheck=100/101; timeoutTimer переполнение ladd).
* G5 refute: Δ < 0.1пп при >30% навигирующих мобов в сцене → REFUTED_CENS (числа).
* G6 NCDFE: T1=0, flat==nested после пересборки блобов (javap-гейт build-скрипта).

## 7. Скаффолд (этот коммит)

* `src/nav_deadband.rs` — чистое std-ядро транскрипции §1.3 (канон скаффолда P44
  navmath_flat.rs: без jvmti/include_bytes; RegisterNatives/include_bytes/ретаргет — v1
  wiring-коммит). STRICT eq lever `cmp459_p46`, ERR-ladder, fcmpl/dcmpg/dcmpl NaN-семантика,
  юнит-тесты stuck/timeout/deadband-краёв.
* `navmath/net/minecraft/world/entity/NavDeadbandOps.java` — СТАБ моста (javap-контракт §1.3,
  native navStuckBatch сигнатура, decideJava-скелет, batchOk-латч, fcmpl-хелпер; FQCN не
  клэшится с P44 MoveOps — отдельный класс в том же navmath-пакете).
* `src/lib.rs` — wiring mod (dormant до v1).

## SOURCES
https://modrinth.com/mod/lithium https://github.com/CaffeineMC/lithium https://aldak.netlify.app/javadoc/26.2.x/net/minecraft/world/entity/ai/navigation/pathnavigation
