# RECON-12C — Вердикт v5-кандидата #13-SBB SKIP-STORE-BB (лег s7166 = run 35428713486) — 2026-09-19 ~15:3x +08

**Вердикт: #13-SBB = REFUTED** (прегистер TASK-318, гейты НЕИЗМЕННЫ; FAIL → REFUTED + rollback). Банк остаётся CUMULATIVE **v3** (inside_cache + flush_diet + region_threads + batch_collector); skip_store_bb default '0' = rollback автоматичен; SkipStoreOps остаётся инфраструктурой (доставка валидна, PG2 PASS).

## Лег
- run 35428713486 (world-bench-3, success), head 896781b, точный v3-банк + skip_store_bb=1 + recon_diag=1 (тот же JFR/remset оверхед, что у диаг-базы 35425246662 → валидный A/B)
- Артефакты: research/gc-recon-2026-09-19/run-s7166-v5-candidate/ (remset.log, refine.log, recon.jfr, gc.log, cpu/alloc/wall-collapsed, server-stdout, spark arCsGa90bZ)

## Таблица гейтов (прегистер TASK-318)
| Гейт | Требование | Измерено | Вердикт |
|---|---|---|---|
| PG2 доставка | sites:1 + «skip_store_ops: defined» + ARMED + 0 NCDFE + pop 150k VALID | «entity_compose: stage sbb composed (Retargeted { sites: 1 })»; «skip_store_ops: defined … in kernel loader»; ARMED [inside->rng->batch->sbb] 205458→205362 B rc=0; NCDFE=0; POPULATION FIXTURE-VALIDITY: VALID 150000/150000 | **PASS** |
| PG3 TPS | last-5 медиана ≥ 1.60 | 1.3/1.4/1.5/1.7/1.8 → **1.5**; база s7165 (тот же recon_diag) = **2.0** (1.4/1.9/2.0/2.3/2.5) | **FAIL** + регресс −25% vs диаг-базы |
| PG4a remset dirty-cards p50 | −10%+ (ожидание 10-25%) | 6,324,224 → 6,356,992 = **+0.52%** (165 циклов → 161; max 7.44M → 7.52M) | **FAIL** (эффект = 0) |
| PG4b card-set CPU-лейн | −10%+ | 25.72% (32,887/127,850) → 23.82% (30,669/128,730) = **−6.74%** | **FAIL** |
| PG4c AABB-аллокация по счёту | −20%+ (база 16.75%) | jfr ObjectAllocationSample: 13,297/78,686=16.90% → 12,041/70,583=17.06% = **−9.45%** по счёту; async-profiler: −10.79% (доля 8.96%→6.75%) | **FAIL** |
| CRASH-FREE | 0 crash / 0 Full / шум ≤5% | 0 crash / 0 Full GC; young-паузы 322 → 316 (−1.9%) | **PASS** |
| Парити-оракул ≥1M | закрыт офлайн | 1,100,000 PASS (skipped 40,000/40,000 форсинг; бит-паритет; identity; ±0.0 strictness) — TASK-319 | **ЗАКРЫТ** |

Итог: 4/6 полевых гейтов FAIL → REFUTED. Ни один банковский критерий не достигнут.

## Научные находки (measurement-by-effect — главная ценность лега)
1. **Value-equal подмножество setBoundingBox НЕ драйвит remset firehose**: dirty-cards не изменились (+0.5% при том же посещении карт p50 3.83M). Механика: движущиеся сущности меняют bb каждый тик → бит-равенство 6 компонент почти никогда не выполняется → skip не срабатывает → putfield (и карта) остаётся. Счёт AABB −9..−11% = скип стреляет только на стационарных сущностях — стоки малы.
2. **Проверка дороже стора**: при почти-всегда-неравных значениях каждый вызов платит 6×doubleToLongBits + 6 cmp + бридж-косвенность (рывок инлайн-профиля) без экономии ничего → TPS 2.0 → 1.5 на идентичной сцене. Классический анти-паттерн «дешёвый стор, дорогой чек».
3. Card-set lane (TOP-1 ~24-26% CPU) подтверждённо драйвится ДРУГИМИ семьями записей: setDeltaMovement (locked-out по парити — окно вынесенного параметра), sync/SynchedEntityData, chunk entity-lists, block-change. Старое→новоеentity-поля в целом НЕ дают управляемого value-equal пространства.

## Следствие для ТОП «→ ∞»
- Семья store-skip (#13-*) ЗАКРЫТА полностью: широкая = paper-REFUTED (RECON-12, канал недостаточен), setDeltaMovement = INFEASIBLE-BY-PARITY (javap-контракт), setBoundingBox = REFUTED полевым A/B (этот лег).
- Свежий ТОП (v5-профиль, card-set lane 23.82%): ТОП-1 = G1/GC-фаза (card-set + oop-scan) без парити-безопасного рычага в классе store-skip → декомпозиция: oop-scan 10.70% и RECON-13 (other-entity аллок 28.04%: LazyEntityCollisionContext 1.19%, Entity$$Lambda 1.16%).
- Приоритет по директиве владельца (тик 14:43): **FREE-HOST профиль-лег 2.5GB (xmx≈2G, fp4/150k/seed42, recon_diag=1)** — низкая куча → young-GC чаще → пересортировка ТОПа под целевой free-hosting профиль. Это следующий диспатч.
