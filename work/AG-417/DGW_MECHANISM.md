# AG-417 w527 — Механика dim_gen_window в pregen-v3 и судьба ghost-сигнала dgw6144

## 1. Что делает окно (статика кода, base a9ff088f = база ghost-ноги)
bench/worldv2/DimForceloadPlugin.java, armPregen(): каждый тик (runTaskTimer, 20/s)
доливает очередь: `while (ifl.get() < gw && !uns.isEmpty()) fire getChunkAtAsync`.
- Окно = верхний лимит in-flight getChunkAtAsync-future'ов (#16f-ремедиация AG-120:
  61347 фьючерсов одним тиком = коллапс очереди Paper).
- Refill непрерывный (каждый тик, main-thread scheduling без IO) → пауза ≤50 мс.
- yml-схема bench-v2.yml: dim_gen_window есть на ВСЕХ базах (a9ff088f/160dad2a/
  2171d6da/f0fc1bcb), env DIM_GEN_WINDOW доходит до плагина (wiring жив и до #17).

## 2. Little-law модель
Стационарный throughput = min(C_paper, W/L_eff), где C_paper — пропускная
способность gen-конвейера Paper (воркеры/IO), L_eff — латентность фьючерса.
При W=256 и наблюдаемых 10.7 ch/s окно НЕ лимитирует: refill-гэп ≤1 тик,
очередь Paper не осушается. Прогноз модели: ch/s( dgw ) ПЛОСКАЯ для dgw >>
числа активных воркеров (~десятки). Большее окно может только вредить
(глубже очередь, больше одновременных loaded-chunk → heap/GC). Роста +24.5пп
от 256→6144 механика окна НЕ объясняет; 384-дип (8.26) тоже.

## 3. Ground-truth ghost-ноги 178b (joblog 110812604372, run 36999153414)
- env-эхо: DIM_GEN_WINDOW=6144, DRAIN_CAP_POLLS=900, RADIUS=1136, SEED=528178,
  RUNNER_CPU_INDEX=6968125 (band [6M,9.5M] warn-mode, in-band).
- 21:42:55 GEN_FIRST_TS; 22:08:25 "DRAIN at +1530s (GEN-DONE gate pass)"
  → 20449/1530s = 13.36 ch/s (воспроизводит число AG-216 13.29-13.36).
- Staged DimForceload 5733 B; Cancelled 22:39:44 (зомби-волна) в bench-фазе.
- ОДНА нога = ОДИН сид + ОДИН раннер: cross-seed, cross-runner сравнение.

## 4. Почему +24.5пп — не атрибуция (3 независимых основания)
1) Канон v23-w526-chs-sigma (AG-189, 4 A/A-пары): соло ch/s нога = ±30% шума,
   pair-Δ мед 24% — +24.5пп лежит ВНУТРИ канонического шума.
2) Механика (п.2): окно плоское при dgw>>воркеров; 384-дип и 6144-пик
   анти-механичны.
3) Свежий харвест AG-428 w527: dgw1536 n=2 mid 11.27 — между 512 и 6144,
   монотонность ломает; 6144 = outlier (их же формулировка).
Плюс: ноги когорты AG-216 — разные сиды И разные раннеры (cpu_idx варьирует,
pairing-law AG-207: <7M vs >9M режет throughput с 0 перекрытий).

## 5. Рецепт для w528 (что НЕ слать и куда смотреть)
- НЕ слать: серт-ноги "dgw-axis ch/s same-boot min-of-3" как сертификацию
  lever-эффекта окна — потолок ≈0, слоты сгорят (famine q≈409).
- ch/s@chunk-gen движение искать в capacity-плечах: число gen-воркеров Paper
  (chunk-system threads), IO-плечо (region-file serialization), цена worldgen
  датапаков — не в fan-out окне.
- Если dgw-ось всё же проверять: только same-seed A/B в одном job (рецепт
  same-boot AG-210) + обязательный readout cpu_idx + pregen ch/s как вторичная
  метрика; TPS-фаза от dgw не зависит (канон AG-288: sustain w-ось σ/host-плоска).
