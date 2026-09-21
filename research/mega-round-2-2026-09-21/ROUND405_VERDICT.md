# ROUND405 VERDICT (interim → финал в конце тика, 2026-09-22 00:20→0x:xx +08, TASK-405)

## Сводка ноги за ногой (pair-by-runner, min-of-3, ARM-ПРУФ обязателен)

### 1) JNIBULK FUSED — финал min-of-4 (ЗАКРЫТО)
Серия (cmp402_comp+cmp403_jnibulk ARMED, ветки round-404-bleg*/jb*):
- jb2 2.55@6694402 → +10.9pp (anchor 2.3@6706928)
- jb3 2.3@6663598 → 0.0pp
- bleg1 2.55@6694402 → +10.9pp (GREEN-CANDIDATE, items 31.17→0.00, nav_ai 9.34, broadphase 12.50, fluid 17.41)
- bleg2 2.3@6663598 → 0.0pp (PARITY/LOW)
**ИТОГ: медиана +5.5pp, БИМОДАЛЬНОСТЬ ПО РАННЕРУ (6694402→+10.9, 6663598→0.0 — детерминированно). Вердикт: jnibulk-fused ПОВЕРХ comp ПРИБАВКИ НЕ ДАЁТ (push-слой уже закрыт comp). Вектор открыт для базы БЕЗ comp.**

### 2) STAGCOMP день-4 дрейф-контроль
- scd4 (7-я нога серии, cmp402_stagcomp @60fe902): **3.1 TPS @7115766, ARMED ×3** (cmp402_comp + cmp402_stagcomp ×2), GC total 22.3s / Full 9 (норма-банк), items→0.00%
- pair: vs 2.2@7125977 = **+40.9pp**; vs 2.40@7089030 = +29.2pp
- Серия stagcomp теперь 7 ног: −15.4 / +2.2 / +20.8 / +20.8 / +28.6 / +31.3 / **+40.9** → медиана +20.8pp (стабильна), топ-ноги 3 дня подряд ≥+28 → ступень ЗДОРОВА, шум от раннер-дня.

### 3) Якоря 405
- anchora/anchorb @6a0879f: BAND-DISCARD ×2 (runner_cpu_index=10454579 вне 6.0-9.5M, fast-fail) → ре-роллы anchorc/d (35631465535/35631493642) в полёте.
- При отсутствии fresh-якорей pair scd4/comp против пула 401-404 (2.2@7125977 ближайший к 7.1M-пулу).

### 4) Композиция cmp405_stagtick (stagcomp⊕tickplane) — верхний агент
- Ветка round-405-f-comp @fef3746: merge round-403-c-tickplane → 60fe902 ЧИСТЫЙ; гейт-чеклист 10 сайтов (mobs_manager/mobs_grid/mobs_soa/items_manager/items_index/stagger/collide_batch/tickplane.rs + java MobPushOps/ItemEntityManager), классы пересобраны javac --release 21 (cp: patched-kernel.jar+paper-api 1.21.10+adventure 4.24), cargo check --lib PASS
- comp1 35630922520 / comp2 35630952157 @fef3746 — в полёте.

### 5) Волна-1 R-векторов (закон 6 RUST-FIRST)
- TASK-405-A nav→Rust bulk (cmp405_navrust): worktree @60fe902, имплементация
- TASK-405-B fluid→Rust bulk (cmp405_fluidrust): worktree @60fe902, имплементация
- TASK-405-C getEntities→Rust-индекс (cmp405_eindex): push @877077d (recon javap-контракт EntityLookup/ChunkEntitySlices)

## Инциденты тика
- sparse-checkout set в linked worktree (f-comp) перезаписал общий patterns → main-worktree потерял 832 tracked artifact-файла + untracked bleg-абсорбы. Оформлено purge-коммитом 0eb844c (прецедент e9bd2dc); kernel-jar возвращён 1d66496. Урок: sparse-операции в worktree → сразу git status в main.
- Диск 96% → 66% (sparse f-comp 2.6G→5.4M, rm /tmp/kx*).

## БАР 80%
Не взят (текущая лестница: stagcomp +20.8 медиана → comp-композиция в полёте → R-векторы волны-1). МЕРЖ НЕТ.
