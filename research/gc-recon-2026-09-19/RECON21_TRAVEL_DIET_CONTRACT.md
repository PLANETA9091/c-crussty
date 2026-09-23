# RECON-21 — javap-контракт #14 v2 TRAVEL-DIET (TravelDietOps) — 2026-09-20 ~03:0x +08 (TASK-342)

База: свежая аллок-атрибуция alloc-окна s7177 (12,392 сэмпла, банк v3, живое окно AP —
`recon21_travel_attr.py`) + javap-дампы реального kernel jar (run-s7178-recal, Temurin 21,
`recon21_javap.sh`) → `contract-traveldiet-s7182b/`. S7-закон: javap-контракт до редиректов.

## 1. Свежая атрибуция travel-листьев (s7177, % alloc-окна)
| сайт | сэмплы | % окна |
|---|---|---|
| Vec3.add | 753 | **6.08%** (travel-доля 286 = 2.31%; остальное ai/goals+nav — лейн TOP-2 CPU, НЕ travel) |
| EntityDimensions.makeBoundingBox | 585 | **4.72%** (584 через Entity.makeBoundingBox(Vec3): checkInsideBlocks + collidedWithShapeMovingFrom) |
| AABB.inflate | 482 | **3.89%** |
| Vec3.multiply | 377 | **3.04%** |
| travel-чейн доля листьев | 1248 | **10.07%** |

## 2. Что уже убивает v1 (s7182 zero_alloc+skip_store_bb, в полёте)
- `updateFluidHeightAndDoFluidPushing` целиком (там и `AABB.inflate(D)` @ Entity:13569) → inflate-доля в основном мертва под v1.
- `collidedWithShapeMovingFrom` → её сайт makeBoundingBox мёртв.
- Остаток makeBoundingBox: `checkInsideBlocks` (per-step) — перекрыт банкованным inside_cache частично.

## 3. v2a COLLIDE-DIET (первоочередная, лейн travel-collide = 7.71% CPU якоря, TOP-4)
javap-факты (Entity.txt):
- `private Vec3 collide(Vec3)`: `AABB.expandTowards(Vec3)` @off126, `CollisionUtil.getEntityHardCollisions` @off154, `AABB.expandTowards(DDD)` @off355+off372 — временные боксы запроса коллизий, результат выбрасывается → скалярный запрос по 6 double без материализации.
- `private static float[] collectCandidateStepUpHeights(AABB, List<VS>, float, float)`: `expandTowards(Vec3)` @off6 — временный бокс.
- `public void move(MoverType, Vec3)` (578 строк, moonrise-патч): Vec3.add ×2, Vec3.multiply ×2, Vec3.scale ×1 — темпорари математики движения; коллайд-вызов 1.
- Лестница RECON-10 (native-collide-2026-09-19): eps **1.0E-7** (COLLISION_EPSILON), dcmpg/dcmpl лестницы бит-в-бит; performCollisions 120 юнитов / 8 dcmp. C2 инлайнит лестницу (CPU-атака была REFUTED), но АЛЛОК-сайты живут в байт-коде тела — аллок-атака валидна.

## 4. v2b TRAVEL-MATH (вторая очередь)
- `private Vec3 handleRelativeFrictionAndCalculateMovement(Vec3)` (48 строк): getDeltaMovement ×3, moveRelative ×1, setDeltaMovement ×1 — цепочки Vec3 scale/add темпорари.
- `travelInFluid` (якорь s7177) + `getInputVector` — тот же класс математики.

## 5. Механика редиректов (прецедент S7-164 ZeroAllocOps — без изменений)
- TravelDietOps.java: receiver-prepended statics, РОВНО ОДИН classfile (S7-163 урок, cargo-тест на вложенные классы), delivery в kernel loader, BRIDGE_READY → entity_compose stage-7 body-redirect (same descriptors, length-preserving).
- Верификация: javap-дифф скомпилированного бриджа против дампа (прецедент SkipStoreOps 0-138 идентичны); бит-точный порядок double-операций + dcmp-лестницы RECON-10; никакой мемоизации между вызовами (median-exact парити).
- Workflow input: travel_diet (гварды: существующие 25 инпутов не трогать, лимит 25 — мёртвые дропать, НЕ zero_alloc).

## 6. Прегистер-гейты будущего лега #14 v2 (travel_diet=1 поверх банка v3)
- PG-T1 delivery: travel_diet=1 + банк v3 маркеры, Retargeted{sites:N} в stdout, NCDFE=0, pop 150k VALID.
- PG-T2 crash-free: 0 threw/NPE, soak TPS polls ≥3.
- PG-T3 ДВОЙНОЙ БАР (закон 2026-09-20): runner в банде 7.8M..9.2M; нормализованная дельта (med/runner)/(1.40/8493973) И абсолютная med/1.40 — ОБЕ ≥ +10% → CANDIDATE GREEN → подтверждающий мин-of-2 → банкинг v4; одна ось = НЕ GREEN.
- PG-T4 GC sanity: young ≤ 174, 0 Full; PG-T5 DONE-park N/A (прецедент AP-PID).

## 7. Порядок ветвления
s7182 (v1) GREEN → банкинг v4, v2 не нужен. s7182 < +10% хотя бы одна ось → v2a COLLIDE-DIET
реализуется из ЭТОГО контракта без дополнительного RECONа (реализация + гейты + commit/push +
dispatch в одном тике).
