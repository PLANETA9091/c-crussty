# RESEARCH-459-CX6 — ticking-bucket affinity shuffle (TASK-459-85, WILD закон 11, v18.2)

Идея C-X6 (своё безумие): в RegionTickOps (region_threads=4) перераспределение мобов
по тик-бакетам по принципу ЛОКАЛЬНОСТИ (соседние чанки → один воркер) ради
кэш-локальности broadphase-путей; порядок тиков бит-в-байт — shuffle меняет ТОЛЬКО
bucket-принадлежность, не последовательность. STRICT dormant, lever `cmp459_cx6`.

## 1. Текущая карта (контракты, ≥5 чисел)

| # | Факт | Число | Источник |
|---|------|-------|----------|
| 1 | bucketOf w=4 = ((rx&1)<<1)|(rz&1) — checkerboard по 8-чанк регионам: ортогональные соседи НИКОГДА не на одном воркере, P(same worker|adjacent)=0 | REGION_CHUNKS=8, w=4 → 4 бакета | entityinside/net/minecraft/world/entity/RegionTickOps.java:821-829 (s7169 blob), константа REGION_CHUNKS=8 |
| 2 | RECON-15 (s7169 wall profile) статических бакетов: main bucket-0 478 сэмплов vs 651 на хелпера, total work 2432, critical path 651 → mean parallelism 3.74/4 = 93% | 478/651/2432/93% | RegionTickOps.java:59-63 (RECON-15/TASK-333) |
| 3 | Main PARKED на DONE-барьере 121/900 = 13.4% wall — статическая партиция workload-UNAWARE (imbalance+jitter страдают, не шарятся) | 13.4% | RegionTickOps.java:62-64 |
| 4 | Broadphase-лейн (push/collide/query) = 8.8-10.9% на ногах | 8.8-10.9% | docs/LAB_LEDGER.md ПОДСИСТЕМА: broadphase |
| 5 | Доминация взаимодействий локальна: push ~1 блок, merge ~0.5 блока, collide = movement AABB vs 8x8-чанк регионы | 1 / 0.5 / 8x8 | RegionTickOps.java:19-23 (feasibility gate S7-155 GREEN) |
| 6 | Shared Level.random пути = 0.19% профиля (race-толерантность паритета), остальные горячие пути — per-entity RNG | 0.19% | RegionTickOps.java:46-48 |
| 7 | Снапшот-партиция: int s = bucketOf(e, w) — ЕДИНСТВЕННЫЙ сайт распределения (RegionTickOps.forEach), intra-bucket порядок = snapshot порядок = ваниль | 1 сайт | RegionTickOps.java:573,579-583 |

Вывод из карты: ваниль-бакетинг w=4 выбирает МАКСИМАЛЬНОЕ пространственное
разделение воркеров (checkerboard — защита от межпотоковых гонок push/merge),
но платит за это кэш-локальностью: рабочий сет каждого воркера разбросан по
всей карте с шагом 16 чанков → broadphase-проходы (EntityLookup секции,
chunk-refs, AABB-регионы) холодные на каждом тике каждого воркера.

## 2. Гипотеза C-X6 (affinity shuffle)

Замена mapping только на w∈{2,4}, SHUFFLE ONLY MEMBERSHIP:
affinity(rx,rz,w=4) = ((floorDiv(rx,2)&1)<<1) | (floorDiv(rz,2)&1) —
2x2-БЛОКИ 8-чанк регионов = 16x16-чанк тайл на воркера (checkerboard тайлов).
- Соседние 8-чанк регионы на одном воркере: P = 1/2 (горизонталь и вертикаль) vs 0 ваниль.
- Рабочий сет воркера: пространственно когерентный тайл 16x16 чанков вместо
  разреженной пыли → секции/чанк-рефы broadphase живут в L1/L2 ядра воркера.
- Порядок тиков БИТ-В-БАЙТ: партиция аппендит в snapshot-порядок независимо
  от bucket id (RegionTickOps.forEach:579-583) — affinity меняет только
  индекс бакета, intra-bucket подпоследовательность = подпоследовательность
  ванильного снапшота (оракул G2, unit-тесты в src/bucket_affinity.rs).
- w∉{2,4}: mapping НЕ тронут (hash-фолбэк как в ванили).

Риски (честно): (a) тайл-швы 16x16 — push/merge партнёры на тайл-границе
переходят межворкерные (ваниль-паритет уже принимает interleave-класс S7-155);
(b) affinity УВЕЛИЧИВАЕТ локальную плотность на воркере → синхронизация
EntityLookup-секций внутри тайла горячее, межтайловые гонки реже по числу пар.

## 3. Источники (≥2, верифицированы curl 200)

1. https://github.com/PaperMC/Folia — regionised multithreading: «Folia groups
   nearby loaded chunks to form an independent region... its own tick loop...
   executed on a thread pool in parallel» — канон СПАЦИАЛЬНОЙ когерентности
   тик-юнита: близкие чанки тикает один поток. Что взято: принцип соседство→один
   воркер (регион = spatial tile), компромисс по межрегион-границам.
2. https://en.wikipedia.org/wiki/Work_stealing — locality-aware scheduling:
   «Attempts to improve on the multiprogramming work stealer have focused on
   cache locality issues»; parallel-depth-first «better performance... where the
   cores of a chip multiprocessor share a cache». Что взято: осознанный выбор
   STATIC affinity (tile-mapping) против чистого steal: steal-mode (S7-167)
   уже держит баланс, affinity добирает кэш-локаль на статической карте.
3. https://github.com/PaperMC/Folia/wiki (200) + https://modrinth.com/mod/moonrise (200) —
   Moonrise TickThread-стек нашего кернела (ca.spottedleaf.moonrise.* импорты
   RegionTickOps) — вектор эволюции в сторону region-локальности.

## 4. CAPTURE-МАТЕМ (Δ-прогноз и потолок)

lane = broadphase доля тик-wall = 9% (середина 8.8-10.9);
miss-share broadphase на checkerboard (разброс снапшота, шаг 16 чанков,
LLC/NUMA-cold секции) ≈ 0.6-0.8; capture тайлирования (рабочий сет /4,
когерентность 16x16) ≈ 0.3-0.5; parallel-eff 0.93 (RECON-15).

Δ = 9% × 0.6..0.8 × 0.3..0.5 × 0.93 ≈ +1.5..+3.8пп → честный прогноз
Δ +2пп (одиночный вектор, STRICT dormant, только w=4 нога).

Потолок (100% capture): 10.9% × 0.8 ≈ +8.7пп < +20 → по закону 13a
самостоятельный мерж-кандидат НЕВОЗМОЖЕН: вектор = КОМПОЗИЦИОННЫЙ климб-
кандидат поверх чанк-носителя (cmp456_chunkmono) + P31 INSIDE-BATCH —
affinity-дельта добавляется в ногу, а не в пару сама.

## 5. Preregistered гейты (G1-G6, канон S7-150)

- G1 ARM/эффект: env CRUSSTY_LEVER_FLAG == "cmp459_cx6" (STRICT eq, урок
  TASK-400-D полу-вооружённого моста) AND CRUSSTY_REGION_THREADS>=2;
  эффект-маркер: harness-census — ≥10% снапшота меняет bucket id vs ваниль
  (функция mapping_delta_report в src/bucket_affinity.rs — число на борту).
- G2 lockstep бит-в-байт: RegionLockstepHarness-оракул — для одинакового
  снапшота каждая per-bucket подпоследовательность = подпоследовательность
  ванильного порядка (0 перестановок внутри бакета; тест test_order_preserved).
- G3 NCDFE=0 / threw=0 / AIOOBE=0: define ДО arm (EARLY-define канон,
  урок ×456-отравленных пар); blob-rebuild по lesson-408 (спящие гейты
  запрещены): javap flat==nested 10/10 ДО диспатча носителя.
- G4 TPS-бар vs банк v4: pair = leg_norm − anchor_norm, Δ≤50k pair-fresh,
  min-of-3; суб-бар → CLIMB-композиция (§4).
- G5 fail-closed: env отсутствует → ваниль-путь бит-в-байт; scaffold
  регистрирует 0 byte-хуков (dormant-invisible: модуль не определяет классов,
  не ретранформирует, blob-байтов не инклюдит).
- G6 band 6.0-9.5M + GC-мониторинг (Full=9 депресс-кластеры), ре-ролл ≤2.

## 6. Scaffold (что в ветке round-459-cx6)

- src/bucket_affinity.rs — чистая политика маппинга + lever-гейт + оракулы
  порядка/баланса/локальности (7 unit-тестов GREEN) + register/activate
  fail-closed, wired в lib.rs (mod + register + activate).
- entityinside/net/minecraft/world/entity/BucketAffinityOps.java — java stub
  (контракт будущей пересборки blob'а: affinityBucketOf/vanillaBucketOf для
  lockstep-оракула; НЕ вшит в RegionTickOps, build/ blob'ов не содержит).
- Wire-точка будущего носителя (задокументирована, НЕ исполнена): единственный
  сайт int s = bucketOf(e, w) RegionTickOps.java:573 → affinity-вариант при
  ARMED; javap-сверка сайта до/после обязательна (NCDFE-канон).
- Урок ×459-инфра: worktree /tmp был вычищен внешним пурджем между scaffold и
  commit → воссоздан по ветке; артефакты бэкапятся в
  /home/z/rounds/ROUND-459/cx6-artifacts/ ДО любых операций в /tmp.
