# S32 UNLOAD-BUDGET RESEARCH (ROUND-468) — Moonrise max(50, backlog*0.05)

## Вердикт: REFUTED_CENS (офлайн, javap+модель; CI-ран = A/A-ценз, см. preregistration)

1. Формула УЖЕ в кернеле: ChunkHolderManager.processUnloads() — javap
   patched-kernel.jar (== CI versions/1.21.10/purpur-1.21.10.jar): backlog = сумма
   ChunkUnloadQueue.sections; bipush 50 @129; iload_3 i2d; ldc2_w 0.05d @133;
   dmul @136; d2i @137; Math.max @138 -> istore budget @141. Переносить нечего
   (baseline == Moonrise, Л43).
2. Вызов 1x/тик main-thread: ChunkMap.tick(BooleanSupplier) @42 ->
   ChunkMap.processUnloads(BooleanSupplier) @12 -> ChunkHolderManager.processUnloads.
3. Ранний return при backlog=0: offsets 119-123 (iload_3; ifgt 124; return).
4. Единственный производитель очереди: ChunkUnloadQueue.addChunk — 1 коллер
   NewChunkHolder (бинарный скан 9809 классов кернела); вход требует падения
   ticket-level (is_unload_safe=ticket_level).
5. Модель backlog при r640/150k: radius 640 -> TILES=ceil(640/256)=3 -> 6x6 tiles =
   36 cmds -> 96x96 = 9216 chunks forceload (workflow L44 "~9.2k"; RESULTS_LEDGER
   run5 "36 forceload cmds = 9216 chunks"); 0x "forceload remove" в run_world3.sh;
   fake_players=4 one-shot teleportTo (BenchFakePlayersPlugin:164, positional
   anchors), view/simulation-distance=10 -> player-тикеты ПОД forceload-квадратом;
   observed ticket-level 31/31 константен (NewChunkHolder dumps 2026-09-17) ->
   backlog(t) == 0 весь ран => budget never binds.
6. Гипотетический one-shot backlog 9216 -> budget max(50,460)=460 chunks/тик ->
   слив 9216/460 = 21 тик (~1.05s @20TPS) — вне окна в любом случае.
7. Остаточная цена мёртвого лейна: retrieveForAllRegions (ArrayList + пустой
   hashtable iter + sort) ~0.1-0.2 us/тик -> <=0.0004% ядра -> dnorm ~0.000 пп.
8. Профиль-ценз (негативная): chunk-system 6.4-13.9% scene (RECON-24/29) разбит
   на broadphase 4.90/5.14%, PalettedContainer.get 3.57/3.58%, getBlockState ~1.0,
   readPalette 0.6 — processUnloads/unload-фреймы ОТСУТСТВУЮТ (0 сэмплов).
9. Потолок capture: лейн% (0.0004 предельный, фактически 0) x 100% = 0.0004пп << 20пп
   при A/A-шуме ±10пп (S35/S36) и bank v5 MAE 9.12пп — порт-код-дельта = placebo A/A
   (урок navmath-1: не диспатчить несуществующую дельту).
10. Где формулой СТОИТ заниматься: 19b энт-лестница с реальным churn тикетов
    (despawn-storm/400k) — при backlog>0 floor 50/тик = 1000 chunks/s слив;
    сегодня (r640/150k/300s) поверхность = 0.

## Preregistration этого рана (docs-only commit поверх c1196321, код байт-идентичен)
- Прогноз: norm_v5 в ваниль-коридоре дня (-13.22..+12.09), направленности НЕТ;
  band 6.0-9.5M; это vanilla-on-base датум №3 для PHASE-3 вопроса базы МЕРЖ №10
  (canary-406 -13.32 @6.92M; master-2635 -13.61 @6.97M).
- REFUTED-критерий формулы (если бы несли дельту): norm вне ±10пп коридора -
  интерпретировать шумом/host, не эффектом формулы (поверхность мертва).
