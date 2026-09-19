# S7-170 — NAV-MOBS-GUARD: потокобезопасная канализация navigatingMobs (RECON-22 → реализация, TASK-348)

Статус: **РЕАЛИЗОВАН + ОФФЛАЙН-ГЕЙТ PASS (6/6)**, ждёт on-scene подтверждения (PG-T2 threw=0
на следующей широкобандной ноге). Это инфраструктура ПРАВИЛЬНОСТИ (RECON-22 §4:
«правильность, не ТОП-1-perf»), не перф-рычаг — DUAL BAR к ней не применяется; ожидаемый
эффект на TPS ≈ 0 (хот-лоуп не тронут).

## 1. Javap-контракт (kernel jar s7178-recal, contract-s7170-*.txt)

Поле `ServerLevel.navigatingMobs` (`final Set<Mob>`, fastutil ObjectOpenHashSet) трогают
РОВНО 4 сайта во всём ядре (grep по классам jar + javap):

| Сайт | Операция | Поток в банке v3 |
|---|---|---|
| `ServerLevel.<init>` | putfield new ObjectOpenHashSet | main, boot |
| `ServerLevel.sendBlockUpdated` | getfield + **iterate** (Paper оборачивает сбор в CME-catch с рекурсивным retry — но fastutil «wrapped is null» NPE это НЕ CME и улетает наверх) | main ИЛИ воркер (bu_defer=0) / main (реплей, bu_defer=1) |
| `ServerLevel$EntityCallbacks.onTrackingStart` | `Set.add(Mob)` | main / chunk-system |
| `ServerLevel$EntityCallbacks.onTrackingEnd` | `Set.remove(Mob)` | main / chunk-system |

Все обращения идут через `invokeinterface java/util/Set` → **одно точечное действие —
подмена ЗНАЧЕНИЯ поля** — маршрутизует всех читателей и всех писателей через гвард.
Ноль ретаргетов, ноль изменений classfile.rs/cargo/yml-инпутов.

## 2. Реализация (RegionTickOps.java, S7-170 NAV-MOBS-GUARD)

- `GuardedNavigatingMobs implements Set<Mob>`: внутренний ObjectOpenHashSet (клон
  ванильного), мутации под монитором обёртки; `iterator()/forEach()/spliterator()`
  снимают снапшот-клон под тем же монитором и итерируют замороженную копию.
- **Порядок итерации бит-в-бит ванильный**: fastutil `clone()` копирует хеш-таблицу
  (маска, размер, заполнение) → порядок обхода идентичен негвардированному набору
  (доказано харнессом, [2]).
- `ensureNavMobsGuarded(Entity)`: идемпотентный main-thread-only swap через Unsafe
  (offset поля вычислен от реального ServerLevel), печатает маркер
  `[S7-170] nav-mobs guarded: level=... seeded=N`. `WORKERS<=1` — javac сворачивает
  вызов в no-op (static final константа) → ванильный passthrough бит-в-бит.
- Активация: из уже доставленных ретаргетов `onTickingStart/onTickingEnd` (первый
  поток add/remove сущности происходит задолго до первой фазы воркеров; swap на main,
  воркеры никогда не запускают swap; happens-before воркерам дают барьеры фаз).

## 3. Классы гонки, которые закрывает

- **Класс A (s7186, банк v3 bu_defer=0)**: воркер тикает сущность → setBlock (DoorBlock
  от checkInsideBlocks) → ВАНИЛЬНЫЙ sendBlockUpdated:1883 итерирует navigatingMobs,
  параллельный воркер/chunk-system мутатирует set (Navigation add/remove через
  onTrackingStart/End) → fastutil NPE. Гвард: итерация = снапшот под монитором,
  мутация = под монитором → гонка структурно невозможна.
- **Класс B (s7176/s7180, bu_defer=1)**: main-реплей BlockUpdateOps.vanilla:117
  итерирует, пока воркеры ещё мутируют. Тот же гвард закрывает (реплей тоже идёт
  через подменённое поле).

## 4. Оффлайн-гейт (scripts/run_navmobs_lockstep.sh, NavMobsLockstepHarness) — PASS 6/6

1. Структурная линковка с РЕАЛЬНЫМ kernel jar: поле резолвится, Unsafe offset=308,
   `ensureNavMobsGuarded` присутствует, обёртка реализует Set.
2. **Порядок-паритет**: 240 000 опций (LCG add/remove/contains) по ванильному и
   гвардированному репликам; полные итерационные последовательности сравнивались
   поэлементно на каждом 4k-чекпоинте — **бит-в-бит идентичны** (52 030 adds
   идентичны, contains идентичны).
3. Снапшот-семантика: замороженный вид, мутация в середине итерации не видна и не
   бросает; свежий итератор видит добавленное.
4. **Конкурентный стресс**: 8 потоков × 50k опций, 4 снапшот-ридера → **0 исключений**
   на гварде (50 131 полных обходов, 9.59M частичных чтений); **сырой контроль
   взорвался ТОЧНОЙ продакшн-сигнатурой**: `NullPointerException ... "this.wrapped"
   is null` (ObjectOpenHashSet$SetIterator) — класс s7176/s7180/s7186/s7189
   воспроизведён оффлайн.
5. Финальная консистентность: обход 9 908 == size(), все элементы — реальные Mob.

## 5. Parity-прегистер (RECON-22 §4, расширенный)

- Порядок recomputePath для идентичного содержимого набора СОХРАНЁН (клон таблицы).
- Принятый класс: окно между снапшотом и циклом recomputePath — наблюдение
  кросс-поточного состояния того же класса, что уже принят барьером владельца для
  region-тиков (S7-155); ванильная конкурентная итерация была UB (Paper ловит CME и
  ретраит — этот путь становится мёртвым, зафиксировано).
- Отсутствие гонки ≠ отсутствие interleave: mob может быть удалён между сбором и
  recomputePath — recomputePath не трогает navigatingMobs (javap), испорченных
  итераторов нет, per-entity catch не задействуется.

## 6. Гейт следующей ноги (preregister)

Следующая широкобандная нога (после этого коммита) обязана: PG-T1 маркер
`[S7-170] nav-mobs guarded` присутствует ( absorb_s7189.py теперь печатает
guarded-marker), **PG-T2 threw=0** (гонка закрыта на сцене; все предыдущие ноги
банка v3 имели threw=1..3 того же класса), GC young ≤174 / Full 0. Если threw>0 того
же класса — гвард не сработал, вердикт FAIL + отккат точки подмены.
