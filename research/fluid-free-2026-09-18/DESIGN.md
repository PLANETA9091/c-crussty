# FLUID-FREE-SECTION — STEP-0 + дизайн (S7-138, ARCH-ATTACK рычаг #5)

## 1. Идентификация лейна (ценз 35275967738 + профиль S7-131)
- fluid-чтения = 56% клиентов топ-1 kernel CPU-функции PalettedContainer.get
  (guard slow 473 + cellsUnchanged 82 + hasSameAbove/getFlow 363 сэмплов из 1746).
- НЕ покрыт гейтом INSIDE-CACHE: updateFluidHeightAndDoFluidPushing — независимый
  от checkInsideBlocks путь (baseTick каждой сущности, 2 вызова/тик: WATER 0.014 +
  LAVA ultraWarm?0.007:0.0023333).
- Чёрн лейна: Vec3.multiply 174 (2.4%) + getFlow 116 (1.6%) + AABB.deflate +
  MutableBlockPos + sections[][] + Object2DoubleMap.put + Vec3-цепочки getFlow.
- CPU: тройной цикл AABB x директный PalettedContainer.get на КАЖДУЮ ячейку;
  доминанта итераций = isEmpty (сущности НЕ в жидкости платят скан в пустоту).

## 2. javap-контракт (purpur-1.21.10, /tmp/entity_jap.txt)
public boolean updateFluidHeightAndDoFluidPushing(TagKey<Fluid> tag, double speed):
  1) touchingUnloadedChunk -> false
  2) AABB box = getBoundingBox().deflate(0.001) [аллокация]
  3) moonrise: кэш sections[][] по чанкам AABB [2D-массив, getChunk(FULL,false)]
  4) тройной цикл x/y/z по box: state = sections[x>>4][z>>4].states.get(
     (x&15)|((z&15)<<4)|((y&15)<<8))  [ПРЯМОЙ контейнерный get]
     fs = state.getFluidState(); if (fs.isEmpty()) continue;   [ДОМИНАНТА]
     else: fs.is(tag) -> height/getFlow [Vec3-аллокации, push-аккумулятор];
           ИНАЧЕ: LAVA-ветка (FluidTags.LAVA + immutable BlockPos — огонь)
  5) хвост: fluidHeight.put(tag, maxH) ВСЕГДА; if (flow==Vec3.ZERO) return inFluid;
     иначе push-физика (normalize/scale/setDeltaMovement, Player-чек 0.003/0.0045)
Call-sites (оба invokevirtual, 3B): updateInWaterStateAndDoWaterCurrentPushing
  offset 39 (WATER, 0.014; за ним wasTouchingWater/splash/resetFall);
  updateInWaterStateAndDoFluidPushing offset 41 (LAVA; fluidHeight.clear() в голове
  обёртки, финал isInWater()||lava).

## 3. HIT-семантика (все секции AABB fluid-free)
Ваниль при нуле не-пустых fluid-ячеек: цикл не находит ничего => fluidHeight.put(
tag, 0.0), flow==ZERO => return false. LAVA-ветка огня невозможна (жидкостей нет
вовсе). Побочные эффекты обёрток (wasTouchingWater=false, fluidHeight.clear()
в голове) исполняются САМИ — бридж их не трогает. Бридж-HIT обязан сделать
ТОЛЬКО: fluidHeight.put(tag, 0.0) + return false.

## 4. Дизайн рычага
- Гейт: ретаргет ОБОИХ invokevirtual call-sites updateFluidHeightAndDoFluidPushing
  -> invokestatic FluidOps.fgate(Entity, TagKey, double)Z (3B->3B, receiver-first,
  механика patch_push_entities S7-133). Тело ванильного метода НЕ ТРОГАЕТСЯ:
  miss -> e.updateFluidHeightAndDoFluidPushing(tag, speed) (обычный вызов,
  рекурсии нет — ретаргечены только 2 сайта обёрток).
- FluidOps.fgate(Entity, TagKey, double)Z:
    if (ARMED && FluidOps.allFluidFree(e)) { e.fluidHeightUnsafePut(tag, 0.0); return false; }
    return e.updateFluidHeightAndDoFluidPushing(tag, speed);
  allFluidFree: AABB.deflate-границы -> перечислить LevelChunkSection[] под AABB
  (getChunk FULL,false — как ваниль) -> для каждой секции:
    if (section.crusstyFf == 1 && section.crusstyFfGen == states.crusstyPGen) HIT;
    else ленивый пересчёт: scan 4096 ячеек states.get(i) -> ни одной не-пустой
    fluid => crusstyFf=1, crusstyFfGen=states.crusstyPGen; иначе crusstyFf=0.
    Ложный miss безопасен (пересчёт = истина).
- Инвалидация (event-driven, без поллинга): field-injection в PalettedContainer
  public volatile I crusstyPGen; ИНКРЕМЕНТ в КАЖДОМ мутаторе (getAndSet, set,
  getAndSetUnchecked, swapCounts? — сверить с S7-131 списком; resize НЕ меняет
  логическое содержимое, но gen++ допустим = ложный miss). Field-injection
  механика paletted.rs S7-131 (4 поля демукса добавлялись той же машиной).
- LevelChunkSection field-injection: public I crusstyFfGen; public B crusstyFf
  (-1 неизвестно, 0 есть-жидкости, 1 fluid-free).
- Порядок записи при пересчёте: сначала crusstyFfGen=gen, ПОТОМ crusstyFf
  (write-order для видимости; однопоточный region-tick — moonrise лицензирует).
- Мутации мимо мутаторов: нет известных (все записи block-state идут через
  PalettedContainer-мутаторы; прямые field-writes data отсутствуют в ядре).
  Остаточный риск => фейл-домина: ложный miss (не ложный HIT).

## 5. Ожидания (preregistered, X150K vs база 35275967738)
- Предпосылка сцены: items лежат на суше (воздух/земля — секции без жидкостей);
  9216 чанков — большинство секций fluid-free. Оценка покрытия HIT >= 90%
  fluid-сканов.
- Гейты: fixture зелёные (behavioral parity, splash/fire-эффекты на воде/лаве
  не затронуты — miss-путь для мокрых секций), ARMED-маркеры
  ("defined FluidOps", Retargeted{2}), PalettedContainer.get fluid-доля
  (473+82+363 сэмплов база) ↓ >= 60%, fluid-лейн чёрна (2.4%+1.6%+deflate)
  ↓ пропорционально HIT, entity-фаза без регрессии >0.5pp, young GC ↓.
- Частичный гейт = калибровка leg #2; промах = REFUTED-BY-ECONOMICS default 0.

## 6. Статус имплементации
- [x] STEP-0 javap-контракт (этот документ)
- [ ] FluidOps.java + fluidHeight Unsafe-резолв (по образцу InsideBlockOps)
- [ ] paletted.rs: инъекция crusstyPGen + 4 инкремента мутаторов
- [ ] LevelChunkSection: инъекция crusstyFf/crusstyFfGen (новый patcher)
- [ ] classfile.rs: patch_fluid_gate (2 ретаргета обёрток)
- [ ] rust-тесты + FlushDiet-стиль харнесс (OFFLINE PASS)
- [ ] env CRUSSTY_FLUID_FREE + run_world3 + workflow input
