# S7-155 RECON-3 — FEASIBILITY-ГЕЙТ region-threaded entity ticking

База: CUMULATIVE 35330129145 (cpu-collapsed 52341 сэмплов), kernel e2992d63; структурный census — s7155_kernel_census.py/javap (S7155_KERNEL_CENSUS.md).

## 1. Раскладка entity-фазы (58.5% CPU = 30625 сэмплов)

| класс работы | сэмплы | % entity-фазы | % total CPU | region-локальность |
|---|---|---|---|---|
| AI/brain/goal/navigation/sensing (local) | 8984 | 29.3% | 17.16% | почти полная (sensing может звать broadphase — уже учтено приоритетом) |
| other-local (tick containers/synched data/etc) | 7533 | 24.6% | 14.39% | полная |
| cross-entity (broadphase) | 7422 | 24.2% | 14.18% | ЗАПРОС к соседям (радиусы ≤1-2 блока push/merge; sensing реже) |
| fluid-scan (block reads) | 5696 | 18.6% | 10.88% | полная (чтение блоков, без других сущностей) |
| movement-integration (block collisions, local) | 990 | 3.2% | 1.89% | полная |

Сверка с S7-154: там broadphase-семейство = 5461 сэмпла = 10.43% CPU (строгие якоря ChunkEntitySlices/CollisionUtil); здесь CROSS = 7422 = 14.18% CPU — шире ключи (+Entity.collide/EntityGetter/EntityLookup-обёртки); обе оценки согласованы по порядку, для Amdahl консервативно берётся широкая.
Item-merge (r≈0.5) в явном виде не выделился (методы вне ключей) — входит в cross/other; на вердикт не влияет (взаимодействия items и так в cross).


## 2. Хазард общего RNG (Level.random) в профиле

Census (javap, весь jar): общий `Level.random` — 132 класса-референсера, из них entity-классов 47; горячий per-tick путь ЧИСТ:

- ItemEntity / Mob: **0** ссылок на общий RNG;
- Zombie: только `hurtServer` (редкий путь); LivingEntity: только `breakItem`; Entity: только `sendBubbleColumnParticles` (в сцене soul-sand нет);
- per-tick RNG = `Entity.random` (109 классов) — per-entity локальный RandomSource → порядок тиков между регионами НЕ влияет на последовательности.

| путь | сэмплы (весь профиль) |
|---|---|
| `Zombie.hurtServer` | 100 |

Вывод: общий RNG в entity-фазе = редкие события; для потокобезопасности достаточно synchronized-обёртки Level.random (вызовы редки — оверхед ~0).

## 3. Структура entity-цикла (javap ServerLevel/EntityTickList)

```ServerLevel.tick: ... → ActivationRange.activateEntities(Level) → entityTickList.forEach(lambda) → [guardEntityTick → tickNonPassenger → Entity.tick() (+passengers рекурсивно tickPassenger)] → tickBlockEntities```

- `EntityTickList` = обёртка над moonrise `IteratorSafeOrderedReferenceSet` (insertion-порядок, безопасные add/remove при итерации) — однопоточная структура; для планировщика шардится по регионам с сохранением insertion-порядка внутри региона;
- `tickNonPassenger` уже потоково-осведомлён (`TickThread.ensureTickThread`, `currentlyTickingEntity: AtomicReference`) — kernel Paper рассчитан на тик-потоки;
- moonrise `EntityLookup` УЖЕ concurrent (SWMRLong2ObjectHashTable, ConcurrentLong2ReferenceChainedHashTable, ConcurrentHashMap, synchronized addChunk/removeChunk); `ChunkEntitySlices` — безлоковые секции (concurrentutil-коллекции) — пишет в индекс только владелец сущности (позиция-мув) → шардирование по регионам даёт disjoint-запись без новых локов.

## 4. Amdahl-сценарии (E=58.5% CPU, non-entity=8.22%, P=3 воркера)

| сценарий | entity-фаза после | main-стена | TPS-потолок |
|---|---|---|---|
| S1: cross-entity СЕРИАЛИЗОВАН (пессимист) | 49.1% | 57.3% | ×1.17 |
| S2: cross region-локален, serial 3% (ожидаемый) | 20.7% | 28.9% | ×2.31 |
| S3: идеал E/P | 19.5% | 27.7% | ×2.41 |

Ключевая чувствительность: S1 vs S2 — разница в ~2× ТПС-потолка. S1 нереалистичен: радиусы доминирующих взаимодействий малы (push ≈1 блок, merge ≈0.5, collide = AABB движения), т.е. взаимодействия пространственно локальны; sensing-радиусы (16-48) в X150K-сцене почти всегда возвращают пусто (целей нет). Геометрия: доля кросс-регионных взаимодействий ≈ 4·r/L (периметр/площадь); при регионе 8×8 чанков (L=128) и r=1-2 → ~3-6% для items/hostiles push+merge.

## 5. Дизайн изоляции (S7-156 прототип)

1. **Регион** = связная группа чанков (старт: 8×8); сущность принадлежит региону своего чанка; пассажиры/транспорт — регион корня (co-residency).
2. **Планировщик** заменяет ТОЛЬКО контейнер цикла: вместо одного `entityTickList.forEach` — разбиение на R region-бакетов (insertion-порядок внутри бакета сохранён) + параллельный tick бакетов на W воркерах + барьер; `Entity.tick()` и всё дерево per-entity вызовов остаются БАЙТ-В-БАЙТ ванилью.
3. **Индекс**: запись в moonrise-секции только от владельца сущности (disjoint по регионам); кросс-регионные чтения — уже concurrent-структуры; переезд сущности между регионами — хэндофф на барьере.
4. **Общий Level.random** — synchronized-обёртка (вызовы редки, оверхед ~0).
5. **Паритет**: per-entity логика ванильна; порядок — region-батчами (в ваниле insertion-порядок глобально перемешан по регионам, кросс-регионное чтение позиции соседа в том же тике меняется на отставание ≤1 тик — статистически эквивалентно, это bar «median-exact parity»); per-entity RNG локален (проверено п.2).
6. **INJECTS-ONLY поверхность**: тело `ServerLevel.tick` правится ретаргетом (прецедент: FluidPushOps/LevelChunk secWrite), Ops-класс RegionTickOps self-contained, rust-wiring по цепочке inside_cache.

## 6. Preregistered гейты S7-156 (прототип, честный A/B min-of-2)

| гейт | критерий |
|---|---|
| PG1 OFFLINE | harness мини-мира: region-параллель vs ваниль-последовательность — per-entity результаты бит-в-бит при одинаковых соседних входах; кросс-регионные кейсы задокументированы |
| PG2 live | 0 NCDFE, ARMED-цепочка живьём, популяция-близнец 150k, 0 падений тик-потоков |
| PG3 TPS | A/B min-of-2 vs CUMULATIVE: TPS ≥ +25% (потолок S2 ×2.3; нога закрывает вопрос ×N-класса) |
| PG4 GC | young GC не выше базы +15% (барьеры/хэндоффы не наливают мусор) |
| REFUTED | TPS-прирост <10% ИЛИ парити-провалы (деспавны/мерджи/РНГ-расползание популяции) не устранимы за 2 итерации |

## 7. ВЕРДИКТ S7-155: ГЕЙТ GREEN

- (a) hot per-tick путь чист от общего RNG (синхронизация редких путей тривиальна);
- (b) доминирующие взаимодействия пространственно локальны (радиусы 0.5-2 блока против региона 128 блоков) — S2-сценарий реалистичен, потолок ×2.3, минимум S1 ×1.1;
- (c) injection-поверхность есть (ретаргет тела ServerLevel.tick — метод уже содержит единый forEach-сегмент для замены; kernel уже многопоточно-осведомлён);
- (d) потолок ≥ ×1.5 = класс ×N, недостижимый для одиночных кэш-рычагов (все REFUTED/GREEN-исчерпаны по S7-154).

Следующий шаг: S7-156 — прототип RegionTickOps (пломбинг → OFFLINE PG1 → preregister dispatch живой ноги A/B min-of-2).
