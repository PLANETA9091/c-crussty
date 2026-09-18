# ABSORB S7-148 — leg #2'' INSIDE-CACHE (ран 35318755582): A/B-НЕВАЛИДЕН ×2
## (парити-контаминация моста + дрейф популяции), ДЕФЕКТ НАЙДЕН И УСТРАНЁН, leg #2''' диспатчен

Дата: 2026-09-18 15:4x-16:0x +08. Job 394666. Агент agent-7625532f.

## 1. Конфигурация рана

Ран 35318755582 (head 135cb89, inside_cache=1, остальные рычаги 0, fp4/300s,
150k/seed42/xmx10G/radius640, guard1), SUCCESS 07:17:58→07:32:36 UTC (~15 мин).
Конфиг ТОЧЕН по preregistered спеке leg #2' (в отличие от leg #2' — девиаций
fp/seconds нет; run-env.txt приложен). База сравнения = base-b (35317176927,
head 4c8f029, все рычаги 0, fp4/300s).

## 2. Фикстура: INJECT VALID, но популяция ДРЕЙФАНУЛА

- INJECT 150000/150000 VALID; alive-check level.players()=4 injected=4.
- entity totals leg2'': **79209 / 77474 / 75826** (последние 3 alive-чек-лапы)
  vs base-b **148391 / 148193 / 148027** — сцена лег-рана полупустая.
- TOPUP-SCAN (600т-период) выстрелил 1 раз (tick=225986659, 07:30:47):
  aliveReal(items=62468, hostiles=11809, passives=8747) = 83024;
  deficit(items=42532, hostiles=18191, passives=6253) = 66976;
  topupSpawnedTotal=0 на момент скана.
- Экономика дрейфа: leg2'' отработал ≈700–900 тиков (TPS-интеграл 1.5→3.0)
  против 240 тиков базы ⇒ валовый ванильный распад (item-merge герды
  items 100357→62468 за 600 тиков ≈ 63/тик, горение/cramming хостайлов)
  успел дренировать пол-сцены. Дренаж S7-147 фиксированными 20/тик <
  ~42/тик валового распада ⇒ дефицит рос быстрее добора.

## 3. ГЛАВНАЯ НАХОДКА: 74 941 NoClassDefFoundError — парити-контаминация моста

server-stdout leg2'' (177MB, sha256 902e1182…) содержит **74 941 стек-трейс**:

```
[07:24:53 ERROR]: Entity threw exception at world:…
java.lang.NoClassDefFoundError: net/minecraft/world/entity/EntityQueryOps
	at net.minecraft.world.entity.InsideBlockOps.gate(InsideBlockOps.java:195)
Caused by: java.lang.ClassNotFoundException: net.minecraft.world.entity.EntityQueryOps
```

- Окно: 07:24:53 → 07:31:29 (весь soak, ~189 исключений/с).
- Причина: hardened HIT-верификация моста (S7-136) звала
  `EntityQueryOps.mutablePos()` — класс ALLOC-DIET-субстрата (S7-133),
  который НЕ определяется в kernel loader при `alloc_diet=0`
  (compile-dep моста на необязательный плагин — дефект сборки-контракта).
- Эффект: Paper per-entity catch прерывал тик сущности («Entity threw
  exception») — ~0.06% entity-tick-вызовов (74 941 из ≈135M) исполнялись
  НЕ-ВАНИЛЬНО (прерванный тик). Median-exact паритет НАРУШЕН в этих вызовах;
  профиль и динамика популяции контаминированы.
- Почему офлайн не поймали: харнесс живёт в одной loader-пространстве с
  EntityQueryOps в classpath (repro CP = entityinside/build:entityquery/build:…)
  ⇒ резолв всегда успешен. На CI-ране мост define'ит только
  InsideBlockOps+$Recorder (inside_cache.rs), EntityQueryOps не в цепочке.
  Урок leg #2 («непроверенный на сцене = риск коллапса») сбылся в новой форме:
  обязательная dependency моста живёт вне его define-цепочки.
- Ретроспектива leg #2' (35314220731): server-stdout 175MB — та же сигнатура
  (74941-класс стеков), «0 crash» не ловит per-entity exception-прерывания.
  ARMED-эвиденс leg2' валиден, но его soak тоже был контаминирован.

## 4. Гейты §156: НЕ выполняются ни в одной проекции

Доли (SHARES, устойчивы к числу тиков):
| метрика | base-b | leg2'' | гейт | факт |
|---|---|---|---|---|
| inside-blocks (alloc) | 40.44% | 37.76% | ↓≥30% | −6.6% отн. — FAIL |
| movement-geom (alloc) | 22.24% | 26.03% | ↓≥25% | ↑ — FAIL |
| PalettedContainer.get (CPU) | 3.38% | 3.02% | ↓≥15% | −10.6% — FAIL (близко) |
| young GC count | 125 | 165 | ↓ | ↑ — FAIL (окна несопоставимы) |

На-тик нормировка (base 240 тиков vs leg ≈700–900) даёт ↓65–75% по всем
семьям и CPU-на-тик ↓72% — но интерпретация осложнена (a) NCDFE-прерываниями
(часть «ускорения» может быть следствием пропущенных тиков сущностей),
(b) дрейфом популяции 148k→79k (полупустая сцена легче). ВЕРДИКТ ПО
PREREGISTERED ПРОТОКОЛУ НЕВОЗМОЖЕН — это честный вывод абсорба.

TPS: база 0.8–0.9 → leg 1.5→3.0 (5m/15m 2.0–3.0). 0 tick-behind, сервер жив
весь soak. ARMED-цепочка ПОЛНА живьём: pristine Entity 205458B (major 65) →
defined InsideBlockOps + $Recorder → computed patch Retargeted{sites: 1}
205522B → serve → armed retransform rc=0.

## 5. Фиксы S7-148 (оба офлайн-верифицированы)

1. **InsideBlockOps SELF-CONTAINED** (устранение NCDFE): ThreadLocal
   mutable-pos ring (8 слотов, zeroed, семантика 1:1 с EntityQueryOps)
   перенесён внутрь моста; единственная внешняя ссылка заменена, compile-dep
   на entityquery/build УДАЛЁН из scripts/build_inside_block_ops.sh.
   Верификация: javac21 --release 21 против patched-kernel 29386794B
   (sha256 e2992d63); 0 ссылок EntityQueryOps в классе (javap); rust 122
   = 121 ok + 1 ignored; REPRO-харнессы INSIDE-CACHE/FLUSH-DIET/ALLOC-DIET —
   все OFFLINE PASS. Новые байты: InsideBlockOps.class 7826B, Recorder 4307B.
2. **BenchPopulationPlugin drain-budget дефицит-драйвен** (устранение дрейфа):
   TOPUP-SCAN период 600→120 тиков; бюджет = clamp(deficit/50, 20, 100)/тик
   (валовый распад leg2'' ~42/тик покрывается с запасом; worst-case
   ~70ms/тик при TPS 3+). В базе (240 тиков) дренаж дремлет bit-for-bit
   (дефицит на первых сканах < бюджета, добор за ~18 тиков) ⇒ base-b
   остаётся структурным близнецом ценза. Compile-OK javac21 CI-эквивалент
   (kernel + 125 libraries; scripts/compile_population_s7148.sh).

## 6. NEXT: leg #2''' (диспатчен в этом тике)

- Конфиг: inside_cache=1, fp4/300s, 150k/seed42/xmx10G, guard1 — head = фикс.
- Гейты абсорба leg2''' (§156, база base-b): fixture зелёные
  (INJECT VALID + популяция ≈148k на цензе ВЕСЬ soak — TOPUP-SCAN живой,
  дренаж держит), ARMED-цепочка полна, 0 NoClassDefFoundError в stdout
  (новый обязательный чек!), внутри-blocks ↓≥30%, movement-геометрия ↓≥25%,
  PalettedContainer.get ↓≥15%, young GC ↓, entity-фаза ≤+0.5pp.
- Если популяция снова дрейфует — калибровка фикстуры продолжается
  (base-c + усиленный дренаж), §156-вердикт откладывается.

## 7. Артефакты

- run-s7147-leg2pp/ (ран 35318755582): BOTTLENECKS_3.md, alloc-collapsed.txt,
  gc.log, run-env.txt — в git; server-stdout.log 177MB / cpu-collapsed 16.8MB /
  wall-collapsed 1.27MB / patched-kernel.jar (e2992d63 = эталон) /
  flamegraph / entity-recon — sha256_leg2pp_extras.txt (download-only).
- sha256 манифест: artifact_hashes_s7148.txt (дополнение).
