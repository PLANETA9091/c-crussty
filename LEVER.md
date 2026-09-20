# LEVER — items_stagger (TASK-395 mega-round, agent B, branch round-395-b-stagger)

## Механизм
Целостная замена тела `ItemEntity.tick()V` (топ-1 лейн базлайна 31.17% java,
run 35528326290) на faithful-зеркало Purpur 1.21.10 байткода (javap-сверка
блок-в-блок: ItemEntity.tick + Entity.tick) с РАСПИСАНИЕМ фаз и
кандидат-кэшем merge-партнёра. Доставка: FluidPushGuardHook/TASK-80 паттерн
(byte-hook pristine capture → `cplug_sdk::asm::replace_body` → retransform),
бридж `net/minecraft/world/entity/ItemStaggerOps` (+`$MergePredicate`,
+`$Cand`) определяется в KERNEL loader. НЕ мемоизация (не fluid_dirty-мемо) —
именно расписание + кэш кандидата с инвалидацией.

## Три schedule-дельты (deviation от ванили ≤ 8 тиков)
1. **rest/move интервал**: ваниль `(tickCount+id)%4 != 0` → пропуск move-
   блока у стационарных (onGround && horizSqr ≤ 1e-5) → левer `%8`.
   Та же форма гейта, длиннее период, offset по entityId (детерминизм:
   offset стабилен по построению — id монотонный счётчик, переиспользования
   нет; пара (tickCount, id) фиксирована на тик).
2. **На rest-skip тиках** дополнительно пропускаются 2 пер-тиковые проверки,
   которые апстрим НЕ гейтит у спящих items: (a) noPhysics-recheck
   (`noCollision(bb.deflate(1e-7))`) + moveTowardsClosestSpace; (b) второй
   fluid-скан `hasImpulse |= updateInWaterStateAndDoFluidPushing()` (baseTick
   уже обновил fluid-состояние этого же тика). Отложка до следующего
   move-тика ≤ 8 тиков.
3. **merge-расписание**: ваниль `tickCount % (posChanged ? 2 : 40)` →
   `(tickCount+id) % (posChanged ? 8 : 40)`; стационарные items сохраняют
   ванильный 40-тик период (только де-синхронизация глобального burst-а по
   id). Отложка для движущихся ≤ 8 тиков (ваниль 2).

## Кандидат-кэш (обязательная часть вектора)
`ConcurrentHashMap<selfId, WeakReference<ItemEntity>>`: последний валидный
merge-партнёр перепроверяется БЕЗ AABB-секции-скана (alive, !removed, тот же
level, isMergable, intersects(scanBox), walls-clip при конфиге). Инвалидация:
смерть/remove партнёра, смена level, уход из scanBox (смена секции), размер
> 2^14 → амортизированный sweep мёртвых weak-refs. Потокобезопасно
(region_threads=4 в банке v4 тикает ItemEntity из нескольких region-воркеров).

## Замер базлайна → усиление (RESEARCH-B.md)
cpu-collapsed.txt run 35528326290: merge-путь ≈ 0.04% ItemEntity.tick
(≪ половины) → по ТЗ stagger распространён на move/collide/noPhysics/второй
fluid-скан стационарных items (дельты 1+2). Апстрим-прецедент: Pufferfish DAB
(max-tick-freq), Paper/Purpur rest-gate %4; superiority-отклонение:
продление %4 → %8 + гейтинг ungated проверок.

## Гейт и паритет
- Активен ТОЛЬКО при `System.getenv("CRUSSTY_LEVER_FLAG") == "items_stagger"`
  (двойной гейт: Rust не регистрирует byte hook и не определяет бридж —
  пустой флаг = точный vanilla путь, байт-в-байт).
- `CRUSSTY_LEVER_ARG=1` → preset: MOVE=8, MERGE=8, cache=on; arg 2..8 =
  интервал (жёсткий кап 8 по контракту ≤8 тиков).
- Пары/пикап/деспавн: те же правила (Objects.equals(target,target),
  areMergable, callItemMergeEvent(other, self), merge(stack,other,64),
  pickupDelay=max, age=min, discard(MERGE/DESPAWN), ItemDespawnEvent →
  age=0 при отмене) — сверено с байткодом offsets 595..823.
- Результаты те же, момент свершения отложен ≤ 8 тиков (см. выше).
