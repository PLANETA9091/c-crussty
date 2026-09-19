# RECON-7 (краткий) — разложение «unclassified» leg#2 — тик 09:08 (TASK-310)

Вход: cpu-collapsed leg#2 (125610). Цель: проверить гипотезу о 41.9%
неразобранного MC-хвоста entity-фазы (маркер-классификатор lanes) и
уточнить прицел рычага #10.

## Итог

«Unclassified» 52736 сэмплов (RECON7-маркеры) — это НЕ неразобранный
MC-хвост: 55%+ из них — GC/JIT-фреймы (oopDesc::size 11.00%,
OopOopIterate<G1ConcurrentRefine> 10.47+2.78%, G1CardSet 5.35+2.20%,
G1RemSet 3.53%, trim_queue 2.83%, RebuildRS 2.84%…), то есть осевой
классификатор (a) уже отнёс их к фазе GC/JIT (33.11%). Расхождение —
разные списки маркеров (RECON7-«known» = только MC-лейны).

Реальные MC-полоски внутри (≤3% total CPU вместе):
- RegionTickOps.bucketOf 1.45% (orch)
- ServerEntity.sendChanges 1.19% (tracker-смежный)
- Entity.setOldPos 1.10%
- ChunkMap$TrackedEntity.clearPlayers 1.08%
- Vec3.distanceToSqr 0.75% (movement/inside sites)
- QuickSort::inner_sort 1.56% (JIT/sort infra)
- WallClock::signalHandler 2.02% (profiler thread — не цель)

## Вывод для ТОП-пересортировки

ТОП-1 = entity-фаза 60.16% подтверждена в составе:
movement/AI-aiStep 20.33% > broadphase 11.26% (2×REFUTED) >
fluid-push 9.76% > inside-pipeline 8.00% > item-entity 7.19% >
мелкие (bucketOf/setOldPos/sendChanges/clearPlayers ~1% каждый).

Рычаг #10 ZERO-ALLOC-INSIDE прицеливается в fluid-push 9.76% +
inside-pipeline 8.00% + collidedWithShapeMovingFrom-сайты внутри
movement-лейна (20.33%): те же makeBoundingBox/deflate/subtract/
collidedAlongVector/clip-цепи. Двойной эффект (CPU лейнов + alloc/GC).
Альтернатива — отдельный рычаг по movement/travel (ванильная физика)
— крупнее, но без готового javap-контракта; по методике следующий
тик: фиксация гейтов #10 от СВЕЖЕГО профиля (урок №6) → оракул →
диспатч.

Артефакты: RECON7_unclassified_leafs.txt, CONTRACT_ZEROALLOC_S7164.txt
(12 методов verbatim: collidedWithFluid, collidedWithShapeMovingFrom,
updateFluidHeightAndDoFluidPushing, collidedAlongVector, contains ×2,
inflate, getCenter, clip ×2, getDirection, clipPoint).
