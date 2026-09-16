# GOAL — 20 TPS на MineShield-3 (north star)

> Зафиксировано владельцем 2026-09-16: «твоя задача сделать 20 тпс, минимальный
> количество мспт на этом майншилд 3 сервере со всеми чанками форс лоадами и что
> бы и мобы спавнились, и деспавнились, когбудто бы игроки есть».
> Ранее: «никаких фоллбеков, всё должно быть ускорено а не просто менять конфиги».

## binding directive (правила ранга рычагов)

1. **NO FALLBACKS**: нативный путь либо работает и побеждает, либо рычаг REFUTED.
   Никаких «фоллбек-веток» в хот-лупе; переключение — на уровне гейта.
2. **NO CONFIG-WINS**: любые «+TPS» от правки конфигов/JVM-флагов — НЕ ускорение
   и не засчитываются в ledger. Только реальный код: Rust/JNI, ASM-patch,
   алгоритм/структуры данных, побеждающий с медианно-точной паритетом.
3. Один рычаг за раунд; STEP-0 kill-gate до кода; гейт >= 3.0% MSPT, CI A/B min-of-2.
4. INJECTS-ONLY в sandbox; CI-буты санкционированы.

## сценарий (scenario fidelity)

- Мир: реальный MineShield-3 (Purpur 1.21.10), все чанки force-load.
- **Требование владельца**: мобы спавнятся И деспавнятся, «как будто игроки есть».
  Сейчас natural spawning не активен (0 игроков в бенче) — сущности мира живы
  (Brain.tick 6.85% подтверждает), но цикл спавна не нагружает тик.
  => BENCH-SCENARIO upgrade (task166+): fake-player注入 для per-player spawn
  алгоритма (без сети, серверные ServerPlayer-заглушки). Это сценарий бенча,
  НЕ игровая правка; фиксируется как требование к world-bench-4.

## база и цель

| метрика | run#10 | run#11 | цель |
|---|---|---|---|
| MSPT avg | 80.86ms | 84.47ms (разброс ~4% => A/B paired обязателен) | **<= 50ms** |
| TPS | ~13.5 | ~13.3 | **20.0** |
| нужно срезать | | | **~30-34ms = ~40% CPU тика** |

## MSPT budget ledger (run#11, % от CPU тика, обновляется каждый раунд)

| лейн / кластер | presence | replaceable-ядро (верифицировано) | статус |
|---|---|---|---|
| entity/mobs всего | 43.3% | — | главный резерв |
| Brain.tick кластер | 6.85% | machinery ~2.3% (itables 0.49+0.35, iterators 1.03, getNode 0.23, sequencedKeySet 0.11, getRunningBehaviors 0.11) + alloc 11.3% тика | **GC-SHAPE-1 ядро (task166)** |
| GC (STW + barriers) | 9.5% | аллокационный rate — F2; verified sites: BlockPos в optimiseRandomTick (new на пик), Brain iterator/views, streams в сенсорах | **рычаг task166** |
| random-tick lane | 5.46% | advanceSeed 1.57% + BlockPos-alloc + хвост SELF; batch-RNG REFUTED соло | REFUTED соло (task165); входит в GC-SHAPE-1 |
| redstone sub-lane | ~3.8% | STEP-0 не сделан | queued (task167?) |
| minecarts | ~5.3% | STEP-0 не сделан | queued |
| Villager | 3.47% | пересекается с Brain | через Brain |
| chunk lane | 9.8% | per-get lens REFUTED; batch-lens REFUTED (3.3% потолок) | closed |
| worldgen | 0.0% | — | closed (измерено) |
| boats (owned) | 0.76% | < 4% гейта | REFUTED (task164) |
| ENT-BP соло | 1.5-1.8% | < 3% | parked (инфра для vehicle A/B) |

## арифметика честного пути к 20 TPS

Сумма всех ЗАКРЫТЫХ честных соло-рычагов сегодня ~5-7% MSPT — мало.
Путь к 40%: только **агрегатные семейства** (каждый патч проходит свой гейт):
- GC-SHAPE (task166+): verified alloc sites серией -> GC 9.5% + correlated CPU
- ENTITY-LENS семейство (Brain machinery, сенсоры-стримы, minecarts STEP-0)
- REDSTONE-LENS (STEP-0 первым раундом)
- entity_mirror infrastructure A/B (vehicle-dense, ENT-BP infra)
- bench-4: fake players (спавн как при игроках) — честная база для всех A/B
Каждый шаг — паритет-банкованный; сводные A/B после каждого семейства.

## калибровка профилировщика (banked, task165)

- **alloc-collapsed leaves НЕ равны new-сайтам**: `AABB.intersects` (чистая
  математика, 0 new байткодом) — топ-лист; `PalettedContainer.get` — 0 new.
  Интерпретация: alloc-профиль = давление корреляции (TLAB-refill окна),
  НЕ карты сайтов. Verifed-site метод: javap `new`-скан по телу метода.
- CPU leaf после JIT-инлайна = нижний физический фрейм (nextInt -> advanceSeed).
