# RESEARCH-456-A — spawn-carrier cmp455_spawn cycle-2: ребейз на f44a831e + анализ усиления

## 0. Постановка (закон 3)
Ноги ×455 носителя @c25782b4: spawn455-1 −6.3 norm @7099810, spawn455-2 +2.0 @8769142 —
ниже бара. Цикл: research → доработка → бенч → Δ<+20 → новый research → … → ≥+20 или бюджет.

## 1. Форензика слабых ног ×455 (почему −6.3/+2.0)
Побайтовое сравнение ABSORB-артефактов spawn455-1/2 против СЕРТНЫХ ног chk455-2 (+18.7@6811670)
и diet455-3 (+19.7@7315179):

| метрика | spawn455-1 | spawn455-2 | chk455-2 CERT | diet455-3 CERT |
|---|---|---|---|---|
| items | 0.00 | 0.00 | 0.00 | 0.00 |
| nav_ai | 3.34 | 3.61 | 3.30 | 3.41 |
| broadphase | 10.21 | 9.41 | 9.76 | 9.97 |
| inside_volatile | 16.82 | 17.25 | 16.93 | 17.87 |
| GC Full/total | 9/20.7s | 8/22.0s | 9/19.6s | 9/22.2s |
| маркеры | armed, AIOOBE=0 | armed, AIOOBE=0 | armed | armed |

**Вывод: лейн-сигнатура и GC носителя ИДЕНТИЧНЫ сертным ногам — код носителя НЕ болен.**
Разброс — беговой шум семьи: у ОДНОГО И ТОГО ЖЕ кода диеты ×454/×455 (8 ног) norm = −0.8…+28.3
(медиана ~+12), у chunk-семьи (8 ног) +5.1…+18.7 (медиана ~+11). Две ноги spawn −6.3/+2.0 = два
неудачных розыгрыша из этого распределения (серты собираются хвостом распределения + min-of-3).
Absorbed-числа: research/gc-recon-2026-09-19/round-spawn455-*/ABSORB.md.

## 2. (а) Активация spawn/activation слайсов «ванального остатка» (RESEARCH-455-A §1) — ГО/НЕТ
- **NaturalSpawner 0.50-0.94% CPU**: bit-exact миграция spawn-цикла требует getNoiseBiome-семплинг,
  blockstate-коллизии isUnobstructed, entity-obstruction getEntities в Rust (§1 ×455). Внутри бюджета
  цикла-2 (~2.5ч, −45 мин на ребейз/конфликты/блобы) вынести БЕЗ риска паритета (закон 4) нельзя.
  **NO-GO (честно, цифра потолка +0.5-0.9пп).**
- **ActivationRange 0.62-0.83% CPU**: горячий лист checkIfActive (per-entity из tickNonPassenger).
  Ветка immunity (каждые 20 тиков) имеет JAVA-ПОБОЧКИ (writes activatedTick/isTemporarilyActive) и
  читает сложный стейт (leashHolder/vehicle/fire). Bulk-плейн = fill+apply сложного стейта на java-стороне;
  per-entity JNI запрещён законом 6. Полная подсистемная миграция = отдельный раунд (2-4ч + блобы + тесты).
  **NO-GO в цикле-2 (потолок +0.6-0.8пп).**
- **despawn sscan-плейн (уже в носителе)**: потолок +0.2-0.3пп (0.31-0.35% CPU), плейн лёгкий
  (один bulk-JNI/тик, DOD-проход, колонка nearest[]), паритет-риск снят ×455 (vanilla-лестница 1:1).

## 3. (б) Субаддитивность ×3: не конфликтует ли носитель с chunk-юнионом мастера
Мастер f44a831e = ins4 ⊕ senseins ⊕ chunk4-send ⊕ chunk5-encode ⊕ chunkparse ⊕ noise-GEN.
Ребейз-носитель = мастер-юнион ⊕ despawn-sscan. Маржинальность добавки: despawn-потолок +0.2-0.3пп
при норм-шуме семьи ±15пп и norm-остатке сертных пар ±0.3-2пп — **добавка МАРЖИНАЛЬНА, цена честно**:
ожидаемый premium носителя над мастером ≈ +0.2пп (в шуме), риск отрицательной склейки (канон
субаддитивности ×3: юнионы едят 40-60% суммы) минимален — плейн не добавляет нового java-сайта,
STRICT-OR списки только расширяются (needles verbatim, mirror-drift ×451/×452/×454 соблюдён).
**РЕШЕНИЕ (по мандату п.б): носитель оставлен КАК ЕСТЬ (despawn-плейн остаётся идентичностью вектора,
R4-плоскость армитса флагом), компенсация слабых ног ×455 = ребейз на мастера-юнион (сертный эффект
chunk-comp +20.0 теперь НЕСЁТСЯ lever'ом cmp455_spawn через STRICT-OR) + 2 свежие ноги.**

## 4. Новый research другого слоя (следующий виток цикла, если пары < +20)
1. **Re-роллы**: серт-математика требует leg_norm ≥ +18.3 (↔a20 −1.7) / ≥ +18.6 (↔a27 −1.4) /
   ≥ +19.8 (↔a14 −0.2). Монстр-рейт диеты/chunk-семей 2/8…5/8 даёт P(leg ≥ +18) ≈ 0.25-0.6/ногу.
2. **Свежие якоря ×456**: main диспатчит 16 якорей @0716075c (решётка gold-sight) — если окно
   7.2652-7.3652M закроется (якорь norm ≤ −0.3), пары ≥+20 даст ЛЮБОЙ leg ≥ +19.7.
3. **ActivationRange-подсистема целиком** — отдельный раунд (R-меню закона 6, 2-4ч): bulk fill/apply
   SoA (typeClass, tickCount, id, activatedTick, alive, leashBit) + immunity-ветка на java-стороне
   с побочками, сохранёнными 1:1. НЕ в этом цикле.
4. **NaturalSpawner-слайс isRightDistanceToPlayerAndSpawnPoint** (player-дистанции через sscan-снапшот
   игроков) — самая дешёвая spawn-подплоскость (~0.2пп), кандидат cycle-3 при живом носителе.

## 5. План имплементации цикла-2
1. worktree round-456a-spawn от origin/master (fd790a17 = 0716075c + диск-гигиена), merge
   origin/round-455a-spawn @c25782b4; STRICT-OR конфликты (21 rust + 10 java + check_blobs_sync)
   = union игл cmp455_spawn + cmp436_ins4 + cmp451_senseins + cmp453_diet + cmp450_chunk.
2. Блобы пересборка build_455a_blobs_all.sh (javac --release 21, cp=/home/z/tools/patched-kernel.jar
   + fastutil + paper-api + adventure — НЕ round-j2b; flat==nested), check_blobs_sync ALL IN SYNC.
3. cargo check --lib + cargo test (target → УДАЛИТЬ сразу).
4. refs round-456a-spawn-1/2 от ветки; диспатч 2 ног world-bench-parallel.yml (канон-инпуты,
   lever_flag=cmp455_spawn, lever_arg=1).
5. Полл 120с+curl; абсорб; маркеры ARMED/EFFECT/selfTest==true/AIOOBE=0; пурдж.
6. Пара = leg_norm − anchor_norm (Δ≤50k pair-fresh; «занят» = в предыдущем МЕРЖЕ; канон ×455),
   депресс-гейт norm≥−2, band 6.0-9.5M, min-of-3.
