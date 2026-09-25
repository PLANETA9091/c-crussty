# RESEARCH-459-CX1 — LCG-эпоха снапшотов entity slices (TASK-459-80, закон 11, v18.2)

- **Вектор**: C-X1 (своё безумие main-B) — вместо полных инвалидаций EntitySectionArrays при мутациях
  держать 64-битную LCG-эпоху **на секцию** (bump = один add), читатель сравнивает эпоху **одним cmp**
  до полного пути (CHM-lookup / перестройка среза). Паттерн secWrite-эпох ×458-P3 (P32 SNAP sidecar +
  P36 Inside-epoch fast-gate), но на **entity slices** (EntitySectionStorage / ClassInstanceMultiMap).
- **Lever**: `cmp459_cx1` — STRICT eq, **dormant** (пустой/чужой флаг = ваниль бит-в-байт; без JNI-wiring,
  без ретаргетов; scaffold = ядро эпох + self-тесты + java-стаб).

## 1. Ground-truth числа (профиль/код, ≥5)

1. Inside-лейн (снапшоты секций уже живы на secWrite-эпохах): PalettedContainer.get **0.487%** +
   SimpleBitStorage.get **0.126%** + readPalette **0.099%** + LevelChunk.getBlockStateFinal **0.498%**
   ≈ **1.3–1.6%** total CPU (PROFILE-B; src/inside_snap.rs header).
2. Item-broadphase лейн, идущий через EntitySectionStorage: **15.66%** item-driven CPU — whole-16³-section
   ClassInstanceMultiMap scan + per-entity AABB (src/items_index.rs: заменён флет-гридом cmp397/399).
3. Снапшот Snap уже несёт `volatile long gen` — «secWrite bumps on real change»
   (InsideSnapOps.java:168, ref-compare old != newState ⇒ идентичная запись не бампает).
4. ×458-P3 вердикты (ROUND-458/BLACKBOARD.md): ID-P32 SNAP sidecar — epoch одним int-cmp до полного
   пути, прогноз java_util −40–60% CHM-части → **+0.8–1.2пп**; ID-P36 флет-массив эпох секций в связке →
   **−0.5–1пп**. Оба — на BLOCK-секциях. C-X1 = тот же приём на ENTITY-секциях, 64-бит.
5. In-repo seqlock-прецедент: cmp399_shard — version сэмплируется до/после обхода ячейки, changed ⇒
   replay той же ячейки (items_index.rs). C-X1 = тот же протокол, но epoch = LCG-шаг, а не чётность ±1.
6. Стоимость путей: bump = **1** fetch_add (lock xadd); reader-hit = **1** Acquire-load + **1** cmp;
   полная инвалидация сегодня = CHM.remove+put (**2** map-операции) + перестройка среза (alloc + копия
   списка среза) + повторный lookup у читателя (**+1** map-операция на чтение).

## 2. Внешние источники (verified HTTP 200)

- **U1** https://www.cl.cam.ac.uk/techreports/UCAM-CL-TR-579.pdf — Keir Fraser, *Practical Lock-Freedom*
  (PhD, Cambridge, 2004): epoch-based reclamation — глобальный epoch-счётчик, писатели бампают эпоху,
  ридеры пинятся за 1 сравнение; reclaim/перестройка откладывается до смены эпохи. Origin-модель
  «писатель = bump счётчика, читатель = 1 cmp».
- **U2** https://en.wikipedia.org/wiki/Linear_congruential_generator — LCG X_{n+1} = (a·X_n + c) mod m;
  теорема Халла–Добелла: при a=1 полный период 2^64 ⟺ c нечётно. Берём c = 1442695040888963407
  (нечётный шаг Кнута) ⇒ **bump одним add** и **полный период 2^64**: k·c ≡ 0 (mod 2^64) при нечётном c
  возможно только при 2^64 | k ⇒ ни один бамп в пределах вселенной не даёт ABA-повтора значения.
- **U3** https://en.wikipedia.org/wiki/Seqlock — протокол стабильного чтения: writer крутит счётчик,
  reader сверяет до/после копии; изменённое значение = replay. C-X1 = seqlock на секцию,
  но вместо чётности — LCG-шаг (тот же контракт, дешёвый writer-side).
- **U4** https://docs.rs/crossbeam-epoch/latest/crossbeam_epoch/ — индустриальная реализация
  epoch-reclamation в Rust (pin/unpin/defer): подтверждает форму API (Guard+epoch), которую повторяет
  Snapshot-тип в scaffold.

## 3. Capture-матем и потолок

- Лейн: entity-slice broadphase запросы (getEntities/ClassInstanceMultiMap по 16³-секции) — общая
  полка с item-броадфазой (15.66% — верхняя граница семейства; моб-часть оценочно **4–8%** CPU в
  профилях тиков ×457–459).
- Захват: steady-state мутации среза = travel-crossings секций (редки), т.е. на мутацию приходится
  много чтений ⇒ фаст-гейт (1 cmp) закрывает 80–95% ходок по полному пути (CHM-walk + rebuild).
- Прогноз Δ: 4–8% × 0.25–0.5 (консервативный захват полки, доля моб-запросов) ≈ **+1.0..2.0пп**
  при полной проводке; как композиция к P32/P36-стилю (same family) — **+0.5..1.0пп** на тик-плацдорм.
- Потолок: lane 4–8% × capture 100% = 4–8пп < 20пп ⇒ C-X1 — **композиционная нога**, не соло-мердж
  (закон 13: нога уходит в CLIMB-план с моб-broadphase-носителем).

## 4. Preregistered гейты (закон 16)

- **G1 ARM**: `[crussty-plugin] cmp459_cx1: selfTest=true BEFORE arm` + define-маркер SlicesLcgEpochOps
  в kernel loader; STRICT eq `cmp459_cx1` (пустой флаг = ваниль, модуль не активирует путь).
- **G2 lockstep**: офлайн-харнесс — бит-в-байт список срезов снапшота vs прямой обход
  EntitySectionStorage при параллельных мутациях (add/remove/travel).
- **G3 NCDFE=0**: $Nested классы определяются вместе с Ops-классом (канон run 35902792520), T1=0.
- **G4 stale-window=0**: reader обязателен double-check эпохи до/после копии (seqlock); любой miss →
  ванильный полный путь; ARM-порядок: инвалидация жива ДО гейта (inside_snap прецедент).
- **G5 паритет + TPS-бар**: population-parity; нога не должна давать деградацию; band 6.0–9.5M,
  min-of-3, pair-stable по прецеденту тиков.
- **G6 fail-closed**: любой Throwable/структурный сбой → DISARM навсегда → ваниль.

## 5. Что уже сделано (scaffold, dormant)

- `src/slices_lcg_epoch.rs`: `SlicesLcgEpoch` (AtomicU64, bump = wrapping_add нечётного LCG-шага),
  `Snapshot<T>` (seqlock double-check), STRICT-гейт `cmp459_cx1`, self-тесты: уникальность эпох на 2^20
  бампов, рекуррентность LCG, инвалидация читателя после bump, seqlock-дифт ⇒ discard, невозможность
  ABA (нечётность c ⇒ нет k∈[1,2^64): k·c ≡ 0 mod 2^64).
- `entityinside/net/minecraft/world/entity/SlicesLcgEpochOps.java`: стаб (bumpEpoch = 1 add,
  epochValid = 1 cmp, STRICT-гейт CRUSSTY_LEVER_FLAG eq cmp459_cx1, selfTest), dormant — не
  определялся в kernel loader, трекед-блобы не тронуты.
- Wiring: `mod slices_lcg_epoch;` в src/lib.rs; БЕЗ register/activate (dormant).

## 6. Инцидент фиксации

16:38 /tmp-пурдж (диск 81→69%) стёр worktree до коммита — восстановлено из staging
(урок: артефакты в /home/z staging до push; /tmp = эфемерно в разгар волны-1).
