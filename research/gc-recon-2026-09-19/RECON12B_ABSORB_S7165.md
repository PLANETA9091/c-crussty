# RECON-12b: ABSORB диагностического лега 35425246662 (S7-165 recon_diag=1) — вердикт #13 SKIP-STORE-DIET и объявление #13-SBB

TASK-318 (тик 14:08 +08, Job 396026). Лег: точный v3-банк + recon_diag=1,
head dd904c7, run 35425246662, ЗАВЕРШЁН success 06:11:36 UTC. Это диагностический
лег (НЕ гейт-лег): JFR-overhead → CPU/TPS числа не идут в банкинг.

## 1. Доставка инструмента — ЦЕЛА (уроки №5-класс)

recon_diag=1 применился: remset.log (debug-строки «Visited cards / Total dirty»,
165 GC-циклов), refine.log (6879 строк), recon.jfr (23MB, profile, dumponexit).
FIXTURE-VALIDITY: VALID. Анализатор recon12_store_firehose.py (smoke-passed)
+ счёт-пересчёт. Проверка javap-контракта до лега: RECON12A_SKIP_STORE_IDENTITY.md.

## 2. Измерения

### remset debug (прямой агрегат old→young активности)
- **dirty cards/cycle p50 = 6,324,224** (max 7,438,336) на young-GC;
- dirty% p50 59.63% (max 89.58%); 165 GC-циклов; sum visited 511.5M карт;
- масштаб firehose TASK-316 (4-6M записей/с) подтверждён снизу: 6.3M карт/цикл
  при 9.4s интервале = ~670k карт/с; записей ≥ карт (коэффициент 6-9 сходится
  с моделью 4-6M/s).

### jdk.OldObjectSample — КАНАЛ НЕДОСТАТОЧЕН (honest tool verdict)
- всего 72 события за 461s: leakage-сэмплер JDK трекает малую долю TLAB-ов и
  репортит ДОЛГОЖИВУЩИЕ объекты (objectAge p50 = минуты; стеки boot-фазы
  paperclip/ZipFile/String/byte[]), НЕ поток promoted-entity-объектов;
- по этим 72: entity-семьи 23.61% — выборка нерепрезентативна по построению;
- ВЕРДИКТ-КАНАЛА: NO-GO-INSUFFICIENT-TOOL (инструмент не ответил на вопрос гейта).

### jdk.ObjectAllocationSample — 78686 сэмплов (солидная аллокационная выборка)
По СЧЁТУ сэмплов:
- **boundingBox/move семья: 29052 = 36.92%**
- other-entity: 22062 = 28.04% (Entity$$Lambda 1.16%, LazyEntityCollisionContext
  1.19% — отдельно-декомпозируемые кандидаты)
- non-entity/other: 22544 = 28.65% (byte[]/char[]/Object[]/сериализация)
- block-change: 2.95%; deltaMovement/travel: 2.89%; sync: 0.50%; chunk-lists: 0.05%
- **entity-семьи суммарно: 71.35%**
- топ-классы: Vec3 14673 (18.65%), AABB 13179 (16.75%), long[] 6.60%,
  MutableBlockPos 4.55%, BlockPos$6 4.50%, BlockPos 4.22%

ВАЖНО: это доля АЛЛОКАЦИЙ, не доля old→young ЗАПИСЕЙ — аллокация ≠ запись
(temps умирают без записи; записи не требуют свежей аллокации). Гейт TASK-316
требовал ИЗМЕРЕННУЮ долю ЗАПИСЕЙ — она этим каналом не измеряется.

## 3. Вердикт по preregister TASK-316 (НЕ переписывается)

Гейт: «измеренная доля entity-полей в old→young ≥ 40% И model-потолок ≥ 2-3%
wall → GO #13; NO-GO = paper-REFUTED #13 без леги».
- измеренной доли ЗАПИСЕЙ нет (OldObjectSample NO-GO-INSUFFICIENT-TOOL;
  ObjectAllocationSample — аллокации, не записи);
- условие GO НЕ ОТКРЫТО → **NO-GO; #13 SKIP-STORE-DIET в широкой формулировке
  (все entity-поля, долевая гипотеза) = paper-REFUTED как НЕДОКАЗАННАЯ**
  (не «ложная» — неверифицируемая этим инструментом; пост-хок порог не
  переписывается, вера запрещена);
- grep identity-использований (RECON-12a, сделан ДО лега): getDeltaMovement —
  5 identity-сайтов (главный — guard в Entity.move offset 280-285) → сеттер
  deltaMovement = INFEASIBLE-BY-PARITY-РИСК для value-equal skip;
  getBoundingBox — 0 identity-сайтов во всех 10 фикстурах → setBoundingBox
  value-equal skip = ПАРИТИ-SAFE ПО КОНТРАКТУ.

## 4. Объявление рычага #13-SBB: SKIP-STORE-BB (узкий, javap-контрактный)

ТОП-1 «→ ∞» = card-set/remset 16.39% CPU (TASK-316) → атака через срез
bb-записей: **value-equal skip в setBoundingBox** (единственный сеттер с
parity-safe контрактом):
- тело-редирект: нормализация verbatim (6 полей, dcmpg «<0→max=min» ×3,
  dcmpl «>64→min+64» ×3, NaN-вербатим) + skip если normalized(arg) равен
  ТЕКУЩЕМУ bb по 6 компонентам (dcmp-равенство, бит-в-бит) → putfield не
  выполняется, new AABB не создаётся;
- эффект тройной: срез self-аллокации new AABB (16.75% аллокационного счёта
  частично), срез old→young карты putfield bb, срез монитора/чтений;
- MEASUREMENT-BY-EFFECT: remset dirty-cards p50 лега-базы 35425246662 =
  6,324,224/цикл при ИДЕНТИЧНОМ оверхеде (recon_diag=1 в обоих) → A/B по
  dirty-cards = прямое измерение срезанных записей (доля entity-полей, наконец);
- гейты (preregister, калибровка от лега-базы 35425246662 — урок №6):
  PG2 доставка (Retargeted sites:1, ARMED chain, 0 NCDFE, pop 150k VALID,
  «skip_store_ops: defined»); PG3 TPS last-5 медиана ≥ порог банка; PG4a
  remset dirty-cards/cycle p50 снижен ≥ 10% (ожидание 10-25%: bb-доля карт
  неизвестна — её и меряем); PG4b card-set/remset CPU-лейн снижен ≥ 10%;
  PG4c AABB-аллокация по счёту снижена ≥ 20% (16.75% база); CRASH-FREE
  0 crash/0 Full/шум ≤ 5; ПАРИТИ-оракул: локстеп setBoundingBox ≥1M сценариев
  бит-в-бит (нормализация + skip-семантика: старый bb vs normalized(arg);
  + identity-инвариант: ссылка bb не меняется при skip — консьюмеров identity
  НЕТ, но инвариант проверяется самим оракулом); банковское правило: PASS →
  CUMULATIVE v5 = v3 + skip_store_bb=1; FAIL → REFUTED + rollback (ops
  остаётся инфраструктурой).
- Реализация (тип+1 = тик 14:43): ZeroAllocOps-прецедент: SkipStoreOps.java
  (1 classfile, no nested — урок №5), method-body redirect
  Entity.setBoundingBox(AABB) (173 юнита, dcmpg/dcmpl verbatim — урок №9),
  entity_compose stage 8, cargo-гварды (resolution-closure), локстеп-харнесс.

## 5. NEXT

- тик+1 (14:43): реализация #13-SBB (SkipStoreOps + redirect + compose stage 8
  + оракул ≥1M бит-в-бит + cargo) → preregister-фиксация гейтов (§4) → диспатч
  лега (v5-кандидат = v3 + skip_store_bb=1, recon_diag=1 для remset A/B).
- тик+2: absorb по гейтам §4 → вердикт; при PASS — v5-банк + свежий ТОП
  (пересортировка «→ ∞»); other-entity 28.04% — RECON-13 декомпозиция
  (Lambda-семья, LazyEntityCollisionContext 1.19%).
- deltaMovement/sync/chunk-lists — закрыты/вне #13 (identity-риск / <1%).

Артефакты: run-s7165-recon-diag/ (remset.log, refine.log, recon.jfr,
cpu/alloc/wall-collapsed, gc.log), RECON12A_RAW.txt, RECON12B (этот документ),
recon12_store_firehose.py (обновлён: object-class фикс + счёт-пересчёт).
Диспатчей за тик 0 (лег 35425246662 поглощён; S7-108 чист).
