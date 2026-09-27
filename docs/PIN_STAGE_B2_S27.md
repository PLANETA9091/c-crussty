# PIN-STAGE B-2 (S27, ROUND-472): BatchCollector family — BIT-PIN RECORD

- Клейм T-472-S27: пины 44/90 → 46/90. Канон Л258/S59-S64; anti-Л216 bit-пин.
- Семья (java-бридж): BatchCollector (zero-map ctor-retarget, embed
  src/batch_collector.rs:42 → NESTED blob) + FlushOps (flush-diet retarget
  target, embed src/flush_diet.rs:49 → NESTED-only, flat-сиблинга никогда
  не существовало → flat==nested n/a by design, embed-путь = nested-путь).

## javap-контракт (javap -p, blobs f39389a6)
- BatchCollector extends InsideBlockEffectApplier$StepBasedCollector:
  public BatchCollector(); public static long instances();
  public void advanceStep(int, net.minecraft.core.BlockPos);
  public void apply(net.minecraft.world.entity.InsideBlockEffectType);
  public void runBefore/runAfter(InsideBlockEffectType, Consumer<Entity>);
  private void flushStep(); private void appendConsumer(Consumer<Entity>);
  private void appendEffect(int, long); private void grow();
  public void applyAndClear(net.minecraft.world.entity.Entity).
  Поля: ORDER/NT/INSTANCES/stepHas/stepPos/currentPacked/lastStep/
  before/after/opKind/opC/opType/opPos/nOps.
- FlushOps: private FlushOps();
  public static boolean fladd(java.util.List, java.util.Collection)
  (erased List.addAll(Collection)Z descriptor).

## Гейт-дельта (scripts/check_blobs_sync.sh, +18 OK-маркеров)
- check_class BatchCollector ×10 (entry-контракт advanceStep/apply/
  applyAndClear + flushStep/instances + runBefore/runAfter/appendEffect/
  grow/INSTANCES); check_class FlushOps ×3 (fladd-дескриптор/isEmpty/addAll).
- check_flat_matches_nested BatchCollector (flat = legacy copy, byte-identical).
- gate_load ×2; gate_source_sibling ×2 (Л180h anti-drift).
- Прогон на блобах f39389a6: ALL IN SYNC exit 0, 0 FAIL.

## anti-Л216 bit-пин (source↔blob BIT-SYNC ДО гейта)
- javac --release 21 -proc:none -cp patched-kernel.jar, rebuild ×2 независимых:
  BatchCollector 04ce702f35bf2703ead2099f4a385acc9aae9d978fa442a2d15bb7b6d778a9d7
  == tracked blob (×2); FlushOps 60fa4c55d7f98555cb938fe3b7f80eccb8fa8387d95463d4a8a398eb2a27fbde
  == tracked blob (×2). 0 const-4-классов дивергенций; набор .class == f39389a6.

## NCDFE-ценз
- CP Class-ценз BatchCollector: 0 cross-Ops bridge refs (только kernel:
  InsideBlockEffectApplier$StepBasedCollector, InsideBlockEffectType(+Applier),
  Entity, BlockPos + JDK) → arm/define-рейс невозможен by construction.
- ncdfe_guard.sh эвристика даёт FP на BatchCollector: идентификатор ПОЛЯ
  `nOps` матчится паттерном `*Ops[.:;]` (javap-комментарий Field nOps:I);
  реальных Ops-touch 0 — FP задокументирован, guard не менялся (канон).
- FlushOps: ncdfe_guard OK (touch=0).

## PIN≠ARM (закон 5)
- Дельта ветки = gate-скрипт + этот маркер-файл. 0 java/rust runtime-байт,
  lever_flag/lever_arg не тронуты, ваниль-лега ожидаемо 0-дельта
  (NCDFE=0, AIOOBE=0, selfTest=true в логе рана).
