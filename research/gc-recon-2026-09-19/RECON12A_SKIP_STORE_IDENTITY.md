# RECON-12a: identity-контракт сеттеров для #13 SKIP-STORE-DIET (TASK-318, тик 14:08)

Статус: RECON-подготовка при летящем леге 35425246662 (S7-108 — диспатчей нет).
Метод: javap -p -c по РЕАЛЬНЫМ kernel-классам tests/fixtures (Entity_real 205458B,
LivingEntity, ServerLevel, Level_real, ChunkMap_real, Brain, LevelChunk_real,
LevelTicks, PalettedContainer, SingleUserAreaMap) — «юнит» = 1 инструкция;
identity-сайт = if_acmpeq/if_acmpne в окне ±8 строк от getfield bb / deltaMovement
/ getBoundingBox / getDeltaMovement (python-свип, надёжнее awk).

## 1. Контракты тел (verbatim)

### setDeltaMovement(Vec3) — 23 юнита
```
0-6:  monitorenter posLock          // СИНХРОНИЗАЦИЯ — часть поток-контракта
7-9:  putfield deltaMovement
12-21: monitorexit + athrow-таблица (structured)
22:   return
```
- putfield ПОД монитором posLock: редирект обязан сохранить монитор-семантику
  (region_threads=4 — сервер многопоточный; пропуск монитора при skip = гонка).

### setDeltaMovement(DDD) — 15 юнитов
```
new Vec3(x,y,z) -> invokevirtual setDeltaMovement(Vec3)
```
- аллоцирует НОВЫЙ Vec3 КАЖДЫЙ вызов (даже при идентичных значениях).

### setBoundingBox(AABB) — 173 юнита (самое тяжёлое тело)
```
мин/макс из аргумента -> 6 dstore;
по каждой оси: d = max-min; dcmpg: if (d < 0) max = min;   // 3 лестницы
               dcmpl: if (d > 64.0) max = min + 64.0;       // КЛАМП 64.0 (ldc2_w #3393)
bb = new AABB(minX, minY, minZ, maxX, maxY, maxZ);          // НОВЫЙ AABB КАЖДЫЙ вызов
putfield bb;
```
- dcmpg-лестницы ×3 (урок №9) + кламп-лестницы ×3 (dcmpl) — нормализация
  bit-exact; NaN: dcmpg(NaN,0)=1 → <0-ветка не срабатывает, dcmpl(NaN,64)=1 →
  >64-ветка не срабатывает (NaN проходит вербатимом).
- self-аллокация: каждый вызов = 1 new AABB (young) + 1 putfield в СТАРЫЙ Entity
  (old→young карта) — двойное ядро store-firehose.

## 2. Identity-свиток (acmp ±8 от поля/аксессора)

| класс | acmp-сайтов всего | на bb | на deltaMovement |
|-------|------|-------|------------------|
| Entity_real | 52 | **0** | 5 (все getDeltaMovement) |
| LivingEntity | 38 | 0 | 0 |
| ServerLevel | 16 | 0 | 0 |
| Level_real | 9 | 0 | 0 |
| ChunkMap_real | 3 | 0 | 0 |
| Brain / LevelChunk / LevelTicks / PalettedContainer / SingleUserAreaMap | — | 0 | 0 |

ЕДИНСТВЕННАЯ identity-семейство — deltaMovement, 5 сайтов Entity:
- **offset 280-285 в move(MoverType,Vec3)**: `param == getDeltaMovement()`
  (vanilla guard: дельта не изменилась за move → сжечь в ZERO);
- 4616, 5429 (knockback/push-семейство: fstore_2-контекст), 5696 (acmpeq ДО
  чтения — сравнение иного значения), 10606 (scale-цепь 0.75).

bb: identity-сравнений НЕТ НИГДЕ (все консьюмеры — value-семантика:
contains/inflate/getCenter/clip/...).

## 3. Пarity-анализ value-equal store-skip

### setBoundingBox — ПАРИТИ-SAFE ПО КОНТРАКТУ (плюс к аргументам TASK-316)
- идентity-сайтов 0 → skip не меняет никакое наблюдаемое identity-сравнение;
- AABB immutable (equals по 6 double) → value-equal ⇒ наблюдаемо неразличим;
- нормализация bit-exact повторяется в бридже (dcmpg/dcmpl verbatim);
- ЭФФЕКТ ТРОЙНОЙ: (1) срезает new AABB self-аллокацию, (2) срезает putfield
  old→young карту, (3) срезает мониторы/чтения 6 полей (мелочь).
- ЧИСТЫЙ КЛАСС #13: value-equal skip внутри редиректа тела setBoundingBox.

### setDeltaMovement — PARITY-РИСК (черно-белой ясности нет, окно найдено)
- vanilla move() ЗОВЁТ setDeltaMovement ВНУТРИ тела ДО identity-guard:
  offset 263 (stuck-ветка: движение затухло → setDeltaMovement(Vec3.ZERO),
  сброс stuckSpeedMultiplier на 252), и ПОСЛЕ: offset 800 setDeltaMovement(DDD)
  (финал после коллизий), 1115;
- guard `movement == getDeltaMovement()` (282-285) сравнивает ВНЕШНИЙ
  параметр с полем: если параметр вынесен вызывающим ДО skipнутого сеттера с
  value-equal (B != M по ссылке, B.equals(M)) — vanilla guard=false (поле
  B), skip-версия guard=true (поле не писалось, = M) → vanilla НЕ сжигает
  дельту, skip СЖИГАЕТ → следующее движение различается. ПАРИТИ СЛОМАН на
  этом окне;
- окно требует: move-сайт с параметром, вычисленным РАНЬШЕ setDeltaMovement
  (не inline getDeltaMovement()). Статически не сводится к нулю без полной
  верификации ВСЕХ сайтов вызова move() (LivingEntity.travel/itemEntity/
  baseTick/...) — большая работа, целесообразность решит OldObjectSample;
- доп. риск: monitorenter posLock — skip обязан держать монитор (гонка).

## 4. Решение-матрица для absorb (лег 35425246662)

OldObjectSample producer-семьи → jfr print:
- если bb-поток (boundingBox/move family) доминирует среди entity-полей:
  **GO #13-SBB** (value-equal skip ТОЛЬКО setBoundingBox(+DDD-обёртки не
  существует)) — parity-safe по контракту §3; потолок = bb-доля × 16.39%
  card-set; порог ≥ 2-3% wall (preregister TASK-316);
- если deltaMovement-поток доминирует: полная верификация move-сайтов
  (джавап-контракт всех внешних вызовов move() с вынесенным параметром) —
  отдельный preregister, иначе NO-GO-семейства deltaMovement;
- sync/chunk-lists потоки: вне #13 (TASK-316 списки — «sync/списки»
  оценивались в 40-70% совокупно; решит измерение).

Артефакты: /tmp/entity318.txt (дизассемблирование Entity_real), фикстуры
javap-дампы; этот документ. Лег НЕ диспатчен (S7-108 — 35425246662 в полёте).
