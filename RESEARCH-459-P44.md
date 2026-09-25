# RESEARCH-459-P44 — TASK-459-69 (WILD, закон 11, тик-459, c-crussty v18.2 финал)

**Идея ID-P44** (карточка: RESEARCH-458-P.md @ origin/round-458p-ideas): MoveControl/Navigation
navmath-плоскость — плоские массивы `(operationType, posDelta, rot, speedMod)` всех мобов →
ОДИН bulk-JNI → готовые решения → java применяет в порядке set-итерации. v1 = MoveControl.tick only.

## 1. Механика (javap-транскрипция — канон nav_plane.rs, снята с живого ядра)

Источник: `/home/z/tools/patched-kernel.jar` (purpur 1.21.10, Mojang-mapped, классы от 2025-12-11),
`javap -p -c net/minecraft/world/entity/ai/control/MoveControl` + `net/minecraft/util/Mth`.

MOVE_TO ветка `MoveControl.tick()` (offset'ы 208..501 — **javap-verbatim**):

```
dx = wantedX - mob.getX();  dz = wantedZ - mob.getZ();  dy = wantedY - mob.getY()
d3 = (dx*dx + dy*dy) + dz*dz                     // 255-268: лево-ассоц dadd, ВНИМАНИЕ: dy второй!
if d3 < 2.500000277905201E-7  -> setZza(0.0F)    // 270-287: ldc2_w exact literal, dcmpg, return
f9 = (float)(Mth.atan2(dz,dx) * 180.0d / 3.1415927410125732d) - 90.0f   // 288-305: dmul,ddiv,d2f,fsub
mob.setYRot(rotlerp(mob.getYRot(), f9, 90.0f))   // 307-328: MAX_TURN=90.0f inline
mob.setSpeed((float)(speedModifier * getAttributeValue(MOVEMENT_SPEED))) // 329-351: dmul,d2f
jump-gate: dy > (double)maxUpStep && dx*dx+dz*dz < (double)Math.max(1.0F,bbWidth)  // 391-425
        || (!shapeEmpty && mobY < shapeMaxY+blockY && !DOOR && !FENCE)            // 428-501
        -> getJumpControl().jump(); operation := JUMPING                          // 484-498
иначе operation := WAIT (поставлен в 208 ДО математики)
```

`rotlerp(a,b,max)` (offset 0..34, javap-verbatim): `d=wrapDegrees(b-a); if dcmpl(d,max)>0 d=max;
if fcmpg(d,-max)<0 d=-max; e=a+d; if e<0 e+=360f else if fcmpl(e,360f)>0 e-=360f` — NaN-семантика
fcmpl(-1)/fcmpg(+1) транскрибируется как есть (rotlerp(NaN) возвращает NaN без клампов).

`Mth.wrapDegrees(float)` (0..30): `f = v % 360f (frem); if f >= 180f f -= 360f; if f < -180f f += 360f`.

**КРИТИЧЕСКОЕ ОТКРЫТИЕ (javap, не из паблик-сорсов):** `Mth.atan2(y,x)` в ядре — НЕ passthrough
`Math.atan2`, а таблицный fast-atan2 (offset'ы 0..219):
```
d4 = x*x + y*y; if isNaN(d4) -> NaN
negX/negZ-флаги, swap если y > x (dcmpl ifle)
r = fastInvSqrt(d4); y *= r; x *= r
d11 = x + FRAC_BIAS;            // FRAC_BIAS = longBitsToDouble(4805340802404319232) = 2^44 = 17592186044416.0
i   = (int)doubleToRawLongBits(d11);            // l2i low-32
d14 = ASIN_TAB[i]; d16 = COS_TAB[i];            // [257] doubles
d18 = d11 - FRAC_BIAS; d20 = x*d16 - y*d18;
d22 = ((6.0d + d20*d20) * d20) * 0.16666666666666666d;
d24 = d14 + d22 (+ 1.5707963267948966d-swap / 3.141592653589793d-flip / neg по флагам)
```
`fastInvSqrt` (0..39): `half=0.5*x; l=rawBits(x); l=6910469410427058090L-(l>>1);  // 0x5FE6EB50C7B537AA
x=longBitsToDouble(l); return x * (1.5d - (half*x)*x)` — magic в байткоде `...090`,
а НЕ `6910469410427058089L` из паблик-дампов — сверено, компилированный литерал истина.
Таблицы ASIN_TAB/COS_TAB генерятся в `<clinit>`: `for i in 0..257 { d0=i/256.0; d1=Math.asin(d0);
COS_TAB[i]=Math.cos(d1); ASIN_TAB[i]=d1 }` (offset'ы 283..325) — в Rust пересоздаются той же
формулой, паритет гейтом (oracle-харнес ниже).

Operation enum ordinal'ы (`MoveControl$Operation`): WAIT=0, MOVE_TO=1, STRAFE=2, JUMPING=3.
JUMPING-passthrough ветка tick() (504..574: setSpeed; onGround||isInLiquid&&isAffectedByFluids → WAIT)
— фазный widen-2, в v1 НЕ батчится.

## 2. Сайты и web-источники (≥2, raw-феч проверен)

1. **C2ME (RelativityMC/C2ME-fabric, README ver/1.21.1, raw fetched)** — батч-дисциплина:
   производительность через многоядерный параллелизм и батч-очереди работ вместо per-entity
   синхронного исполнения; «taking advantage of multiple CPU cores in parallel» — прецедент,
   что тяжёлые per-entity решения выносятся из тик-потока в плоские батчи. Батч-плоскость P44
   переносит ту же дисциплину на MoveControl.tick.
   https://github.com/RelativityMC/C2ME-fabric ; https://modrinth.com/mod/c2me-fabric
2. **Lithium (CaffeineMC/lithium-fabric, README master, raw fetched)** — «optimize many areas of
   the game… doesn't change any game mechanics or visuals, it just makes the game run faster»:
   канон parity-дисциплины (оптимизация = ваниль-поведение бит-в-бит, меняется только исполнитель).
   Lithium оптимизирует ai/pathing миксинами на стеке; P44 берёт ту же цель (nav_ai/MoveControl),
   но исполняет батч-плоскостью + decideJava-фолбэк.
   https://github.com/CaffeineMC/lithium-fabric ; https://modrinth.com/mod/lithium
3. Карточка-референс: RESEARCH-458-P.md ID-P44 (q12_move.json — javap-контракты лейна).

## 3. Дизайн v1 (одна плоскость, один ретаргет)

- Java (новый `navmath/net/minecraft/world/entity/MoveOps.java`, класс
  `net/minecraft/world/entity/MoveOps`): collect-проход по мобам в ПОРЯДКЕ ИТЕРАЦИИ СЕТА,
  плоские массивы `ops[n] i32`, `meta[n*4] i32` (rotBits, maxUpStepBits, bbWidthBits, shapeFlags),
  `pos[n*6] f64` (wantedXYZ, mobXYZ), `speed[n*2] f64` (speedModifier, moveSpeedAttr),
  `shapeMaxY[n] f64` (заранее: `shape.max(Y)+blockPos.getY()` на java) → ОДИН нативный вызов
  `navMoveBatch` → apply-проход: `setYRot/setSpeed/setZza/jump()/operation` в том же порядке.
- Rust (`src/navmath_flat.rs`): бит-в-бит ядро `move_decide` (транскрипция §1: dsub/dmul/dadd
  порядок, exact-литерал 2.500000277905201E-7, fast-atan2 + fastInvSqrt + rotlerp/wrapDegrees,
  fcmpl/fcmpg NaN-семантика) — канон nav_plane.rs `decide` (TASK-405-A).
- v1 скоуп: op MOVE_TO=1 (полное ядро) + op WAIT=0 (setZza(0)); STRAFE/JUMPING → out=3
  «не батчится, vanilla inline» (фазный widen-2 — как дисциплина карточки «много сайтов»).
- Ретаргет: v1 = 1 сайт (MoveControl.tick), диспатч через classfile.rs в момент активации рычага;
  остальное — widen следующими тиками.

## 4. Parity: decideJava fallback бит-в-бит

- Любой rc<0 (ERR_STRUCT=-1 / ERR_RANGE=-2, лестница mobs_soa) или throwable → one-shot latch
  `batchOk=false` → java-реплика `decideJava` с ТОЙ ЖЕ математикой через реальные вызовы
  `Mth.atan2/Mth.wrapDegrees` (публичные, сама JVM) — поведение идентично ванили, отличается
  только исполнитель. Канон: nav_plane.rs «java-реплика decideJava (бит-в-бит та же математика)».
- Таблицы JAVA-AUTHORITATIVE (ЗАМЕР CLOSED): первая попытка — регенерация clinit-генератора
  в rust — ДАЛА ulp-дрейф (10/514 ячеек ASIN/COS_TAB расходятся на 1 бит vs HotSpot Math.asin/cos;
  oracle-проба против живой Mth это поймала — см. §7.1). Финальный дизайн: MoveOps static-init
  копирует СОБСТВЕННЫЕ Mth-таблицы живой JVM (reflection) и передаёт их одним navMoveInit —
  паритет ПО ПОСТРОЕНИЮ, гейт не нужен; не-init/OOB → ERR_STRUCT → decideJava disarm.
- ЗАМЕР (scaffold-коммит, bit-exact vs patched-kernel.jar Mth):
  atan2 8/8, rotlerp 15/15, wrapDegrees 6/6 hex-бит в бит; полный MOVE_TO decision-batch 8/8
  кейсов (move/wrap-tail/jump-narrow/jump-shape/door/eps-stop/WAIT/STRAFE) — java
  net.minecraft.util.Mth.atan2 + rotlerp-реплика против rust navmath_flat::move_batch
  (java-таблицы) = DECISION-PARITY PASS.
- STRICT eq lever: `cmp459_navmath` (пустой/чужой флаг → класс не определяется, ретаргет не
  компонуeтся, хук не регистрируется → ваниль бит-в-байт по построению, канон nav_plane.rs).

## 5. NCDFE-канон (S7-163)

Бридж-класс ровно ОДИН classfile (S7-163: nested-класс детонирует как NoClassDefFoundError на
первом entity tick — TECH-DUD leg#1; пины: collidebatch_source_declares_no_nested_classes,
chunk_parse.rs:474, zero_alloc.rs:206, traversal.rs:234). MoveOps.java: финальный класс без
nested, `--release 21` (major 65 ≤ kernel JVM 21), define_class fail-closed (parse_diag.rs:230
паттерн), NCDFE/UCVE на define → lever disarm, не crash. Build-скрипт с find-гейтом «ровно 1
выход» — паттерн scripts/build_collidebatch_ops.sh (v1 wiring, НЕ в этом scaffold-коммите).

## 6. Δ-прогноз (capture-матем)

- Лейн nav_ai: MoveControl+Navigation.tick = 60-70% остатка лейна (карточка ID-P44).
- Захват батч-плоскостью остатка × 30-40% (оценка карточки, консервативна: ONE JNI на тик
  против per-mob виртуальных диспетчей set-итерации) → **Δ +0.8-1.2пп** к ноге; с P41 →
  +1.5-2пп. Пререгистрированный гейт v1: A/B min-of-2 (контроль lever='' на том же sha),
  пар медианная ≥ +0.5пп иначе lever в банк как TECH-DUD с javap-номерами.

## 7. Риски

1. **Таблицы asin/cos ulp-дрейф** (Rust libm vs JVM Math) — ЗАМЕРЕНО и ЗАКРЫТО: дрейф реален
   (10/514 ячеек, 1 бит), поэтому прод-ядро использует java-авторитетные таблицы через
   navMoveInit (§4); rust-генератор оставлен только под cfg(test) якоря.
2. **Set-итерация CME** — collect/apply по snapshot-массиву, исключение → фолбэк decideJava для
   всего батча (канон NavPlaneOps CME-retry; в v1 упрощаем: catch → disarm).
3. **Порядок применения** — rot/speed ДО jump-гейта в ванили (offset'ы 307-351 < 391); apply
   сохраняет порядок (out=2 применяет rot, speed, затем jump) — иначе не бит-в-бит.
4. **l2i-обрезка индекса таблиц** — i = low-32 бита raw-bits; в Rust `(bits as i32)`;
   OOB → ERR_RANGE (java-фолбэк), не паника.
5. **NCDFE/UCVE** — §5; фазный widen снижает площадь ретаргета до 1 сайта в v1.
6. NaN-траектории (d3=NaN) — javap-семантика dcmpg/fcmpl воспроизведена (§1), тесты
   navmath_flat::tests фиксируют.

## 8. Артефакты этого коммита (scaffold)

- `src/navmath_flat.rs` — ядро-транскрипция + ERR-лестница + STRICT-гейт + тесты (std-only,
  без jvmti-импортов — регистрация RegisterNatives в v1 wiring-коммите по паттерну nav_plane.rs
  register_native).
- `navmath/net/minecraft/world/entity/MoveOps.java` — java-стаб (native-сигнатура navMoveBatch,
  batchOk latch, decideJava-скелет, javap-offsets в комментариях). Компиляция (build-скрипт) —
  v1 wiring, NCDFE-канон §5.
- `lib.rs`: `mod navmath_flat;` (мёртв в v1 без арма рычага — fail-closed по построению).
