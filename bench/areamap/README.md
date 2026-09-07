# Area-map bridge — headless unit smoke (TASK-11)

Живой сервер не может протестировать same-state fast path: хук armed только
при наличии реальной player-tracking map (нужен игрок), иначе dormant. Этот
смок гоняет JAVA-половину бриджа (`SingleUserAreaMapOps.run()`) напрямую
через стабы — без сервера, без патча, без игрока.

## Что проверяется

| Сценарий | Что доказывает |
|----------|----------------|
| S1 parity | apply-цикл (декодинг key = z<<32\|x, op 0=add/1=remove, порядок колбеков) == naive set difference; сетка: move/negative/nested/disjoint/touch, d ∈ {0,1,2,8,16,32} — 32 > INITIAL_CAP 578 → grow-path |
| S2 fast-path | same-state (x,z,d) → **0 нативных вызовов, 0 колбеков**; moved → ровно 1 вызов (защита отvacuous counter) |
| S3 MIN_VALUE | fromX == Integer.MIN_VALUE → 0 вызовов, 0 колбеков |
| S4 threads | 4 потока × 40 кейсов — ThreadLocal scratch изолирован, parity на каждом |
| S5 real-native | REAL `libpaper_native_jni.so` согласуется с naive на всей сетке |

## Метод

- Фейковый `PaperNativeAreaMap` (тот же FQCN, статический JAVA-метод —
  бинарно совместим с invokestatic бриджа) со счётчиком вызовов → fast-path
  наблюдаем. Два classpath-режима: `classes-fake` (S1-S4) и `classes-real`
  (нативные декларации + System.load, S1+S4+S5).
- Reference считается НЕЗАВИСИМО в драйвере (вторая реализация naive —
  ловит ошибки и в фейке, и в бридже).

## Запуск

    scripts/build_area_map.sh                     # свежие bridge-классы
    cd bench/areamap && ./run_smoke.sh            # оба режима + отчёт

## Результат (2026-09-08, JDK 21.0.12, 2-CPU sandbox)

    FAKE MODE: AREAMAP SMOKE: ALL PASS   (S1×~30 кейсов, S2×4, S3×2, S4)
    REAL MODE: AREAMAP SMOKE: ALL PASS   (S1×~30, S4)

Same-state fast path (JNI-переход на каждый idle-update) — верифицирован. [ERRATA TASK-33: «~115ns» = устаревший пол до аудита BATCH_NS (3baa0f7); канон 35–90ns — `bench/p500/results/P500_REPORT_v2.md`, см. также results/APPLY_BENCH.md.]

## Fuzz parity (TASK-15)

Дифференциальный фаззинг поверх смоука TASK-11: тот же контракт, но теперь
seeded-randomized — сетки, координаты (включая MIN/MAX/zero) и потоки мутаций.
Headless `cargo test`, БЕЗ JVM/JNI/.so — верифицируется in-repo семантика
JAVA-половины бриджа (`SingleUserAreaMapOps.run()`), а не закрытый натив.

### Метод

- Крейт `area-map-fuzz/` (standalone, workspace-excluded, 0 зависимостей):
  - **World F (fast path)** — точный Rust-порт `SingleUserAreaMapOps.run()`:
    MIN_VALUE guard → same-state shortcut → grow-only scratch (INITIAL_CAP 578,
    то же doubling с overflow-guard) → enumeration-стенд (контракт TASK-11:
    adds = new∖old op0, removes = old∖new, keys = z<<32|x) → apply-цикл с
    декодингом key; счётчики enumeration-calls / ops / fast-path-hits
    (наблюдаемость S2).
  - **World R (reference)** — независимый наивный per-cell set difference
    (форма оракула TASK-11: removes старый квадрат, затем adds новый), без
    fast path, без scratch, без keys.
  - Один и тот же mutation-строк подаётся в оба мира; parity = element-wise
    w×h grid (окно, row-major) + точное равенство tracked-множеств (покрывает
    и клетки вне окна).
- RNG: xorshift64* inline, per-case seeds через splitmix64 от фиксированного
  `BASE_SEED = 0x5EED_5325_97B0_0015` — любой фейл воспроизводим из
  (base_seed, case_index), panic печатает seed, dims и первый отличающийся
  индекс (row, col) + чанк-координату.

### Сьюты (`cargo test -p area-map-fuzz` из `area-map-fuzz/`)

| Сьют | Кейсов | Что доказывает |
|------|--------|----------------|
| fuzz_parity_main | 12 000 (155 279 мутаций, 115 002 395 ops) | parity fast-path vs apply-loop на random dims (0x0 / 1xN / 8x8 ~55%, 64x64 ~42%, 256x256 ~4%), d 0..64, same-state давление 25%, d=0 20%, MIN/MAX/0 координаты, guard-инициализация |
| fuzz_adversarial_idempotency | 1 500 | только no-op state writes → 0 enumeration-calls, 0 колбеков, сетка бит-в-бит (idempotency fast path, S2-подобный counter-guard) |
| fuzz_edge_coordinates | 440 детерминированных | seam MIN/MAX: pack/decode round-trip, move/move-back/3×no-op/grow-shrink, точные счётчики fast-path |

Найденная семантика (НЕ баг, задокументировано): MIN_VALUE guard СТАРШЕ
fast path — update, чей FROM-state имеет x == Integer.MIN_VALUE, ловится
guard'ом (0 ops) даже при равных квадратах; оба мира консистентны, parity
не нарушается. Поведение закрытого натива на wrap-сшивке координат
(|diff| ≥ 2^31) контрактом не покрыто — за пределами сетки TASK-11 S5.

### Результат (2026-09-08, rustc 1.98.1, 2-CPU sandbox)

    release: 3 passed; 0 failed — 12.3s   (12_000 + 1_500 + 440 = 13 940 кейсов)
    debug (overflow-checks on): 3 passed — 222s, без паник
    sensitivity-check: инъекция бага (fast path без oldD==newD) ловится
    мгновенно (edge-сьют + main case=8325, seed 0xa93dd160616e8f37)

Багов parity в shipped-логике НЕ НАЙДЕНО. Запуск:

    cd area-map-fuzz && cargo test --release            # ~12s
    cd area-map-fuzz && cargo test --release -- --nocapture   # + сводки
