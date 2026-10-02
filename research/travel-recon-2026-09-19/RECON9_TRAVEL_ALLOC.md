# RECON-9: аллокационная и CPU-атрибуция travel-пути → вердикт paper-REFUTED #11 → кризис ТОПа → рычаг #12

**Дата**: 2026-09-19, тик 12:08 +08 (Job 396026) — TASK-314
**База**: банковый профиль CUMULATIVE v3 (35399980345; 128124 CPU / 10263 alloc) — урок №6 (гейты/вердикты только от свежего профиля банка)
**Метод**: exact deepest-match по полному фрейму (endswith; урок подстрочных багов: `/Entity.collide` ⊂ `/Entity.collidedWithFluid`); скрипт `recon9_travel_alloc.py`, сырой вывод `RECON9_raw.txt`

## 1. Аллокационная ось: entity AABB+Vec3 = 4115 (40.1% всего аллок-давления)

| сайт (deepest-match) | сэмплы | % entity AABB+Vec3 | статус |
|---|---|---|---|
| unmatched | 1031 | 25.1 | гетерогенный (push/collision-контексты без CORE-маркера) |
| EntityDimensions.makeBoundingBox | 598 | 14.5 | НЕ атакован; тело МЕЛКОЕ (инлайнибельное → урок №8) |
| CollisionUtil (внутр. temps) | 587 | 14.3 | moonrise-код, редирект = вербатим стороннего оптимизатора |
| checkInsideBlocks | 515 | 12.5 | inside (bank); collided-часть — #10 REFUTED |
| AABB.collidedAlongVector | 430 | 10.4 | #10 REFUTED |
| collidedWithFluid | 351 | 8.5 | #10 REFUTED |
| Entity.move | 235 | 5.7 | НЕ атакован; тело большое, но аллок-часть мала (5.7%) |
| setPosRaw / collide / travel / findSupportingBlock / остальное | 408 | 9.9 | мелочь |

**Не-атакованная конкретная поверхность #11** (makeBoundingBox+move+setPosRaw+collide+travel+findSupportingBlock+applyEffects): **1201 сэмпл = 11.7% всего аллок-давления**.

## 2. CPU-ось: travel-семья = математика и чанковые чтения, НЕ аллокации

collide/travel CPU-семья = 7823 = **6.11% всего CPU** (move 3.31 + collide 0.80 + travel 0.79 + findSupportingBlock 0.43 + setPosRaw 0.31 + …); CollisionUtil-внутренние стеки 4.15% (broadphase-родня, 2×REFUTED); PalettedContainer.get 1.31%. Урок №10-обобщение: zero-alloc редирект НЕ снижает CPU тела (доказано #10: 18.7%==18.7%) — CPU-ось travel (арифметика + палитровые чтения) редиректом не снимается вообще.

## 3. ВЕРДИКТ #11 ZERO-ALLOC-TRAVEL: **REFUTED-BY-ECONOMICS (paper, БЕЗ CI-бута)**

- Потолок аллок-оси: 11.7% давления × young-GC STW ≤ 7.1% wall = **≤ 0.83% wall-clock** (идеальные условия, все сайты сняты) — это 2-3% MSPT-класс, явно запрещённый владельцем.
- CPU-ось: конверсии нет (урок №10-обобщение).
- Риск: makeBoundingBox — мелкое инлайнибельное тело → урок №8 предсказывает регресс как в #10.
- Прецедент честности: REFUTED на бумаге дешевле REFUTED CI-легом (лег #10 уже заплатил за этот урок 1 CI-бутом).

## 4. Кризис ТОПа (нормальный этап «→ ∞»)

Все ≥5% пожиратели свежего ТОПа (v3) закрыты:
- fluid-push 11.0% CPU — #10 REFUTED (редирект регрессивен); кэш-форма = класс 3×REFUTED fluid-кэшей (позиции движущихся); ПАРК;
- inside-pipeline 10.6% — inside_cache в банке; остаток = vanilla-семантика entityInside; ПАРК;
- travel-physics 9.15% CPU + 13.87% alloc — #11 paper-REFUTED (§3); CollisionUtil-внутрянка = moonrise-оптимизирована;
- broadphase 6.5% — 2×REFUTED; goal-selector/navigation — ПАРК (RNG-parity / hit-rate≈0, RECON-8);
- aiStep/entity-other 6.4% — гетерогенный, атакующих ≥5% нет (RECON-8);
- GC/JIT 33-35% — даунстрим аллок-давления; zero-alloc поверхности исчерпаны (#10/#11), JVM-флаги запрещены;
- item-entity 4.0%, tracker 2.1% — <5%, не трогать.

**Вывод: текущий арсенал эры (кэш/мемоизация, редирект-скаляризация, батч-коллектор, планировщик) исчерпан на этом профиле. Нужен НОВЫЙ КЛАСС рычага.**

## 5. Рычаг #12: NATIVE-COLLIDE (Rust/JNI батч-порт collide-пути) — единственный не-попробованный класс миссии

**Миссия прямо перечисляет: «Rust/JNI порты горячего пути», «батч-операции».**

- Цель: collide-лестница travel-семьи — CollisionUtil CPU 4.15% + Entity.move 3.31% + collide 0.80% ≈ **8.3% CPU** + сопутствующие аллокации (CollisionUtil-temps 587 + move 235 + expandTowards и пр.).
- Архитектура: батч-вызов — ОДИН JNI-переход на регион-батч (массив задач: позиции/дельты/боксы; массив результатов: смещённые дельты), внутри Rust — bit-exact double-математика коллизий (IEEE754 детерминирован → vanilla-parity достижим по построению, проверяется бит-в-бит оракулом 1M+ сценариев). Чтения блоков — через переданный буфер палитр/секций (копия снимаемой области, O(батч) вместо O(сущность)).
- Риски (честно): JNI-overhead (снимается батчингом), копирование секций (стоимость O(активные секции)/тик — надо мерить), сложность вербатим-переноса CollisionUtil-семантики (сотни строк: лестница expand→collide→contract по осям, пороги 1.0E-7, border-ветки). Это multi-tick рычаг.
- План (по прецеденту #10): тик+1 = RECON-10 (javap-контракт collideBoundingBox/performCollisions лестницы verbatim) + каркас Rust-ядра + JNI-мост; тик+2 = оракул бит-в-бит (портированная математика vs ваниль, ≥1M сценариев) + гейты от свежего профиля (урок №6) + preregister; тик+3 = compose-интеграция + диспатч.
- Критерий GO/NO-GO после RECON-10: если вербатим-поверхность лестницы > ~400 байткодов с внешними вызовами (world/border/палитры) — вердикт INFEASIBLE-BY-VERBATIM и закрытие направления (честный REFUTED-BY-DESIGN, без CI-бута).

## 6. Учёт

- Артефакты: `research/travel-recon-2026-09-19/{RECON9_TRAVEL_ALLOC.md, recon9_travel_alloc.py, RECON9_raw.txt}`
- GOAL СТАТУС ×1, CLAIMS TASK-314, worklog ×2, атомарный append; диспатчей за тик 0 (RECON-тик, S7-108 чист)
- NEXT: RECON-10 по #12 (javap-контракт лестницы collide) → GO/NO-GO → реализация/закрытие направления
