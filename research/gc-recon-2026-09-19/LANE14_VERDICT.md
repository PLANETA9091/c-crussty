# ЛЕЙН #14 (TRAVEL/ALLOC-DIET) — ФИНАЛЬНЫЙ ВЕРДИКТ-ДОК (TASK-350, 2026-09-20)

## Вердикт: REFUTED — лейн ЗАКРЫТ по исключению чартера (потолок <10% доказан)

Прецедентная база: RECON-17/20 STEAL (доказанное закрытие вердикт-доком с
потолком <10%). Обязательство «ТОП-1 ОБЯЗАН УПАСТЬ» исполнено: лейн атакован
тремя валидными ногами, потолок доказан свежим RECON по CPU-оси.

## Три валидные широкобандные ноги (протокол v8-REGRESSION, DUAL BAR vs ANCHOR-SLOW 1.60 @ 6680195)

| нога | коммит | рука | runner | median5 | normalized | absolute |
|---|---|---|---|---|---|---|
| s7189 (run 35464575112) | ba54f32 | v2a scratch-slot collide | 6918984 | 1.3 | **−21.6%** | **−18.8%** |
| s7192 (run 35467929550) | a5c353d | v2b travel-math (+S7-170 desync-негвард) | 6782504 | 1.6 | **−1.5%** | **+0.0%** |
| s7193 (run 35469278586) | 3529175 | v2b travel-math + S7-170 delivered | 6951881 | 1.6 | **−3.9%** | **+0.0%** |

Ни одна ось ни одной ноги не достигла +10%. Ни одна не была BAND-DISCARD —
все три валидны (band OK, PG-T1 PASS, young 144..158 / Full 0).

## Почему REFUTED (три независимых доказательства)

1. **CPU-потолок (RECON-23, 9ea3bd1)**: travel-поддерево = 4.37–5.10%
   scene-CPU inclusive на s7178/s7184/s7189 — кросс-раннер стабильно;
   даже 100% устранение не даёт ≥+10% TPS.
2. **Под-лейнов ≥5% сцены нет** (RECON-23): максимальный travel-misc-kernel
   1.31% сцены; collision-core 0.95–1.05% — actionable-рычага ≥10% внутри
   лейна не существует.
3. **GC-канал мёртв на CPU-оси** (RECON-23): gc-barrier под travel = 0.1%
   лейна (5–7 сэмплов); доля travel на alloc-оси 7.3–7.9% < 10%.

Механика провала рычагов: (a) v2a scratch-slot зеркало дороже C2
scalar-replacement ванильных темпорари (8×ThreadLocal.get/вызов + Unsafe);
(b) v2b travel-math (getInputVector + travelInFluid + SKIP
handleRelativeFriction) убивает аллокации, но аллокации этой зоны не конвертируются
ни в CPU (инлайн жив), ни в TPS (буфер молодого поколения не давит).

## Что остаётся в банке из лейна #14

- travel_diet yml-дефолт 0 (не банкуется — двойной бар не взят).
- **S7-170 NAV-MOBS-GUARD — ОСТАЁТСЯ КАК КОМПОНЕНТ ПРАВИЛЬНОСТИ**: верифицирован
  на сцене s7193 (маркер PRESENT, threw=0 против threw=1..3 во всех ногах
  банка v3; класс-A RECON-22 гонка navigatingMobs структурно закрыта).
  Delivery-урок (blob desync a5c353d → фикс 3529175) институционализирован
  в scripts/check_blob_sync.py.

## Переход ТОП-1

Свежий профиль (s7189 BOTTLENECKS_3): семейство GC 26.8% scene-CPU
(G1-потоки 14.6% + oop-barriers 12.2%) = новый ТОП-1. RECON-24 (44c5666)
разложил alloc-ось до под-лейнов ≥5%: jdk-collections/serde 25.5%,
**inside-blocks/fluid-scan 22.3%** (Vec3 41% + AABB 34% — цель существующего
рычага zero_alloc #10 S7-164), jvm-core 21.5%, travel/movement 11.0%,
chunk-system 6.4%. Первый рычаг нового ТОП-1: **zero_alloc=1 изоляция
(нога s7194, dispatch_s7194.py / absorb_s7194.py, гейт: zeroin-маркер
sites:3 + [S7-170] marker + threw=0 + DUAL BAR; young-drop = вторичный
сигнал)**.
