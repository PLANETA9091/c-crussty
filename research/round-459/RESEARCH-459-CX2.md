# RESEARCH-459-CX2 — C-X2 mob-ai DAG scheduler snapshot (TASK-459-81, WILD-безумие, закон 12e)

Ветка round-459-cx2 = master 0d147876, worktree --no-checkout /tmp/wt459-cx2 + sparse
(src, cplug-abi, cplug-sdk, mobai, goalops, randomtick, .github, scripts). STRICT dormant:
lever `cmp459_cx2` STRICT-eq, пустой/чужой флаг = ваниль бит-в-байт (контракт ×406/×421).

## 0. Идея (своё безумие)

За тик построить DAG зависимостей моба goals→sensors→brain ОДИН раз за N тиков
(snapshot-фаза); тики между — ПЛОСКИЙ проход по замороженному топологическому
порядку (flat pass), без перестроения структур и без итератор-машинерии; любая
мутация топологии (add/removeGoal, behavior add, activity switch) бампает
epoch-счётчик → следующий тик перестраивает (epoch-инвалидация, dirty-flag).
N=4 (дефолт, согласован с golden-window cmp406_aibatch: floorMod(golden32+tick,N)).

## 1. Привязка к кодовой базе (javap/код ground truth)

- `Mob.serverAiStep()V` — единственная точка AI-планы: 4 сайта ретаргета уже
  известны (goal_selector.rs: GoalSelector.tick()V ×2 @161/@183 чётные тики;
  tickRunningGoals(Z)V ×2 @113/@136 нечётные, Paper-сплит). C-X2 = надстройка
  НАД этими точками: вместо per-tick скана goal-множеств — чтение плоского
  frozen-порядка из снапшота.
- `Brain.startEachNonRunningBehavior` — cfdump-контракт brainhook.rs: тройная
  вложенная итерация (TreeMap priorities → per-priority Map → Set behaviors) =
  3 iterator-аллокации на моба на тик. Снапшот-линза уже доказана legal:
  availableBehaviorsByPriority setup-stable — ЕДИНСТВЕННЫЙ сайт мутации во всём
  классе = addActivityAndRemoveMemoriesWhenStopped (BrainOps.java cfdump @693).
  Это готовое epoch-условие для brain-яруса DAG.
- mobs_ai.rs (cmp406_aibatch): nav/ai плану уже сжата окном 1/4: nav_ai
  **14.16% → 3.31%** (ABSORB chkmono457-14). Остаток 3.31% = то, что бегает на
  on-window (ванильных) тиках моба — ровно его и переупорядочивает DAG-снапшот.

## 2. Профильные числа (лист-ранжирование, chkmono457-14 монстр-нога)

| сущность | сэмплы | доля CPU |
|---|---|---|
| Zombie.tick | 18677 | 18.12% |
| Skeleton.tick | 5999 | 5.82% |
| Spider.tick | 5002 | 4.85% |
| Creeper.tick | 3941 | 3.82% |
| **Brain.tick** | **559** | **0.54%** |
| **Brain.tickEachRunningBehavior** | **101** | **0.10%** |

Мандатный остаток лейна (декомпозиция на под-срезы): nav/ai **2.75-3.2пп**
(после cmp406-окна) + brain **1-1.5пп** (Brain.tick 0.54 + tickEachRunning 0.10 +
startEachNonRunning-плечо ~0.4-0.9 по task168 sizing 1.6-1.7% CPU-лейн до F2).

## 3. CAPTURE-МАТЕМ (law 14: lane × capture = Δ)

- nav/ai 2.75-3.2пп × capture **35-55%** (амортизация topology-reads 1/N=4 +
  снятие iterator-чёрна плоского прохода; evidence capture: cmp421 flat
  priorities уже снял priority-скан — остаток = структурные обходы) → **+0.96..1.76пп**
- brain 1-1.5пп × capture **60-80%** (setup-stability cfdump-доказана —
  единственный mutation-site известен, epoch-инвалидация бесплатна) →
  **+0.60..1.20пп**
- **Δ прогноз = +1.6..3.0пп**; **потолок = +4.7пп** (capture 100% обоих под-срезов)
- Единица изоляции: C-X2 НЕ дублирует cmp406 (rate-батч всей планы) и cmp421
  (flat priorities) — он съедает ИХ остаток: on-window ванильный скан структур.
  Композиция по cplug-sdk ordering contract (serve-in-bytes, как goal_selector).

## 4. Источники (рисёрч ≥2 URL, verified HTTP)

1. **https://www.gdcvault.com/play/1022127/Three-States-and-a-Plan-The** (200 OK) —
   Jeff Orkin, «Three States and a Plan: The AI of F.E.A.R.» (GDC 2006). GOAP:
   цели → действия = planning graph (DAG зависимостей); план строится РЕДКО,
   исполняется часто — прямой аналог snapshot/flat-pass-разделения. Взято:
   разделение build-фазы (дорогая, редкая) и execute-фазы (плоская, частая).
2. **http://www.gameaipro.com/GameAIPro/GameAIPro_Chapter06_The_Behavior_Tree_Starter_Kit.pdf** (200 OK) —
   GameAIPro ch.6 «The Behavior Tree Starter Kit». BT = тик распространяется по
   дереву; плоская память узлов + стабильный порядок обхода = практичный шедулер.
   Взято: frozen-обход структуры + плоские node-столбцы вместо ptr-графа.
3. **https://robohub.org/introduction-to-behavior-trees/** (200 OK) — терминология
   BT vs FSM, tick-семантика. Взято: инвариант «тик = проход по готовой структуре».
4. **https://www.gamedeveloper.com/programming/behavior-trees-for-ai-how-they-actually-work-i**
   (403 curl bot-block, текст подтверждён цитатой discussions.unity.com) — Chris
   Simpson, «Behavior trees for AI: How they actually work». Взято: event-driven
   реакция на изменение состояния = epoch-инвалидация при мутациях.

## 5. Скаффолд (WIRED, STRICT dormant)

- `src/ai_dag_snapshot.rs` — чистый Rust (std-only, zero unsafe): node-таблица
  (tier Sensor/Goal/Brain × denseId), edge-список, Kahn-топосорт (детерминизм:
  сортировка по (tier, insertion-индекс) — insertion-порядок = паритет с ванилью),
  epoch-счётчики (build_epoch, invalid_epoch), dirty-flag, note_tick/should_rebuild
  (dirty ∨ ticks_since_build ≥ N), flat_pass по замороженному порядку, invalidate().
  `lever_enabled()` = env STRICT-eq `cmp459_cx2`; без флага — данные без вайринга,
  byte-indistinguishable от ванили. Юнит-тесты: детерминизм топосорта,
  epoch-инвалидация, период-ребилд, flat-pass порядок.
- `mobai/net/minecraft/world/entity/MobAiDagOps.java` — java-стаб контракта
  (javap-верифицируемый: эпохи, invalidate(denseId), gate=false dormant; аналог
  MobAiOps.java-паттерна; компиляция артефакта — только на ARM-фазе).
- lib.rs: `mod ai_dag_snapshot;` — крейт владеет вайрингом (закон 6/12e).

## 6. PREREGISTERED ГЕЙТЫ (закон 16)

- G1 ARM: env `CRUSSTY_LEVER_FLAG=cmp459_cx2` → grep-маркер `ai_dag_snapshot`
  в cpu-collapsed ≥1 на ARM-ноге; пустой флаг = 0 вхождений (спящий-гейт тест).
- G2 lockstep: плоский проход по frozen DAG == ванильный порядок обхода
  (insertion-stable, бит-в-байт оракул на parity-фикстуре goal_selector).
- G3 NCDFE=0, threw=0, AIOOBE=0 (канон).
- G4 популяция-паритет 140-165k, items 0.00 на носителе.
- G5 A/B min-of-3, band 6.0-9.5M; FAIL-CLOSED: epoch-drift → ваниль на тик.
- G6 fuzz-инвалидация: offline-цикл mutate→rebuild→flat 10^6 итераций,
  расхождение порядка = 0.

## 7. Вердикт-число

WIRED 4ф (ai_dag_snapshot.rs + MobAiDagOps.java + lib.rs-mod + RESEARCH-копия);
Δ прогноз +1.6..3.0пп (потолок +4.7пп = nav/ai 2.75-3.2 + brain 1-1.5);
следующий шаг = wiring-фаза (retarget 4 сайтов serverAiStep поверх
goal_selector-композиции) + A/B-нога. CI = гейт компиляции скаффолда.
