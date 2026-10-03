# AG-17 w528 — dgw≡w унификация + dose-ценз + prereg-вердикт w529 (0-POST)

## 1. Унификация осей (собственная верификация, не по чужим словам)
- bench-v2.yml: инпут `dim_gen_window` (L50-56, default 256, коммент AG-400: matrix
  256/512/1024, 256 = champion canon 9.9-11 ch/s) → env `DIM_GEN_WINDOW` (L140).
- run_benchv2.sh L215: `export DIM_GEN_WINDOW="${DIM_GEN_WINDOW:-256}"` (+#16f коммент:
  bounded in-flight getChunkAtAsync fan-out), L219-220: run-env атрибуция (dual-write AG-43).
- DimForceloadPlugin.java: `genWindow()` L98-106 (default 256) — ЕДИНСТВЕННЫЙ потребитель
  env; используется только как верхний лимит in-flight getChunkAtAsync в refill-цикле
  тика pregen-v3 (`while (ifl.get() < gw && !uns.isEmpty()) fire` L205+).
- Следствие: `w`-номенклатура ног (w896/1024/2048/2240/5376/6144/8192) ≡ `dgw`-номенклатура
  ≡ ОДНА ось DIM_GEN_WINDOW (подтверждает AG-384/AG-365 w527 независимо).
  Ось действует ТОЛЬКО на gen-фазу (pregen ch/s); sustain-TPS инертен
  (AG-384: sustain 19.995 post-GEN; канон AG-288) ⇒ ось не может двигать TPS@20k/TPS@dp50k.

## 2. Механика (код) — что окно может и не может
- Окно = счётчик in-flight фьючерсов; refill каждый тик (≤50мс), main-thread без IO.
- Little-Law (AG-417 w527): ch/s = min(C_paper, W/L_eff). При W=256, chs≈10.7:
  окно связывает iff L_eff ≥ 24с/чанк; refill-гэп ≤1 тик ⇒ окно НЕ лимитирует при
  W >> числа активных gen-воркеров Paper (worker-threads авто из CPU — Moonrise
  chunk-system, конфиг-кноб в харнессе ОТСУТСТВУЕТ, grep bench/ — 0 хитов конфига).
- Burst-столл динамика ghost-6144 (AG-384): волны 0-42.6 ch/s, mean 13.46 — глубокая
  очередь = батчевый режим, heap/GC давление растёт с W (кандидат клиффа-1024).

## 3. Dose-таблица (терминальные ноги, boot-нормировка σ_res 15.1% AG-497/28)
| dgw/w | ch/s | z_res | источник |
|---|---|---|---|
| 192 | 8.56 | -0.90σ | AG-497 ghost |
| 256 | med 10.73, CV 25.2% (n=11 kernel-eq) | 0 | AG-497 |
| 384 | 8.26 | -0.81σ | AG-497 |
| 512 | 12.32 | +1.33σ | AG-497 |
| 1024 | клифф DRAIN-TO 2.27 + xmx-арм ОТМЕНЕНЫ (4/4 cancel/fail: 37000751397 c, 37000805239 f, 37006067657 c, 37006124951 c) | — | AG-221/237/266, live API 06:59Z |
| 1536 | mid 11.27 (n=2) | ~0 | AG-428 |
| 5760 | 8.83 | +1.05σ (raw -17.7%) | AG-497 |
| 6144 | 13.29-13.46 | +1.98σ | AG-216/255/315/348/384, n=1 |
| 8192 | in_progress 37026652511 (bench ETA ~10Z, не зомби AG-2/31/38) | — | w529 |

Вердикт: паттерн НЕ-Монотонен (192/384 НИЖЕ 256; 1024 клифф; 6144 пик n=1) =
boot-шум σ15-25% + heap-эффект больших окон. Сигнала lever +20пп НЕТ.

## 4. Чек queued-ног w528 (все = одна и та же ось, live API 06:58Z)
- dgw6144a 37102118677 / dgw6144b 37102148945 (AG-500) — queued; это СОЛО-ноги 6144
  (seeds 527500/528500), не A/B-пара: сравнение с ghost-256-медианой = cross-boot,
  запрещено каноном v23-w526-chs-sigma (AG-189) до same-boot вердикта.
- w2240 37104571264 / w5376 37104577627 (AG-30) — queued, та же ось (доза окна).
- w6144/w5120@r800, w8192/w2048 — хвост очереди / in_progress.
- aa480s1 37101120026 (sameboot-канарейка 2-в-1-job, AG-480) — queued pos~14: ЕДИНСТВЕННЫЙ
  легальный путь серта оси = same-boot A/B min-of-3 после её SUCCESS.

## 5. PREREG (фальсифицируемый, харвест w529)
Для каждой queued-ноги оси (dgw6144a/b, w2240, w5376): z_res по boot-прокси
(AG-28 dual-proxy: boot Done(Xs) + rci).
- ВЫПОЛНЯЕТСЯ если все 4 ноги |z_res| < 2σ_res ⇒ потолок оси dgw/w = 0пп →
  CENS-вердикт оси, не слать больше solo-dgw POST-ов (экономия ~2 слот-ч/ногу).
- ЛОЖНА если ≥1 нога z_res > +2σ_res ⇒ Little-Law неполна, same-boot A/B серт
  (рецепт AG-480/349, харнес bench-v2-sameboot.yml / bench-v2-scw.yml 3-boot
  counterbalanced) остаётся на столе.
FW-поправка: 4 ноги, α=0.05 → порог per-leg |z|>2.5 для коллективного +2σ-заявления.

## 6. Redirect рычага ch/s (S-ось)
Окно не лимитирует ⇒ ch/s движение искать в: (а) cpu-воркеры (runner-пул бимодален,
pairing |dIdx|≤3% канон AG-13), (б) IO-плечо region-serialization, (в) цена worldgen
датапаков, (г) heap-политика (xmx) для больших окон. Все — AG-417 recipe, подтверждён.

## 7. Источники
Код: bench-v2.yml L50-56/L140, run_benchv2.sh L215-220, DimForceloadPlugin.java
L98-106/L205+ (клон 2026-10-03 ~07:00Z). Live runs API 06:58-06:59Z (9 run-id).
Архив: AG-400/95/120/11/237/221/241/266/285/428/417/216/255/315/348/384/189/288/13.
