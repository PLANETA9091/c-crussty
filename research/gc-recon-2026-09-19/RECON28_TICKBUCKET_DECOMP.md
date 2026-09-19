# RECON-28 — декомпозиция RegionTickOps.tickBucket (оркестрация vs функции)

## s7194: tickBucket-поддерево = 74797 = 58.4% scene-CPU из 128104

- функциональные под-вызовы: 47454 = 37.0% сцены
- **оркестрация-собственно: 27343 = 21.3% сцены**

### Функциональный разрез

| домен | сэмплы | % сцены |
|---|---|---|
| zeroin-fluid | 9618 | 7.5% |
| body-other | 8816 | 6.9% |
| ai-pathfind | 7505 | 5.9% |
| broadphase-getEntities | 5680 | 4.4% |
| gc/jdk-leaf | 5118 | 4.0% |
| inside-gate | 4022 | 3.1% |
| collision/travel | 3436 | 2.7% |
| volatile-reads | 2186 | 1.7% |
| blockstate-access | 954 | 0.7% |
| sync/tracker | 119 | 0.1% |

### Оркестрационный контур (топ-10 лист-хвостов, тело entity-тика ОТСУТСТВУЕТ)

| хвост | сэмплы | % сцены |
|---|---|---|
| get | 2315 | 1.8% |
| <init> | 1032 | 0.8% |
| getItem | 925 | 0.7% |
| getCollisionsForBlocksOrWorldBorder | 781 | 0.6% |
| vtable stub | 696 | 0.5% |
| getValue | 684 | 0.5% |
| floor | 603 | 0.5% |
| lambda$tick$4 | 590 | 0.5% |
| add | 571 | 0.4% |
| setOldPos | 566 | 0.4% |

### Топ-8 лист-хвостов всего поддерева

| хвост | сэмплы | % сцены |
|---|---|---|
| get | 7937 | 6.2% |
| getEntities | 2900 | 2.3% |
| updateFluidHeightAndDoFluidPushing | 2681 | 2.1% |
| vtable stub | 2198 | 1.7% |
| getNode | 1929 | 1.5% |
| getValue | 1673 | 1.3% |
| <init> | 1651 | 1.3% |
| tick | 1440 | 1.1% |

## s7189: tickBucket-поддерево = 75896 = 59.7% scene-CPU из 127150

- функциональные под-вызовы: 47851 = 37.6% сцены
- **оркестрация-собственно: 28045 = 22.1% сцены**

### Функциональный разрез

| домен | сэмплы | % сцены |
|---|---|---|
| zeroin-fluid | 9637 | 7.6% |
| body-other | 8921 | 7.0% |
| ai-pathfind | 7456 | 5.9% |
| broadphase-getEntities | 5942 | 4.7% |
| gc/jdk-leaf | 5212 | 4.1% |
| inside-gate | 3887 | 3.1% |
| collision/travel | 3477 | 2.7% |
| volatile-reads | 2217 | 1.7% |
| blockstate-access | 970 | 0.8% |
| sync/tracker | 132 | 0.1% |

### Оркестрационный контур (топ-10 лист-хвостов, тело entity-тика ОТСУТСТВУЕТ)

| хвост | сэмплы | % сцены |
|---|---|---|
| get | 2535 | 2.0% |
| <init> | 1228 | 1.0% |
| getItem | 984 | 0.8% |
| getCollisionsForBlocksOrWorldBorder | 765 | 0.6% |
| vtable stub | 759 | 0.6% |
| add | 689 | 0.5% |
| getNode | 662 | 0.5% |
| getValue | 651 | 0.5% |
| floor | 573 | 0.5% |
| lambda$tick$4 | 549 | 0.4% |

### Топ-8 лист-хвостов всего поддерева

| хвост | сэмплы | % сцены |
|---|---|---|
| get | 8655 | 6.8% |
| getEntities | 3112 | 2.4% |
| updateFluidHeightAndDoFluidPushing | 2697 | 2.1% |
| vtable stub | 2362 | 1.9% |
| getNode | 2008 | 1.6% |
| <init> | 1897 | 1.5% |
| getValue | 1571 | 1.2% |
| tick | 1395 | 1.1% |

## Кросс-раннер

- поддерево: s7194 58.4% vs s7189 59.7%
- оркестрация: s7194 21.3% vs s7189 22.1%
- функции: s7194 37.0% vs s7189 37.6%

## ВЫВОДЫ (прегистер ЭПОХА-2)

1. Оркестрация-собственно = целевая доля flat batch dispatcher;
   функциональные под-вызовы остаются при любом диспетчере.
2. Если оркестрация >=5% сцены: рычаг ЭПОХА-2 = компаньонный flat-цикл
   (inline isRemoved/guard-гейты, прямой entity.tick() вместо
   lambda$ensureHelpers$4 -> accept -> guardEntityTick -> lambda$tick$4
   цепочки, батч-префильтр removed/passenger до цикла; семантика
   per-entity try-catch сохраняется бит-в-бит копией guard-логики;
   порядок обхода median-exact сохранён той же EntityTickList
   итерацией).
3. Если оркестрация <5%: точечный рычаг запрещён чартером ->
   системный вывод о fractal-равновесии сцены финализируется доком.

