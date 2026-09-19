# ABSORB S7-164 leg#1 (35417195790) — TECH-DUD (дефект доставки редиректа, не REFUTED)

**Дата**: 2026-09-19, тик 11:08 +08 (Job 396026) — TASK-312
**Лега**: run 35417195790, head eb89b7d, SUCCESS 11:14:20 +08 (~16 мин полёта), v4-кандидат = CUMULATIVE v3 + zero_alloc_inside=1
**Вердикт**: **TECH-DUD** — экономика рычага НЕ измерена (первичная зона редиректа не исполнялась); REFUTED не выставляется; средовой DUD исключён (стек 100% в наш класс).

## 1. Форензика

- **66 × NoSuchMethodError**: `'boolean net.minecraft.world.level.ZeroAllocOps.collidedWithShapeMovingFrom(net.minecraft.world.entity.Entity, net.minecraft.world.phys.Vec3, net.minecraft.world.phys.Vec3, java.util.List)'` на `Entity.collidedWithShapeMovingFrom(Entity.java)` — с ДВУХ живых сайтов: `InsideBlockOps$Recorder.visit` (stack 1) и `Entity.lambda$checkInsideBlocks$2` (stack 2) — оба через `forEachBlockIntersectedBetween` ← `checkInsideBlocks` ← `applyEffectsFromBlocks` ← aiStep. Все 66 = наш класс: guardEntityTick ловит, тик сущности обрывается (parity сломана).
- 1 × NPE fastutil «wrapped null» (`sendBlockUpdated:1883` ← FarmBlock.fallOn) — исторический шум семейства s7161/s7163-leg2 (0–5/ран бенд).
- 4 × голых «NPE: null» без стека (fast-throw кандидаты; сущности на y≈−28..−29) — происхождение из лога неустановимо; лега #2 ответит (исчезнут = наш сайт в fast-throw форме; останутся ~4/ран = средовой шум).
- Итого 71 «Entity threw exception» >> гейта 0–5 → CRASH-FREE FAIL; ARMED-цепь rc=0 жива до конца (PG2-маркеры: «stage zeroin composed (Retargeted { sites: 3 })», «ARMED chain [inside->rng->batch->zeroin] … rc=0», «zero_alloc_ops: defined», pop 150k VALID, telemetry 4 точки, 0 NCDFE).

## 2. Корень (агентский, класс «замыкание резолюции редирект-графа»)

`ZeroAllocOps.java` НЕ содержал статика `collidedWithShapeMovingFrom(Entity;Vec3;Vec3;List)Z` — его ванильное тело было ИНЛАЙНЕНО в скалярный `collidedWithFluid` (по census считалось, что сайт только там). Но патчер по same-name правилу генерирует `invokestatic` для ВСЕХ трёх сайтов (sites:3), и третий не резолвится против доставленного бриджа. Почему не поймано барьерами:

- **defineClass/HotSpot-верификатор**: верификация НЕ резолвит методы (lazy resolution) — патченный Entity проходит верификатор без ZeroAllocOps вообще.
- **Lockstep-оракул**: зовёт ZeroAllocOps из исходников (compile-time связка) — дескрипторную сверку с патченными байтами не делает.
- **Cargo-тесты**: проверяют структуру патченных байтов Entity (/sites==3), но не сверяют цели с classfile-ом бриджа.

Родня уроку №5 (S7-163 leg#1 NCDFE вложенного класса): доставка = доставка ГРАФА; новая грань — **методное замыкание** (не только классное множество).

## 3. Фикс (этот же тик, head e82d81f)

1. `ZeroAllocOps.java`: добавлен `public static boolean collidedWithShapeMovingFrom(Entity, Vec3, Vec3, List<AABB>)` — вербатим ванили (makeBoundingBox(from) → to.subtract(from) → collidedAlongVector), скаляры = уже оракул-проверенные примитивы (50k makeBox + 300k collidedAlongVectorScalars; новой математики нет).
2. **`ZA_REDIRECT_TARGETS`** (classfile.rs): единая таблица (site, site-desc, target, target-desc) для патчера И гварда — дескрипторы не могут разъехаться по построению.
3. **`zeroalloc_resolution_closure(bridge)`** (classfile.rs): парсит методную таблицу доставляемого ZeroAllocOps.class и требует ТОЧНОГО (name, desc) для каждой цели; Err → объяснимый текст.
4. **Runtime-гвард fail-closed** (zero_alloc.rs activate()): closure-проверка ДО define_class; провал → громкий лог + dormant (сервер живёт ванильно, никакого шторма) — конвертирует класс дефекта «краш-шторм» → «молчаливо-громкий дизарм».
5. **Cargo-тест** `zeroalloc_embedded_declares_all_redirect_targets` — на ДО-фиксных байтах падал бы (ловит класс оффлайн навсегда).
6. Верификация: build_zeroalloc_ops.sh OK; сьют **167/0/1**; оракул **ZEROALLOC LOCKSTEP PASS 350k** + HotSpot-верификация редиректа + «redirected methods present».

## 4. Инцидент диспатча (прозрачность)

Первый редиспатч ушёл на НЕзапушенный head (8edcab5, дефектный код) — ран 35418617801 отменён через API (cancel 202) до начала бута, фикс запушен (8edcab5 → e82d81f), лега #2 редиспатчена чисто. Урок: диспатчить только после push-проверки (`git status -sb` / remote-head == local-head).

## 5. Статус гейтов leg#1 (для истории, НЕ банк)

| гейт | значение | вердикт |
|---|---|---|
| PG2 (маркеры/NCDFE/pop) | все маркеры живы, 0 NCDFE, pop VALID | PASS формально |
| PG3 TPS | медиана last-5 = 1.60 (n=6, сырые 1.5–2.7) | неизмеримо честно (тики сущностей обрывались) |
| PG4a collided-лейн | NoSuchMethodError на первом вызове | НЕИЗМЕРИМО |
| PG4b/c | — | НЕИЗМЕРИМО |
| CRASH-FREE | 71 исключение (66 наш дефект) | **FAIL** |

Итог: ни один экономический гейт не измерим → TECH-DUD по прецеденту TASK-309; гейты leg#2 НЕИЗМЕННЫ (PG2/PG3/PG4a/PG4b/PG4c/CRASH-FREE от свежего v3-профиля, урок №6).

## 6. NEXT

absorb леги #2 (35418679791, head e82d81f) по неизменным гейтам → PASS: CUMULATIVE v4 = v3 + zero_alloc_inside=1; FAIL: REFUTED + rollback zero_alloc_inside=0. Параллельно выполнен RECON-8 (см. research/movement-ai-recon-2026-09-19/RECON8_MOVEMENT_AI.md): следующая цель по сортировке ТОПа — travel-physics (кандидат рычага #11 ZERO-ALLOC-TRAVEL), гейты калибровать от свежего профиля банка.
