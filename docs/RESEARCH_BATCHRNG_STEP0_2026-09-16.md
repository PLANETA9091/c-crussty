# BATCH-RNG STEP-0: байткод-контракт + REFUTATION (task165/S7-96)

Дата: 2026-09-16. Данные: run#11 (93,223 cpu samples) + байткод из материализованного
booted-kernel (purpur-1.21.10.jar, sha256 e2992d63abd2c254…, paperclip+cache
детерминированный = идентичен CI-буту; скрипт scripts/materialize_kernel.sh).

## 1. Контракт RandomSource-машины lane (из байткода, НЕ из догадок)

`ServerLevel.optimiseRandomTick(LevelChunk, int randomTickSpeed)` — полный псевдокод:

```java
// ca.spottedleaf.moonrise SimpleThreadUnsafeRandom simpleRandom (field)
sections = chunk.getSections(); minSection = WorldUtil.getMinSection(this);
boolean fluidFollow = !PlatformHooks.get().configFixMC224294();
for (int s = 0; s < sections.length; s++) {
    int yBase = (s + minSection) << 4;
    LevelChunkSection sec = sections[s];
    if (!sec.isRandomlyTickingBlocks()) continue;              // moonrise block counting
    ShortList ticking = ((BlockCountingChunkSection) sec).moonrise$getTickingBlockList();
    for (int i = 0; i < randomTickSpeed; i++) {
        int pick = simpleRandom.nextInt() & 4095;              // <- RNG шаг
        if (pick >= ticking.size()) continue;                  // 12-бит индекс вне списка
        int packed = ticking.getRaw(pick) & 65535;
        BlockState st = (BlockState) sectionStates.get(packed); // PalettedContainer.get
        BlockPos pos = new BlockPos(                            // <- verified ALLOC SITE
            (packed & 15) | chunkX, ((packed >> 8) & 15) | yBase, ((packed >> 4) & 15) | chunkZ);
        st.randomTick(this, pos, simpleRandom);                 // JVM-тело (do-not-duplicate)
        if (fluidFollow) { FluidState fs = st.getFluidState();
            if (fs.isRandomlyTicking()) fs.randomTick(this, pos, simpleRandom); }
    }
}
```

`SimpleThreadUnsafeRandom.advanceSeed()` — точный 48-бит LCG (равен java.util.Random):

```
value = (value * 25214903917L + 11L) & 281474976710655L;   // (1<<48)-1
nextInt() = (int)(advanceSeed() >>> 16);
```

Байткод-доказательства: `research/rng-recon-2026-09-16/*.javap`.

## 2. Kill-gate арифметика (run#11, prereg >= 3.0% MSPT)

| статья | % CPU тика |
|---|---|
| lane presence (optimiseRandomTick ancestry) | 5.46% |
| SELF цикла | 2.36% |
| advanceSeed leaf (RNG, JIT-инлайн nextInt→advanceSeed) | 1.57% |
| world-reads leaf (PalettedContainer.get/SimpleBitStorage и др.) | 1.40% |
| прочие (ShortList.getRaw, Vec3i.<init>) | 0.13% |

Replaceable для batch-RNG = advanceSeed (1.57%) + RNG-доля SELF. Верхняя граница
(RNG+весь SELF = 3.93%) НЕДОСТИЖИМА: JVM-сторона обязана итерировать пики, читать
PalettedContainer, аллоцировать BlockPos и диспатчить randomTick — весь остаток
SELF остаётся. Честный потолок: **1.6-1.9% CPU ≈ 1.3-1.6ms @ 84ms < 3.0% гейта**.

### ВЕРДИКТ: BATCH-RNG REFUTED (пре-код). Контракт и LCG-ядро задокументированы
для будущего использования (GC-SHAPE-1 использует тот же JNI-батч-паттерн).

## 3. Verified alloc site в lane (единственный в lane через javap-скан)

`new BlockPos` на КАЖДЫЙ пик внутри optimiseRandomTick (offset 180-213) —
alloc-профиль: optimiseRandomTick = 2.36% топ-лист аллокаций. Это вход в
**GC-SHAPE-1** (task166): устранение аллокаций verified-сайтов серией.

## 4. Калибровка alloc-профиля (критично для будущих раундов)

alloc-collapsed leaves НЕ являются точными new-сайтами: `PalettedContainer.get`
(3.10% топ-лист) — 0 `new` в happy-path байткоде; `AABB.intersects` (1.39%) —
чистая математика без аллокаций. Вывод: alloc-профиль = давление/корреляция
(TLAB-refill окна), карту сайтов даёт ТОЛЬКО javap `new`-скан. Все прошлые и
будущие "alloc site" интерпретации сверяются по байткоду.

## 5. Рекон-инфраструктура (banked)

- Recon bug #2 исправлен в world-bench.yml: find тестировал бы content
  (net/minecraft/server/level/ServerLevel.class) каждого jar-кандидата, а не
  `head -1` (run#12 взял ванильный cache/mojang_1.21.10.jar вместо
  versions/1.21.10/purpur-1.21.10.jar).
- Оффлайн-материализация кернела: scripts/materialize_kernel.sh (paperclip
  против cache/mojang_*.jar, убит ДО server-main — не бут; JDK/javap через
  scripts/ensure_javap.sh в /tmp/jdk21).
