# RESEARCH-H — TASK-401-H composite (round-401-h-comp)

## Vector
КОМПОЗИТ: bfcomp (shardgrid+lifetime-heap, cmp399_bfcomp) + devirtfix (D1+D2, cmp399_devirt)
+ mobpush (mobs_grid, cmp399_mobpush) под ЕДИНЫМ композитным флагом `cmp401_comp`.

## Merge map (base origin/master d26f524)
1. `origin/round-400-a-bfcomp` (fbb6251) — конфликт src/lib.rs (mod item_merge vs mod items_*):
   разрешил СОХРАНИВ ОБА (item_merge = master items_oss-система под своим флагом, не мешает).
2. `origin/round-400-c-devirtfix` (99a0ab4) — конфликты:
   - src/items_manager.rs: gate — оставил family-gate + добавил cmp401_*; маркер "bridge ready"
     заменил despawn2-маркером bfcomp (информативнее).
   - entityinside/.../ItemEntityManager.java: merged = bfcomp-база (despawnv2 machinery, 736 строк)
     + devirt-блок `goalContainsAnyFlags` (D2 redirect target) + гейты cmp401_comp.
   - src/classfile.rs: конфликт на хвосте — СОХРАНИЛ ОБЕ стороны: patch_utf8_gate (bfcomp,
     легаси cmp399_shard CP-патч путь) + devirt-тройка (patch_synched_data_get /
     patch_goal_contains_flags / devirt_resolution_closure + mod devirt_patchers).
   - RegionTickOps.java: авто — остался lifetimeTick (bfcomp) + devirt-изменения.
3. `origin/round-400-j-mobpush` (e7d083a) — конфликт src/lib.rs (register/activate хвосты):
   разрешил последовательно devirt::register/activate + mobs_manager::register/activate.
   item_merge.rs НЕ удалён (merge-base 22f1982 — deletion не пропагируется на master-side add).

## Флаговая карта cmp401_comp (паритет rust/java — полу-вооружённый мост невозможен)
| Подсистема | Rust gate | Java gate |
|---|---|---|
| items shard-grid (B) | items_index::shard_mode: cmp399_shard ∥ cmp399_bfcomp ∥ **cmp401_comp** | (нет мнения — rust-side mode selector) |
| item-мост/индекс natives (J-base) | items_manager: items_subsys2 ∥ cmp399_* ∥ **cmp401_*** | ItemEntityManager.ENABLED: items_subsys2 ∥ cmp399_* ∥ **cmp401_*** (класс пересобран) |
| lifetime-heap despawn2 (F) | items_manager despawn2-маркер: cmp399_despawn2 ∥ bfcomp ∥ **cmp401_comp** | ItemEntityManager.DESPAWN2: cmp399_despawn2 ∥ cmp399_bfcomp ∥ **cmp401_comp** |
| devirt D1 (SynchedEntityData fusion) | devirt.rs: cmp399_devirt ∥ **cmp401_comp** | (retransform-хук, java-гейта нет) |
| devirt D2 (GoalSelector redirect) | devirt.rs + resolution closure по IM_BYTES | ItemEntityManager.goalContainsAnyFlags — присутствует в пересобранном классе |
| mobpush (J400) | mobs_manager.lever_matches / mobs_grid.lever_mode: cmp399_mobpush ∥ **cmp401_comp** | MobPushOps.leverEnabled: cmp399_mobpush ∥ **cmp401_comp** (класс пересобран) |

ВАНИЛЬНОСТЬ: пустой/чужой флаг → все гейты false → 0 хуков, 0 define, 0 retransform
(наследование от bfcomp/devirt/mobpush fail-closed матриц; тест lever_flag_matches_for("")==false).

## round-400-d-wakefix — РЕШЕНИЕ: НЕ мержить
Корень wakefix = рассинхрон rust-gate (cmp399_*) и java-ENABLED (точный items_subsys2) в OLD
J-base → полу-armed мост → AIOOBE(-1,16385). В композите рассинхрон невозможен ПО ПОСТРОЕНИЮ:
каждая пара rust/java-гейтов — идентичный предикат одного env (таблица выше), java-классы
пересобраны javac под композит. Мерж wakefix (откат rust-gate на eq(items_subsys2)) КОНФЛИКТУЕТ
с family-гейтами bfcomp и ничего не добавляет (wakeup-подсистемы в композите нет).

## Пересборка классов (javac --release 21, класс major 65)
- CP: /tmp/patched-kernel.jar:/home/z/tools/fastutil.jar:/home/z/tools/paper-api.jar:
  /home/z/tools/adventure-api-4.17.0.jar:/home/z/tools/adventure-key-4.17.0.jar
- entityinside/build/.../ItemEntityManager.class: 19142 → **19956 B** (despawnv2 + devirt + cmp401 gate;
  javap-пруф: ldc cmp399_/cmp401_/cmp399_despawn2/cmp399_bfcomp/cmp401_comp + public static goalContainsAnyFlags)
- mobpush/build/.../MobPushOps.class: 8816 → **8845 B** (ldc cmp399_mobpush + cmp401_comp)
- cargo check --lib PASS; cargo test --lib **210 passed / 0 failed** (1 ignored).

## ARM-маркеры композита (server-stdout, обязательны до вердикта)
1. `[crussty-plugin] cmp401_comp: ARMED shards=64 ... heap=lifetime-minheap ... mobpush=sharded-grid devirt=D1+D2 (composite TASK-401-H)` — items_manager
2. `[crussty-plugin] cmp401_comp: ARMED heap=...` — despawn2-маркер
3. `[crussty-plugin] cmp401_comp: devirt stage synched_data_get ARMED (...)` — D1
4. `[crussty-plugin] cmp401_comp: devirt stage goal_contains_flags ARMED (...)` — D2
5. `[crussty-plugin] cmp401_comp devirt: ARMED sites=N` — devirt финал
6. `[crussty-plugin] cmp401_comp ARMED shards=64 ... radius_gate=1.0 ...` — mobs_grid
7. `cmp399_despawn2: lifetime-heap active (enmass=...)` — java LOG (literal-метка подсистемы)

## Pre-existing test debt — исправлено в ветке (2 теста, только тесты, прод не тронут)
1. `classfile::devirt_patchers::synched_data_get_fused_body_shape` (ломался и на devirtfix-базе):
   ассерт ждал placeholder [0,0] CP-индексы, патчер кладёт РЕАЛЬНЫЕ (0x2b/0x8e/0x119). Фикс:
   opcode-shape assert (2a b4 * 2b b6 * be b4 * b0) + члены резолвятся (проверки ниже по тесту).
2. `mobs_grid::tests::overflow_and_span_fallbacks` (ломался и на mobpush-базе): два суб-ассерта:
   overflow = -(cap)=-1 (НЕ -(total): contract подтверждён java grow ×4: new int[(-n)*4]);
   absurd-span ERR_RANGE недостижим через grid_query — MAX_SPAN-гвард живёт в native mob_query
   (тест звал grid_query напрямую). Оба ассерта выправлены по прод-контракту.

## Дальше
- dispatch.py → world-bench-parallel.yml, lever_flag=cmp401_comp, lever_arg=1, inputs РОВНО по пайплайну.
- Вердикт: pair-by-runner vs same-day якоря (35567789116 / 35567805608 / 35567822006) + 6 ног TASK-400.
