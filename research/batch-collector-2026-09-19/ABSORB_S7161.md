# ABSORB S7-161 — BATCH-COLLECTOR ctor-retarget leg (35391679176, head b732b86)

**Run:** 35391679176, v2 + batch_collector=1, SUCCESS. **Verdict: REFUTED-BY-ECONOMICS (PG3+PG4'' FAIL) → rollback batch_collector=0; CUMULATIVE v2 остаётся банком.**

## Гейты (preregister до диспатча)
| Гейт | Значение | Вердикт |
|---|---|---|
| PG2 | NCDFE=0, pop 150k VALID, «batch_collector: defined» + «Entity collector-ctor retarget composed (Retargeted { sites: 1 })» (Entity 205458 → 205546 = rng+batch); first-swap seen (eid=4 — СТАРАЯ системная сущность, единственный легитимный ensure-свап) | **PASS** |
| PG3 | TPS last-5 медиана 1.50 < 1.60 | **FAIL** (в пределах вариан эры 1.60/2.40, но ниже капа) |
| PG4'' | young GC 161 ≤ 180 OK; collector-family 2345→2836, per-work (2836/1.5)/(2345/1.7) = 1.37× (варианс v2 ±0.4%) | **FAIL** |
| CRASH-FREE | 0 NPE / 0 uuid-dup / navmob=0 | PASS |

## Разложение collector-family
| Лист | leg5 (v2) | S7-161 | комментарий |
|---|---|---|---|
| flushStep | 1657 | 983 | **per-work −41%** |
| advanceStep | 491 | 61 | −89% |
| apply+applyAndClear+проч | 97 | 67 | |
| RecordedEffect.<init> | 33 | 0 | |
| МЕТОДЫ ИТОГО | 2345 | **1297** | **per-work 0.63× — рычаг работает** |
| BatchCollector.<init> | — | **801** | инфра (см. открытый вопрос) |
| BatchCollector.ensure | — | **737** | фоновый instanceof-гейт (~0.18мкс × 40M вызовов — убрать из горячего пути) |
| ИТОГО | 2345 | 2836 | +20.9% абс, +37% per-work |

## Инженерные выводы
1. **Сами методы BatchCollector стабильно быстрее ванили** (flushStep −41% per-work, advanceStep −89%, RecordedEffect eliminated, young GC 161 = уровню leg#5). Два подряд REFUTED — оба раза из-за ИНФРАСТРУКТУРЫ доставки, не из-за семантики.
2. **ensure-вызов в RegionTickOps.tickBucket должен быть удалён** (737 сэмплов чистого фонового instanceof на 40M вызовов/окно): при ctor-патче он не нужен вовсе.
3. **BatchCollector.<init> = 801 сэмпла при инжекте ВНЕ окна и нулевой заметной ротации в leg#5 (vanilla ctor = 0 сэмплов)** — объяснение требует живой телеметрии (счётчик INSTANCES в ctor + периодическая печать). Гипотезы: (a) непрерывный натуральный спавн-поток мобов на живой сцене («мобы спавнятся/деспавнятся как с игроками») + тяжёлый ctor (10 ArrayList); (b) JIT-эффект размещения параллельным воркером. S7-162 отвечает точно.
4. **S7-162 (композиция)**: хронология этого рана перевернула порядок активаций относительно leg#5 (region computed 919-921 → inside armed 1031) — chain при inside-retransform: inside serve 205522 → region serve 205546 (READY) → итог composed ✓; НО ПРИ ОБРАТНОМ ПОРЯДКЕ АКТИВАЦИЙ (leg#5) итог = rng-only и inside-гейт вероятно терялся. Постоянная корректность требует ЕДИНОЙ compose-цепочки Entity в одном hook'е (inside → fluid_free → fluid_dirty → rng → batch → set_patch), а не двух superseding-хуков.
5. Vanilla-parity: 0 NCDFE/0 NPE/0 uuid-dup/pop valid на всём цикле — fail-closed дисциплина цела.

## Вердикт
- **BATCH-COLLECTOR рычаг закрыт с вердиктом REFUTED-BY-ECONOMICS** (две независимые попытки доставки: ленивый Unsafe-свап S7-160, ctor-ретаргет S7-161 — оба REFUTED по экономике профиля). rollback batch_collector=0; v2 (inside_cache+flush_diet+region_threads=4) остаётся банком; код dormant-invisible.
- INJECTS-ONLY цел (1 CI-лег = preregister A/B).

## NEXT (S7-162 — «ТОП-ПОЖИРАТЕЛЬ → ∞»)
1. Единая compose-цепочка Entity (фикс S7-162-композиции) + телеметрия INSTANCES; при подтверждении спавн-потока — возможна повторная атака collector-семьи «на вырост» (методы доказанно −37..−41% per-work), но только если инфраструктурный хвост станет <10% семьи.
2. Свежий ТОП (этот ран): entity-фаза 56.60% (inside-pipeline 20.4% фазы → traversal-часть; broadphase 18.7%; fluid 16.8%; AI 8.3%; movement 8.2%; nav 4.9%) → GC/JIT 36.6% → tracker ~2%. Рефутед-классы кэшей не реанимируются; attackable ≥5% под-лейны: traversal-часть inside-pipeline (плоский обход вместо guava-итератора — высокий parity-риск, нужен javap-контракт DirectionalIterator) и… ниже порога всё.
3. Возврат к ТОП-1 по кругу после S7-162.
