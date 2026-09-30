# WAVE_MEMORY — волна-515 → волна-516 (консолидация MAIN ×515, 2026-10-01)

## База лестницы S (честные числа)
- S_515 = 47.73 (TPS@150k ≈20.9 + ch/s@chunk-gen 22.0 + dp@50k ≈4.8); цель +20% = 57.28
- Волна-515 S НЕ подняла (ΔS=0 — все ноги в полёте); лестничный вердикт: REFUTED_CENS (AG-310/368: реализм волны +14.6%, P_joint≈0.004, честный потолок посчитан)
- **BENCH-V2 РОЖДЁН** (~60 независимых веток-стендов). Первые базы:
  - AG-68: 449.12/433.90 ch/s marked-rate (gen-only, bim≥15s = GEN-LIMITED канон)
  - AG-283: 13.00 ch/s total (ov 11.35 / nether 18.45 / end 15.07, boot 30.8s, 20367/20368)
  - AG-426: 7.97 ch/s full-pressure (per-dim 6.41/11.81/10.64, batch-peak 51.2)
  - AG-391: потолок sync-bulk 8.06 ch/s (capture-матем: 20367/2528s)
- Ветки-ядра BENCH-V2: канон-линия AG-433 @86c2d29 + AG-104 @4508dce; arbiter AG-72; pair-math AG-397; ladder-вердиктор AG-448

## Инфра-уроки ×515 (каноны, не наступать повторно)
1. **sync-`/forceload` на main-thread = FAKE-GREEN**: success-раны с 0 heartbeat-сэмплов/0 метрик; фикс async-драйвера готов (AG-211/234/319/426 патчи) → мёрж-кандидат ×516
2. **Tectonic 3.0.29 пин = FATAL-алиас** на 1.21.10 (fmt121/gv26.3, молча vanilla) → канон Tectonic 3.0.25 sha512 7b3c5dee… (AG-192/266/313/386/395/410/412)
3. vanilla `/forceload` = overworld-only; nether/end только плагином `setChunkForceLoaded` (AG-248/420/182) — «20k×3dim одновременно» без плагина = молча 20k×1dim
4. **0 игроков → natural spawn ≡ 0**: «бешеный спавн» требует fake-players (BENCH-4 паттерн) или summon-storm (AG-76/187/228/342/380)
5. Tectonic∪Terratonic требует Lithostitched-DATAPACK — на Modrinth НЕ существует (только mod-jar) → lithofree-ребилд/санитизация (AG-117/130/403/434)
6. concurrency per-ref: 2 диспатча на одну ветку = канцел первого (~14 подтверждений) → **1 нога = 1 ветка-алиас** (Л188b канон волны)
7. branch-only workflow: dispatch 404/422, пока yml не в default-ветке → push-триггер / plumbing / мёрж yml в master (AG-90/288/328/344/347)
8. **POI off-main crash** (`Accessing poi chunk off-main`, Feature placement nether) — новый STZ ×515 (AG-222/258/308)
9. git-plumbing offline-канон при df>90%: commit-tree/hash-object + POST refs FULL-sha + GET-verify; sparse-worktree `git add` сносит дерево (AG-64/86/155/403/155)
10. **band-кризис**: новая генерация ubuntu-latest cpu_index ~11.2M ≫ канона 9.5M → 2/4 ног BAND-DISCARD (AG-418); calib runs-on-pin на ветке
11. seed-коллизии ×19/8 групп (SEED-LEDGER AG-173/340) → seed-gate в dispatch-скрипты ×516
12. W466-REPAIR: мир world466-stress-v1 с битым level.dat жёг ~6 ног (AG-355 фикс-режим готов)

## Горячие вилки ≥+20 (наследованы + обновлены волной)
- **компо SWAR-X re-arm ⊕ H07 Hilbert**: +16.3..+28.8, центр 22.6 — единственный ≥+20 MERGE-кандидат ×516 (AG-368; H07-спека AG-108, lever cmp515_h07hilbert)
- W8-φ wiring: ханки доставлены + блоб-ребилд, leg-1 run-36755645983 в полёте (AG-48/165) → min-of-3 при CLEAN
- №24 GATE-3 (≥22.74): ~20 новых легов залпом волны (seeds 1669-1907 @3fefb39) — харвест ×516; SWAR-X re-arm соло REFUTED min-of-3 = +9.12 (AG-452)
- dp G-B2: k=8/22 q05 0.2140 **HOLD** (NO-FIRE 4/4 dp17-20; AG-26/166) → редирект dp-слотов
- pack-guard WILD-01: G-B PASS popcnt/ents 0.51→22.02 (AG-140), G-C min-of-3 не закрыт → leg-2 i64 CSR (AG-40)

## Абсорб волны-514 (in-flight)
- 82/100 terminal (65 success/17 fail/18 in_progress) — числовой харвест банка ×516 (артефакты ≥96 ax-ног @7a62df9)

## Следующий размер волны (закон 7)
- Волна-515 НЕ чистая (1 context-deadline FAIL + 40 launch-truncation платформой) → **волна-516 = 500**
- Чистая волна → 600; платформенный потолок одного tool-блока зафиксирован: ~460 Task-вызовов

## Анти-повторы ×516
- НЕ плодить ещё 60 bench-v2-ядер: выбрать 1 канон (линия AG-433/104 + async-фикс + плагин-dim-forceload + fake-players), мёрж в master, остальным агентам — A/B-ячейки/пары поверх канона
- диспатч-норма ≥100 CI перевыполнена (~700 run-id волной); флот 43-49 — залпы ≤40 канон
- BENCH-V2 pair-math до async-фикса драйвера ЗАПРЕЩЁН (все success-раны FAKE-GREEN класс F)
