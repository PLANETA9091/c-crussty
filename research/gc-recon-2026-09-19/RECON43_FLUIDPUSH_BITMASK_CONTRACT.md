# RECON-43: ARCH-LEVER #16 FLUIDPUSH-BITMASK — контракт архитектурной замены дата-плейна fluid-скана

Тик TASK-388 (2026-09-21). Ответ на директиву владельца: «тики бесполезны — нужны
замены РЕАЛЬНЫХ архитектур с лютым бустом». Точечные рычаги исчерпаны (RECON-42);
осталась ровно одна крупная не-атакованная архитектура: **дата-плейн per-entity
fluid-скана** (лейн 14.6% java = оркестрация 8.1% + чтения 5.1% + FLOW 0.8%).

## Почему прошлые атаки лейна проиграли (уроки в дизайн)
- fluid_guard (TASK-80, в банке): whole-body hook, но отрицательный fast-path
  cell-снапшот позиционно-нестабилен → hit-rate 2.0% живой сцены.
- fluid_dirty (S7-153, REFUTED): per-(entity,cells) memo — hit ≈0% (мобильный
  скан), GC +18.6%. Урок: кэш должен быть НЕ позиционным, а СЕКЦИОННЫМ.
- inside_bitmask #15 (LANE-OPEN): all-air секционный гейт не конвертит, потому
  что секции мира НЕ пустые (блоки везде) → hasOnlyAir≈false всегда. Урок:
  предикат должен быть «секция без ТЕГА-жидкости» — в мире это истинно для
  большинства секций (вода только в океанных/речных объёмах).

## Механизм (замена архитектуры чтения)
1. **Секционные fluid-битмапы**: на секцию — long[128]: 4096 слотов × 2 бита
   (WATER/LAVA по fs.is(tag), построение = 1 проход sec.states 4096 слотов).
2. **Invalidation**: generation-счётчик PalettedContainer — инфраструктура
   demux-патча (PalettedContainerOps, S7-131, скомпилирован, dormant) используется
   ТОЛЬКО как write-bump ledger (легковесная половина demux; read-path demux,
   REFUTED, НЕ используется). Альтернатива-B (fallback если compose тяжёл):
   secWrite-хук fluid_dirty как единственный инвалидирующий источник.
3. **Гейт (median-exact класс, прецедент #15/RECON-33)**: перед сканом тега T:
   бит-тест объёма [floor..ceil-1]×deflate(0.001) по битмапам 1-2 секций (2-4
   AND-а) → CLEAN ⇒ бит-точный negative-tail обоих тегов
   (fluidHeight.put(tag,0.0)×2 + return false — хвост верифицирован по
   FluidPushGuardHook.slow @318: put стоит БЕЗУСЛОВНО, pure-negative = 0.0).
   Не-CLEAN ⇒ существующий banked slow-путь (бит-точный) без изменений.
4. Точка входа — FluidPushGuardHook.updateFluidHeightAndDoFluidPushing (уже
   whole-body hook в банке v4): консультация гейта ДО cache/slow. Ретаргетов
   Entity НЕ требуется (compose-цепь не трогается).

## Оценка
- Стоимость гейта ≈ 40-80ns/entity/тик против ~470ns скана (15319 сэмплов /
  25000 movers / 750 тиков) → потолок снятия ~12-14% java-сцены при hit ≥90%
  (структурный, не позиционный — позиционная нестабильность уроку S7-153 не
  подчинена: битмап пер-СЕКЦИЯ, не пер-сущность).
- Rebuild-бюджет: 160µs/секция; допустимо ≤40 rebuild/тик (FLOW 0.8% сцены).

## Preregister-гейты (leg s7206, банк v4 + fluid_bitmask=1)
- G1: fluid-family (n-push lane) ↓≥60% в cpu-collapsed при ARMED-маркере.
- G2: 0 NCDFE, ARMED-маркер «fluid_bitmask: gate armed», absent dormant-маркера.
- G3: популяция 150k VALID; G4: DUAL BAR vs банк v4 (2-точки: 2.6@8551924,
  2.2@6653417, normalized min ≥ +10% И absolute-интерполяция ≥ +10%).
- G5: offline lockstep — storm-мутации секций: CLEAN-вердикты бит-в-бит = ваниль
  negative-tail; не-CLEAN = slow-путь (существующий harness-паттерн S7-152).
- G6: young GC не выше базы +5% (rebuild-буферы — thread-confined, 0 alloc на
  CLEAN-пути).
- REFUTED-критерий: G1 недостижим при живом ARMED и rebuild >40/тик ⇒
  fluid_bitmask=0 (анти-урок fluid_dirty), лейн закрывается ВЕРДИКТОМ ЭКОНОМИКИ.

## Статус имплементации
STEP-0/контракт — этот тик. NEXT (TASK-389): (a) fluid_bitmask.rs wiring + 
FluidBitmaskOps.java (javac21 против kernel); (b) generation-ledger выбор A/B по
сложности compose; (c) lockstep-harness ALL PASS; (d) dispatch s7206.
