# AG-143 w527 — payload: аудит измеряемости гейтов (g)/(j) GO-528 + патч

## Артефакты
- MobAiOps.java.patched — полный патченный файл (база master 8184f1e0)
- diff_ovh.patch — унифицированный дифф
- claims_AG143.md — prereg клейма
- MEMORY.md — уроки саба

## Факты аудита (grep-пруф, мастер 8184f1e0)
1. mobai/net/minecraft/world/entity/MobAiOps.java: `nanoTime` = 0 hits;
   `currentTimeMillis` = 0; лог-события: ARM_LOGGED (первый skip), epoch-ok
   (one-shot), DATA-PLAN (one-shot) — пер-тик fill чисел НЕТ.
2. src/mobs_ai.rs:469 fn ai_epoch: Instant/elapsed/Duration в теле = 0
   (Instant-использования файла — init/order-deadline на :261/:310, не epoch).
3. bench/world3/run_world3.sh: grep aiEpoch/aiwindow/epoch-fill = 0 wiring;
   parity-фаза (7.5/7.6) — единственная post-stop измер-фаза.
4. Gate-модель: (g) fill<=0.74ms/тик (AG-108 FACT 17:0xZ; AG-104 prereg
   «fill<=0.74ms до GO»). 0.74ms/тик при 20 TPS = 1.48% одного ядра.
   Spark/async-profiler сэмпл-атрибуция лейна 1.5%: для ±0.2пп @95% надо
   ~10^4 сэмплов внутри лейна → сэмпл-прокси статистически неадекватен;
   прямой таймер = 2×System.nanoTime ≈ 40-60ns/тик (≈0.0004% тика).

## Патч (семантика окна НЕ меняется; только телеметрия)
- t0 = nanoTime() перед aiEpoch; fill = nanoTime()-t0 после publishStamps
  (вкл. JNI-эпоху + bulk-проход стампов + volatile-публикацию = полный
  оверхед окна за тик; one-off массив-grow исключён).
- Аккумуляция ТОЛЬКО из-под EPOCH_LOCK (slow-path эпохи) — thread-safe.
- ERR_STRUCT/ERR_RANGE-эпохи в статистику не попадают (ваниль-фолбак тика).
- maybeOvhReport: раз в CRUSSTY_OVH_EVERY (env, default 6000, 0=off):
  LOG.info "[crussty-plugin] aiwindow-ovh: tick=.. epochs=.. fill_mean_us=..
  fill_max_us=.. n=.. windowLen=..".
- Порогов гейтов в коде НЕТ (prereg-числа вне кода — канон единой истины).
- Оверхед патча на hot-path skipAi: 0 (измерение только в slow-path эпохи
  1/тик). На эпохе: +2 nanoTime вызова.

## Адюдикация w528 (для исполнителя GO-лега)
1. После стопа: grep "aiwindow-ovh" server-stdout.log → последний репорт.
2. (g) PASS: fill_mean_us <= 740 (prereg AG-104/108; fill_max_us — контекст,
   в prereg не гейт).
3. (j): fill_mean_us = измеренный оверхед окна/тик — вход в честный центр
   capture-матем (AG-111/106 лестница).
4. Контроль артефакта: epochs ≈ тиков в стат-фазе (дроп warmup); n=windowN.

## Ограничения
- javac в песочнице нет (JRE-only, /tmp/jdk21 испарился) → compile-вериф
  отложена в CI blob-build; стат-вериф: баланс скобок OK, idiom-паттерны
  файла сохранены, конструкты — уже присутствующие в файле (nanoTime впервые,
  но java.lang, импорта не требует).
- Build-path: blob-сборка mobai/ → без новых файлов — whitelist-риск AG-42
  (GoalMemoOps) не применим: меняем только существующий MobAiOps.java.
