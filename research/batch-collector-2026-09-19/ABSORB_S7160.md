# ABSORB S7-160 — BATCH-COLLECTOR leg (35387310239, head 72a7f55)

**Run:** 35387310239, v2 + batch_collector=1 (bit-exact vs preregister), SUCCESS ~19:58 UTC.
**Verdict: REFUTED-BY-ECONOMICS (PG4'' FAIL) → rollback batch_collector=0; CUMULATIVE v2 остаётся банком. Рычаг сам по себе работает, свап-инфраструктура не удерживается.**

## Гейты (preregister до диспатча)
| Гейт | Значение | Вердикт |
|---|---|---|
| PG2 | NCDFE=0, pop 150000 VALID, ARMED: «batch_collector: defined …kernel loader» + «ARMED first-swap (eid=4)» + v2-цепь жива | **PASS** |
| PG3 non-regression | TPS last-5 медиана 1.60 (≥ 1.60 на границе; leg#5 = 1.70, хвост 2.0 vs 2.5) — в пределах вариан leg#4/leg#5 (1.60/2.40 на идентичной v2) | PASS (формально) |
| PG4'' | young GC 161 ≤ 180 **OK** (−10.6% vs leg#5, full=0); collector-family 2345→2780, per-work 1379→1738 (**+26%**, ваниль-варианс метрики per-work leg#4/leg#5 = 1374/1379 = ±0.4%) | **FAIL** |
| CRASH-FREE | 0 tracker-NPE, 0 uuid-dup, navmob=0 | PASS |

## Разложение collector-family (лист-атрибуция)
| Лист | leg5 (v2) | S7-160 (v2+bc) | Δ |
|---|---|---|---|
| flushStep | 1657 | 1035 (Batch) | **−38% (per-work −34%)** |
| advanceStep | 491 | 85 (Batch) | **−83%** |
| apply | 97 | 13 | −87% |
| applyAndClear | 63 | 45+5 | −35% |
| RecordedEffect.<init> | 33 | **0** | zero-alloc подтверждён |
| **BatchCollector.<init>** | — | **728** | свап-инфра |
| **BatchCollector.ensure** | — | **713** | свап-инфра |
| appendEffect | — | 156 | плоский append |
| ИТОГО | 2345 | 2780 | +18.6% абс., **+26% per-work** |

## Root-cause: свап не удерживается между тиками
- Ротация сущностей в профайл-окне ≈ 0 (Entity.<init> = 4 сэмпла в S7-160 vs 7 в leg#5) — НОВЫХ сущностей нет.
- Тем не менее BatchCollector.<init> = 728 сэмплов ≈ миллионы свапов за окно (~60% популяции на тик в <init>-эквиваленте) — `ensure` видит в поле ВАНИЛЬНЫЙ коллектор повторно на тех же сущностях.
- Поле `Entity.insideEffectCollector` — **private final**; единственный писатель ядра = ctor (javap census: единственный NEW-сайт Entity.<init>:193-200). Физический механизм потери Unsafe-записи в final-поле (C2-обработка final-полей вокруг Unsafe putObjectVolatile / OopMap-иерархия) — на живой диагноз S7-161 (прямой счётчик свапов + периодическая печать).
- ПРЯМОЙ ЭФФЕКТ РЫЧАГА ПОДТВЕРЖДЁН: все overridden-методы BatchCollector дешевле ванили на единицу работы (flushStep −34%, advanceStep −83%, apply −87%, RecordedEffect eliminated). Если свап станет раз-на-сущность, ожидаемая экономика = 2345 → ~1200-1400 листов (−40..−50% семьи, −1..1.3% total CPU) + alloc-плюс (RecordedEffect/immutable → 0).

## Вердикт и откат
- **REFUTED-BY-ECONOMICS по preregister PG4''**: инфра-накладуха свапа (1441 сэмпл) перевешивает прямой выигрыш (~700 сэмплов). batch_collector=0 в будущих ранах; v2 (inside_cache+flush_diet+region_threads=4) остаётся банком. Код остаётся в master dormant-invisible (каноничная дисциплина эры: ALLOC-DIET-прецедент).
- ИНЖЕКТЫ: 1 CI-лег = санкционированный preregister A/B; sandbox-бутов 0.

## NEXT (S7-161 — «ТОП-ПОЖИРАТЕЛЬ → ∞» продолжается)
1. Гарантированно стойкий свап: **rust-ретаргет NEW-сайта StepBasedCollector в Entity.<init> → BatchCollector** (санкционированный класс рычага — как RngOps S7-158d: classref-замена + invokespecial-дескриптор совпадает; probe-гейт + retransform Entity rc=0). Это закрывает ensure/ctor-инфраструктуру полностью (1441 → ~0).
2. Альтернативная ветка, если ретаргет NEW отклонён: живой диагноз механизма потери записи (счётчик + печать раз в 10с) — 1 лег.
3. После закрытия collector-подлейна: residual-хвост (вне inside-pipeline: fluid-residual 1.74%, data-sync 1.44%, tick-оркестрация 1.20% — все <5%) и возврат к ТОП-1 (entity-фаза, broadphase 10.6%/fluid 9.1% — REFUTED-кэши не реанимируются).
