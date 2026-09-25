# RESEARCH-459-H05 — WILD H05: papaya-lockfree шард-ридеры EntitySection-массивов (broadphase read-path)

Агент: TASK-459-76, v18.2 закон-11 (тиком 459, c-crussty). Идея из ×457 копилки — СВОБОДНА (BLACKBOARD ×458/×459: ID-H05 → СВОБОДНО; слот 76 pending). Цель-скелет: lockfree шард-ридеры в стиле papaya (concurrent map, читатели без блоков) поверх entity-slices broadphase-пути; STRICT DORMANT scaffold (Rust-модуль + Java-stub, ARMED=false, нулевое поведение на банке до вайринга).
Смежный носитель-рисёрч: RESEARCH-459-L11 (тот же слот идеи с другой ноги, метрики и lockstep-оракул переиспользованы как базис; мой вклад — внешние источники механики papaya/EBR + scaffold round-459-h05 + NCDFE-канон).

---

## 1. Профили-основания (носитель chk-14, банк ×458/×459)

| метрика | число | источник |
|---|---|---|
| broadphase-лейн, монстр-нога chk-14 (run 8687055, norm +21.7, NCDFE=0) | **9.36% CPU** | RESEARCH-459-L11 §1 |
| базлайн лейна до носителя | 15.66% (носитель уже срезал −6.30пп) | RESEARCH-459-L11 §1 |
| остаток лейна по банку ног (L03-окно) | 8.93..10.92% (миссия: 8.8-14.7пп остаток) | RESEARCH-459-L03 §32 |
| reader-core (собственно чтение шард-таблиц, достижимо ридерами) | **4.0% CPU (4107/103062)** | RESEARCH-459-L11 §5 |
| HashMap.getNode (JDK collections, broadphase-доля) | 1459 сэмплов / 1.4% | RESEARCH-459-L11 §1 |
| voxel-хвост лейна (недостижим шард-ридерам) | ~57% лейна | RESEARCH-459-L11 §5 |

## 2. Механика из внешних источников (≥2 URL, закон 14)

1. **papaya (docs.rs / GitHub ibraheemdev)** — https://docs.rs/papaya , https://github.com/ibraheemdev/papaya : «fast and ergonomic concurrent hash-table for **read-heavy workloads**; ergonomic **lock-free API** — no more deadlocks; powerful atomic operations». Механика: большинство операций требуют `Guard`, добываемый через `HashMap::guard()`/`HashMap::pin()` (https://docs.rs/papaya/latest/papaya/struct.HashMap.html); читатель пинит **эпоху** на входе (легкая acquire-операция, НЕТ блокировки), писатели копируют-заменяют (COW) затронутые сегменты и публикуют release-стором; удалённые узлы живут до выхода всех читателей эпохи (epoch-based reclamation). Ключ к latency: **read-path = 1 атомарная загрузка + 1 инкремент счётчика**, никакого mutex/RWLock на горячем пути.
2. **anchormap (GitHub mcrepeau)** — https://github.com/mcrepeau/anchormap : concurrent hashmap, где элементы «**заякорены** и никогда не двигаются при конкурентном доступе» → lock-free чтение любым числом потоков без локов. Прямой аналог нашей задачи: EntitySection-массив шарда публикуется целиком (никогда не мутирует in-place), мутатор собирает НОВЫЙ массив и свапает указатель — старый массив валиден вечно для читателей своей эпохи.
3. **Relativistic programming hash tables (Triplett, PSU 2008)** — https://pdxscholar.library.pdx.edu (Scalable Concurrent Hash Tables via Relativistic Programming): читатели идут полностью без синхронизации, обновления соблюдают релятивистские ограничения порядка (публикация указателя после заполнения содержимого); реклейм по эпохам. Это академический каркас того же паттерна, что papaya-ридеры.
4. **VBR: Version Based Reclamation (Sheffi 2021, DROPS)** — https://drops.dagstuhl.de : EBR-методы быстры, но НЕ гарантируют lock-freedom и держат память дольше нужного; VBR позволяет консервативно читать «возможно удалённое» с проверкой версии → для нас: **fail-closed fallback** дешевле доказуемой lock-freedom (см. §5 риски).

Вывод механики для c-crussty: шард = регион 32×32 чанков (прецедент L11 ChunkSlicesRegion-эквивалент), в шарде `AtomicPtr<Snapshot>`; `Snapshot { epoch: u64, sections: [u64; N] }` — иммутабельный flat-массив EntitySection-слайсов. Читатель: pin epoch → acquire-чтение ptr → сверка epoch (drift → fallback ваниль) → flat-чтение без локов. Писатель: build новый Snapshot → release-публикация → retire старого → реклейм по кваесценции эпохи. Rust-модуль держит epoch-механику + счётчики; Java-stub — sidecar с тем же протоколом (odd/even seqlock-семантика как в L11).

## 3. Parity-контракт (закон 16)

- Порядок выдачи read-path: идентичен ванили (z,x,y-asc → storage-idx) — бит-в-байт lockstep G2 (оракул L11: 0/300 mismatches ×3 уже PASS на epoch-прототипе).
- Снапшоты только для СТАТИЧНЫХ секций; любой drift/мутация в окне чтения → читатель молча уходит в lock-ваниль (KIND_DYNAMIC-аналог CollideBatchOps-прецедента). Частичных результатов НЕТ (fail-closed).
- NCDFE-канон (T1): EARLY-define Java-stub в раннем arm-хуке ДО первого broadphase-запроса (прецедент EntityGoalQueryOps @ MobPushOps.pushables:467; коммиты d73758a3/5ecd841a/9d71b461); `<clinit>` трогает ТОЛЬКО JDK-типы → ранний define не может упасть NCDFE; зеркальная игла-поле для javap-проверки. Гейт: **T1 NCDFE=0**, иначе DELIVERY-FAIL.

## 4. CAPTURE-МАТЕМ (lane% × захват% = Δ)

- Захват reader-core: papaya-механика снимает lock/virtual/lookup-надбавку (кадры 5-6 leaf-ранжирования) + микро 1.21× медиана на скан-ядре (L11 G2-perf: locked-vanilla 538-587 µs/q vs epoch 434-738 µs/q) → консервативно **захват 20-35% reader-core** → Δ = 4.0% × 0.20-0.35 = **+0.8..+1.4пп**.
- В терминах лейна: 9.36% × захват 8-15% = Δ **+0.8..+1.4пп**; потолок компоненты = 9.36% × 43% (доля reader-core) = **+4.1пп**; абсолютный потолок лейна +9.36пп (недостижим — voxel-хвост).
- Компо-место: swing-слой климба chk-14 (дефицит пары +10.7пп): P31 (+5-8) ⊕ P32+P36 (+1.5-2.5) ⊕ **H05-ридеры (+0.8-1.4)**. Сам по себе НЕ pair-maker.

## 5. Риски (честно)

1. **JMM-гонка окна** узкая: race-demo L11 G2b 0/400 аномалий ×3 = INCONCLUSIVE — аргумент за epoch-ридеров = снятие фенс-налога, не наблюдаемая коррупция. Смягчение: fail-closed + сверка epoch на КАЖДОМ чтении.
2. **Rebuild-шторм**: при rebuild каждые 8 мутаций fallbacks 316-338/400 (L11 G2c) — в проде rebuild per-tick-phase, ожидаемая доля fallback ≪1% (гейт G1-эффект: fallbacks<1% иначе REFUTED).
3. **Память retired-снапшотов**: ephemeral young, но некваесцентный читатель держит эпоху → гейт G3 rebuild-alloc ≤ +0.5MB/s, young ≤128+10%.
4. **Спящий гейт = placebo-класс** (×425/roar-2): scaffold без вайринга ничего не даёт — финал честно фиксирует DORMANT, эффект-числа только после вайринга на носителе d73758a-класса.
5. **Double-source конфликт**: L11 уже вести research по этой идее — мой run обязан оставлять только scaffold+branch+CI (не дублировать вайринг-планы), идея после этого в копилке помечается занятой обоими слотами.

## 6. PREREGISTERED ГЕЙТЫ (ветка round-459-h05, при будущем вайринге)

1. G1 ARM/эффект: boot-маркер `papaya_shard_reads ARMED`, счётчики epochStableReads>0, rebuilds>0, fallbacks<1%; лейн broadphase ↓ от 9.36%.
2. G2 lockstep бит-в-байт: emission id-sequence 1000 запросов lever-on/off идентичны; офлайн-оракул 0/300 ×3 (уже PASS у L11).
3. G3 GC: young ≤128+10%, Full ≤9, rebuild-alloc ≤+0.5MB/s.
4. G4 популяция: pop 140-165k, churn band chk-14 (delta 5499 / 3.6%).
5. G5 TPS-бар: Δ ≥ +0.8пп median min-of-3, band 6.0-9.5M, ре-ролл ≤2.
6. G6 fail-closed: NCDFE=0 (T1), threw=0, AIOOBE=0, selfTest==true; epoch-drift/OOB → авто-fallback ваниль; иначе REFUTED.

## 7. Источники

1. https://docs.rs/papaya — papaya crate docs (lock-free API, Guard/pin, read-heavy).
2. https://github.com/ibraheemdev/papaya — papaya source (features: ergonomic lock-free API, atomic ops).
3. https://github.com/mcrepeau/anchormap — anchored-elements concurrent hashmap (lock-free readers, элементы не двигаются).
4. https://pdxscholar.library.pdx.edu — Triplett 2008, Scalable Concurrent Hash Tables via Relativistic Programming.
5. https://drops.dagstuhl.de — VBR: Version Based Reclamation (Sheffi 2021), EBR-ограничения.
6. Внутренние: RESEARCH-459-L11 (метрики chk-14, lockstep-оракул, NCDFE-канон), RESEARCH-459-L03 (банк лейна 8.93..10.92).
