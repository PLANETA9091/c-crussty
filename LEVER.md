# LEVER.md — `items_offthread` (TASK-397-D, MEGA-ROUND-2)

**Lever id:** `items_offthread` (`CRUSSTY_LEVER_FLAG=items_offthread`, `CRUSSTY_LEVER_ARG=1`).
**Gate:** flag == `items_offthread` AND `CRUSSTY_REGION_THREADS>=2` AND `CRUSSTY_REGION_STEAL != "1"`.
Любой другой флаг / нет флага = dormant-invisible: ни один байт ядра не меняется,
ни один хук не регистрируется (ванильный путь ПО ПОСТРОЕНИЮ, не по фоллбеку).

## Механизм

- СКАН (off-thread): воркер RegionTickOps, тикнувший свой бакет (tickBucket,
  слот = регион), сразу после свипа сущностей сканирует СОБСТВЕННЫЙ бакет
  (post-sweep позиции = состояние, которое каждый item видел в свой тик;
  кросс-поточных чтений НЕТ) и продюсирует merge-решения (a,b) в per-slot
  буфер: клеточный хеш cell=2.0 + stable counting placement (порядок внутри
  ячейки = snapshot order) → детерминизм без сортировки.
- ПРИМЕНЕНИЕ (main): после DONE-барьера + deferred-дренов (phase inactive =
  ванильный протокол мутаций EntityTickList) RegionTickOps.postJoinHook
  применяет решения в порядке слотов 0..W-1; каждый merge = НАСТОЯЩИЙ
  приватный vanilla `ItemEntity.tryToMerge` через кэшированный MethodHandle
  (направление поглощения по count — то же, что vanilla this.tryToMerge(other)).
- Байт-хук: `ItemEntity.tick()V` — единственный горячий сайт
  `invokevirtual mergeWithNeighbours()V` (census javap s7204: tick=1 сайт,
  второй сайт в teleport остаётся ванильным) ретаргетится length-preserving
  (invokevirtual→invokestatic, CP-append) на
  `ItemMergeOps.tickMerge(ItemEntity)V`. ARMED → return (скан был off-thread);
  не ARMED → public-API реплика ванильного тела (parity fail-closed).
- ItemMergeOps (+SlotState) определяются Rust-ом в kernel loader после boot
  (fluid_bitmask/flush_diet паттерн); статик-иниц биндит MethodHandles
  (isMergable/tryToMerge, setAccessible) и вешает хуки в RegionTickOps
  (retry-демон, RegionTickOps может определиться позже).

## Паритет (ванильная семантика)

Предикат кандидата, бокс inflate(itemMerge[,itemMerge-0.5]), точный
AABB.intersects пост-фильтр, Paper cadence gate (moved ? 2 : 40 по floor
позициям), isMergable self-gate, walls-check clipDirect, слияние самими
ванильными байтами tryToMerge — всё построчно сверено с javap s7204
(mergeWithNeighbours/tryToMerge/areMergable, areMergable = public static).

## Девиации (задокументированы, в пределах окна ≤1 тик)

1. Решение применяется пост-барьером конца тика, а не mid-sweep (сдвиг ≤1 тик).
2. Cross-bucket соседи (бокс через границу квадрантов) находятся на следующий
   cadence-тик.
3. Порядок пар = snapshot order (≈ vanilla insertion order), а не broadphase-
   порядок внутри getEntitiesOfClass.
4. Merged-away item успевает дотикать свою физику/age до discard'а в конце тика.

## Fail-closed пути

- Нет флага / чужой флаг → хуки не регистрируются, байты не трогаются.
- region_threads<2 или STEAL=1 → dormant (скан живёт в static-bucket tickBucket).
- ItemEntity не загрузился за 180s / нет престинов / patch outcome не
  Retargeted/AlreadyPatched / define_class упал → hook stays dormant (ваниль).
- MethodHandle bind failed → tickMerge = vanilla-реплика; ARMED=false.
- Wiring не удался за 30×2s → vanilla fallback, ARMED=false (System.err маркер).

## ARM-маркеры (server-stdout)

- `[crussty-plugin] items_offthread: pristine sighting net/minecraft/world/entity/item/ItemEntity 28904 bytes (major 65)`
- `[crussty-plugin] items_offthread: computed patch for ... (28904 -> 28976 bytes, Retargeted { sites: 1 })`
- `[crussty-plugin] items_offthread: net/minecraft/world/entity/item/ItemEntity armed, retransform rc=0`
- `[crussty-plugin] items_offthread: hooks wired into RegionTickOps, ARMED`
- `[crussty-plugin] items_offthread: hook serve ... 28976 bytes`
Оффлайн-цензус (cargo test itementity_tick_merge_retargets_exactly_one_site):
ровно 1 сайт в tick()V, idempotent, 28904 -> 28976 байт, major 65 сохранён.
