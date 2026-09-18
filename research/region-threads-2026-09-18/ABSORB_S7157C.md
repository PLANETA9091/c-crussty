# S7-157c — ABSORB leg #2 (35363758352): REGION-THREADS — экономика ДОКАЗАНА (TPS +66.7%), банкование отложено: tracker-race NPE + PG4 GC

Дата: 2026-09-19 ~01:0x +08. Leg #2 run 35363758352 (S7-157b build 880e406,
mid-tick gate включён). База: CUMULATIVE 35330129145 (inside_cache=1 +
flush_diet=1). Протокол: preregister §S7-155 §6 / §S7-156 §5, гейты
PG2/PG3/PG4, анализатор absorb_s7157.py. Артефакт: run-s7157b-leg2-artifact/
(фетч из отменённого по 75-мин wall job — bucket через if:always();
БЕНЧ-ДАННЫЕ ПОЛНЫ И ВАЛИДЕН: все 3 профайлер-окна завершены ДО крэша,
spark upload 15:54:53, shutdown начат 15:54:54).

## Конфигурация и валидность

- run-env.txt: inside_cache=1 + flush_diet=1 + **region_threads=4**, demux/
  alloc_diet/fluid_free/fluid_dirty=0; fp4/300s/150k/seed42/xmx10G — бит-в-бит
  preregister-протокол leg'а.
- INJECT 150000/150000 VALID (items=105000, hostiles=30000, passives=15000).
- 0 NoClassDefFoundError (stdout 261KB).
- ARMED полная цепочка живьём: region_threads computed patches (ServerLevel
  142812→142952 Retargeted{sites:1}; EntityCallbacks 12348→12504
  add=Retargeted{sites:1} remove=Retargeted{sites:1}; Level hook 96331) →
  serve ×3 → **ARMED, retransform rc ServerLevel=0 EntityCallbacks=0 Level=0**;
  inside_cache Entity 205458→205522 Retargeted{sites:1} rc=0; flush_diet
  StepBasedCollector 5695→5798 Retargeted{sites:2} rc=0. Kernel e2992d63.
- Runner: 4 ядра, runner_cpu_index=6757887 (база 9778900 — раннер медленнее
  ~31%: TPS-сравнение медианное, не абсолюнтое — учтено в вердикте).

## Персонал параллелизма (главный вопрос leg'а)

Раскладка RegionTickOps-стеков cpu-collapsed.txt (leg, 128596 сэмплов total
vs база 52341 — больше работающих потоков):

| Форма стека | Сэмплов | Кто |
|---|---|---|
| Thread.runWith → lambda$ensureHelpers → tickBucket | 54057 | 3 helper-воркера |
| ServerLevel.tick → forEach → parallelTick → tickBucket (+bucketOf 16) | 9441+9385+16 | main slot 0 |
| **tickBucket total** | **71547 (55.64% CPU)** | lane целиком |

**Оффлоад воркерами = 54057/71547 = 75.6%** — точно дизайн W=4 (main slot 0 +
3 helper'а). Main дополнительно держит снапшот/диспатч (forEach 18723 вкл.
не-tickBucket фазы). Amdahl-проверка: TPS_max ≈ 1/(0.24 main-entity-остаток +
non-entity 0.08 + native ~0.30×1.31-GC) ≈ 1.6 — наблюдаемые 1.5–1.7 легли под
потолок ×2.31 по S2-сценарию.

Mid-tick gate (S7-157b) живьём: `Level.guardEntityTick → RegionTickOps.
midTickTasks` = 159 сэмплов, ВСЕ на main; **0 сэмплов pump'а на воркерах** —
NoSuchElementException из leg #1 не воспроизвёлся (фикс работает).

## Гейты

| Гейт | Критерий | Факт | Вердикт |
|---|---|---|---|
| PG2 | 0 NCDFE + ARMED + популяция-близнец | 0 NCDFE; ARMED полна; 150000 | **PASS** (формально) |
| PG3 | TPS ≥ +25% (REFUTED < +10%) | медиана crawl 0.90 → 1.50 = **+66.7%** | **PASS** |
| PG4 | young GC ≤ база+15% (≤135 vs 118) | **155 (+31.4%)** | **FAIL** |

TPS crawl raw: база [18.5| 0.7, 0.7, 0.8, 0.9, 0.9], лег [20.4| 1.1, 1.3,
1.4, 1.5, 1.7] — лег растёт по soak (1.1→1.7), база плоская. Прочее:
fluid-family 8.90% (база 9.91% — лейн не тронут, доля ниже из-за параллелизма);
ItemEntity.tick 21.39%, Zombie.tick 12.33% total CPU.

## Два инцидента (банкование отложено именно из-за них)

1. **Tracker-race NPE, 15:54:54** (сразу ПОСЛЕ spark upload 15:54:53 — soak
   завершён, данные валидны): `NullPointerException: Cannot invoke
   "EntityTrackerEntity.moonrise$getTrackedEntity()" because "entity" is null`
   @ ChunkMap.newTrackerTick:1017 ← ChunkMap.tick ← ServerChunkCache.tick ←
   ServerLevel.tick:815. Root-cause гипотеза: воркер удаляет сущность
   (discard/despawn) → moonrise EntityLookup.remove на потоке воркера
   гоняет main в ChunkMap.newTrackerTick (итерация tracker-карты без
   защиты от конкурентного удаления). Deferred FIFO сериализует только
   EntityTickList-callbacks, НЕ EntityLookup/tracker.
2. **UUID-dup WARN, 15:54:24 (1×; база 0×)**: Arrow 166658 (заспавнен
   skeleton-выстрелом = НА ВОРКЕРЕ) получил UUID, уже занятый Rotten Flesh
   166659, позиции в разных регионах (~900 блоков) → EntityLookup «can't
   add». Гипотеза: генерация UUID новых сущностей на параллельных потоках
   алиасит seed (проверить Entity ctor → UUIDUtil.createRandomUUID(seed
   от чего?) на воркер-путях). Causal-связь с инцидентом 1 возможна
   (30 сек между событиями) — проверить при root-cause.

## Вердикт (по preregister §S7-155 REFUTED-критерию: прирост <10% ИЛИ неустранимые парити-провалы)

**НЕ REFUTED, НЕ GREEN-к-банкованию — PARTIAL/экономика-ДОКАЗАНА**: прирост
+66.7% ≫ +10% (первый рычаг эры, реально сдвинувший TPS); оба инцидента —
инженерные гонки фиксируемого класса (сериализация/гейт), а не семантические
парити-провалы потребительской логики. Но крэш-лег не банкуют: default-кандидат
не может падать в tracker-tick. PG4 (+31.4% young GC) — цена снапшот/бакетных
аллокаций на тик + 3 воркер-стека; нужна диета (переиспользование бакетов/
снапшот-массивов между тиками) до ≤135.

## NEXT (S7-158)

1. Root-cause tracker NPE: дизасм ChunkMap.newTrackerTick/EntityLookup.remove;
   фикс-класс = гейт/делегирование (паттерн mid-tick gate) — worker-удаления
   через отложенный drain на барьере ИЛИ removal-safe итерация tracker'а на main.
2. Root-cause UUID-dup: javap Entity ctor/UUIDUtil — чей seed на воркер-пути;
   при общем источнике — synchronized/ThreadLocal-сидирование.
3. GC-диета моста (PG4): reuse бакетов/снапшот-массивов, убрать per-tick
   аллокации lambda/коллекций.
4. Offline-харнесс: child-тест конкурентного discard во время tracker-tick
   (регресс-проверка фикса 1).
5. Leg #3 (мин-оф-2) на фикс-билде; banking только при PG2+PG3+PG4 PASS
   без крэшей.

## Гигиена CI (наблюдение leg #2)

Job сгорел 59 мин ПОСЛЕ завершения бенча (server exit 15:55:21 → wall cancel
16:54:35): подвис фаза между `cmd stop`/kill-цепочкой и report_world3.py
(BOTTLENECKS_3.md не сгенерирован; collapsed-файлы готовы). FIFO-фикс S7-157b
закрыл смерть java, но post-soak фаза осталась без liveness-watchdog.
S7-158: добавить watchdog/таймауты на shutdown-фазу + non-blocking cmd.
Артефакты спасены if:always() (bucket world3-bench 10558153304).

## Хэши артефакта

- server-stdout.log eb0edaa0b244761919a52e001c3bbdc6c79f23913a802e082a8e831702009db7
- cpu-collapsed.txt 22f635938945e34f1313a1496be0f22b0778925b715f6af1635c9619bbc63992
- gc.log 0563700ca78418d4c118268ee42bf4f976affe1b4ed65cb8047a288ac0ab9b75
- run-env.txt 0a74d54301d4c4d60d846d61fc38a50b6269a924b9d3ea960f402c885da0ce7a

INJECTS-ONLY цел: leg #2 — санкционированный preregister A/B leg (job
завершён wall-cancel ПОСЛЕ сбора данных; CI-бут не тратился на пустой прогон).
