# ABSORB S7-163 leg#2 (35410873485) — FLAT-TRAVERSAL (рычаг #9) — ВЕРДИКТ

Тик 09:08 +08 (2026-09-19), Job 396026, TASK-310. Лега #2 = v4-кандидат
(v3 + flat_traversal=1, head 61f93d6), старт 08:53:29 +08, финиш 09:09:40 +08
(16.2 мин), conclusion **success**. База сравнения: CUMULATIVE v3 (leg
35399980345 = run-s7162-leg2-artifact). Инструмент: `absorb_s7163_leg2.py`,
сырой вывод `ABSORB_S7163_LEG2_raw.txt`.

## Гейты (preregister, GOAL СТАТУС S7-163 — НЕ менялись)

| гейт | условие | факт | вердикт |
|---|---|---|---|
| PG2 | 0 NCDFE, pop 150k VALID, ARMED chain [+traversal] rc=0, nested-маркеры | armed=True; «defined nested …TraverseOps$LongTable» + «(+1 nested)»; pop 150000 VALID; NCDFE=0 | **PASS** |
| PG3 | TPS last-5 медиана ≥ 1.60 | 1.700 (n=6) | **PASS** |
| PG4a | traversal-лейн ≥ −50% (per-work) | 9572 → 7314 (−23.6%); per-work 5318 → 4302 (−19.1%) | **FAIL** |
| PG4b | orchestration-хвост ≥ −70% | 1168 → 2 (**−99.8%**) | PASS |
| PG4c | young GC ≤ 154 | **129** (−16%), 0 Full; total STW 21.13s → 16.99s (−19.6%) | PASS |
| CRASH-FREE | 0 crash-report / 0 NCDFE / 0 Full | всё чисто; 5 отловленных «Entity threw exception» (исторический шум параллельного тика: s7160=1, s7161=3, v3=0; стек идентичен s7161 — sendBlockUpdated/fastutil, НЕ traversal) | **PASS** |

**ИТОГ: FAIL (по PG4a) → по банкинг-правилу CUMULATIVE v4 НЕ банкуется;
flat_traversal=0 (банк остаётся v3); TraverseOps остаётся инфраструктурой.**

## Разбор PG4a — почему лейн не сжался на 50%

1. **Порог был откалиброван на leg#5-профиле** (v2-банк): orchestration-хвост
   оценивался в 2900–3400 из 9286 сэмплов (31–37% лейна). К моменту traversal-лега
   внутри уже-банкованной v3 orchestration-часть была 1168/9572 = 12% лейна —
   часть хвоста съели inside_cache/batch-рычаги. Порог −50% от лейна стал
   архитектурно недостижим: max возможный выигрыш ≈ 12–15%.
2. **Масса лейна — не orchestration**: leg2-лейн 7314 = flat self (forEachCorners
   472 + LongTable.add 335 + floor/DDA) + visit-обработка, которую parity
   требует сохранить (PalettedContainer.get 270, getBlockStateFinal 217,
   lambda$checkInsideBlocks$2 401, flushStep 369, внешний vanilla LongSet.add
   466, collision-temps Vec3.add 165).
3. **Рычаг в точке приложения сработал честно**: guava-итераторы −99.8%,
   young GC −16%, STW-время −19.6%, alloc-доли семьи упали
   (AABB 20.2%→12.3%, BlockPos 16.3%→6.0%, Vec3 19.9%→13.1%).
4. TPS 1.70 vs 1.80 (n=1/1, исторический разбег 1.6–1.9) — PG3 PASS
   (абсолютный порог), но эффекта TPS не видно: выигрыш ~1.8% CPU не
   превышает шум.

Этика: гейт НЕ переписывается пост-хок; вердикт FAIL фиксируется как есть.
Урок эры №6: **порог гейта, калиброванный на профиле ДО-банкованной фазы,
молча устаревает после каждого забанкованного рычага — пороги надо
выражать от свежего профиля лега-базы, не от исторической оценки.**

## Свежий ТОП leg#2 (методика «ТОП-ПОЖИРАТЕЛЬ → ∞»)

Ось (a) CPU (125610 сэмплов): entity-фаза (tickBucket lane) **60.16%** →
GC/JIT 33.11% (было 35.49%) → tracker 2.08%.

Под-лейны entity-фазы (грубые маркеры, full RECON — следующий тик):
- movement/AI-aiStep **20.33%** (25537)
- broadphase/collisions 11.26% (2×REFUTED — кэш-классы не трогаются)
- fluid-push (updateFluidHeight) **9.76%** (12258)
- inside-pipeline (visit/getBlockState/LongSet) **8.00%** (10043)
- item-entity 7.19% (9030); AI-goals/brain 0.68%; nav 0.51%
- unclassified **41.91%** (52649) — требует RECON-разложения

Ось (b) alloc: семья AABB+Vec3+BlockPos = **31.4%** (было 56.2%); лейны:
inside-pipeline 21.7% аллока, tickBucket-orch 20.7%, fluid-push 6.6%.
Ось (c) gc.log: young 129, медиана паузы 152.6ms, total STW 16.99s.

## NEXT

ТОП-1 = entity-фаза. Кандидат-рычаг #10 ZERO-ALLOC-INSIDE (RECON6) попадает
в fluid-push 9.76% + inside-pipeline 8.00% + collidedWithShapeMovingFrom-сайты
в movement — двойной эффект (CPU лейнов + alloc/GC-ось). Также в
preregister S7-163 прямо назван «следующий слой»: collidedWithFluid/
getBlockState под visit-лямбдой. Следующий тик: RECON-7 (разложение
unclassified 41.9% + уточнение под-лейнов leg2-профиля) → фиксация #10 или
крупнейшего ≥5% под-лейна → javap-контракт → оракул → dispatch после
пересортировки по методике.
