# RECON-4 (S7-162, prep для S7-163) — TRAVERSAL-ПОДЛЕЙН inside-pipeline
Артефакт: run-s7161-batch-collector-artifact/cpu-collapsed.txt (127749 сэмплов, лег 35391679176).

## Числа (ось CPU)
- forEachBlockIntersectedBetween: **9286 сэмплов = 7.27% CPU**; caller 100% = Entity.checkInsideBlocks (движущиеся сущности — позиционно-НЕзависимая семья, не кэш-класс).
- Orchestration-хвост (заменим БЕЗ ванильной семантики — порядок блоков сохраняем бит-в-бит):
  * guava AbstractIterator.hasNext/next: 968+475 = **1443** (15.5% лейна)
  * BlockPos.betweenCornersInDirection: **765**
  * BlockPos$$Lambda.iterator ×2 + betweenClosed: **473**
  * LongOpenHashSet.add+<init> вне лямбды: **369** (per-check аллокация)
  * Σ orchestration ≈ **2900-3400 сэмплов ≈ 2.3-2.7% CPU**
- Под visit-лямбдой (5126): collidedWithFluid **1671** (32.6%), getBlockState **870**, BatchCollector.advanceStep **755** (атакован S7-162), LongOpenHashSet.add **505** (visited-set), self 406, getEntityInsideCollisionShape 176, entityInside 124, isAlive 250.

## Кандидат S7-163 — рычаг #9 FLAT-TRAVERSAL
Ретаргет ЕДИНСТВЕННОГО сайта forEachBlockIntersectedBetween в Entity.checkInsideBlocks → TraverseOps.forEachFlat: плоский long-packed обход (та же последовательность блоков бит-в-бит — javap DirectionalIterator/betweenCornersInDirection контракт), visited-dedupe инлайном (семантика сета сохранить), ванильная visit-лямбда НЕ тронута (parity surface = только порядок блоков). Позиционно-независим (не кэш — движущиеся ок). Гейты: javap-контракт порядка + lockstep-харнесс (random AABB/direction, бит-в-бит последовательность long'ов vanilla vs flat) + PG1-дайджест ретаргета.
Ожидание: −1.5..−2.2% CPU (orchestration-хвост), аллок-плюс (LongOpenHashSet per-check уходит) → молодой GC.
ОСТОРОЖНО: collidedWithFluid/getBlockState (2541) — следующий слой, ТОЛЬКО после traversal-оркестрации; кэш-классы на них REFUTED ×3.
