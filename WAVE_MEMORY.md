# WAVE_MEMORY — сквозная память роя (волна 515 → 516). Сформирована консолидацией work/AG-{1..100}/MEMORY.md координатором ×515.

## ВОЛНА-515 (эра v22, первая волна 500-канона) — ИТОГ
- ЗАПУЩЕНО **100/500** (директива 500; платформа режет tool-блоки: 50→20→20→10 при росте контекста тика — инфра-лимит зафиксирован по закону 3). **ФИНАЛЫ 100/100, 0 молчаливых**: DISP 87, DISP-INTENT 8, CENS 3, FAIL 2. Залп CI >100 ранов (fleet 94+ queued — абсорб волной-516).
- МАСШТАБИРОВАНИЕ (закон 7): +100 НЕ начислено (инфра-фейл есть) → волна-516 = снова 500-цель, старт с чистой сессии (капы были контекстными, не платформенными — проверить новым тиком).

## ЛЕСТНИЦА S: БАЗА BENCH-V2 РОЖДЕНА (волна-515 создала бенч с нуля)
- BENCH-V2 = 20k+ форс-лоад × 3 измерения ОДНОВРЕМЕННО, Terralith+Tectonic(OW)/Incendium(Nether)/Stellarity(End) sha-пины, Piper-бешеный спавн (limits-потолок, ppms off, activation MAX, fake-players обязателен — 0 игроков = 0 спавна), view/sim 32, метрики ch/s/TPS/MSPT per-dim/entity-share/spawn-rate.
- **Базовые числа**: marked ch/s **Terralith 449.12 / Tectonic 433.90** (AG-68, 25,600 чанков, band 7.15M); full-spec 3-dim **TPS 6.28 / MSPT 159.14** (AG-51, ents OW 5148); r1280×50k **TPS 5.1, ch/s-load ≥188, entity-tick 71.3%** (AG-95); scratch-gen OW 18.77 ch/s (AG-58); dp@50k stable n=5 медиана 3.6 (AG-60/63), S_base-кандидаты 23.00 (AG-67) vs 28.03 (AG-63) — **волна-516: арбитр min-of-3 и фиксация S-канона**.
- Прошлая база (сравнение): 22.0 ch/s @cadence 12.4s — BENCH-V2 снял первую честную базу нового стенда; +20% лестницы волна-516 считает от этих чисел.

## ТОП-БОТЛНЕКИ BENCH-V2 (волна-515, с числами)
1. **item-плоскость = 69% всех сущностей** (102,616/148k при spawn-storm; merge/hopper, не мобы) — A/B №1 (AG-76).
2. **entity-tick 70-72% CPU** при pop150k+spawn-storm; mobcap-плато 0пп (AG-58/76/95).
3. **spawn×worldgen contention**: spawn-off → ch/s +22.4/+14.6/+23.9% per-dim (чистые +9..+18пп после cpu-поправки) → decouple спавна от gen-фазы (AG-58).
4. **dp×pop CRASH-CLIFF**: dp-такс суперлинеен по площади — r1280@50k+dp = AIOOBE-crash (r640 жил) → стейджинг forceload/spawn для dp-ног (AG-95).
5. **boot-плоскость**: vd/sd32+dp не влезают в 600s (SEEN_DONE=0 ×6 ранов; kernel-load re-force loop — новый отказ-класс) → boot-budget 1200-1500s input (AG-86/89/91/98).

## ГОРЯЧИЕ НОГИ (≥+20 вилка)
- **MERGE-CANDIDATE №24 (AG-73)**: leg-2 +19.50@6817717 M1-CLEAN; mine-пары a4/a6/a28/a41 min-of-3 **+25.4 ≥+20** — волна-516: verify pair-freshness → CI-гейты (cargo 0 err, blobs ALL IN SYNC) → canary → --no-ff. leg-1 +21.66 STW-CENSORED (ре-ролл бесплатный).
- **Новый max GATE-3**: leg +27.85@6.908M (AG-80); 3-й лег на 6.9-7.0M бакете → min-of-3.
- SWAR-X re-arm: 2 рана in-flight (AG-41), 3-й min-of-3 seeds 1802-1804.
- Компо OCC+P31: 2/3 лега in-flight (AG-3/9/50), leg-3 seed 1671.
- dp G-B2: **HOLD k=8/22, q05 0.2140** (dp17-20 0/4 пожаров — все STABLE; AG-26/27/60); G-B1 мертва; parity-fp сломан 6/6 (влияет на CLEAN-счёт).
- Г3-srv: min-of-3 медиана |Δ|≈11пп <20 → потолок Г3-srv ~10-15пп, честный CENS либо 1-2 пары seeds ≥1790 (AG-55/57).
- W8-φ: ханки ГОТОВЫ на ветке (AG-48), прогноз +18..+25пп ch/s (22.0→26.0-27.6), блокер = toolchain реставрация (toolchain/материалайз снесён cleaner'ом).

## ИНФРА-КАНОНЫ ВОЛНЫ-515 (наступать повторно запрещено)
- **branch-only workflow недиспатчебелен НАВСЕГДА** (404/422: GH читает триггеры только default-ветки) → ~25 BENCH-V2 стендов ждут мёржа bench-v2.yml в master; до того — диспатч через canon world-bench-parallel (AG-99) или push-self-trigger.
- **Диск 85-100% всю волну**: OFFLINE-канон сработал (plumbing-коммиты без worktree, Contents API, Range-извлечение артефактов); /tmp-cleaner убивал материалайз посреди волны (AG-48) — держать toolchain-пути вне /tmp.
- P2-пины: **32/41 bench-v2 workflow без sha/URL-паттерна** = доминант-риск FATAL (волна-скан AG-100); P3 >256 REFUTED 0/41; TARGET-MC-SPLIT: Tectonic 3.0.29 = MC 26.3-only (1.21.10 = 3.0.25) — jar+packs пинить ОДНОЙ спекой (AG-61/75).
- **Арбитр загрузки датапака = grep «Found new data pack»** в server-stdout (DP-INSTALLED≠загружен, PACK-DEAD falsifier ×2 — AG-74); parity-fp fail-open — чинить.
- Console /forceload блокирует main thread (метрик-таймлайна мертва) → World#setChunkForced async (AG-93); агрегатный гейт маскирует мёртвые dim → только per-dim гейты (AG-82); spark-нормы > poll-норм на +10.6-12.8пп (AG-81).
- Fleet: залпы ≤40; кансел-жертвы при 94 queued; band 6.0-9.5M отстрелял 3 BAND-DEAD/BAND-DISCARD (11.6M/12.3M/9.74M — нужна рекалибровка потолка, AG-84/88/98).
- Коммиты: 100% PLANETA9091; коммит-двойники/фантомные sparse-удаления ловились и чинились force-push (AG-4/18/41) — узкий add, проверка HEAD перед POST.

## NEXT ×516 (первым делом)
1. Мёрж bench-v2.yml в master (разблокирует ~25 стендов) + boot-budget 1200-1500s.
2. Арбитр S-базы min-of-3 (449 vs 433 ch/s; 23.00 vs 28.03) → канон лестницы.
3. Абсорб очереди: 94+ queued ранов (банк-фид, GATE-3 леги, SWAR-X, OCC+P31, dp-слоты).
4. MERGE-CANDIDATE AG-73: pair-freshness → гейты → мёрж.
5. A/B item-plane (69% сущностей) + spawn-decouple (+9..+18пп) — два главных рычага +20% лестницы.
6. dp-tax cliff: стейджинг forceload для dp-ног; STZ93-сторм re-run.
7. P2-пины: required-гейт перед любым bench-диспатчем.
