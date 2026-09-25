# RESEARCH-459-P26 — chunk-send burst coalescing (ID-P26, WILD закон 11, TASK-459-62)

Агент: TASK-459-62 (WILD-агент закона 11: безумие проверяется диспатчем, а не перечислением).
Ось: chunk-send (закон 8, player-visible chunk-loading). Развитие cmp437_chunk4 (send-snapshot,
TASK-438-C) ⊕ cmp444_chunk5 (encode-cache, TASK-444-B). НЕ дублирует agent-M: дельты пакетов
он НЕ делает, P26 тоже (байты пакетов НЕ меняются — только межтайминг flush).

---

## 1. Механика (javap ground truth, tests/fixtures/PlayerChunkSender.class, round-396-a jar)

Ванильный `PlayerChunkSender` УЖЕ батчит решение-сторону, но НЕ flush-сторону:

| контракт | значение | смысл |
|---|---|---|
| MIN_CHUNKS_PER_TICK | **0.01f** | минимальная квота чанков/тик (анти-голодание) |
| MAX_CHUNKS_PER_TICK | **64.0f** | потолок чанков/тик — это и есть окно burst |
| START_CHUNKS_PER_TICK | **9.0f** (private) | стартовая квота при join |
| MAX_UNACKNOWLEDGED_BATCHES | **10** | потоковое окно батчей до ack клиента |
| pendingChunks | LongOpenHashSet | очередь чанков (ванильный порядок обхода = порядок отправки) |
| batchQuota | float | min(desiredChunksPerTick, max(...)) в тике |
| tick() хвост | ClientboundChunkBatchFinishedPacket(list.size()) | **ванильный делимитер батча уже существует** |

Ключевая строка bytecode tick(): цикл `invokestatic sendChunk(...)` по списку собранных чанков →
каждая итерация заканчивается `ServerGamePacketListenerImpl.send(packet)` → ваниль каждый пакет
уходит в канал отдельным write+flush (netty: flush = syscall-домен, writev/epoll). Т.е. внутри
ОДНОГО тика join-burst даёт до 64 отдельных flush на игрока при уже существующем батч-протоколе.

План P26: оконное слияние flush — N чанков ванильного батча одного тика (окно) пишутся БЕЗ
промежуточных flush, ОДИН flush на границе окна (хвост tick() = ClientboundChunkBatchFinishedPacket
— естественная граница). Свет-пакеты группой: ClientboundLightUpdatePacket в том же окне.

## 2. Внешние источники (механика подтверждена)

1. **Krypton (CaffeineMC/Velocity-family)** — «Flush consolidation to lower server CPU usage
   (and reducing the impact from frequent flushes)»; мод оптимизирует именно networking stack:
   consolidation флашей = меньше CPU + меньше impact частых flush. URL:
   https://modrinth.com/project/fQEb0iXm , https://github.com/CaffeineMC/krypton ,
   https://www.curseforge.com/minecraft/mc-mods/krypton-reforged
2. **Netty io.netty.handler.flush.FlushConsolidationHandler** (канонический хендлер):
   «Flush operations are generally speaking expensive as these may trigger a syscall on the
   transport level. Thus it is in most cases (where write latency does not matter) it is
   recommended to consolidate flushes» — консолидация после N pending flush (дефолт 256) или
   по read-complete границе. URL: https://netty.io/4.1/api/io/netty/handler/flush/FlushConsolidationHandler.html
3. **PulseNet** (server-sided fabric networking optimization «packet batching», compatible with
   Krypton) — независимое подтверждение направления batch/flush-plane. URL:
   https://modrinth.com/project/C5TrjUEt

## 3. Parity-план (порядок = ваниль-очередь)

* **Байты пакетов НЕ меняются**: chunk4 (snapshot, isUnsaved-ключ) + chunk5 (encode-cache,
  бит-в-бит selftest) уже гарантируют идентичность полезной нагрузки; P26 меняет ТОЛЬКО
  межтайминг flush (write vs write+flush гранулярность).
* **Порядок отправки = ванильный порядок очереди** (pendingChunks LongSet-обход в tick()) —
  окно не переупорядочивает, консервирует итерацию; внутри окна пакеты уходят подряд, один
  flush в конце окна.
* **Окно ≤ 1 тик**: границей окна служит ванильный конец батча (ClientboundChunkBatchFinishedPacket)
  → flush-латентность клиенту = константа окна ≤ 1 тик (без искусственных задержек: flush НЕ
  откладывается за пределы тика, только гранулярность внутри тика).
* **События per-send сохранены**: PlayerChunkLoadEvent + debugSynchronizers.startTrackingChunk
  на КАЖДЫЙ send (как в chunk4-мосте) — события не коалесцируются.
* **anti-xray shouldModify bypass** — как в chunk4 (пакет per-player, мимо кэша/окна — не коалес
  пер-плеерный путь в v1).
* Латч-безопасность: lever-флаг STRICT-OR на свой id cmp459_p26; старые сертифицированные
  id (cmp437_chunk4, cmp444_chunk5, cmp450_chunk, cmp452_mega, cmp453_diet) НЕ получают окно —
  их семантика заморожена. Fail-closed: любой дефект гвардов → хук dormant.

## 4. Δ-прогноз и capture-матем

* Лейн players_packets в бенче = **0.01%** spark-лейнов (монотон-член оси, не wall-heavyweight).
  P26 целит НЕ в spark-лейн, а в **netty/syscall-хвост burst-окна**: join 4 fake players ×
  START 9.0 → квота растёт к MAX 64.0; при N тиках burst = до 64 flush/тик/игрок → консолидация
  ≈ 64:1 syscall-амортизация в окне.
* Прогноз карточки: **+0.3-0.8пп** netty/syscall-хвост burst (carrier-зависимый, monotone;
  VERDICT-NUM скаффолда: WIRED-файлы + CI ниже).
* Потолок: syscall-домен бенч-раннера вне spark main-lane → capture считается от burst-хвоста,
  не от TPS-лейна; честный маркер: если netty-хвост на рантайме < 0.1% wall — эффект в шуме
  (refute-гейт G5 ниже).

## 5. Риски

1. **flush-гранулярность ядра**: фактический write/flush сайт сидит в Connection/netty-пайплайне
   (kernel-closed); v1-скаффолд консолидирует через ванильную границу батча (tick-хвост), полный
   channel.flush() поверх отражательной поверхности — следующий лаб-лег (javap Connection на
   round-396-a jar). Риск-класс: спящий гейт (x425) — лечится обязательной пересборкой блобов +
   raw-cp маркером cmp459_p26 (x93-урок) — сделано в этом же коммите.
2. **TCP-интерактивность**: консолидация flush ≤ 1 тик не добавляет задержки сверх тика (ваниль
   и так шлёт батч-финиш пакет в конце цикла). Риск только при ошибочном удержании за границей
   тика — гейт G4 (порядок+состав пакетов клиенту бит-в-бит, lockstep-лог сэмплов).
3. **NCDFE-канон**: мост определяется ЦЕЛИКОМ в kernel loader, ZERO nested/lambdas (test
   no-nested паттерн chunk4/chunk5 сохранён); selfTest()Z до ARM; EARLY-define retry x10.
4. **Гонка двух плоскостей на PlayerChunkSender**: при cmp459_p26 хук PlayerChunkSender
   принадлежит ТОЛЬКО chunk_send6 (chunk_send.rs на этом флаге dormant — STRICT-OR disjoint),
   блоб ОДИН и тот же (ChunkSendOps) — конфликт перезаписи тела невозможен.

## 6. Preregistered гейты (закон 16)

* G1 ARM: stdout "[crussty-plugin] cmp459_p26: ARMED chunk-send window coalescing".
* G2 эффект: "p26 window stats" (windowOpens/coalesced/flushMarks > 0 после burst).
* G3 order-parity: ванильный порядок очереди бит-в-бит (лог сэмплов позиций).
* G4 window-константа: flush-латентность ≤ 1 тик (батч-финиш делимитер).
* G5 refute: netty/syscall-хвост < 0.1% wall → эффект в шуме, вердикт REFUTED_CENS (числа).
* G6 NCDFE: T1=0, flat==nested после пересборки блобов (javap-гейт build-скрипта).

## SOURCES
https://modrinth.com/project/fQEb0iXm https://netty.io/4.1/api/io/netty/handler/flush/FlushConsolidationHandler.html https://github.com/CaffeineMC/krypton https://modrinth.com/project/C5TrjUEt
