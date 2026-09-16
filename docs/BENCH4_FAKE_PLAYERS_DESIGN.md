# BENCH-4 FAKE-PLAYERS — DESIGN + PREREGISTRATION (task170, S7-98)

> Owner condition (2026-09-16, binding): «...и что бы и мобы спавнились, и
> деспавнились, когбудто бы игроки есть». Сейчас natural spawning СТРУКТУРНО
> выключен (0 players => 0 spawnable chunks per PlayerMobDistanceMap).
> Это СЦЕНАРИЙ бенча (не игровая правка, не config-win): fixture, который
> делает каноническое условие владельца измеримым и честным.

## 1. Verified spawning contract (javap, purpur-1.21.10.jar)

- `NaturalSpawner.spawnForChunk(ServerLevel, LevelChunk, SpawnState, List<MobCategory>)`
  — per spawnable chunk drain; категории из
  `getFilteredSpawningCategories(SpawnState, boolean, boolean, boolean, ServerLevel)`.
- Per-position gate:
  `isRightDistanceToPlayerAndSpawnPoint(ServerLevel, ChunkAccess, BlockPos.MutableBlockPos, double)`
  — РАБОТАЕТ ТОЛЬКО ОТ СПИСКА ИГРОКОВ (`level.players()`), включая despawn-логику
  mob.checkDespawn (nearestPlayer-запросы).
- Cap-state: `NaturalSpawner.createState(int, Iterable<Entity>, ChunkGetter,
  LocalMobCapCalculator, boolean)` — caps от полного entity-обхода мира.
- moonrise: `PlayerMobDistanceMap` (chunk_system/player) обновляется от
  add/remove-player событий; spawnable-chunk set = per-player радиусы — 0 игроков
  = пустой сет (измерено run#15: «0 spawnable chunks»).
- Despawn-плечо (owner требует ДЕСПАВН тоже): distance-based despawn
  (`Mob.checkDespawn` → getNearestPlayer) + cap pressure — оба оживают от
  реальных ServerPlayer.

## 2. Инъекция (единственный праймари-путь, без фоллбеков)

**ServerPlayer-in-the-server** (Citizens-style, bench-only):
1. Плагин `BenchFakePlayers` в CI-harness (CI-буты санкционированы; sandbox не
   бутится — INJECTS-ONLY сохраняется):
   - `new ServerPlayer(server, level, gameProfile, clientInformation)` с
     минимальным `Connection`-стабом (без Netty-канала);
   - регистрация в PlayerList так, чтобы `level.players()` содержал игрока И
     PlayerMobDistanceMap получил add-событие;
   - позиции: N игроков сеткой по force-load зоне (покрытие 9216 чанков);
   - тик-глушение: ServerPlayer.tick() может звать connection — стаб отвечает
     no-op; сами игроки не двигаются, не отправляют пакеты;
   - profile-имена `BenchFake-N`, без сохранения в usercache/world.
2. N = стартово 4 (по одному на квадрант), параметризовано; новые прогонки
   фиксируют N в run-env.txt (paired-дисциплина).
3. Верификация контракта в прогоне: `paper mobcaps world` показывает
   spawnable chunks > 0; F4 churn = ACTIVE (spawn И despawn дельты); если
   spawnable chunks == 0 — прогон INVALID (gate в report_world3.py).

## 3. Что это меняет (предварительная анатомия)

- Новый лейн: spawn-алгоритм (NaturalSpawner + isValidSpawnPostitionForType:
  biome/structure запросы, blockstate-чтения) + despawn-цикл + рост числа мобов
  (owner-условие: mobs spawn/despawn ACTIVELY) => AI-лейн вырастет.
- Per-player cost: PlayerMobDistanceMap update, chunk-tracking, player tick.
- БАЗОВАЯ ЛИНИЯ ИЗМЕНИТСЯ (вероятно вверх по MSPT — это честно; north star
  применяется к каноническому сценарию владельца).
- ВСЕ будущие A/B — на bench-4 (paired same-boot, run-env.txt уже шипится).

## 4. Pre-registered gates (task170)

1. **Fixture-validity gate** (не MSPT): spawnable chunks > 0 + F4 churn ACTIVE
   (spawn и despawn дельты > 0 за окно) — иначе прогон валидации fail.
2. **Baseline gate**: bench-4 baseline (N=4) замерен min-of-2 paired; дельта к
   bench-3 документируется как сценарная (НЕ модульная) — не засчитывается ни
   в какую сторону.
3. После bench-4: **fresh recon** профиля нового сценария (полная анатомия заново)
   — пере-ранкинг всех лейнов; только после этого выбирается следующий
   оптимизационный рычаг (если у кого-то появится >=3% replaceable-ядро).
4. Паритет-закон для fixture: fake-players ИДЕНТИЧНЫ между A и B ногами
   (тот же N, позиции, UUID-я) — иначе A/B инвалиды.

## 5. Риски

- ServerPlayer.tick/void-пути могут NPE без валидного connection → стаб
  расширяется по месту падения (это bench-инфраструктура, не игровой код).
- Purpur может иметь собственные патчи списка игроков (PlayerList#addPlayer
  механика) — проверяется booted-javap на месте.
- despawn может съесть свежеспавн слишком быстро (mobcap pressure) — ок,
  это И ЕСТЬ каноническое условие владельца.
