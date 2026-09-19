# RECON-32 — дрилл inside-gate-семьи (NEXT 355) + вердикт inside-gate-диеты

## s7194 (сцена 128104)

| домен | сэмплов | % сцены |
|---|---|---|
| vanilla-discovery (движущиеся) | 11638 | 9.08% |
| effects-apply | 2815 | 2.20% |
| other | 1 | 0.00% |
| context-tail | 1 | 0.00% |

### vanilla-discovery лист-хвосты (топ-8)

-  0.63%    807 add
-  0.62%    791 gate
-  0.57%    735 <init>
-  0.54%    691 get
-  0.48%    616 computeNext
-  0.42%    532 flushStep
-  0.36%    457 lambda$checkInsideBlocks$2
-  0.29%    376 G1SATBMarkQueueSet::filter

## s7189 (сцена 127150)

| домен | сэмплов | % сцены |
|---|---|---|
| vanilla-discovery (движущиеся) | 11898 | 9.36% |
| effects-apply | 2909 | 2.29% |

### vanilla-discovery лист-хвосты (топ-8)

-  0.72%    913 <init>
-  0.71%    899 add
-  0.61%    775 gate
-  0.57%    729 get
-  0.56%    715 computeNext
-  0.38%    481 flushStep
-  0.35%    448 isEmpty
-  0.31%    389 lambda$checkInsideBlocks$2

## ВЕРДИКТ (клейм 3.1% ОПРОВЕРГНУТ в большую сторону + переклассификация)

1. Inside-семья на CPU-оси = **11.28/11.65% сцены** (s7194: discovery 9.08 +
   effects 2.20; s7189: 9.36 + 2.29) — в 3.7× больше клейма «inside-gate ~3.1%»
   в списке ЭПОХИ-2A (клейм считал только guard-хвост, RECON-27b ~0.8%;
   полная семья = vanilla-discovery для ДВИЖУЩИХСЯ + applier + банк-инфра).
2. Структура (листья, s7194): PalettedContainer.get 855, visit-set add 819,
   Recorder.flushStep 806 + gate 791 + <init> 747 (банк-инфра ~1.8%),
   applyEffectsFromBlocks 658, guava-iterator computeNext 616,
   lambda$checkInsideBlocks$2 457, isEmpty 444, vtable 409, G1SATB-barrier 390,
   forEachBlockIntersectedBetween 331. Стеки чистые: tick →
   applyEffectsFromBlocks → checkInsideBlocks → forEachBlockIntersectedBetween
   → visit (ItemEntity/Mob-муверы каждый тик).
3. Статусы под-кусков: (a) visit-set/iterator-интернал = внутренность
   REFUTED #9 flat_traversal (RECON-25) — повторный рычаг ЗАПРЕЩЁН;
   (b) alloc-диета RecordedEffect = REFUTED #12 (alloc −47%/CPU −66% →
   TPS −17..−6%); (c) effects-apply = ванильная семантика парити
   (freeze/honey/fire) — не убирается; (d) банк-инфра = собственный банкинг.
4. ЕДИНСТВЕННЫЙ легальный рычаг семьи = **секционный solid-bitmask
   пре-фильтр** (event-driven dirty-флаги на изменения блоков — санционировано
   миссией): short-circuit checkInsideBlocks/applyEffectsFromBlocks, когда все
   секции расширенного AABB чисто-воздушные. Сырой потолок = discovery ~9.1%
   сцены (воздушная доля муверов); TPS-конверсия по эмпирике диет-семейства
   ~0 (5 контрпримеров: структурный эффект подтверждён, TPS не растёт);
   MSPT-пропорциональная верхняя оценка ~13-16% TPS недостижима эмпирически.
   НА ТЕКУЩЕМ ПРОТОКОЛЕ (ДВОЙНОЙ БАР) НЕ БАНКИНГУЕТСЯ; **при переходе
   владельца на вариант B (min-MSPT) — главный кандидат-флагман**.
5. ВЕРДИКТ: «inside-gate-диета» как диета = **7-я ДОК-ВЕРИФИКАЦИЯ закрытия**
   (прецедент RECON-17/20/23/26/30/31/32) — ни один безопасный под-кусок не
   обещает ≥+10% обеих осей. Компонент A «inside-gate 3.1%» переклассифицирован:
   семья 11.3/11.6%, легальный потолок ~5-6% TPS (сырой ~9% CPU при
   конверсии ~0). Итог ЭПОХИ-2A: все компоненты проверены (broadphase ≤3%
   RECON-30, volatile 0-1% RECON-31, inside — здесь, контур 1.1% RECON-28,
   chunk-read 3.6% RECON-29) — суммарный ЛЕГАЛЬНЫЙ потенциал < двойного бара
   при конверсии ~0. **Вариант A окончательно не рекомендован; решение
   владельца между B (флагман = inside-bitmask) и C.**
