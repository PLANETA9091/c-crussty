# ORACLE: INSIDE-BITMASK LOCKSTEP — ALL PASS (lever #15, TASK-358)

**Дата**: 2026-09-20 ~09:3x+08 · **Скрипт**: `scripts/run_inside_bitmask_lockstep.sh`
(одна команда) · **Харнесс**: `entityinside/harness/InsideBitmaskLockstepHarness.java`
(пакет `net.minecraft.world.entity` — прямой доступ к package-private
`Entity.Movement` и `InsideBitmaskOps.sweptHullInto`) · **Ядро**: s7194
patched-kernel.jar (тот же, с которого снят RECON-33 контракт).
Прецедент: TravelDietLockstepHarness (v2a ×29, GOAL ×29).

## Что доказано офлайн против РЕАЛЬНОГО ядра

1. **STRUCTURAL + ARM**: `InsideBitmaskOps` линкуется против реального kernel
   jar; `armState() == "ARMED"` — MethodHandle к private
   `checkInsideBlocks(List, StepBasedCollector)` и Unsafe-offset
   `ChunkAccess.sections` резолвятся на реальных классах (точный live-путь
   probe-then-patch драйвера inside_bitmask.rs).
2. **HULL-SUPERSET (ядро median-exact)**: 1,000,000 рандомизированных и
   адапверсариальных кейсов (walk-дельты гауссовы/осевые, zero-move
   betweenClosed-путь, телепорт-масштаб ×100, суб-deflate микро-шаги
   2e-5, лендинги на chunk/section-границах ±{1e-9, 1e-6, exact deflate
   9.999999747378752E-6, ±0.4999}, отрицательные координаты, мульти-Movement
   тики ×1..3): **210,513,534 позиций посетил РЕАЛЬНЫЙ
   `BlockGetter.forEachBlockIntersectedBetween` — КАЖДАЯ внутри
   блочного домена hull ячеек РЕАЛЬНОГО `InsideBitmaskOps.sweptHullInto`**
   (та же формула, что ест в hot path через HULL_TL scratch). ⟹ если ячейки
   hull all-air, то КАЖДЫЙ vanilla-визит — air-визит (lambda bc 34..65:
   возврат до любого наблюдаемого действия) ⟹ skip ≡ vanilla по наблюдаемому
   состоянию (эффекты/collector/visitedBlocks/бюджет нетронуты — пины
   RECON-33 §2).
3. **SECTION TRUTH**: 20,000 последовательностей на РЕАЛЬНОМ
   LevelChunkSection + РЕАЛЬНОМ PalettedContainer (249,217 реальных
   setBlockState-операций, палитра air/stone/water/lava/oak_leaves/
   powder_snow): `hasOnlyAir() == ручной all-air скан 4096` И
   поле `nonEmptyBlockCount == ручной пересчёт !isAir` — бит-согласованно.
   Флюиды НЕ-air → водные секции принципиально не скипаются.

## Биом-контейнер (офлайн-нюанс)

В 1.21.10 нет статического biome-реестра (worldgen = datapack), поэтому
LevelChunkSection строится напрямую из двух PalettedContainer
(`Strategy.createForBlockStates(Block.BLOCK_STATE_REGISTRY)` + raw-cast
биом-strategy с null-дефолтом — биом-контейнер тестом не трогается). Библиотеки
ядра: paperclip INSTALL-ONLY (`java -Dpaperclip.install=true`, НЕ boot —
INJECTS-ONLY дисциплина).

## Честный резидуал вне офлайн-охвата (дисциплина TravelDiet)

- `Level.getChunk(II,ChunkStatus,Z)` против живого ServerLevel chunk-map
  (та же инфраструктура, что у getBlockState; null-чанк и выход за индексы —
  fail-closed в ваниллу);
- ветка активных debugSubscribers (гейт возвращает false → ванилла);
- живой поток эффектов (нужен booted мир) — обеспечивают гейты CI-лега на
  реальной сцене 150k (median-exact дисциплина).

## Статус

INSIDE-BITMASK (#15) = реализация (6eb3274+1c703f7) + оракул ALL PASS,
dormant (env `CRUSSTY_INSIDE_BITMASK`). Активация = решение владельца
option B (min-MSPT флагман, сырой потолок ~9.1% CPU, TPS-конверсия ~0 —
под ДВОЙНОЙ БАР не банкингуется, вердикт RECON-32/33 стоит).
