# AG-334 MEMORY (≤15 уроков)
1. «command-context» в cpu-collapse = TimerQueue/FunctionCallback dp-функция с @e-селекторами, НЕ fake-player команды.
2. Атрибуцию по имени класса в листе проверять полными стеками: leaf EntityLookup.get на 49.8% сидит под EntitySelector.findEntities.
3. BenchPopulationPlugin (topup-луп) на pop150k = 0.00% фреймов — C32.1/C82.1 канон; any «topup жжёт CPU» claim — проверять по профилю.
4. dp-пак stz3v2 (16fa1a32, 707 files) = «l» из 491-C59: 49.76% cpu на pop150k WBP ноге — скан-налог живёт на pop-ногах тоже.
5. Capture-путь = per-type entity index (R1-пул C41 47.76pp) pair-legal; редукция dp — EV<0 не трогать.
6. CAS-PUT доски с stump-guard (>700KB) и GET-fresh перед PUT — прошёл с 1 попытки.
7. Артефакты качаются одним curl по artifact id из /actions/runs/<id>/artifacts; collapsed-профили 16MB — парсить python-агрегатом.
