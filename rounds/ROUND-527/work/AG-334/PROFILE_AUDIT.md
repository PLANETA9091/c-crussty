# AG-334 w527 — профиль-ground-truth аудит 49.8% (run 37000490372, pop150k WBP, арт world3-bench id 11255614992)

## Метод
cpu-collapsed.txt (13414 строк, 55697 сэмплов, async-profiler) из артефакта run-37000490372.
Агрегация: доли фреймов BenchPopulationPlugin / getEntities-сканов / command-context; головы и хвосты стеков.

## Результаты
1. **BenchPopulationPlugin = 0.00% фреймов** (0 сэмплов; scan/spawn под-плоскости = 0).
   → атрибуция «topup-луп BenchPopulation жжёт 49.8%» (AG-209 collapsed / AG-226 потолок) НЕВЕРНА.
   → ledger Л-475-C32.1 (topup-scan ≤0.06-0.3% cpu) и Л-474-C82.1 (вес = спавны, не сложность) ПОДТВЕРЖДЕНЫ.
   → фикс-план AG-226 (патч BenchPopulationPlugin) ловит 0% — dead end.
2. **49.93% сэмплов содержат command+context-плоскость; 49.76% = один цепочка:**
   `ServerLevel.tick → tickTime → TimerQueue.tick → FunctionCallback.handle → ServerFunctionManager.execute
   → Commands.executeCommandInContext → ExecuteCommand → EntityArgument.getOptionalEntities
   → EntitySelector.findEntities → EntitySelector.addEntities → ServerLevel.getEntities
   → EntityLookup.get (+getEntityStatus/Entity.getType/tryCast)`
   38 стеков-вариантов; листья: EntityLookup.get 23.3%, NodeIterator.findNext 7.7%, moonrise$getChunkStatus 7.7%, Entity.getType 5.6%.
   Источник = запланированная (schedule→TimerQueue) функция датапака **stz3v2** (DP-INSTALLED sha 16fa1a32, files=707,
   = dp-пак «l» из worklog 491-C59/C21) с @e-сканами. Это НЕ BenchPopulation и НЕ ваниль-тики.
3. Кросс-репродукция 491-C59: 54.73% ALL-CPU по dp-r2 профилю (52,581 сэмплов) ≈ мой 49.76% на pop150k WBP-ноге.
   C59-класс (dp-l O(N)-скан-налог) живёт и на pop-ногах, не только на dp-ранах.
4. Capture-лейн остаётся kernel per-type index на живом носителе (= R1-пул 47.76pp CLM-C41, pair-legal, после снятия C04).
   Редукция датапака (351→1 скан) — REFUTED 491-C59 (EV<0), не воскрешать.

## Вердикт
- FAIL: topup-атрибуция 49.8% REFUTED (BP-плагин 0.00%).
- FACT: 49.8% = dp-l @e-сканы через TimerQueue/EntitySelector; C59 подтверждён независимым профилем.
