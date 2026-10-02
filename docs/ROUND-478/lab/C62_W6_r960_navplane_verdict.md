# C62 / 478-W6 — r960 NavPlaneOps композиция r960-стратум+canon: вердикт-матем → REFUTED_CENS (ЛАБ v19.0, тик 2026-09-28 02:4x-03:0x +08, master 386887a8)

## 0. Вход (все числа из артефактов W4-x4 + A9-док, 0 новых диспатчей)
- **W4-x4 run 36358072152** (round-477-w4-x4, SUCCESS, r960 fresh-gen, canon-мир afb3a0b3, 300s/150k/seed42) → cpu-collapsed **115,202** сэмплов: NavPlaneOps inclusive **0 (0.000%)** — lever DORMANT на canon; sendBlockUpdated incl **1024 (0.889%)**; vanilla shouldRecomputePath incl **884 (0.767%)**.
- **A9 (Л-478-A9, terr640 run 36331223216, 102,699 сэмплов)**: NavPlaneOps.handle leaf-self 2.205% / incl 3.105%, НО 100% = vanilla block-update плоскость (LeavesBlock.randomTick→removeBlock incl 1.319% + FlowingFluid/LavaFluid spreadTo incl ~1.6% + CraftBlockState.place ~0.3%); decision-kernel navDecide УЖЕ Rust bulk-JNI (закон 6); apply-сторона = vanilla PathNavigation.recomputePath в порядке сета (закон 4 ваниль-паритет). Потолок terr-ноги: центр +2.65пп / щедрый +3.73пп / абсурд +5.59пп < +20 → REFUTED_CENS (дефицит ×3.6..×7.6); реалистично +0.5..+2.0пп < merge-гейта +2пп.
- **A4 (Л-478-A4)**: NavPlaneOps 0 leaf-фреймов (ancestry 4.6–5.25%) → **kill-потолок ~1пп < +2пп**.
- Стратум-множитель composition canon→Terralith: **×2.5** (A9 §2: 2.205% self vs банк 0.886%).

## 1. Вердикт-матем композиции r960-стратум + canon (гейт: capture ≥2пп → диспатч)
Все легальные ветки композиции (Δnorm = ΔCPU% × k[1.2,1.8], канон C13/C95.3):
1. **canon-банк × стратум-множитель (щедрая)**: shouldRecomputePath 0.767% × 2.5 = **+1.92пп** < 2пп;
2. **вся sendBlockUpdated-плоскость × k1.8 (абсурд)**: 0.889% × 1.8 = **+1.60пп** < 2пп;
3. **A4 kill-потолок**: 0 leaf-фреймов → **~1.0пп** < 2пп;
4. **terr-нога real (A9 §3)**: +0.5..+2.0пп < 2пп — на терра-мире lever ARMED даёт 3.105% incl, но весь инкл = vanilla block-update; Rust-first v2-ваниринг двигает Java-сбор входов, не убирает его.
- Дефицит к гейту 2пп: ×1.04 (щедрая 1.92) .. ×2.0 (kill 1.0); к абсурд-раунд-бару +20пп: ×10.4..×20.
- Максимум всех веток = **+1.92пп < 2пп** → композиция r960-стратум+canon гейт НЕ пробивает; гипотеза-дельта НЕ открывается.

## 2. Вердикт
**REFUTED_CENS: потолок композиции +1.92пп (0.767% DORMANT × k2.5 стратум) < 2пп** (kill-потолок ~1пп, абсурд-плоскость +1.60пп, terr-real +0.5..+2.0пп). Диспатч round-478-w6-r960 НЕ легален (placebo-запрет: capture < гейта на всех ветках); 0 диспатчей, 0 код-дельт, N1=1 (соло-ценз на W4-x4+A9 артефактах), N2=0. Закон-5 чисты (нет fluid/mega/bitmask/fluid_dirty — fluids в терра-ноге = фидеры ЗАПРЕЩАЮЩИЕ, не цель), NCDFE T1=0, band 6.0–9.5M ✓ (36358072152 in-band), ваниль-паритет ✓ (закон 4 — nav-apply не трогаем). W4-x4 «band-open 2.205%» был терра-стратум числом (c74_terr 102,699), НЕ r960-canon: r960 fresh-gen canon = 0.000% — безумие закрыто кросс-поверхностным контролем A9.

## 3. Следующий ботлнек (закон 18)
**SynchedEntityData-плоскость 3.445% incl** (getHealth 0.54+0.42 / isSprinting 0.50 / ItemEntity.getItem 0.43 / packDirty 0.25) — легальная generic-плоскость (не закон-5), CLAIM 479. Палитра-плоскость 5.47% self (PalettedContainer.get 4.208 + SimpleBitStorage 1.260) на терра — юридически заблокирована сверху (фидеры fluid-family закон-5 + collision-scan уже kernel-armed). BENCH-4 fixture G3.0-класс на терра-мирах — блокер norm-транспорта стратума (владелец МЕРЖ №17-класс фикс).
