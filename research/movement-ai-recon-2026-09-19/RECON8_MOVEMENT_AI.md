# RECON-8: декомпозиция movement/AI-поверхности entity-фазы (ТОП-1)

**Дата**: 2026-09-19, тик 11:08 +08 (Job 396026) — TASK-312
**Методика**: «ТОП-ПОЖИРАТЕЛЬ → ∞» (директива владельца 2026-09-19 ~00:51 +08) — ТОП-1 entity-фаза 60.16% (leg#2 35410873485), coarse-под-лейн «movement/AI-aiStep 20.33%» разложен до классов поведения.
**Контекст**: лега S7-164 ZERO-ALLOC-INSIDE (35417195790) в полёте → S7-108 блокирует диспатч → тик = RECON (прецедент TASK-308/RECON-6).

## 1. Метод атрибуции

**Deepest-match**: каждый стек entity-фазы (маркер `RegionTickOps.tickBucket`/`tickNonPassenger` — классификатор линии RECON-6/7) сканируется от ГЛУБОЧАЙШЕГО фрейма к корню; первый фрейм, совпавший с паттерном класса, именует класс. Поведенческий контекст (targeting/goals/navigation) побеждает собственных калеев (`getEntities`, `AABB.intersects`) — экономически честная атрибуция AI-сканов. Классификатор тот же на обоих профилях → межпрофильная сверка.

## 2. Таблица классов (доля от ВСЕГО CPU)

| класс | leg#2 (35410873485, 125610) | v3 (35399980345, 128124) | статус |
|---|---|---|---|
| fluid-push | **10.32%** | 11.00% | атакуется рычагом #10 (в полёте) |
| inside-pipeline | **9.68%** | 10.60% | inside_cache=1 банке; collided-часть — в #10 |
| travel-physics | **9.15%** | 8.56% | НЕ атакован на скалярном уровне |
| broadphase | 6.52% | 5.87% | 2×REFUTED — не трогать |
| aiStep/entity-other | 6.40% | 6.08% | гетерогенный резидуал (см. §4) |
| baseTick-other | 4.89% | 4.41% | |
| item-entity | 4.03% | 3.65% | |
| goal-selector | 3.49% | 3.25% | см. §5 (парковка) |
| navigation | 3.29% | 3.28% | см. §5 (парковка) |
| sensing | 1.34% | 1.12% | |
| controls | 0.56% | 0.51% | |
| brain | 0.48% | 0.41% | |
| targeting | 0.33% | 0.31% | |

Entity-фаза целиком: 75974 = **60.48%** CPU (leg#2) / 75667 = 59.06% (v3) — суммирование классов сходится без остатка. Ранжирование стабильно на двух независимых легах (n=2, разброс ≤0.6 п.п.) — сортировка ТОПа под-лейнов пригодна для фиксации следующего рычага.

Coarse-лейн «movement/AI-aiStep 20.33%» (маркеры RECON-7) = travel-physics 9.15 + goal-selector 3.49 + navigation 3.29 + sensing 1.34 + controls 0.56 + targeting 0.33 (+ хвост в entity-other) — декомпозиция закрыта.

## 3. Sub-splits атакуемых классов (leg#2)

- **travel-physics 9.15%** (alloc 13.87% всего давления — 2-й после inside-pipeline): `LivingEntity.travel` 6510 (56.6%) → `Entity.move` 4404 (38.3%) → `Entity.collide` 580. Топ-листы: `CollisionUtil.getCollisionsForBlocksOrWorldBorder` 725, `PalettedContainer.get` 584, `Level.findSupportingBlock` 407, `Entity.setPosRaw` 379, (v3: `AABB.<init>` 476).
- **navigation 3.29%**: `createPath` 3363 (81.3%) → findPath(A*) — топ-листы `PathTypeCache.get/compute` (кэш УЖЕ существует), `PalettedContainer.get`, `getBlockStateFinal`.
- **goal-selector 3.49%**: `selector-tick` 2663 (60.8%) — **чистая итерация** `ObjectLinkedOpenHashSet$SetIterator.next` 690 (15.7% класса) + `GoalSelector.tick` 488 + `tickRunningGoals` 375 + vtable 338 + `Profiler.get` 250; `canUse`-тела 1540 (35.1%).
- **targeting 0.33%**: NearestAttackableTargetGoal 70.3% класса; `AttributeMap.getValue` 64 — крупнейший лист.

## 4. aiStep/entity-other 6.40% — гетерогенный, ≥5% атакующего НЕТ

Лидеры резидуала (доля от 8035): `ServerLevel$$Lambda.accept` 10.8%, `Entity.setOldPos` 7.2%, vtable stubs 6.3%, `Entity.setTicksFrozen` 5.5%, SynchedEntityData-чтения ~7.7%, `checkDespawn` 2.3%, `refreshDirtyAttributes` 2.3%. Все ≤0.7% общего CPU → внутри-под-лейнов ≥5% нет; класс паркован (мелкая середина запрещена владельцем).

## 5. Парковки с вердиктами (чтобы не бить в закрытые двери)

- **goal-selector (3.49% < 5%)**: стоимость = СЕМАНТИКА ванили (полная итерация availableGoals дважды/тик + running-цели). Любое сокращение итераций меняет поток RNG: `canUse` случайных целей (RandomStrollGoal.nextInt(10) и т.п.) имеет side-effect на RandomSource моба → median-exact parity недостижим архитектурно. ПАРК без права атаки.
- **navigation (3.29% < 5%)**: 81% createPath, но цель stroll-путей случайна на моба → мемоизация hit-rate ≈ 0% (урок fluid_dirty S7-152 «REFUTED-BY-ECONOMICS: hit-rate ≈ 0%»). PathTypeCache в ядре уже кэширует PathType. ПАРК.
- **broadphase (6.52%)**: 2×REFUTED ранее. Не трогать.

## 6. ВЕРДИКТ RECON-8: следующий рычаг

**ТОП-1 не-атакованный под-лейн ≥5% = travel-physics: 9.15% CPU + 13.87% alloc.**

Кандидат рычага **#11 ZERO-ALLOC-TRAVEL** — тот же класс, что #10 (НЕ кэш): скалярные double-тела горячих аллокаторов/лестниц travel-пути (`handleRelativeFrictionAndCalculateMovement` → getInputVector/Vec3-цепи; `Entity.move` → makeBoundingBox/inflate/expand-лестницы; `collide` → collideWithShapes-temps), IEEE754 бит-в-бит, без RNG-поверхности → vanilla-parity по построению. Примитивы #10 (`makeBox`, `collidedAlongVectorScalars`, `clipPoint`, `getDirection`) переиспользуются напрямую.

**Отличие от REFUTED ALLOC-DIET (S7-133)**: тот атаковал call-site ретаргеты мёртвых контейнеров (push-обёртка ArrayList, MutableBlockPos-кольцо) — убрал аллокации, но НЕ трогал скалярную математику тел move/travel; #11 атакует тела методов (класс #10, механика METHOD-BODY REDIRECT уже в арсенале эры и верифицирована 350k-оракулом).

**Порядок (по методике)**: absorb #10 → пересортировка ТОПа по трём осям от СВЕЖЕГО профиля банка → калибровка гейтов #11 ТОЛЬКО от свежего профиля лега-базы (урок №6) → javap-контракт 12-15 методов travel-пути → lockstep-оракул → entity_compose stage 8 → preregister → диспатч.

## 7. Артефакты

- `research/movement-ai-recon-2026-09-19/recon8_movement_ai.py` — декомпозер (deepest-match, sub-splits, alloc-ось, кросс-чек v3)
- `research/movement-ai-recon-2026-09-19/RECON8_raw.txt` — полный вывод
- Вход: `research/flat-traversal-2026-09-19/run-s7163-leg2-artifact/` (свежий банк-базовый профиль), `research/batch-collector-2026-09-19/run-s7162-leg2-artifact/` (v3 кросс-чек)
