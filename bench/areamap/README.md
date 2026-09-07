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

Same-state fast path (~115ns JNI на каждый idle-update) — верифицирован.
