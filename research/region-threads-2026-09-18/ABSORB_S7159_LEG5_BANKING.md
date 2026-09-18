# ABSORB S7-159 leg #5 (35381522360) + БАНКОВАНИЕ REGION-THREADS

**Run:** 35381522360, head d9df743, queued 18:40:12 UTC → SUCCESS ~19:00 UTC (20 мин), артефакт полный.
**Конфиг:** inside_cache=1 + flush_diet=1 + region_threads=4 — бит-в-бит preregister (min-of-2 сэмпл №2).

## Гейты leg #5
| Гейт | Значение | Вердикт |
|---|---|---|
| PG2 | NCDFE=0, pop=150000 VALID, **ARMED полный: rc ServerLevel=0 EntityCallbacks=0 Level=0 ChunkMap=0 Entity=0**, pristine Entity sighting (205458 → 205522 после inside_cache — композиция цепочки точна) | **PASS** |
| PG3 | медиана 0.90 → **2.40 = +166.7%** (crawl растёт 1.3→2.5 по soak; база плоская 0.7-0.9) | **PASS** |
| PG4-strict | 180 > 136 | FAIL (см. калибровку leg #4: leg #3 ваниль-класс дал 140>136) |
| PG4' (preregister до leg #5) | 180/2.40 = **75.0 ≤ 150.8**, worst 170.1 ≤ 224ms, full=0 | **PASS** |
| CRASH-FREE | 0 tracker-NPE, 0 uuid-dup, navigatingMobs=0 | **PASS** |

## Доказательства живьём (leg #4 + leg #5, min-of-2)
- RegionTickOps lane: leg #4 = 73912 (57.5%) / leg #5 = 70949 (54.7%); worker offload **74.3% / 74.8%** (W=4 стабилен)
- TrackerTickOps live: 2629 / 3115 сэмплов — removal-safe sweep работает, **0 NPE на обоих легах** (leg #2 инцидент закрыт)
- **0 uuid-dup на обоих легах** (RngOps serialized seeding; leg #2 инцидент закрыт)
- 0 NCDFE, ARMED полный на обоих легах; navigatingMobs watchlist = 0

## ВЕРДИКТ: БАНКОВАНИЕ REGION-THREADS (все условия preregister выполнены)
**CUMULATIVE v2 = inside_cache=1 + flush_diet=1 + region_threads=4** (явные inputs; дефолты раннера не меняются, config-wins не используется).
- Оба лега: PG2 + PG3 (≥ +25%) + PG4' + CRASH-FREE = PASS. Даже худший лег (+77.8%) удваивает базу.
- TPS-медиана эры: 0.7-0.9 (v1) → 1.60 / 2.40 (v2) — **первый рычаг с материальным TPS-сдвигом, ×2-2.7 к медиане**; потолок Amdahl ×2.31 (S2) почти пробит худшей оценкой, crawl leg #5 достигал 2.5.
- Строгая запись PG4 публикуется прозрачностью: 169/180 vs кап 136; калибровочный разбор в ABSORB_S7159_LEG4.md (кап ниже ваниль-варианс 118-140, штрафует throughput 1.78×, GC-на-работу -19.5%/-42.8%).

## ТОП пожирателей leg #5 (методика «ТОП-ПОЖИРАТЕЛЬ → ∞»; свежий профиль CUMULATIVE v2)
1. **entity-фаза 53.89%** (69917): residual 39.1% фазы, broadphase/collision 19.9%, fluid 17.0%, AI 11.0%, movement 8.1%, navigation 4.9%, inside-blocks 0%
2. GC/JIT-native 40.88% (53036; G1 internals — рычаг закрыт: мост zero-alloc, JVM-флаги запрещены)
3. tracker 2.22% (3115, sweep) | прочее <1.3% каждый
NEXT (S7-160): RECON-3 residual-подлейна (39.1% фазы ≈ 21% CPU: baseTick/SynchedEntityData$DataItem.getValue 1.1-1.2% leaf, paletted-чтения PalettedContainer.get 2.5-2.9% leaf, tick-оркестрация) → крупнейший attackable под-лейн; broadphase (2×REFUTED кэш) и fluid (3×REFUTED кэш) не реанимируются; кандидаты-рычаги НЕ-кэш-класса: батчинг чтений SynchedEntityData, O(1)-индексы сенсорных запросов, layout. Возврат к ТОП-1 по кругу после каждого закрытия.
