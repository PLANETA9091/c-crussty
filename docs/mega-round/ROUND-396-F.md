# ROUND-396-F — items_mono (MEGA-ROUND-1, вектор F)

Task: TASK-396-F · Agent: mega-round-agent-f · Ветка: `round-396-f-items_mono`
Дата: 2026-09-21 · Базовый коммит: origin/master `8d5b7dc` (TASK-395 инфра)

## 1. Что это

JIT-девиртуализация item-лейна entity-tick (АРХИТЕКТУРНАЯ ЗАМЕНА call-структуры,
не диета). ТОП-1 ботлнек базлайна (банк v4, fp=4): items/ItemEntity.tick =
**31.17% java (36051/115655)**, ~70% популяции 150k = item-сущности. Точка —
ЕДИНСТВЕННЫЙ megamorphic-диспетч цикла:

```
ServerLevel.tickNonPassenger(Entity)  [javap bc 80, fixture-кенсус: ровно 1 site]
    invokevirtual net/minecraft/world/entity/Entity.tick()V   ← MEGAMORPHIC
        → ретаргет на invokestatic RegionTickOps.entityTick(Entity)V
            if (entity instanceof ItemEntity item) item.tick();  // МОНОМОРФНЫЙ site
            else entity.tick();                                  // ванильный invokevirtual
```

Механика JIT: megamorphic site (≥3 целей type-profile: ItemEntity/Mob*/XP/…)
теряет inline-cache fast path и уходит в vtable-stub путь, а главное —
блокирует инлайн ветки (C2: "not inlineable (megamorphic)"). После сплита
item-ветка — ровно ОДНА цель на site (invokevirtual ItemEntity.tick) →
monomorphic direct call + инлайн всей цепочки tickNonPassenger → entityTick →
ItemEntity.tick в фрейм воркера. Пруфы (живые URL, curl-проверены):
RESEARCH-F.md (/home/z/rounds/ROUND-396/RESEARCH-F.md) — Shipilëv Quark #16
(megamorphic IC→VtableStub, perfasm-лог), Black Magic Method Dispatch
(inline-cache механика), HotSpot Inlining wiki (C2 bytecodeInfo/devirt),
Wikipedia Inline caching, mechanical-sympathy тред (bimorphic-сплит).

## 2. Паритет (бар: median-exact, порядок тиков)

- НЕ физический сплит списка: type-test внутри вызова — порядок тиков, состав
  и обвязка бит-в-байт ванильные (lambda$tick$4 гейты isRemoved/isEntityFrozen/
  checkDespawn/vehicle, guardEntityTick crash-wrap + midTick pump,
  ensureTickThread/currentlyTickingEntity/setOldPosAndRot/tickCount/
  ActivationRange.checkIfActive/postTick/пассажиры — ВСЁ остаётся ядром).
- Байт-контракт: receiver-prepended static, идентичная stack-форма — тот же
  приём, что forEach/BU-DEFER/guardEntityTick ретаргеты (банк v4).
- item.tick() — виртуальный вызов того же override: даже при появлении
  подкласса ItemEntity поведение валидно (site становится bimorphic).

## 3. Гейт и флаг-дисциплина

- `CRUSSTY_LEVER_FLAG == "items_mono"` (run_world3.sh:459 экспортит инпут
  world-bench-parallel.yml). Без флага — байты ядра бит-в-байт ванильные,
  entityTick никогда не вызывается (dormant-invisible).
- Rust: `src/items_mono.rs` — владелец флага + наблюдаемость (`LEVER ARMED`
  / `dormant` маркеры); compose-патч `classfile.rs::
  patch_serverlevel_entity_tick` вклеивается в ServerLevel-байты в
  `region_threads.rs::activate` (после forEach и опционального BU-DEFER —
  один hook serve, один retransform), строгий Retargeted{1}, fail-closed
  (shape mismatch → весь hook dormant).
- Java: тело-мост `RegionTickOps.entityTick` (класс уже дефайнится в kernel
  loader — ZERO новых bridge-классов, ZERO NCDFE-поверхности); [S7-F]
  ARMED-маркер при bridge clinit; телеметрия batch_collector печатает
  items_mono-состояние.

## 4. Верификация (локально, до диспатча)

1. **javap** скомпилированного моста: `instanceof ItemEntity → checkcast →
   invokevirtual ItemEntity.tick / else invokevirtual Entity.tick`, major 65.
2. **Pure-java харнесс** `scripts/run_items_mono_harness.sh` →
   `entityinside/harness/ItemsMonoHarness.java` (INJECTS-ONLY, real kernel jar):
   - GATE1 structural: байты моста define+link против реального ядра,
     entityTick public static void (стек-форма контракта ретаргета);
   - GATE2 wiring: ItemEntity type-ref + entityTick символ в байтах моста;
   - GATE3 behavioral: РЕАЛЬНЫЙ байткод entityTick на child-first shadow
     loader (stub Entity/ItemEntity/OtherEntity) — детерминированная
     70/30 популяция (seed 42): **1000/1000 тел тика идентичны ванильному
     виртуальному диспетчу, порядок сохранён, 700 mono-lane / 300 vanilla**.
   Результат: ALL GATES GREEN.
3. **cargo-тесты** (classfile.rs, гоняет CI-сборка компиляцией):
   `items_mono_serverlevel_retargets_exactly_one_entity_tick` (fixture
   ServerLevel.class — ровно 1 site + idempotence + Methodref-аудит) и
   `items_mono_composes_with_region_tick_foreach` (совместимость с банком v4
   на одних байтах).

## 5. Ожидания и вердиктная рамка

- Потолок (RESEARCH-F §4): консервативно +8..15% TPS-эквивалента; зачёт ≥ +5%,
  амбиция ≥ +15%, < +3% = честный RED. Измерение — только собственный ран
  (world-bench-parallel.yml, ref = эта ветка, банк v4-флаги + lever_flag).
- items-лейн в профиле может НЕ упасть (это dispatch-оптимизация: метод
  ItemEntity.tick исполняется как исполнялся) — верить TPS dual-bar.
- RED-гейты: STW total > 18.8s / heap > 10G.

## 6. Состав ветки

- `entityinside/net/minecraft/world/entity/RegionTickOps.java` — entityTick
  + ITEMS_MONO-парс + [S7-F] маркер + телеметрия; пересобранный
  `entityinside/build/.../RegionTickOps.class` (major 65, Mut/Guarded не
  тронуты).
- `src/classfile.rs` — patch_serverlevel_entity_tick + 2 теста.
- `src/region_threads.rs` — compose-блок ITEMS-MONO (strict, fail-closed).
- `src/items_mono.rs`, `src/lib.rs` — флаг-владелец/observability + wiring.
- `entityinside/harness/ItemsMonoHarness.java`,
  `entityinside/harness/itemsmono/`, `scripts/run_items_mono_harness.sh` —
  оффлайн-харнесс (прогон: scripts/run_items_mono_harness.sh).
