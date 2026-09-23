# RUNBOOK S7-150 — FLUID-DIRTY (ARCH-ATTACK, пост-эра)

Дата: 2026-09-18 18:0x-18:3x +08 · Task ID: S7-150 (Job 394666, тик 18:08) · agent-7625532f
Статус: STEP-0 + RECON завершены; имплементация — S7-151 (следующие тики)

## 1. Ранжирование пост-эры (вход: CUMULATIVE 35330129145, валидная зелёная база)

Источник: `run-s7149b-cumulative/cpu-collapsed.txt` (52341 self-time сэмплов),
анализатор `s7150_recon.py` → `S7150_RECON.md`. Фаза entity tick = 58.5%.

Ранжирование attackable-лейнов (§S7-139 калибровка: доля в total CPU проверена):

| ранг | лейн / функция | доля total CPU | вердикт |
|---|---|---|---|
| 1 | **fluid-push family** (`Entity.updateFluidHeightAndDoFluidPushing` self 2.61% + callees: getFluidState, palette-чтения под сканом, getHeight/getFlow) — 36.4% лейна ItemEntity (4466/52341 = 8.5%) + 9.8% лейна Zombie (674) + прочие мобы | **~10%** | **ТОП-1, атакуем (FLUID-DIRTY)** |
| 2 | move/collision (AABB.intersects 1.87%, CollisionUtil 1.03% + callees) — 23.0% лейна ItemEntity | ~5.4% | фронт №2 (следующая нога) |
| 3 | inside-blocks residual (flushStep CPU 0.82% + checkInsideBlocks) — 21.8% лейна ItemEntity | ~5.1% | пост-INSIDE-CACHE остаток |
| 4 | entity tracker (ServerEntity.sendChanges + clearPlayers + distanceToSqr в ChunkMap.tick) | ~2% | фронт №3 |
| 5 | merge-search (getEntities broadphase) — 2.2% ItemEntity / 22% Zombie | ~2-3% | ниже порога у items; у Zombie — часть AI-фронта |
| 6 | GC native (28.8%) | 28.8% | атакуется только через alloc-диету: дважды REFUTED (S7-133b, S7-149) — закрыто |
| 7 | merge-индекс items | 2.2% | МИКРО-КЛАСС (≤§S7-139 порог) — не брать |

Merge-search у items = 2.2% лейна (только 265 сэмплов) — вопреки гипотезе O(n²)-мерджа,
100k items почти все не-pass-через merge (stackable-гейт + перекрытия редки) → микро-класс.
PalettedContainer.get (3.23% leaf) — распределён по скан-лейнам (fluid/inside/collision);
автономная атака (демукс) REFUTED (S7-148d) — не рычаг.

## 2. STEP-0 javap-контракт (kernel e2992d63, javap21 /tmp/toolchain)

Файлы: `step0_updateFluidHeightAndDoFluidPushing.txt`, `step0_updateInWaterStateAndDoFluidPushing.txt`,
`step0_baseTick_gate.txt` (дизасм из patched-kernel.jar рана 35330129145).

### 2.1 Вход (baseTick:123-132, результат `pop` — игнорируется)
```
updateInWaterStateAndDoFluidPushing():Z   // вход в fluid-конвейер каждого тика
  fluidHeight.clear()                     // Object2DoubleMap<TagKey>
  updateInWaterStateAndDoWaterCurrentPushing():
      vehicle instanceof AbstractBoat && !boat.isUnderWater() → wasTouchingWater=false (скан пропущен)
      else → updateFluidHeightAndDoFluidPushing(WATER, 0.014):
          true → (if !wasTouchingWater && !firstTick) doWaterSplashEffect(); resetFallDistance(); wasTouchingWater=true
          false → wasTouchingWater=false
  lavaScale = dimensionType.ultraWarm() ? 0.007 : 0.0023333333333333335
  lavaHit = updateFluidHeightAndDoFluidPushing(LAVA, lavaScale)   // пишет lastLavaContact при хитах
  return isInWater() || lavaHit
затем безусловно: updateFluidOnEyes(); updateSwimming()
```

### 2.2 Ядро `updateFluidHeightAndDoFluidPushing(TagKey, double)` — СКАН = ЧИСТАЯ ФУНКЦИЯ
```
1. touchingUnloadedChunk() → return false
2. aabb = getBoundingBox().deflate(0.001)
3. span: x0=floor(minX)..x1=ceil(maxX)-1; y0=max(minSectionY<<4, floor(minY))..y1=min(maxSectionY<<4|15, ceil(maxY)-1); z аналогично
4. pushedByFluid = isPushedByFluid()          // локальная 13
5. ПРЕД-ФЕТЧ матрицы sections по чанкам span (getChunk(cx,cz,FULL,false).getSections())
6. Vec3 flowAcc = ZERO; double hAcc=0; int flowN=0; boolean hitFlag=false
7. по каждому блоку span: state = section.states.PalettedContainer.get((y&15)<<8|(z&15)<<4|(x&15))
     fluid = state.getFluidState()
     if (!fluid.isEmpty() && fluid.is(tag)):
        if tag==LAVA: lastLavaContact = pos.immutable()      // ПОБочный эффект — последний хит
        d = pos.y + fluid.getHeight(level, pos)
        if (d >= aabb.minY):
           hitFlag=true; hAcc=max(hAcc, d - aabb.minY)
           if (pushedByFluid):
              flowN++; flow = fluid.getFlow(level, pos)
              flowAcc = flowAcc.add(flow.scale(hAcc < 0.4 ? hAcc : 1.0))
8. fluidHeight.put(tag, hAcc)                  // ВСЕГДА (даже без хитов)
9. if (flowAcc == Vec3.ZERO /*ref-cmp*/) return hitFlag
10. vec = flowAcc.scale(1.0/flowN); if !(this instanceof Player) vec = vec.normalize();
    vec = vec.scale(motionScale)
    if (|dm.x|<0.003 && |dm.z|<0.003 && vec.length()<0.0045) vec = vec.normalize().scale(0.0045)
    setDeltaMovement(dm.add(vec)); return true
```
Ключевой факт: **шаги 1-7 = чистая функция (span, fluid-состояния блоков span, isPushedByFluid)**.
Шаги 8-10 = дешёвая постобработка без мировых чтений (но зависящая от dm/Player/motionScale).
→ Мемоизировать можно ТОЛЬКО шаги 1-7 (scanResult = {hAcc, hitFlag, flowAcc, flowN, lastLavaContact}),
постобработку выполнять всегда по vanilla-последовательности.

### 2.3 Дельта-эффект скана (что должно совпасть для median-exact parity)
- fluidHeight[tag] = hAcc — кэшированное значение пишется так же, как свежесканное
- hitFlag → вода: splash/fall/wasTouchingWater транзишены; lava: return wrapper'а
- flowAcc≠ZERO → тот же пуш по той же формуле (dm-зависимость сохраняется — постобработка живая)
- lastLavaContact — переписывается кэшированным BlockPos при хите

## 3. Рычаг FLUID-DIRTY (класс: кэш/мемоизация + event-driven dirty-флаги вместо поллинга)

- **FluidPushOps** (self-contained bridge, паттерн InsideBlockOps S7-148 — без kernel-полевых
  зависимостей, ThreadLocal/memo внутрь моста):
  - `memoGet(entity, tag, span, stamps)` → scanResult|null; `memoPut(...)`
  - хранение memo: статический кольцевой буфер по entity-хешу (пattern INSIDE-CACHE ring,
    детали зафиксировать при имплементации; без утечек — ключ = ссылка + generation)
- **Dirty-stamp ledger**: `FluidOps.bump(sectionKey)` — хук в LevelChunk.setBlockState
  (сравнение old/new FluidState по ССЫЛКЕ — FluidState-синглтоны; bump ТОЛЬКО при реальном
  изменении fluid-состояния клетки — не на каждый setBlock). Fluid-tick/explosion/waterlog —
  все проходят через setBlockState ⇒ покрытие полное. Ledger: Long→long версия секции.
- **Хит-условие**: span бит-в-бит + pushedByFluid + версии всех секций span не изменились
  ⇒ скан выдаёт бит-в-бит тот же результат (входы идентичны) ⇒ пропуск = vanilla-эквивалент.
  Движущиеся сущности (span меняется) сканируются каждый раз как в vanilla; покоящиеся
  (100k items на земле) пропускают скан. Статичная вода не мутирует ⇒ не дёргает ledger.
- **Патч-секция**: тело `updateFluidHeightAndDoFluidPushing` — прелюдия memoGet → HIT:
  кэш-пайплайн (fluidHeight.put + lastLavaContact + пуш-постобработка); MISS: vanilla-скан
  (побайтно исходная последовательность) + memoPut. Wrapper (2.1) НЕ трогается.
  NOTE: FLUID-FREE (S7-143) сплайсила 15041→15088 байт — механика section-splice отработана.
- **Анти-урок FLUID-FREE (S7-148d)**: тот кэш был per-SECTION "has-fluids" вердикт на
  demux-пути — в водном мире секции почти все "не-пустые" ⇒ 0 хитов + оверхед демукса −20% TPS.
  FLUID-DIRTY кэширует per-ENTITY результат скана: хит не зависит от наличия воды в секции;
  зависимость только от (не-движения + отсутствия мутаций воды) — оба массово истинны на X150K.
  Демукс НЕ нужен (fluid_free=0, demux=0 — работаем на прямом пути скана).

## 4. Ожидание эффекта
- Снятие ~10% total CPU скана (суб-микросекунды на хит против ~8-14 palette-чтений × 2 тега);
- сопутствующая alloc-диета: new MutableBlockPos (new на КАЖДЫЙ вызов!), Vec3.add-цепочки,
  immutable-Pos lava — всё исчезает на хитах → young GC relief (28.8% native lane);
- TPS: консервативно +5-10% на X150K при TPS 0.8-0.9 (доля лейна / диминишинг entity-фазы).

## 5. Preregistered гейты (честный измерительный протокол A/B min-of-2; НЕ лотерея)

Ран leg: inside_cache=1 + flush_diet=1 (база CUMULATIVE-конфиг) + fluid_dirty=1,
против base-b 35317176927 / CUMULATIVE 35330129145 (паринг world_sha256, fp4/300s/150k/seed42/xmx10G):
- §S7-150-G1: fluid-family lane (`updateFluidHeightAndDoFluidPushing` self + callees под ним)
  ↓ ≥ 60% отн. CUMULATIVE (принцип: лейн не ниже ~8% до, хит-рейт ≥ 80% из логов моста)
- §S7-150-G2: 0 NCDFE в stdout (новый §156-чек), ARMED-маркер FluidPushOps в логе запуска
- §S7-150-G3: популяция-паритет (близнец base-b, дельта ≤ 1%), TOPUP-SCAN жив
- §S7-150-G4: TPS не хуже базы min-of-2 (медианы окон 0.7-1.0 зоны; регресс >10% = REFUTED)
- §S7-150-G5: parity-фикстура OFFLINE: lockstep-реплей 20000 ops (случайные span-сдвиги +
  storm мутаций воды) — cached-результат бит-в-бит равен vanilla-скану (hAcc double, hitFlag,
  flowAcc x/y/z, flowN, lastLavaContact); хит-книга на мутациях (после bump — только MISS до recache)
- §S7-150-G6: young GC не выше базы (не-регресс; ожидание — ниже за счёт alloc-диеты скана)
- REFUTED-критерий: G1 недостижим при ARMED-живом мосте и хит-рейте ≥80% ⇒ экономика лейна
  переоценена; G4 регресс ⇒ оверхед моста > выйгрыша (анти-урок демукса) ⇒ дефолт fluid_dirty=0.

## 6. Декомпозиция имплементации (S7-151+)
1. FluidOps.java (FluidPushOps): ledger + memo ring + scanResult record; компиляция javac21
   против kernel e2992d63 (pattern FluidOps S7-143 / InsideBlockOps S7-148 self-contained)
2. Патч-секция тела updateFluidHeightAndDoFluidPushing (patch_section, utf8-only, байт-в-бит)
   + хук bump в LevelChunk.setBlockState (второй сплайс)
3. Rust-хук флага (fluid_dirty.rs) + wiring world-bench.yml / lib.rs / run_world3.sh
4. FluidDirtyHarness OFFLINE (Parity-стаб closure: PalettedContainer*+Strategy*+Configuration*
   — по прецеденту S7-143; nest-partner RecordedEffect из kernel)
5. Compile-OK CI-эквивалент + rust 122 ok + harness PASS → preregister dispatch (A/B min-of-2)

INJECTS-ONLY: 0 sandbox boots; CI-буты санкционированы.

---
## ДОПОЛНЕНИЕ S7-151 (2026-09-18 18:4x-19:1x +08) — ИМПЛЕМЕНТАЦИЯ FLUID-DIRTY ЗАБАНЧЕНА

- **FluidPushOps.java** (пакет net.minecraft.world.entity, self-contained — S7-148-урок,
  ни одного cross-bridge dep): scanArmed = guard touchingUnloadedChunk → span (клэмп-математика
  ванили) → сбор секций span + текущих штампов (детерминированный cz→cx→sy) → HIT-проверка
  (span бит-в-бит + pushedByFluid + nsec + refs секций + штампы) → MISS: mirrorScan (полная
  реимплементация шагов 2-7 javap-контракта: пред-фетч rows-матрицы, обход x→y→z, i2f/fadd/f2d
  высоты, dcmpg/ifge NaN-семантика, ref-identity Vec3.ZERO, dcmpg-форма sc-ветки) + capture →
  postprocess (шаги 8-10 всегда: fluidHeight.put, lastLavaContact, pushTail бит-в-бит).
  ДВА набора слотов (WATER/LAVA, NSLOTS 2^17) — иначе теги вымывали бы друг друга каждый тик;
  модифицированные теги → чистая ванилла. Capture в свой слот (ping-pong hardening S7-136).
  НОЛЬ Unsafe (все члены доступны same-package). ThreadLocal ring (ScanOut/секции/штампы/mpos) —
  ноль аллокаций на HIT.
- **Ретаргеты (classfile.rs)**: patch_fluid_dirty_entity (оба wrapper-сайта → FluidPushOps.scan,
  строгая 1+1) + patch_fluid_dirty_levelchunk (единственный сайт LevelChunkSection.setBlockState
  в LevelChunk.setBlockState → FluidPushOps.secWrite, строго 1). CENSUS v2 (S7151_CENSUS.md):
  во всём kernel ровно 2 вызова скана (оба в Entity-обёртках) и ровно 1 сайт section-write —
  покрытие полное, рекурсии нет (тело ванильного метода не тронуто, miss → обычный invokevirtual).
- **Rust wiring**: src/fluid_dirty.rs (env CRUSSTY_FLUID_DIRTY, hook LevelChunk, define
  FluidPushOps+$ScanOut в kernel loader, BRIDGE_READY-протокол) + inside_cache.rs compose-цепочка
  (Entity байты = inside + [fluid_free] + fluid_dirty, wait_bridge_ready 60s — S7-143/S7-148
  LinkageError-уроки) + lib.rs (mod/register/activate). enabled_pub в inside_cache для WARN.
- **Тесты**: 9 новых roundtrip-тестов classfile.rs (retarget 2/1 строгие, methodref-триплы в пуле,
  idempotent байт-в-байт, fail-closed на чужом классе, compose with inside) — **полный suite
  132/0/1** (было 121/0/1). Release-сборка OK.
- **Harness OFFLINE PASS exit 0** (structural/wiring/armed/ledger tier): патченные Entity+LevelChunk
  линкуются над реальным kernel (verifier pass); ссылки на FluidPushOps + invokestatic на месте;
  ARMED; ledger на РЕАЛЬНОЙ секции kernel: air→water bump 0→1 + old-state возврат + делегация
  записи; water→water (синглтон) НЕ бампит (нет ложной инвалидации); water→air bump 1→2;
  air→stone НЕ бампит (обычные блоки не дёргают).
- **Артефакты**: artifact_hashes_s7151.txt (14 файлов, sha256); tests/fixtures/LevelChunk_real.class
  (bac0a84d…); tests/out/{Entity,LevelChunk}.fluiddirty.patched.class (dump из rust-тестов).
- **S7-152 (следующий тик)**: поведенческий lockstep на мини-Level (реальные ItemEntity/ArmorStand +
  Level-стаб с реальными LevelChunk/Section): FluidPushOps.scan vs ванильный скан бит-в-бит
  (return, fluidHeight-биты, dm-биты, lastLavaContact) на позиционных свипах (границы чанков/секций,
  дробные координаты, вода/лава/высоты течений, dm-ветка 0.003/0.0045) + шторм мутаций (G5-гейт) →
  затем preregister dispatch (inside_cache=1+flush_diet=1+fluid_dirty=1, A/B min-of-2 vs CUMULATIVE).

---
## ДОПОЛНЕНИЕ S7-152 (2026-09-18 19:0x-19:4x +08) — ПОВЕДЕНЧЕСКИЙ LOCKSTEP PASS (G5 core)

- **Стена офлайн-конструирования**: vanilla Level ctor кастует `this` к ServerLevel
  (CraftWorld в ctor, disasm 1222-1297) + SpigotWorldConfig/PurpurWorldConfig — подкласс Level
  через ctor невозможен; прямой ctor LevelChunk кастует level к ServerLevel (disasm 99-43) +
  зовёт MinecraftServer.getServer().registryAccess() → PalettedContainerFactory. РЕШЕНИЕ:
  Unsafe.allocateInstance фикстуры ("scan-contract fixture") — ровно поля javap-контракта скана:
  Entity {level, bb, id, deltaMovement+posLock, fluidHeight, lastLavaContact, firstTick=false;
  isPushedByFluid=константа true (базовая импл)}, Level {minY/maxY/minSectionY/maxSectionY/
  sectionsCount (finals через Unsafe), isClientSide=false, captureTreeGeneration=false,
  levelData=proxy; 20 абстракций + getChunkSource() реализованы подклассом MiniLevel},
  LevelChunk {chunkPos, sections, levelHeightAccessor, level}.
- **FluidDirtyLockHarness** (entityinside/harness/, scripts/run_fluid_dirty_lock.sh): реальный
  vanilla-скан (kernel bytecode) vs FluidPushOps.scan на близнецах-ItemEntity; всё, что зовёт
  скан — реальный kernel-код (touchingUnloadedChunk → hasChunksAt → moonrise$areChunksLoaded →
  chunkSource.hasChunk; тройной цикл; PalettedContainer.get; getHeight/getFlow с соседними
  чтениями через level.getFluidState → реальные секции). Заполнение мира — реальные
  LevelChunkSection.setBlockState (тот же делегат, что у secWrite, без bukkit-периферии).
- **Результат — FLUID-DIRTY LOCKSTEP PASS**: Сцена A (24 позиции: внутри воды/кромка
  0.888…/сухой камень/границы чанков x и z/дробные × 4 dm-профиля × WATER 0.014 + LAVA 0.007 —
  бит-в-бит паритет return/fluidHeight-биты/dm-биты + HIT bookkeeping) · Сцена B (лава+вода
  столбами, кромка лавы, граница секций — lastLavaContact parity) · Сцена C (flowing уровни 1-3 +
  ШТОРМ: источник→течение 8 / течение→воздух / вода→камень / камень→вода — каждая мутация = MISS
  + бит-в-бит со свежей ванилью; воздух→камень НЕ бампит) · Сцена D (unloaded-guard: оба пути
  false без записи). Статистика: hit=49 miss=94 vanilla=0.
- **Гейты S7-151/S7-152 статус**: G5 (OFFLINE lockstep) = PASS core; G1-G4/G6 — на живой ноге
  (живой A/B). Признак диспатча: next tick S7-153 — preregister dispatch (inside_cache=1+
  flush_diet=1+fluid_dirty=1, fp4/300s/150k/seed42/xmx10G, A/B min-of-2 vs CUMULATIVE 35330129145).
- Артефакты: artifact_hashes_s7152.txt; RUNBOOK дополнен (этот блок).

---

## S7-153 ABSORB (тик 19:43+08) — нога диспатчена и поглощена тем же тиком

- Диспатч: run 35341241628, head 38afaa3 (пломбинг: world-bench.yml input fluid_dirty +
  run_world3.sh export CRUSSTY_FLUID_DIRTY + dispatch_s7153.py), SUCCESS 11:46:00→12:03:29 UTC.
- Инцидент пуша: GitHub push protection заблокировал токен в dispatch_s7153.py:26 —
  исправлено (токен из remote URL по правилу 1b; урок: секреты в скриптах диспатча запрещены).
- Вердикт: **REFUTED-BY-ECONOMICS** — hit-rate ≈ 0% на живой сцене (scan 1592 ≈ vanilla 1590
  сэмплов в стеках), fluid-family 9.91%→10.40%, young GC 118→140, TPS паритет, 0 NCDFE,
  ARMED полна живьём. fluid_dirty = 0 (забанкован).
- Корневая причина + уроки: research/fluid-dirty-2026-09-18/ABSORB_S7153.md.
- NEXT (S7-154): RECON-2 unclassified-фазы (33-39% entity-цикла) до классов поведения —
  последний крупный непокрытый фронт; ранжированные остаточные лейны (move/collision 5.4%,
  inside-blocks residual 5.1%, tracker ~2%) — микро-класс по отдельности.
