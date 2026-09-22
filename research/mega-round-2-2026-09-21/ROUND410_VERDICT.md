# ROUND410_VERDICT (tick-410, 2026-09-22 08:43→? +08, interim)

## База-410 (якоря ×3 @8ef9e5b ваниль, абсорбированы)
- anchora **2.50 @ 8548287** (items 30.96%, band OK)
- anchorb **2.95 @ 8880170** (items 29.19%) — медленный пул, высокий TPS
- anchorc **2.60 @ 8827927** (items 29.18%)
- Пул-410 = {2.50@8548287, 2.95@8880170, 2.60@8827927} ⊕ банк {2.4@6604167, 2.2@6976430, 2.25@6833921, 2.30@6846036}. Пул бимодальный: 8.5-8.9M медленные раннеры.

## Вердикты тика
| вектор | нога | pair (ближайший раннер) | статус |
|---|---|---|---|
| **MULTI cmp409_multi @f7d04d2** | multi3 **3.0 @ 6464483** | **+25.0** (vs 2.4@6604167 Δ140k; против 4 быстрых якорей +25.0/+33.3/+30.4/+36.4 медиана ≈+31.9) | **min-of-3 ЗАКРЫТ: 3/3 GREEN** (multi1/2 +20.8 ×2, multi3 +25.0) — серия медиана ≈+21pp, items 0.00 ×3, nav_ai 7.54-10 (низший 7.54), GC 18.5s. 3.0 TPS = рекорд эры. БАР 80% НЕ ВЗЯТ — мержа нет |
| sscan-ветка (vanilla+despawn+stagger) | eleg2b 2.60@6598051 / eleg3b 2.50@6771554 | **+13.0 / +8.7 (медиана +10.9 ×2)** | GREEN ×2, эффект-маркеры ×2 (despawn-scan gate hit tick 23 + epoch ok) — мост b405fe1 РЕАЛЬНЫЙ; на multi-базе маргинал ≈+1пп (перекрытие mob-плоскости, 3-й случай эры) |
| eindex min-of-3 | cleg3 2.20@6790872 / cleg4(k2) 2.20@6667072 | −2.2 / −2.2 | **ЗАКРЫТ PARITY/RED**: cleg1 0.0 / cleg3 −2.2 / cleg4 −2.2, broadphase флэт 15% — lookup-индекс мимо горячего пути |
| eindex cleg5b @d05c930 | run 35673647163 | — | **RED-LEVER-BUG**: 1027× AIOOBE (Index -32/-64, length 256) в server-stdout; ваниль/cleg4 = 0 AIOOBE → from_add/from_rem хуки removeEntity/moveEntity портят секции → FIXTURE-INVALID ×2 детерминированно. Ре-ролл запрещён; 4-сайтный индекс выжен из k3-ветки (UPPER-NOTE-cleg5b.md в worktree C) |

## Посимвольный профиль broadphase (RESEARCH-C-k3, подтверждён верхним)
ChunkEntitySlices.getEntities **48.6%** лейна (вход EntityLookup.getEntities(AABB) 27.1%), CollisionUtil block-collision 37.1% — cleg4 ≡ ваниль ±1пп (патчи честно спали на мёртвых сайтах). Пивот C: goal-target/sensing query-класс (batch-snapshot R2).

## Волна-410 (R-векторы закона 6, в полёте)
- **A-k5**: A* node-pool (navplane, base 8ef9e5b) — дизайн locked (pool laundering + generation-теги).
- **B-k2**: fluid СЕКЦИОННЫЙ bulk-JNI редизайн (base 8ef9e5b, flag cmp410_fluidsec) — анти-урок bleg1.
- **C-k3**: pivot на query-класс (RESEARCH-C-k3 STEP1 done).
Ноги прилетят этим тиком → вердикты pair-by-runner по пулу-410.

## Лестница к 80% (честная математика)
multi +21 (mob-плоскость закрыта, перекрытия) → fluid 16-18% + inside 12% + broadphase-query 15% = оставшиеся ~45% wall — только их зачистка даёт путь ×1.8. Состав: multi⊕fluid⊕inside⊕query — каждый вектор = Rust-плейн по закону (6).
