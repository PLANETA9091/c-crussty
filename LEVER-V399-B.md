# LEVER-V399-B — cmp399_shard (agent B, TASK-399 MEGA-ROUND-3, vector shardgrid)

## Тезис (1 строка)
Замена глобального `RwLock<Inner>` (round-398-J item-grid, src/items_index.rs) на
шардированный seqlock-grid: 64 шарда по `mix64(cell_key) & 63`, lock-free
per-cell read-путь ⇒ устранение лоч-конкуренции idx_query на 4 region-воркерах
(пост-J профиль round-j2b: ~33k+ запросов/с, скрытая цена rust-пути 12-17% wall).

## Почему superiority (vs база J = origin/round-398-j-subsys2)
1. **Read-путь вообще не берёт лок.** Legacy: каждый idx_query делает
   `IDX.read()` — атомарный RMW на ОДНОМ слове (ping-pong кеш-лайна между 4
   воркерами) + std RwLock writer-preference: любой writer (block-crossing,
   phase-4 drain) заставляет прибывающих читателей фатекс-спать. Sharded:
   читатель делает 2 Acquire-загрузки версии шарда на клетку и 0 записей в
   общие слова — читатели не взаимодействуют ни друг с другом, ни с писателями.
2. **Писатели не останавливают читателей.** В legacy write-заголовок блокирует
   ВСЕ новые idx_query на время unlink+link (+ grow_grid rehash паузы на
   глобальном lock). В sharded писатель бампает версию шарда (odd→mutate→even,
   под глобальным writer-mutex — линейность записи как в legacy); читатель лишь
   повторяет одну клетку (n0-rollback). Стойла читателей исчезают как класс.
3. **Нет глобального rehash-pause**: таблицы фиксированной ёмкости (16384
   слота/шард ⇒ 1M ключей; бенч 150k клеток ≈ 2.4k/шард — 6× запас по нагрузке
   62.5%), per-id массивы фиксированы (1<<20 id; Java реиспользует id через
   freeIds). Переполнение — fail-closed (ERR_STRUCT → disarm → vanilla).
4. **Паритет по построению**: та же структура (открытая адресация, интрузивные
   цепочки head/next, тот же cell_key, тот же порядок обхода клеток ⇒ идентичные
   кандидат-сеты и их порядок); меняется ТОЛЬКО конкурентность. Двухрежимность:
   флаг `cmp399_shard` → шардированный путь; флаг `items_subsys2` (и любой
   другой) → legacy-путь байт-в-байт (тело legacy_query извлечено дословно).
   Транзиентный cross-cell skew при конкурирующем переезде item'а ограничен
   семантикой ванильного EntitySectionStorage (per-section snapshot) и полностью
   поглощается java-рефильтрами (isAlive/level/AABB/isMergable) — двойных
   мерджей нет: первый tryToMerge делает other.isAlive()=false.

## Арм/гейты (контракт TASK-399-B)
- `src/items_manager.rs`: arm-gate расширен `f == "items_subsys2" || f.starts_with("cmp399_")`.
- Режим: `CRUSSTY_LEVER_FLAG == "cmp399_shard"` → шардированный grid (OnceLock,
  процесс-константа); иначе legacy global-RwLock.
- Java-гейт (ItemEntityManager.<clinit> ENABLED) под cmp399_* CP-патчится на
  лету (`classfile::patch_utf8_gate`: единственный Utf8 "items_subsys2" →
  "cmp399_shard", bytecode-transparent, self-verified re-parse; javac в контуре
  отсутствует — бинарник = артефакт javac round-398-J).
- Громкий маркер: `[crussty-plugin] cmp399_shard: ARMED shards=64 ...`.
- Std-only: атомики Acquire/Release/Relaxed + std Mutex; внешних concurrency-
  крейтов нет (Cargo.toml не менялся).

## Дедлок-свобода
Читатели не берут ни одного лока; писатели держат один глобальный Mutex без
вложений. Единственный порядок — begin/end версии шарда внутри мьютекса.

## Ожидание
Read-скейлинг на 4 воркерах: устранение RMW ping-pong + писательских стоулов
idx_query (кандидат №1 скрытой цены 12-17% wall) ⇒ композит J+shard ≥ +5% TPS
(зачёт), амбициозно ≥ +15%.
