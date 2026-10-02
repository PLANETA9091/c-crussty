# ROUND-405 BOTTLENECK (2026-09-22 00:2x +08, TASK-405)

## Контекст
- Топ-комбо эры: cmp402_stagcomp (comp⊕stagger) @60fe902 — pair +20.8pp медиана (серия 6 ног, шум −15.4..+31.3), НЕ смержено (бар 80%).
- JNIBULK FUSED (mob_push_step 1-переход, bfeafe8): серия min-of-4 +10.9/0.0/+10.9/0.0 → медиана +5.5, БИМОДАЛЬНОСТЬ ПО РАННЕРУ (6694402→+10.9, 6663598→0.0); поверх comp (bleg1/bleg2 = cmp402_comp+cmp403_jnibulk ARMED) ПРИБАВКИ НЕТ — push-слой уже закрыт comp.
- Якорный пул (pair-by-runner, runner_cpu_index): 2.40@7089030 / 2.1@6898063 / 2.3@6706928 / 2.25@6621660 / 2.1@6781844 / 2.6@9297453 / 2.40@6741944 / 2.20@6728002 / 2.2@7125977 / 2.6@7237388 / 2.3@6797859.

## ТОП-3 ЛЕЙНА в ARMED-ногах (bleg1, total=111257 сэмплов)
1. **fluid 17.41%** (updateFluidHeightAndDoFluidPushing/FlowingFluid/getFluidState) — ГЛАВНЫЙ RUST-ТАРГЕТ закона (6): перенос fluid-tick батчем в Rust, ОДИН JNI-вызов/тик-батч. ВАЖНО: fluid_dirty-мемо и fluid_bitmask ЗАПРЕЩЕНЫ (закон 5) — только честный перенос вычислений.
2. **broadphase 12.50% + inside_volatile 12.54%** (getEntities 9.45% докомпозитного профиля, EntitySectionStorage/CollisionUtil/checkInsideBlocks) — R2: getEntities/radius-сканы в Rust-индекс с bulk-синком секций за 1 JNI-вызов.
3. **nav_ai 9.34%** (PathNavigation/GoalSelector/Brain/behavior/PathFinder; было 14.16% до stagcomp) — R1/R3: nav→Rust bulk (батч запросов путей → 1 JNI-вызов → готовые пути).

## Скрытая цена JNI-границы: 12-17% wall (дельта-метод RECON-44) — стирается ТОЛЬКО bulk-дизайном (per-entity JNI = дизайн-ошибка).

## План тика (закон 6 RUST-FIRST)
- Волна-1 (все R-векторы): A=nav→Rust bulk (cmp405_navrust), B=fluid→Rust (cmp405_fluidrust), C=getEntities→Rust индекс (cmp405_eindex). База всех: 60fe902.
- Верхний агент: якоря ×2 + stagcomp день-4 дрейф-лег (в полёте: 35625851249/35625876464/35625905853), композиция stagcomp⊕tickplane (если окно позволит).
- Бар 80% pair-stable (min-of-3, parity PASS, RAM/CPU без регресса). Ступени фиксируются в карту векторов.
