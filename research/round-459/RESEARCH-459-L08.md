# RESEARCH-459-L08 — navmath javap-транскрипция (ID-P44: MoveControl.tick + Navigation)
Лаб-агент тика 459, v18.3 (законы 13-16). Kernel jar канон: `research/gc-recon-2026-09-19/round-396-a/patched-kernel.jar` (29 386 794 B), javap=/home/z/tools/jdk-21.0.12.1+1/bin/javap. Канон паттерна: `src/nav_plane.rs` + `entityinside/.../NavPlaneOps.java` (cmp405_navplane, TASK-405-A).

## 1. Декомпозиция профилей (числа сэмплов, лист-ранжирование)
| профиль | TOTAL | срез | числа |
|---|---|---|---|
| chkmono457-14 (монстр-нога, CPU) | 103062 | nav-листья | EntityGoalQueryOps.snapshotQuery 2754=2.7%; NavPlaneOps.handle 826=0.8%; Brain.tick 559=0.54%; Brain.tickEachRunningBehavior 101=0.10%; MoveControl.tick CPU-self <639 (ниже top-40 cutoff) = **<0.6%** |
| chkmono457-14 (wall, 63661) | 63661 | nav inclusive | Mob.serverAiStep 294=0.46%; PathNavigation.* 240=0.38%; GoalSelector.tick 101=0.16%; MoveControl.tick 1=0.002%; purpur.WASD 4=0.006% |
| chkmono457-16 (депресс, wall 63662) | 63662 | nav inclusive | PathNavigation.* 348=0.55%; Mob.serverAiStep 282=0.44%; GoalSelector.tick 95=0.15%; MoveControl.tick 1; snapshotQuery 3128=3.0%; NavPlaneOps.handle 1471=1.4% (CPU) |
| round-anchor458-33 (якорь, CPU 115503) | 115503 | nav-листья | GoalSelector.tick 796=0.7%; Brain.tick 871=0.75%; tickEachRunningBehavior 189=0.16%; Sensing.tick 726=0.6%; Mth.floor 1082=0.9% (нав-сосед) |
| round-anchor458-33 (wall 61256) | 61256 | nav inclusive | PathNavigation.* 872=1.42%; Mob.serverAiStep 495=0.81%; GoalSelector.tick 484=0.79%; **MoveControl.tick 12=0.02%**; MoveControllerWASD.tick 16; DrownedMoveControl 6; GuardianMoveControl 2; SmoothSwimmingMoveControl 1 |
| chkmono457-11 (депресс) | CPU | nav | snapshotQuery 3246=3.1%; NavPlaneOps.handle 1327=1.3% |

Ключевой факт 1: nav_ai-остаток 2.75-3.2% (канон LEDGER) подтверждён: snapshotQuery 2.7/3.0/3.1% (три профиля) + Brain 0.54-0.75% + GoalSelector 0.7-0.79%; НО **MoveControl.tick сам по себе <0.6% CPU** — стоимость не в арифметике тика, а в плотности вызовов (150k мобов × 20 тик/с) и в getBlockState/getCollisionShape-хвосте. Поэтому bulk-JNI должен бить на ПЛОСКОСТЬ (батч мобов → 1 вызов), а не на микро-арифметику одного моба.

Ключевой факт 2 (риск-находка): в ядре Purpur-обёртка — `org/purpurmc/purpur/controller/MoveControllerWASD.tick` (javap: rider==null||!isControllable → `vanillaTick()` → `invokespecial MoveControl.tick()`) — ретаргет тела MoveControl.tick покрывает и обёртку автоматом; НО `Drowned$DrownedMoveControl.vanillaTick`/`Guardian$GuardianMoveControl.vanillaTick`/`SmoothSwimmingMoveControl` перегружают tick() БЕЗ super-вызова → остаются ванилью (fail-safe по построению, паритет не страдает).

## 2. javap-контракты (4 ≥ 2, канон round-396-a jar)
### 2.1 MoveControl.tick (585 байткодов, 3 ветки)
- STRAFE (offset 10-195): `speed = (float)(speedModifier d2f) * attr(MOVEMENT_SPEED)` (d2f@20, fmul@28); `norm = Mth.sqrt(ff*ff + rr*rr)` (fmul,fadd,Mth.sqrt@150 — sqrt = f2d→Math.sqrt→d2f, IEEE-точен); clamp `norm<1.0f→1.0` (fcmpg@158); `fdiv speed/norm` @68; sin/cos поворота: `Mth.sin(getYRot()*0.017453292f)` (fmul@92, ldc 0.017453292f=(float)(π/180)); `f8=strafeForwards*cos − strafeRight*sin`, `f9=strafeRight*cos + strafeForwards*sin` (fsub@122, fadd@134); `isWalkable(f8,f9)` false → strafeForwards=1, strafeRight=0; setSpeed/setZza/setXxa; op=WAIT.
- MOVE_TO (198-504): `d=wantedX−getX()`, `d0=wantedZ−getZ()`, `d1=wantedY−getY()` (dsub@226/239/252); `d2=(d*d + d1*d1) + d0*d0` ЛЕВО-АССОЦ (dmul/dadd@255-267); гейт-ареста: `d2 < 2.500000277905201E-7d` (ldc2_w@272 — вербатим-константа ядра) → setZza(0)+return; **yRot-математика**: `(float)(Mth.atan2(d0, d) * 180.0 / 3.1415927410125732d) − 90.0f` (dmul@296, ddiv@300, d2f@301, fsub@304; **π = (double)(float)Math.PI = 3.1415927410125732** — канонический MC-артефакт f32-π в f64-делителе!); `setYRot(rotlerp(getYRot(), f, 90.0f))`; `setSpeed((float)(speedModifier * attr))` (dmul@347, d2f@348); jump-решение: `d1 > (double)maxUpStep` (f2d@400, dcmpl@401) && `(d*d + d0*d0) < (double)Math.max(1.0f, getBbWidth())` (fconst_1@412, fmax@420, f2d@423, dcmpg@424) && `mob.getY() < shape.max(Y) + (double)blockPos.getY()` (dadd@457) && `!state.is(DOORS) && !state.is(FENCES)` → jump(); op=JUMPING.
- JUMPING (504-577): setSpeed; `onGround || (isInLiquid && isAffectedByFluids)` → op=WAIT (ядерная правка 1.21.x).
- WAIT/default (577-585): setZza(0).
- rotlerp(f,f,90f): `wrapDegrees(b−a)` clamp ±90, wrap ±360 (fcmpl@12, fcmpg@23, ldc 360.0f).

### 2.2 PathNavigation.tick (185 байткодов)
`tick++` (iinc-паттерн iadd/putfield@0-7); `hasDelayedRecomputation → recomputePath()`; `isDone → return`; `canUpdatePath → followThePath()` ИНАЧЕ over-the-top-прогон: `tempMobPos.y > nextEntityPos.y (dcmpl@84) && !onGround && floor(x)==floor(x₂) && floor(z)==floor(z₂)` → `path.advance()`; хвост: `setWantedPosition(vec.x, getGroundY(vec), vec.z, speedModifier)`. getGroundY: `isAir(below) ? vec.y : WalkNodeEvaluator.getFloorLevel(level,pos)`. followThePath: `maxDistanceToWaypoint = bbWidth>0.75f ? bbWidth/2 : 0.75f − bbWidth/2` (fcmpl@16, fdiv@28, fsub@44); `d0=|getX()−(node.x i2d + 0.5)|` (Math.abs@77), d1=|getY()−node.y|, d2=|getZ()−(node.z+0.5)|; arrive: `d0 < (double)maxDist(f2d@122) && d2 < (double)maxDist && d1 < 1.0 (dconst_1@140)`; cut-corner: `canCutCorner(next.type) && shouldTargetNextNodeInDirection` → advance; doStuckDetection.

### 2.3 PathFinder.findPath (private, 726 байткодов, A*)
`runLimit = (int)((float)maxVisitedNodes * rangeScale)` (i2f@112, fmul@115, f2i@117 — f32-округление лимита!); pop→closed=true; reach-гейт: `node.distanceManhattan(target) <= (float)followRange (i2f@209, fcmpg@210)`; skip-гейт: `node.distanceTo(best) >= followRange (fcmpl@270)`; соседний цикл: `e=distance(node,nb)`; `nb.walkedDistance = node.walkedDistance + e` (fadd@327); `f = (node.g + e) + nb.costMalus` (fadd@338+344, лево-ассоц); гейты `nb.walkedDistance < followRange (fcmpg@353)` и `f < nb.g (fcmpg@372)`; `nb.h = getBestH(nb,targets) * 1.5f` (fmul@401); `f = nb.g + nb.h` (fadd@429/448); BinaryHeap.changeCost/insert. getBestH: min по targets из Node.distanceTo, старт 3.4028235E38f. distance = Node.distanceTo: `(f*f + g*g) + h*h` → `Mth.sqrt` в f32, где `f=i2f(dx)` (isub→i2f@9, fmul/fadd@34-41).

### 2.4 Mth (математическое ядро — ТРАПЫ бит-в-байт)
- `Mth.sin(x) = SIN[(int)(x*10430.378f) & 0xFFFF]`, `Mth.cos(x) = SIN[(int)(x*10430.378f + 16384.0f) & 0xFFFF]` — ТАБЛИЦА 65536 f32, НЕ IEEE-sin! Заполнение: `SIN[i]=(float)Math.sin(i2d(i)*3.141592653589793d*2.0d/65536.0d)` (lambda$static$0: i2d@11, dmul@15/19, ddiv@23, Math.sin@24, d2f@27).
- `Mth.atan2(y,x)` = кастомный fast-atan2, НЕ Math.atan2: `z=y*y+x*x`; NaN→NaN; флаги x<0/y<0 (dcmpg), swap если x>y (dcmpl@65); `d9=fastInvSqrt(z)`; `y*=d9; x*=d9`; `d11=FRAC_BIAS+x`; `i13=(int)doubleToRawLongBits(d11)` (l2i@118); `d14=ASIN_TAB[i13]; d16=COS_TAB[i13]`; `d20 = x*d16 − y*(d11−FRAC_BIAS)` (dmul/dsub@145-153); `d22 = ((6.0d + d20*d20)*d20)*0.16666666666666666d`; `d24=ASIN_TAB+d22`; коррекции: swap→`π/2−d24` (1.5707963267948966), y<0→`π−d24`, x<0→`−d24`.
- `Mth.fastInvSqrt(x)`: `d0=0.5*x`; `i=6910469410427058090L − (doubleToRawLongBits(x) >> 1)` (ldc2_w@12, lshr@18, lsub@19 — магия вербатим из jar); `x=longBitsToDouble(i)`; `return x * (1.5 − (d0*x)*x)` (одна Newton-итерация; ассоциативность dsub: 1.5−((d0*x)*x), затем x-дmul).
- `FRAC_BIAS = Double.longBitsToDouble(4805340802404319232L)` = 0.125; `ASIN_TAB[257]: ASIN_TAB[i]=Math.asin(i/256.0d); COS_TAB[i]=Math.cos(ASIN_TAB[i])` (static{} @2354-3183: i2d, ddiv 256.0d, Math.asin@300, Math.cos@309).
- `Mth.sqrt(f) = (float)Math.sqrt((double)f)` — IEEE-точен (f2d@1, d2f@5).

## 3. IEEE754-транскрипционная таблица (канон nav_plane: javap → Rust бит-в-байт)
| # | javap-инструкция | Java/канон | Rust-эквивалент (бит-в-байт) |
|---|---|---|---|
| T1 | `dsub/dmul/dadd` (d*d+d1*d1)+d0*d0 | лево-ассоц, без FMA | `((d*d + d1*d1) + d0*d0)` f64, НЕ `f.mul_add` (FMA меняет ULP) |
| T2 | `ldc2_w 2.500000277905201E-7` | f64-константа | `const MOVE_EPS: f64 = 2.500000277905201e-7;` |
| T3 | `d2f / f2d / i2d / i2f / f2i / l2i` | усечение (round-toward-zero для →int) | `as f32`/`as f64`/`as i32`/`as i64` (Rust `as` = то же усечение; `f64 as i32` — NaN→0, сатурация — совпадает с j2i) |
| T4 | `Mth.atan2(d0,d)*180.0/3.1415927410125732` | f64 mul→div→d2f→fsub 90.0f | `((mth_atan2(z, d) * 180.0) / 3.1415927410125732f64) as f32 − 90.0f32`; π = `f64::from(f32::from_bits((std::f64::consts::PI as f32).to_bits()))` → 3.1415927410125732 |
| T5 | `Mth.atan2` | fast-таблица | полный порт §2.4: fast_inv_sqrt + ASIN_TAB/COS_TAB (генерация T7) + флаги/swap; НЕ вызывать f64::atan2 (другой ULP) |
| T6 | `fastInvSqrt` магия | `6910469410427058090L − (bits>>1)` | `f64::from_bits(6910469410427058090u64.wrapping_sub((x.to_bits() >> 1)))`; шаг: `x*(1.5−(d0*x)*x)` |
| T7 | таблицы atan2 | `ASIN[i]=asin(i/256.0); COS[i]=cos(ASIN[i]); FRAC_BIAS=f64::from_bits(4805340802404319232)` | ленивая статика `OnceLock<[f64;257]>` (Math.asin/cos = IEEE-транзитивны на x86-64 JDK21: libfdlibm StrictMath-equivalent; ОРАКУЛ-тест обязателен) |
| T8 | `Mth.sin/cos` | SIN[65536] | `SIN[(idx & 0xFFFF) as usize]` с `idx=(int)(x*10430.378f)` = `(x*10430.378f32) as i32`; cos: `+16384.0f32` ДО `as i32`; таблица = `for i: (i as f64*PI*2.0/65536.0).sin() as f32` (f64-sin→d2f) |
| T9 | `fcmpg/fcmpl/dcmpg/dcmpl` | NaN-семантика различает g/l | Rust `x < y` / `x > y` на f64/f32 идентичны (NaN→false); branch-порядок транскрибировать вербатим по javap |
| T10 | `Mth.sqrt(F)` | f2d→sqrt→d2f | `(x as f64).sqrt() as f32` (не f32::sqrt) |
| T11 | `rotlerp` wrapDegrees | fcmpl/fcmpg клампы | `let i=wrap_deg(b−a); i=min(i,90.0f32); i=max(i,−90.0f32); let j=i+a; if j<0 {j+360} else if j>360 {j−360} else {j}` — каждый fcmp по javap |
| T12 | followThePath 0.75f | `bbWidth>0.75f ? bbWidth/2f : 0.75f−bbWidth/2f` | f32-математика verbatim, `as f64` на сравнении |
| T13 | i2d+0.5 dadd (узлы) | `node.x as f64 + 0.5` | `f64::from(node.x) + 0.5` |
| T14 | PathFinder runLimit | `(int)((float)maxVisitedNodes * rangeScale)` | `((maxVisited as f32) * range_scale) as i32` — f32-произведение ДО усечения |

## 4. Bulk-JNI дизайн решения тика одним вызовом (паттерн navDecide + ERR-ladder)
Класс `net/minecraft/server/level/MovePlaneOps` (клон дисциплины NavPlaneOps.handle, канон бит-в-байт):
1. **Коллект-пасс** (per-tick батч по бакету RegionTickOps): для каждого моба с `mob.getMoveControl()!=null` прочитать Unsafe-офсетами: `operation` (0=WAIT,1=STRAFE,2=MOVE_TO,3=JUMPING → meta flags), `wantedX/Y/Z` (f64×3), `mob xyz` (f64×3), `yRot` (f32-биты в i32), `speedModifier` (f64), `attr(MOVEMENT_SPEED)` (f64), `maxUpStep` (f32-биты), `bbWidth` (f32-биты), shape-гейт (java-сайд: `shape.isEmpty()`, `shape.max(Y)+posY`, `is(DOORS)`, `is(FENCES)` → pre-computed jflag i32: bit0 isEmpty, bit1 above, bit2 door, bit3 fence). Плоские массивы: meta[i*8..], double xyz[6n], misc[3n]. NO per-entity JNI (урок law 6: per-entity = design error).
2. **ОДИН вызов** `moveDecide(n, meta, wanted, mobxyz, misc, out)` `(I[I[D[D[I)I`: Rust ядро транскрибирует ветки STRAFE/MOVE_TO/JUMPING §2.1 бит-в-байт (таблица §3), out[i*3]=yRot-биты, out-скорость-биты, jump-flag/op-next.
3. **Apply-пасс в коллект-порядке**: java применяет `setYRot(Float.intBitsToFloat)`, `setSpeed`, `jump()`, op-запись — ванильные мутаторы сохранены (parity: syncher/трекер не обходим), порядок = порядок итерации бакета (как ванильный MoveControl.tick-порядок; мобы независимы — перекрёстной связности нет, jump() пишет только флаг jump-поля моба).
4. **ERR-ladder (fail-closed)**: rc<0 (ERR_STRUCT=-1/ERR_RANGE=-2) или Throwable → one-shot latch `batchOk=false` → java-реплика `decideJava` (та же математика §2.1 verbatim) для ВСЕГО вызова; поведение идентично, отличается исполнитель. Хвост getBlockState/getCollisionShape остаётся в java-коллекте (мир не читаем из натива — lockstep-безопасность).
5. **Спящий-гейт-профилактика (урок-408 ×2)**: пустой lever-флаг = класс не дефайнится + ретаргет не компонуется (region_threads compose) → ваниль бит-в-байт; после вайринга javap flat==nested 10/10 EQUAL + strings-проверка блоба на маркер.
6. Точка ретаргета — тело `MoveControl.tick()` (585 байткодов): покрывает Purpur-WASD через `vanillaTick()→invokespecial` (§1 факт 2); подклассы Drowned/Guardian/SmoothSwimming остаются ванилью — допустимо, capture их доли 9/61k wall ≈ 0.015%.
7. Extension-lane (P44b, отдельная нога): `PathNavigation.tick`-хвост `setWantedPosition(nextEntityPos.x, getGroundY, z, speedModifier)` и followThePath-арифметика — транскрипция T12/T13; PathFinder A* (§2.3) — пока только контракт, отдельный вектор (heap+узлы — heavy-структура, не флет-плоскость; запрет law 5 flat_traversal не нарушаем, но и не форсируем).

## 5. Внешние источники (≥3)
1. **Lithium `ai.pathing`/`ai.goal`** (CaffeineMC/lithium, mixins `ai.pathing`, `ai.goal`): кэш node-типов (PathNodeCache) и чистка GoalSelector — взято: подтверждение, что в nav-плоскости доминируют обращения к миру (.getBlockState/getCollisionShape), а не арифметика → джава-коллект остаётся, натив берёт только decision-ядро (п.4.1).
2. **Pufferfish async pathfinding** (config `async-pathfinding`, AsyncPathProcessor/AsyncPathProvider): перенос findPath с main-потока; взято: дисциплина off-main для тяжёлого PathFinder (extension-lane п.4.7), у нас — bulk-JNI на region-воркерах вместо async-треда (инфра уже region_threads).
3. **Purpur MoveControllerWASD** (org.purpurmc.purpur.controller): wrapper-дизайн tick→vanillaTick; взято: javap-факт делегации (§1) → единая точка ретаргета тела MoveControl.tick.
4. **Moonrise (spottedleaf)**: замена entity-сечений/collisions; взято: прецедент STRICT-parity плоскостей и дескриптор канала CollideBatchOps → батч-патерн collector→strict-java-tail (внутри уже канон P31).
5. **MC-issues MC-4 / MC-189565** (pathfinding/AI lag на больших популяциях): мотивация потолка — lane растёт с популяцией (nav wall 0.78%→1.16% при 150k на якоре с pop-скейлом).

## 6. CAPTURE-МАТЕМ (lane% × захват% = Δ-прогноз; потолок = lane% × 100%)
- Лейн (канон LEDGER, подтверждён профилями): nav_ai/goalops/brain **2.75-3.2%** CPU.
- P44-срез = MoveControl.tick + PathNavigation.tick-семейство = **60-70%** лейна → 1.65-2.24пп.
- Захват bulk-JNI (decision-ядро, java-хвост сохранён): **30-40%** среза.
- **Δ-прогноз P44-ядра** = 1.65×0.30 .. 2.24×0.40 = **+0.50..0.90пп** к норме ноги.
- Δ-прогноз семейства P41-P47 (весь остаток × 30-40%): 2.75×0.30=0.83 .. 3.2×0.40=1.28 → **+0.83..1.28пп** (сходится с LEDGER-оценкой P44 +0.8-1.2).
- **Потолок**: P44-срез = lane×70% = **+2.24пп**; весь nav_ai-лейн × 100% = **+3.2пп**. Вывод (закон 13a, честная потолочная математика): P44 НЕ выводит на +20 сам — это композиционный кирпич CLIMB; композиция на носителе chk-14 (+21.7) с P31/P32-P36 — план добивания.
- Популяционная чувствительность: nav-wall доля якоря 458-33 (0.79-1.42%) vs монстра 457-14 (0.29-0.46%) — lane нафикстуре стабилен, но createPath-хвост чувствителен к узким проходам (AmphibiousPathNavigation.isStableDestination в flamegraph 457-14).

## 7. Preregistered гейты G1-G6
- **G1 ARM/эффект-маркеры**: stdout `[crussty-plugin] moveplane: cmp459_move ARMED (moveDecide bulk batch -> MovePlaneOps)` ДО пурджа; javap flat==nested 10/10 EQUAL; strings блоба содержит маркер (анти-спящий-гейт урок-408); счётчик out-применений >0 в окне.
- **G2 lockstep бит-в-байт оракул**: офлайн-харнесс — Java Mth/MoveControl.tick-математика vs Rust moveDecide на 10^7 рандомизированных (seed=42) конфигов wanted/xyz/yRot/speed → 0 ULP-расхождений в yRot(f32-биты)/speed/jump-triplet; SIN/ASIN/COS-таблицы дампятся из jar и сравниваются побайтово с Rust-генерацией.
- **G3 young/Full GC**: Full ≤9 (банк 457-14), young-паузы в банке ±15%; alloc-профиль: new Vec3/AABB-доля nav-хвоста не растёт (чурCollector не аллоцирует лишнего — grow-патерн NavPlaneOps).
- **G4 популяция-паритет**: 140-165k живых, F4 churn ACTIVE (spawn/despawn exercised), top-12 entity-tick сплит ±10% vs якорь.
- **G5 TPS-бар vs банк v4**: норм = median/TPS_exp(runner)−1; band runner_cpu_index [6.0M, 9.5M], ре-ролл ≤2; min-of-3; pair Δ≤50k pair-fresh.
- **G6 fail-closed disarm**: threw=0, AIOOBE=0, batchOk-disarm-счётчик=0 в бенч-окне (disarm>0 → вердикт REFUTED_CENS-исполнителя, но не паритета — java-реплика покрывает); NCDFE=0 до вердикта (канон Delivery).

## 8. Гипотеза-дельта (для диспатча CLIMB-волной)
Гипотеза: bulk-JNI `moveDecide` на бакете двигающихся мобов (MOVE_TO-доминанта фикс-сцены: zombie 4570 + drowned 4536 + husk 5473) снимает 30-40% среза 1.65-2.24пп → нога +0.5-0.9пп поверх носителя paldelta/chunkmono-семьи; в композиции с P41 (neighbor cache) и P45 (navigatingMobs pre-gate) → +0.8-1.3пп. Отличие от ×456-A nav_plane: nav_plane бил sendBlockUpdated-плоскость (shouldRecomputePath), P44 бил тиковую move-плоскость — лейн не пересекается, композиция STRICT-OR допустима (armed() канон nav_plane.rs уже ор-ит флаги).

## 9. CLIMB-миссия (закон 13b, числа)
1. Имплементить MovePlaneOps.moveDecide по §4 + таблица §3 (оценка 250-350 строк java+rust), оракул G2 локально.
2. Диспатч ноги: база = master 6e019e41 + lever cmp459_move; прогноз ноги сам-по-себе +0.5-0.9пп (суб-бар) → композиция с носителем cmp456_chunkmono d73758a3 (chk-14 +21.7): пара-план окно B [8637055,8737055], якорь ≤+1.7.
3. Если захват <30% (эффект-маркеры G1), апгрейд гипотезы: включить PathNavigation.tick-хвост (§4.7) → срез до 2.24пп.
