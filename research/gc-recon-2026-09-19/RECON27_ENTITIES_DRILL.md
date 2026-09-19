# RECON-27 — дрилл entities/mobs-tick до под-лейнов (>=5%)

## s7194: семья = 48862 = 38.1% scene-CPU

| домен | сэмплы | % scene-CPU | % семьи | >=5% сцены |
|---|---|---|---|---|
| other:get | 4393 | 3.4% | 9.0% |  |
| other:getEntities | 3537 | 2.8% | 7.2% |  |
| other:updateFluidHeightAndDoFluidPushing | 2700 | 2.1% | 5.5% |  |
| ai/goals-selector | 2541 | 2.0% | 5.2% |  |
| nav/pathfinding | 2232 | 1.7% | 4.6% |  |
| baseTick/common | 1562 | 1.2% | 3.2% |  |
| other:vtable stub | 1534 | 1.2% | 3.1% |  |
| other:getValue | 1299 | 1.0% | 2.7% |  |
| other:intersects | 1207 | 0.9% | 2.5% |  |
| other:getValueVolatile | 841 | 0.7% | 1.7% |  |
| other:isEyeInFluid | 833 | 0.7% | 1.7% |  |
| other:getNode | 652 | 0.5% | 1.3% |  |

## s7189: семья = 49320 = 38.8% scene-CPU

| домен | сэмплы | % scene-CPU | % семьи | >=5% сцены |
|---|---|---|---|---|
| other:get | 4697 | 3.7% | 9.5% |  |
| other:getEntities | 3883 | 3.1% | 7.9% |  |
| other:updateFluidHeightAndDoFluidPushing | 2787 | 2.2% | 5.7% |  |
| ai/goals-selector | 2328 | 1.8% | 4.7% |  |
| nav/pathfinding | 2116 | 1.7% | 4.3% |  |
| other:vtable stub | 1642 | 1.3% | 3.3% |  |
| baseTick/common | 1605 | 1.3% | 3.3% |  |
| other:getValue | 1278 | 1.0% | 2.6% |  |
| other:intersects | 1119 | 0.9% | 2.3% |  |
| other:getValueVolatile | 874 | 0.7% | 1.8% |  |
| other:isEyeInFluid | 867 | 0.7% | 1.8% |  |
| other:getChunkNow | 643 | 0.5% | 1.3% |  |

## Кросс-раннер стабильность доменов

| домен | s7194 % сцены | s7189 % сцены | стабильно |
|---|---|---|---|
| other:get | 3.4% | 3.7% | ДА |
| other:getEntities | 2.8% | 3.1% | ДА |
| other:updateFluidHeightAndDoFluidPushing | 2.1% | 2.2% | ДА |
| ai/goals-selector | 2.0% | 1.8% | ДА |
| nav/pathfinding | 1.7% | 1.7% | ДА |
| baseTick/common | 1.2% | 1.3% | ДА |
| other:vtable stub | 1.2% | 1.3% | ДА |
| other:getValue | 1.0% | 1.0% | ДА |
| other:intersects | 0.9% | 0.9% | ДА |
| other:getValueVolatile | 0.7% | 0.7% | ДА |
| other:isEyeInFluid | 0.7% | 0.7% | ДА |
| other:getNode | 0.5% | 0.5% | ДА |

## ВЫВОДЫ

1. Под-лейны >=5% scene-CPU внутри нового ТОП-1 = кандидаты рычага
   (прегистер: javap-контракт -> локстеп-оракул -> прегистер ->
   диспатч, как в эпоху #12).
2. other:*-домены ниже 1% не трогаются (чартер: побочные <5%).
3. Если НЕТ домена >=5% — лейн entities/mobs-tick потенциально
   fractal-мал: нужен deeper RECON (по классам мобов) до >=5%.

