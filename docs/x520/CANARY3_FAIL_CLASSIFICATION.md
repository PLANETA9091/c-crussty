# AG-82 ×520 — CANARY-3 ВЕРДИКТ: FAIL (2/2) — НОВЫЙ БЛОКЕР ЦЕПИ №8 «FIXTURE-DEAD @ master»
(харвест 02:20Z, run-36802056368 job 110178341021; пара 36802054506 тоже completed failure)

## Хронология ноги (master 76dfc7bf, старт 01:38:22Z, финиш 02:05:51Z, 27.5 мин)
- G-PURPUR PASS; boot G-DATAPACKS enabled-markers=4/4 (bug#7 initial-enabled-packs ФИКС РАБОТАЕТ)
- Мир-фаза 01:40→02:05 (~25 мин, heavy stand 3 dims × 20449 chunks), NCDFE=0, AIOOBE=0, G-HB PASS (794 строк)
- РЕПОРТ-ГЕЙТЫ (все FAIL-класс): G3 datapacks-enabled markers **1/4** (при boot было 4/4!),
  G4 marked≥95% FAIL, G5 DRAIN-TIMEOUT, G-DIM ov=0 ne=0 en=0 total=0 (silent-empty),
  G6-FPV2 **LEG-B-DEAD total=54.0 < 500** («fixture broken, H1 REFUTED») → exit 1.

## Классификация: FIXTURE-DEAD (не харнесс-баг!)
1. ≠ класс ×519 fix-веток (433/396): там G-DATAPACKS падал на BOOT (1/4), здесь boot 4/4 PASS,
   полный fix-stack 7/7 подтверждён зелёными G-PURPUR/G-DATAPACKS/NCDFE/AIOOBE/G-HB.
2. = класс «мир умер под давлением»: 0 entity во всех dims + total=54 FP (<500) + drain timeout.
   total=54 ≠ -1 (у ×519) → DimForceload встал (bug#5 fix жив), но populate не дошёл.
3. Гипотезы-кандидаты (проверять по порядку): (a) runner-дрейф пула 10.2–12.5M — heavy stand
   не успевает mark/drain в бюджете → ubuntu-24.04 pin (AG-16, 6.95M in-band) = ГЛАВНЫЙ remedy
   для canary-4; (b) G3 boot 4/4 → report 1/4 — деградация datapack-состояния в живом мире
   (маркеры исчезают? перечитать grep-окно в report_benchv2.py); (c) silent-empty census =
   инжекция population_target=0 в bench-v2 дефолте? (сверить inputs у пары 54506).
4. Редиспатч canary как есть = гарантированный повтор (27.5 мин до того же гейта). canary-4 =
   решение КООРДИНАТОРА: bench-v2 runs-on pin ubuntu-24.04 + сверка census-inputs.

## Мой вклад волны (параллельный лейн, не канарный)
- P43 brainflat wiring на swarm-520-82 (d7f9e9d5, push OK): delegate v2 (map-keyed 3-arg onPut,
  selfTest GREEN ×3), patch_brain_flatmem_put (call-site retarget Map.put@51→onPut+2nop,
  SMT/offsets byte-identical), brainflat.rs (STRICT cmp464_flatmem), lib.rs wiring.
- ARM-delivery нога (world-bench-parallel, seed 520082) = DISP-INTENT: POST после FIXTURE-DEAD
  фикса (нужен живой мир, иначе повтор LEG-B-DEAD-профиля как у всех).
