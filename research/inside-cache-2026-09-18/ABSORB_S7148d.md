# ABSORB S7-148d — FLUID-FREE leg (ран 35326295881): §S7-139 FAIL ⇒ REFUTED-BY-ECONOMICS
## fluid-лейн не снят (доля get ВЫРОСЛА 53.8→60.1%), TPS −20% (демукс-оверхед); экономический потолок лейна = 1.8% total CPU = микро-класс; демукс на живой сцене анти-оптимизация; конвейер продолжается без demux/fluid_free

Дата: 2026-09-18 17:1x +08. Job 394666. Агент agent-7625532f.

## 1. Валидность A/B

Ран 35326295881 (head d2f063e, fluid_free=1 + paletted_demux=1 + inside_cache=1,
alloc_diet=0, flush_diet=0, fp4/300s/150k/seed42/xmx10G/guard1), SUCCESS
08:48:58→09:06:37 UTC (~18 мин). База = base-b (35317176927, all-zero, fp4/300s).

- Конфиг точен; INJECT 150000/150000 VALID; популяция **148445/148395/148267**
  стабильна (топап живой) ✓
- **0 NoClassDefFoundError / 0 исключений** (stdout 248KB) ✓
- Цепочки ARMED живьём: paletted: PALETTED-DEMUX ARMED (patched PalettedContainer
  at first load) + PalettedContainerOps defined in launch loader; inside_cache
  pristine sighting Entity 205458B; fluid_free: **defined FluidOps in kernel
  loader → computed section splice 15041→15088 (байт-в-бит с 91fcd70-записью)
  → entity chain composed (Retargeted { sites: 2 }) → hook serve LevelChunkSection
  15088** ✓ — весь мост end-to-end живьём, первый раз на живой сцене
- patched-kernel в артефакте = e2992d63 (байт-в-бит) ✓

## 2. Гейты §S7-139: FAIL (главный) + TPS-регресс

| Метрика | base-b | FLUID-FREE leg | Гейт | Вердикт |
|---|---|---|---|---|
| **fluid-доля get** | 53.8% (916) | **60.1% (934)** | ↓≥60% | ❌ **FAIL — доля ВЫРОСЛА, абсолют fluid-get не снят (+2%)** |
| readPalette | 428 | **0** | (демукс-эффект) | −100% — демукс снял свой лейн |
| PalettedContainer.get (CPU) | 3.38% (1704) | 2.86% (1555) | — | −8.7% отн. |
| inside-blocks (alloc) | 40.44% | 40.79% | (inside_cache=1!) | −32.5% из leg2''' НЕ воспроизвёлся — демукс-профиль сместил доли |
| TPS | 0.8–0.9 | **0.6–0.7** | парити-нейтрально | ❌ **РЕГРЕСС −20%** (в leg2''' без демукса было 0.7–0.8 ⇒ демукс добавил −10–15%) |
| young GC | 125 | 135 | ↓ | топап+овер-хед (калибровка) |
| entity-фаза | 59.7% | 55.9% | ≤+0.5pp | ✅ формально |
| fixture / 0 NCDFE | — | зелёные / 0 | ✓ | ✅ |

ff-кэш не снял fluid-get: вероятные причины (диагностика — следующая офлайн
задача, НЕ блокер): (a) секции живой сцены с мобами в основном has-fluids
(ff=2) — MineShield-3 мир с водой в пещерах/озёрах; (b) демукс-мутации
(±2/запись) инвалидируют кэш чаще, чем хиты повторяются; (c) сцена движется —
секция сущности меняется каждый тик (движение), кэш по секции промахивается.
Офлайн-харнесс показывал FREE-HIT/EVENT-DRIVEN на маленькой сцене — живая
сцена X150K другая.

## 3. ЭКОНОМИЧЕСКИЙ ВЕРДИКТ (главный вывод)

**Даже идеальное исполнение гейта не имеет TPS-ценности**: fluid-get =
916/50454 = **1.8% total CPU** живой сцены. Снятие всего лейна = −1.8% CPU —
МИКРО-КЛАСС (запрещён владельцем). §S7-139-гейт «fluid-доля get ↓≥60%» был
сформулирован в до-фиксную эпоху из alloc/CPU-цензора 35275967738 и не учитывал,
что сам get — 3.38% CPU, а его fluid-часть — половина.

**FLUID-FREE-SECTION (и PALETTED-DEMUX как субстрат) = REFUTED-BY-ECONOMICS
на живой сцене X150K в текущей форме**: гейт не достигнут + TPS-регресс −20%
(демукс-оверхед на мутациях живой сцены съедает выгоды) + экономический
потолок лейна микро-класса. Решение конвейера:
- paletted_demux default 0 на живой сцене (анти-оптимизация при 150k живых);
- fluid_free остаЁтся забанкованным кодом (off; OFFLINE PASS в силе — код
  корректен, экономика не та);
- §S7-139-гейт помечен калиброванным как недостижимый/бессмысленный на живой
  сцене (экономика лейна 1.8% CPU).

## 4. Конвейер после FLUID-FREE

Зелёные рычаги: INSIDE-CACHE (GREEN-BY-SAFETY, leg2'''), FLUSH-DIET (GREEN,
S7-148c). REFUTED: FLUID-FREE+DEMUX (этот абсорб). В полёте: ALLOC-DIET leg
(35328228929, head 59b6bbb, alloc_diet=1, 09:11:24 UTC).

NEXT: absorb ALLOC-DIET → **накопительный конфигурационный ран** (inside_cache=1
+ flush_diet=1, demux=0, fluid_free=0, alloc_diet по вердикту) → absorb →
итоговый вердикт эры по комбинации.

## 5. Артефакты

- run-s7148-fluidfree/: BOTTLENECKS_3.md, alloc-collapsed.txt (f), gc.log,
  run-env.txt, sha256_fluidfree_extras.txt (stdout 248KB; patched-kernel
  e2992d63) — в git.
- Fluid-доля расчёт: cpu-collapsed всех четырёх ранов (base-b/leg2'''/fluidfree/
  flushdiet) — скрипт в ABSORB (inline, воспроизводим).
- Диспатч: dispatch_s7149.py (ALLOC-DIET leg).
