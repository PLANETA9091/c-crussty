# RECON-26 — свежий CPU-ТОП (s7194) + ВЕРДИКТ GC-семейства (ТОП-1 эпохи)

## CPU-ось s7194 (run-s7194-zeroalloc-v1): 128104 сэмплов

| семья | сэмплы | % оси | >=5% |
|---|---|---|---|
| GC/jvm(G1+barriers) | 42806 | 33.4% | ДА |
| entities/mobs-tick | 17856 | 13.9% | ДА |
| kernel:other | 17362 | 13.6% | ДА |
| jdk-collections/serde | 16739 | 13.1% | ДА |
| chunk-system | 8805 | 6.9% | ДА |
| travel/movement | 7388 | 5.8% | ДА |
| other | 5575 | 4.4% |  |
| inside-pipeline | 5081 | 4.0% |  |
| tracking/sync | 4194 | 3.3% |  |
| fluid-sim | 2224 | 1.7% |  |
| spark/jmx-monitor | 74 | 0.1% |  |

## CPU-ось s7189 (run-s7189-traveldiet-v2a): 127150 сэмплов

| семья | сэмплы | % оси | >=5% |
|---|---|---|---|
| GC/jvm(G1+barriers) | 41357 | 32.5% | ДА |
| entities/mobs-tick | 17423 | 13.7% | ДА |
| jdk-collections/serde | 17409 | 13.7% | ДА |
| kernel:other | 12066 | 9.5% | ДА |
| fluid-sim | 9480 | 7.5% | ДА |
| chunk-system | 8406 | 6.6% | ДА |
| travel/movement | 7199 | 5.7% | ДА |
| other | 5238 | 4.1% |  |
| inside-pipeline | 4355 | 3.4% |  |
| tracking/sync | 4144 | 3.3% |  |
| spark/jmx-monitor | 73 | 0.1% |  |

## Кросс-раннер: s7194 vs s7189

| семья | s7194 % | s7189 % | стабильно |
|---|---|---|---|
| GC/jvm(G1+barriers) | 33.4% | 32.5% | ДА |
| entities/mobs-tick | 13.9% | 13.7% | ДА |
| kernel:other | 13.6% | 9.5% | НЕТ |
| jdk-collections/serde | 13.1% | 13.7% | ДА |
| chunk-system | 6.9% | 6.6% | ДА |
| travel/movement | 5.8% | 5.7% | ДА |
| other | 4.4% | 4.1% | ДА |
| inside-pipeline | 4.0% | 3.4% | ДА |
| tracking/sync | 3.3% | 3.3% | ДА |
| fluid-sim | 1.7% | 7.5% | НЕТ |
| spark/jmx-monitor | 0.1% | 0.1% | ДА |

## ВЕРДИКТ: GC-семейство — НЕ TPS-рычаг, потолок TPS-конверсии <10%

GC-бюджет s7194: 280 young / 0 Full = 17594 ms = 5.9% 300s-окна (RECON-24: 6.5%).

### Основание закрытия (четвёртый прецедент чартера после RECON-17/20/23)

1. **Пять архитектурных рычагов GC-давления REFUTED по TPS при
   ПОДТВЕРЖДЁННОМ структурном эффекте на лейны**: #9 flat_traversal
   (бит-в-бит TraverseOps, TPS −5.6% шум → FAIL PG4a), #11 zero_cursor
   (alloc −21.3%, TPS −22.2%), #12 inside_diet (alloc −47.2% / CPU
   −66..−67% ОБА лега min-of-2, TPS −17..−6%), #14 travel_diet (3
   валидные ноги: −21.6/−18.8, −1.5/+0.0, −3.9/+0.0), #10 zero_alloc
   (−11.9/−12.5% НАОБОРОТ при zeroin COMPOSED sites:3 + threw=0).
2. **Механизм**: молодое GC-давление конвертируется в young-паузы
   (STW) на уровне 6.5% окна; параллельные G1/barrier-CPU ядра
   вне критического пути MSPT (region-воркеры не блокируются).
   Диеты реально снижают evac/barrier-работу (young 140-144 vs 154)
   — TPS неподвижен: стену держит НЕ GC.
3. **RECON-25**: в крупнейшем аллок-под-лейне jdk-collections/serde
   нет домена >=5% вне already-REFUTED #9 (inside-scan-visitset
   7.2/8.3% = visit-set vanilla-тела forEachBlockIntersectedBetween,
   убиваемый TraverseOps); остальные домены <3%. Дрилл исчерпан.

### Следствие (прегистрированный выбор нового ТОП-1)

GC-лейн закрыт как TPS-цель (потолок <10% + 5 REFUTED). Новый ТОП-1
= старшая стабильная CPU-семья s7194 из таблицы выше (candidates:
entities/mobs-tick, kernel:other, travel/movement) с RECON-дриллом
до под-лейнов >=5% до реализации рычага. spark/jmx — не игровой,
вычитается.

