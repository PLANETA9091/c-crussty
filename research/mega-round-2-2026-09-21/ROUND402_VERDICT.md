# ROUND-402 VERDICT (TASK-402, тик 18:08→2x:xx +08, cron 402447)

## База сравнения
- master 3c03dad (= код d26f524). Свежие якоря same-day: **2.40@6741944 / 2.20@6728002** (anchor2 35587640390 = world-bench FAIL ×1, не ≥3 — root-fix не мандат; anc4 master-экстра = INFRA-DELIVERY-FAIL discard).
- TASK-401 same-day: 2.2@7125977 / 2.6@7237388 / 2.3@6797859; TASK-400: 2.4@7071566 / 2.3@6940803 / 2.4@8344070 / 2.35@7112096 / 2.2@6811622.
- Fast-pool ориентир: 2.4@8767633 (TASK-401 G-anchor, Δк нашим fast-ногам <100k).

## Карта векторов (все ARMED-пруф по server-stdout; вердикт pair-by-runner, min-of-3 на кандидатах)

| вектор | ветка/гейт | ноги (tps@runner) | pair | вердикт |
|---|---|---|---|---|
| **stagger (push 1/N + canUse 1/N, golden-hash)** | round-401-i-stagger @d47d944 / cmp401_stagger | 2.70@6780763, 3.10@8803783, 2.75@7115601 | **+12.5 / +29.2 / +5.8..14.6 → медиана ≈+12.5pp** | **ТОП РЕПЛИЦИРОВАННЫЙ (3/3 ARMED, line +17..21 стабилен)** |
| **CMP-402 композит (soa ⊕ grid-mirror ⊕ shardgrid ⊕ lifetime)** | round-402-b-comp @7daf5a1 / cmp402_comp | 2.70@8759893 | **+12.5pp** vs 2.4@8767633; items 31.17→0.00% | реплицированная ступень, НЕ синергетична (сумма компонентов +17.5 не добита) |
| E-soa (SoA read-plane) | round-401-e-soa @9300ad0 / cmp401_soa | 2.45@6584415, 2.80@8823891, 2.10@6638187 | +6.5 / +16.7 / −4.5 → **медиана +6.5pp** | реплицирован (min-of-3 выполнен), ступень |
| nav-sys memo replace | round-401-c-navsys @8904e08 / cmp401_navsys | 2.25@7083077 | −6.3pp (ARMED ×4 маркеров — честный) | RED: memo-лейн <0.5% headroom (leaf-профиль agent-c) |
| stagcomp (comp ⊕ stagger) | round-402-f-stagcomp @79baef9→60fe902 / cmp402_stagcomp | leg1 2.3@6684981 = PARTIAL-ARM (mobs_soa/grid dormant — гейт-баг, чинен 60fe902) → невалидна; leg2/leg3 @60fe902 = **BAND-DISCARD ×2 (11794022 / быстрый пул, fast-fail pre-download за 20с, ре-роллы ≤2 исчерпаны)** | — | **UNPROVEN (band-пул ночи, не ветка-дефект; в дневное окно ре-ран)** |

## Root-causes / инфра-уроки тика
1. **Диск 100% во время абсорбов** (Errno 28) — PHASE-0 гигиена + purge 2.5G tracked-артефактных worktree; patched-kernel.jar восстановлен из /tmp в round-j2b (был снесён purge).
2. Sed-патч dispatch-LEGS испортил словарь → 4 ноги с неверными ref/lever (cancel ×4, чистый ре-диспатч). Урок: словарь ног менять только копированием файла с правкой руками.
3. **Полу-armed stagcomp leg1**: mobs_manager.java_gate_matches не принимал cmp402_stagcomp (soa+grid dormant при ARMED-маркере остальных половин) — фикс 60fe902. Урок: новый композитный флаг должен проверяться grep-ом по ВСЕМ гейт-сайтам rust+java до диспатча (чек-лист: mobs_soa/mobs_grid/mobs_manager/items_index/items_manager/stagger + java IME/MobPushOps/RegionTickOps).
4. **«Bottleneck report gate failure» — имя шага обманчиво**: реальная причина фейлов anchor2/stagcomp2×2 = BAND pre-check fast-fail (runner_cpu_index 11.79M — ночной быстрый пул 10-12M, известная бимодальность). BAND-закон сработал как задуман; ре-роллы ≤2 исчерпаны — вердикта нет, ре-ран в дневное окно.
5. c-nav: nav-memo RED — leaf-профиль: serverAiStep subtree = ChunkEntitySlices.getEntities 1.50% + fastutil goal-churn 1.5% + GoalSelector 1.02% + Sensing 0.41%; реальный рычаг = stagger (подтверждён +12.5pp ×3).
6. item-lane 31.17% в T3-профиле master-якорей — post-J lane-атрибуция (rust-сторона в items-бакете); java-сторона 0%.

## Мерж
- БАР = **+80% pair-stable**. Лучший раунда stagger ≈+12.5pp медиана — **МЕРЖА НЕТ**. stagcomp = UNPROVEN (band-пул), ре-ран NEXT-403.

## NEXT-403
1. **stagcomp ре-ран в дневное окно** (ветка+гейт готовы @60fe902, оба ARM-пруфа каноничны) — при ортогональности ожидание ~+20-25pp.
2. stagger-варианты к 80%: N-скан (N=2/8 вместо 4), stagger на collide/sensing-лейны, динамический N по нагрузке — банч-ноги на готовой ветке d47d944.
3. jnibulk rust-сторона (raw-arena/coarse-stamp — скрытая цена 12-17% wall) — recon agent-d в работе (worktree round-402-d-jnibulk).
4. TICK-PLANE whole-body retarget — recon agent-e (worktree round-402-e-tickplane, setup).
5. Карта ступеней к 80%: stagger +12.5 ⊕ comp +12.5 ⊕ soa(в comp) — сумма ~+25-31pp; нужны ещё 2-3 MEGA-вектора класса «замена подсистемы» либо радикальный срез nav+push+collide (~35% wall) единым rust-плейном.
