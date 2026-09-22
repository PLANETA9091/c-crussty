# TASK-411-A (k5b): AIOOBE root-cause — RED-LEVER-BUG вердикт ak5 = НЕ navpool

Дата: 2026-09-22, агент A, ветка round-411-a-nav (102f535).
Статус: root-cause закрытEvidence-First; navpool fail-dominant hardening реализован
отдельно (гипотезы (a)-(d) директивы — см. tests + src/nav_pool.rs).

## 1. Сигнатура отказа (run 35679132437, round-round410ak5-diag)

- 32,769× `java.lang.ArrayIndexOutOfBoundsException` в WARN BenchPopulation
  ("item/mob spawn failed"): 5× с сообщением `Index -1 out of bounds for length
  131073`, остальные 32,764× БЕЗ сообщения (message-less AIOOBE).
- Окно: population 84k→~98k/150000 (02:30:05–02:30:43), затем watchdog
  "server has not responded" — main thread НАВСЕГДА застрял в
  `Int2ObjectOpenHashMap.containsKey:349` ← `ChunkMap.addEntity:953`
  (watchdog thread dump 02:30:50/55, RUNNABLE, бесконечный probe-цикл).
- INJECT DONE отсутствует → pop=INVALID → DELIVERY-FAIL (ABSORB ak5).

## 2. Чей это баг — доказательство НЕ-navpool

| run | navpool? | AIOOBE | исход |
|---|---|---|---|
| round-round410ak5-diag (35679132437) | ДА (73× navpool EFFECT, арм ок) | 32,769 | DELIVERY-FAIL |
| round-round409anchorb (35666743738) | **НЕТ** (0 «navpool» в логе) | **26,215** (та же `length 131073`) | DELIVERY-FAIL |
| round-round410anchora/b/c | НЕТ | 0 | PASS (pop VALID) |

`round-round409anchorb` = та же конфигурация (region_threads=4, batch_collector=1,
inside_cache=1, flush_diet=1, population 150000/seed 42), БЕЗ cmp405_navplane —
и ТОТ ЖЕ фейл той же сигнатурой → **AIOOBE не является эффектом node-pool**.

## 3. Механизм (javap вербатим из patched-kernel 1.21.10 + fastutil 8.5)

- `length 131073` = `n+1` таблица fastutil open-addressing map при n=131072 —
  в окне tracked-entities 49,153–98,304 (population 150000 растёт именно через
  это окно; после 98,304 resize → n=262144 и таблица пересоздаётся — потому
  фейл-окно и закрывается ~02:30:43).
- Застрявший `containsKey` (watchdog): probe-цикл fastutil
  `while (curr != 0 && (curr = key[pos = (pos + 1) & mask]) != k)` возвращает
  только по пустому слоту; таблица со ВСЕМИ занятыми слотами = бесконечный
  цикл. Легитимный fastutil при size>maxFill(0.75·n) уходит в rehash ДО
  заполнения → полная таблица возможна ТОЛЬКО при рассинхроне size/таблицы,
  т.е. при конкурентной мутации карты (put main-thread популяции vs
  remove/shiftKeys из тик-фазы; ChunkMap.addEntity имеет AsyncCatcher, remove
  путь из region-worker тиков - асинхронный).
- `Index -1 ... 131073`: fastutil MapIterator/ValueIterator сканирует ВНИЗ
  (`key[--pos]`); на таблице без пустых слотов downscan проходит слот 0 →
  key[-1]. Это и есть единственный -1-индексатор в fastutil — совпадает с
  редкими (5×/2×) message-ful AIOOBE.
- 32,764× message-less AIOOBE = явные `throw new ArrayIndexOutOfBoundsException()`
  (без констант-сообщений) в catch-петле спавна той же фазы.

## 4. Вердикт

- Вердикт ak5 «RED-LEVER-BUG: отрицательный индекс в node-pool» —
  НЕПОДТВЕРДИЛСЯ: navpool-код (NodeEvaluator.prepare/getNode retarget +
  navPoolTick JNI) в отказе не участвует; латентная гонка ChunkMap.entityMap
  эпохи region-threads на сцене 150k — флейк- Hazard benches (2 из 5
  недавних 150k-прогонов).
- navpool hardening всё равно реализован (директива): fail-dominant
  try-catch fallback на prepare/getNode (любой Throwable → ванильный
  nodes.clear() для поиска), bound-checked arena + heap-модель в
  src/nav_pool.rs + cargo-тесты empty-pop / negative-hash /
  reuse-during-remove. Parity: fallback = ванильное поведение бит-в-байт.
- Риск ре-ролла: гонка population остаётся (вне скоупа лейна navpool;
  патчить ChunkMap = новый вектор — ЗАПРЕЩЕНО директивой). Контроль: grep
  AIOOBE в новом прогоне; при повторе сигнатуры entityMap — фиксировать
  hazard и ре-роллить (≤2) с пометкой.

## 5. Проверка «что изменилось бы» для 410-якорей

Тот же код, тот же сид 42, тот же порядок чанков → детерминизм сцены, но
гонка тик-фазы vs population-фазы недетерминирована (JIT/потоки/ GC-паузы) →
окно 84k–98k попадает/не попадает в рассинхрон. Согласуется с 3×PASS/2×FAIL.
