# ROUND-478 / lab A14 — inlining-квантование (Л-475-C49) [478-A14]

Tick ×478 MEGA-SWARM v19.0, cmd z-478-A14 (рестарт волны-1 после context-deadline).
CLAIM: 4 граничных сайта 309/282/275/257B = 79–95% FreqInlineSize(325); правило «split-до-арма»;
Δпп-потолок ≤0.5пп = шум.

## 1. javap-аудит (kernel-jar артефакта W3-c5; независимо — classfile-парс Code-attr, конвенция +1)

Канон C49 подтверждён байт-в-байт (реальный javap -p -c, /home/z/tools/jdk-21.0.12.1+1):

| метод | canon javap | classfile parse | % от 325 |
|---|---|---|---|
| `Mob.serverAiStep()V` | **309B** | 310B | **95%** |
| `GoalSelector.tick()V` | **282B** | 283B | **87%** |
| `BlockCollisions.computeNext()` | **275B** | 276B | **85%** |
| `PathNavigation.doStuckDetection(Vec3)V` | **257B** | 258B | **79%** |

6 tick-hot OVER 325B (канон): `CollisionUtil.getCollisionsForBlocksOrWorldBorder` 1066B,
`CollisionUtil.collideX/Y/Z` 574B ×3, `ItemEntity.tick` 588B, `PathNavigation.createPath` 364B
(парс: 1067/575/589/365). Armed-плоскость: `GoalOps.tickGate` **694B** = 2.1× OVER (см. §4).
Дубли 282B: `Mob.checkDespawn` / `Mob.dropCustomDeathLoot` (не тик-хот).

## 2. Hot-path ценз (root→leaf collapsed, 2 независимых артефакта, pop150k/300s)

| сайт | W3-c5 self | W3-c5 incl | dpfull self | dpfull incl |
|---|---|---|---|---|
| Mob.serverAiStep | 0.047% | **13.94%** | 0.049% | **11.96%** |
| GoalSelector.tick | **0.660%** | 11.21% | **0.734%** | 9.26% |
| BlockCollisions.computeNext | 0.034% | 0.31% | 0.017% | 0.12% |
| PathNavigation.doStuckDetection | 0.007% | 0.08% | 0.004% | 0.10% |
| **Σself 4 сайтов** | **0.747пп** | | **0.804пп** | |

Артефакты: W3-c5 run **36357571844** (арт 10944464706, мир afb3a0b3, vanilla-leg) /
C98a-dpfull run **36356927274** (арт 10944632528, мир a13b353a). Hot-path подтверждён
как CALLER-лейны (incl 9–14%), но self-сумма — 0.75–0.80пп на оба мира.

## 3. Δ-потолок сплита (механика, не элиминация)

Сплит «холодная ветка в отдельный метод» НЕ удаляет работу из горячего тела:
(а) вынос холодных веток start/stop/disabled → экономия = I-cache/branch-layout доля self,
(б) пере-открытие inlining — см. §4 = 0 offline.
Realistic capture = холод-доля × self единственного сайта ≥0.3пп gross (`GoalSelector.tick`
0.66/0.73пп) ≈ **0.07–0.22пп**; call-overhead ≤200k calls/s × 2–3нс ≈ 0.005пп.
**Δ-потолок ≤ 0.2пп < 0.5пп шум-бара → REFUTED_CENS, 0 диспатчей.**
Gross 100%-kill Σself = 0.75/0.80пп — физическая верхняя граница, сплитом недостижима.

## 4. Offline PrintInlining-харнес (прошлый тик, /home/z/rounds/ROUND-478/A14/inlining-harness)

Old arm: master `GoalOps.tickGate` 694B (classfile-парс подтверждён) — «inlining prohibited by
policy» + «hot method too big» → НЕ инлайнится в hot-caller (4M вызовов).
New arm: сплит tickGate **282B** — «callee is too large» (C1-бар MaxInlineSize=35) +
«already compiled into a big method» (C2) → **inline-recovery = 0**.
Vanilla `GoalSelector.tick` 283B: «callee is too large» в обоих армах.
Паритет: `done gate=true sink=15999943` в обеих армах (бит-в-бит).

## 5. ВЕРДИКТ

**REFUTED_CENS (потолок ≤0.5пп = шум)** — 0 диспатчей, 0 код-дельт, 0 CI-ран.
Канон «split-до-арма» подтверждён как ГВАРД-ПРАВИЛО (клифф-карта: гвард-арм рядом с 309/282/275/257B
обязан идти ПОСЛЕ сплита), НЕ как +пп-ливер.

## 6. ГЛУБЖЕ

1. Inlining-квантование — риск-грид, не ливер: 4 сайта горячие как caller-лейны (incl 9–14%),
   self-сумма 0.75–0.80пп; даже 100%-kill всех self не даёт ≥1пп.
2. 325B-окно FreqInlineSize достижимо только в специфичных C2-условиях: hot 282B callee
   всё равно не инлайнится на C1-слоях (35B бар) и C2 («already compiled into a big method») —
   сплит 694→282B recovery=0 (харнес-факт).
3. Kernel-факт: на ARMED-ногах vanilla `GoalSelector.tick` = dead code (замещён `GoalOps.tickGate`,
   Л-463-P42); armed-квант-долг = tickGate 694B (2.1× OVER, никогда не инлайнится) — структурен,
   сплитом не лечится.
4. Профиль-инструмент: async-profiler collapsed-стеки сохраняют инлайн-фреймы раздельно
   (tickRunningGoals 0.48–0.60пп self виден отдельно от tick) — лейн-матем по self/incl валидна.
