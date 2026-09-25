# RESEARCH-457-A — chunkmono monster-hunt (TASK-457-A, agent-a)

Вектор: cmp456_chunkmono @d73758a3 (ServerChunkCache scheduling mono-plane
chunk6-sched на серт-стеке, STRICT-OR, закон-8 ось — игроку-видимая chunk-ось).
Цель-мандат (owner 2026-09-23): ≥+20% pair-stable, ≥3 валидных ноги ≥+18 norm,
min-of-3 пар ≥+20. Цикл закона 3 до ≥+20 или честного исчерпания (6 ног/тик).

## 1. Контекст и состояние на старт
- Носитель: round-456c-chunkmono @d73758a3 — NCDFE-фикс (define-gate
  cmp456_chunkmono, mirror-drift иглы) валидирован ×4 ре-роллами NCDFE=0
  (канон ×456: NCDFE>0 = DELIVERY-FAIL, пара аннулируется).
- Фиксированные ноги-носители ×456: chkmono456-4 −0.8@8650222,
  chkmono456-5 +7.9@7209045 (лейн сертный: items 0.00, nav 2.6-3.2,
  broad 9.8-10.2). Монстр-рейт нужен ≥3/ноги ≥+18.
- Main диспатчил ноги 6/7/8 @d73758a3, lever cmp456_chunkmono, канон-инпуты.
- Master 1838ae1d (код f44a831e). Anchor-пул: ×456 банк 14-15 валидных
  (a1 +4.4@8935474, a7 +7.4@7264766, a8 −0.7@6731200, a9 +5.2@6590823,
  a10 −2.0@6865200, a12 +8.1@6966170, a15 +8.0@6756575, a19 +7.5@6803823,
  a22 +4.4@6899724, a23 +6.0@6958213, a27 +10.1@6125089, a29 +2.9@7051107,
  a31 +3.7@6740240, a34 +5.9@6520174) + новые ×457 из ANCHORS.md по мере
  абсорба (проверять каждый цикл).

## 2. Диск-инцидент PHASE-0 (решён)
- Старт диска 83% → 97% (Errno 28 при первом worktree add).
- Root-cause: 3 мёртвых ×456-agent worktree (a2/b2/c2) несли по ~600M
  tracked research-checkout (jars в master-дереве), ×457 agent-b worktree
  +1.1G absorb-артефактов.
- Действие: некритичные финальные RESULT.json b2/c2 скопированы в
  ROUND-456/RESULT-*-final-copy.json; worktrees a2/b2/c2 removed (branches
  целы, всё пушнуто: round-456a-spawn 01688ad2, round-456b-poi 0fa13d72,
  round-456c-chunkmono d73758a3 — нулевая потеря состояния).
- Мой worktree: sparse-checkout (root+scripts+docs only) — не тянет 600M
  research-джаров. Диск после PHASE-0: 60%.

## 3. Цикл закона 3 — журнал
### Cycle-1 (старт ~11:59Z)
- Полл ×20 ранов: chunkmono-6 run 36131753219 in_flight; chunkmono-7 run
  36131765237 FAILURE @22s — step "Runner calibration band gate
  (pair-hunter fast-fail, S7-96d pairing law)" = BAND-FAST-FAIL ИНФРА,
  НЕ ВЕРДИКТ (канон ×456 a17/a18/diet-7/8), ре-ролл ≤2; chunkmono-8 run
  36131776794 in_flight. Якоря ×457 wave-1 (16 шт, main) in_flight.
- dispatch_457a.py написан (argv-guard --dry-run/--leg/--no-batch,
  HARD sha-pin d73758a3, ensure_branch через API git/refs POST);
  preflight --dry-run OK.
- Диспатч ре-роллов держим до абсорба 6/8 (одно окно диспатча за цикл).

### Cycle-1 абсорб (12:20-12:22Z)
- **leg 6** run 36131753219: GREEN-CANDIDATE **+11.5 norm @6423955** (TPS 2.40 vs
  exp 2.15). T1 PASS (NCDFE=0, band OK, pop VALID), threw=0. ARM-маркер
  («chunk-sched mono-plane live» + PATCHED getChunkNow Retargeted{2}), ЭФФЕКТ
  («first getChunkNow fast-path hit»), selfTest=true (schedProbe+shadow+native),
  AIOOBE=0 runtime (2× cmp420_chunk2 biomes selftest FAIL = fixture-шум канон ×454).
  Лейны: items 0.00, nav 3.00, broad 9.36, inside 16.28. drift fail-open latch
  после первых хитов = КАНОН (идентично на chkmono456-4/5). zips/collapsed пурдж.
- **leg 7** run 36131765237: BAND-FAST-FAIL инфра @22s — НЕ ВЕРДИКТ; ре-ролл = leg 9.
- **leg 8** run 36131776794: GREEN-CANDIDATE **+9.1 norm @7084301** (TPS 2.50 vs
  2.29). Все гейты PASS, selfTest=true, лейны items 0.00 / nav 2.96 / broad 9.81.
- **Пары**: leg 8 ↔ a29×456 (+2.9@7051107, Δ33,194) = **+6.2** (суб-бар). leg 6
  @6423955 — в ×456-банке НЕТ якоря Δ≤50k (ближайший a34@6520174 Δ96k) —
  UNPAIRED, ждём ×457-якоря (ANCHORS.md ещё нет, wave-1 16 ранов завершилась
  12:08-12:10Z — main абсорбирует).
- **Диспатч**: legs 9 (ре-ролл слота 7) + 10 @d73758a3, HTTP 204 ×2, 12:20Z.
- Монстров ≥+18 нет: семейный профиль +9..+12 повторяет ×456 (−0.8/+7.9) —
  дисперсия носителя, продолжаем цикл (осталось 2 диспатча до 6 ног).

## 4. Вердикт-гейты (все обязательны, канон ×456/×457)
1. NCDFE=0 (T1, НОВЫЙ) — иначе DELIVERY-FAIL.
2. ARM-маркер cmp456_chunkmono + ЭФФЕКТ-маркер.
3. AIOOBE=0; selfTest==true (FAIL ×1-2 = fixture-шум).
4. Лейн: items 0.00, nav ~3.2, broad ~9.8-10.9.
5. Band runner_cpu_index 6.0-9.5M — вне = BAND-DISCARD, ре-ролл ≤2.
6. threw=1 = REFUTED; block-entity ×10 = fixture-шум; dep-гейт norm ≥ −2.

## 5. Пары
leg_norm − anchor_norm; якорь ближайший по runner_cpu_index Δ≤50k;
pair-fresh («занят» = только в предыдущем МЕРЖЕ; кросс-тиковые валидны);
одна нога ↔ ВСЕ якоря Δ≤50k, min-of-3.

## 6. TASK-457-A2 (рестарт 13:52-14:5xZ) — цикл-2/3: ноги 11-16

### 6.1 Цикл-2 (ноги 11/12/13, диспатч 13:53Z, абсорб 14:05-14:19Z)
- **leg 11** run 36143848689: **−3.6 norm @7040413** — все гейты PASS (NCDFE=0,
  ARM=1, threw=0, band-OK, items 0.00, AIOOBE=0 runtime [2x cmp420_chunk2 biomes
  selftest FAIL = fixture-шум канон]), НО dep-гейт norm≥−2 ПРОВАЛЕН →
  DEPRESSION-сэмпл. GC Full=9, total 22.1s (банк 18.8s/Full=7).
- **leg 12** run 36143863797: **−3.7 norm @7053858** — гейты идентичны, депресс.
  Клатч 11↔12: Δrunner 13.4k, Δnorm 0.1пп = СИСТЕМАТИЧЕСКОЕ GC-окно 7.04-7.05M
  (не код — оба на том же дереве d73758a3).
- **leg 13** run 36143877485: **+2.8 norm @6831587** PARITY/LOW — гейты PASS,
  dep-гейт ОК. Пары: **+4.6 ↔ a16** (−1.8@6843586, Δ12.0k, суб-бар); −0.1 ↔ a18
  (+2.9@6817414, Δ14.2k).
- Пурдж после каждого абсорба (zips/collapsed/jar/spark) — диск 55-56%.

### 6.2 Честный Δ-прогноз семьи (закон 9, после 9 валид-ног)
Распределение 9 валид-ног ОДНОГО артефакта d73758a3: −3.7/−3.6/−0.9/−0.8/+1.3/
+2.8/+7.9/+9.1/+11.5 — медиана ≈ +1.3..+2.8, max +11.5, sd ≈ 5.2пп. Дисперсия
это НЕ код (дерево идентично) — это runner-окно/GC-шум (клатчи: 11↔12
Δ13.4k→0.1пп; Full=9 во всех 4 ногах цикла-2).
- Порог ЛЮБОЙ пары ≥+20 = нога ≥+18.0 (якорь ≥−2 по dep-гейту) СИСТЕМНО.
- +18.0 = +3.9σ от медианы, +1.25σ выше наблюдаемого max → p≈5e-5/нога;
  3 доп. ноги: p≈1.5e-4. Статистически мёртвая охота.
- Лейн-подпись lever'а ФИКСИРОВАНА во всех ногах (broad −5.9..−6.8, nav
  −10.7..−11.4, inside +4.3..+4.6): эффект артефакта насыщен, субаддитивен,
  потолок ноги ≈ +11.5 (хвостовая выборка).
- Волна-3 (ноги 14/15/16, run 36146758534/36146771631/36146784734, 14:20Z) —
  контрольный добор бюджета (закон 3 до исчерпания); ожидание по закону 9:
  медиана семьи, p(монстр ≥+18) < 0.02%.

### 6.3 Пары-банк семьи chunkmono (для серта/мержа)
- chkmono457-8 +9.1 ↔ a29×456 +2.9@7051107 (Δ33.2k) = **+6.2** (суб-бар)
- chkmono457-13 +2.8 ↔ a16-457 −1.8@6843586 (Δ12.0k) = **+4.6** (суб-бар)
- chkmono457-6 +11.5@6423955 — UNPAIRED (окно [6373955,6473955] пусто ×456 и ×457)
