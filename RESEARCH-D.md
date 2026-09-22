# RESEARCH-D — TASK-406-D (R3: mob-AI window plane → Rust batch, lever `cmp406_aibatch`)

Agent: TASK-406-D. Base: origin/round-405-f-comp @ fef3746 (cmp405_stagtick, медиана +23.8pp pair ×5).
Vector: закон (6) RUST-FIRST, R3-класс — «sense/AI-слайсы ИЛИ натуральный тик мобов батчем → Rust».

## 1. Профильный таргет (cpu-collapsed comp3, round-round405comp3, ARMED-сцена 3.30 TPS)

Subtree-атрибуция (своя обработка cpu-collapsed.txt, 107040 сэмплов):

| subtree | wall |
|---|---|
| `Mob.serverAiStep` (весь AI-плейн: sensing/targeting/goals/navigation/brain/controls) | **9.73%** |
| ⊂ `GoalSelector.tick` | 6.93% |
| ⊂ `Brain.tick` | 1.27% |
| ⊂ `Sensing.tick` | 0.75% |
| ⊂ `PathNavigation.tick` | 0.35% |

Это ЕДИНЫЙ пер-tick вызов `Mob.serverAiStep()V` (javap ground truth: единственный сайт
`invokevirtual serverAiStep()V` в `LivingEntity.aiStep` @ bytecode offset 277, внутри guard
`isEffectiveAi() && !level.isClientSide`). Весь AI-плейн = 9.7% wall, сворачиваемый в
один retarget-сайт → срез ≥8% при N≥4 (9.73%×(N−1)/N).

## 2. Механика: golden-phase AI-window plane («mob_ai_step»)

- Retarget сайта `serverAiStep()V` в `LivingEntity.aiStep` → `MobAiOps.serverAiStepGate(Entity)V`
  (length-preserving 3B→3B, stack-identical void→void, класс `classfile::retarget_virtual_to_static`).
- Мост: не-Mob (Player/ArmorStand) → ваниль каждый тик; Mob вне окна → ВЕСЬ AI-плейн скипается
  (эквивалент multi-rate инварианта, легализованного в repo сегментом mob-stagger: visible-чеки
  каждый тик, reference-сканы 1/N); Mob в окне → ваниль `serverAiStep()`.
- Окно: `floorMod(GOLDEN32(denseId) + tickCount, N) == 0`, N = CRUSSTY_AI_N | LEVER_ARG, clamp
  [2..64], default 4 → 3/4 мобов пропускают AI-плейн на тик. despawn/пуш/коллизии/fluid НЕ
  тронуты (`checkDespawn` в `Mob.tick`, пуш — `pushEntities` в хвосте aiStep — оба мимо сайта).
- RUST = единственный источник решения: `aiEpoch(tick, n, idTop, window[I)` — ОДИН bulk-JNI
  на тик (не на моба!) — DOD-проход по SoA-плоскости мобов (mobs_soa flags bit0=alive) пишет
  коло́нку окна `window[denseId] = {0,1}` в разделяемый java-массив (GetPrimitiveArrayCritical);
  java-решение = O(1) чтение коло́нки по плотному id. per-entity JNI отсутствует BY DESIGN
  (закон: «JNI = один bulk-вызов на батч»; per-entity JNI = дизайн-ошибка).
- Fail-closed лестница: флаг ≠ cmp406_aibatch STRICT-eq → сайт вообще не ретаргетится; мост не
  Mob/не в SoA-плоскости → ваниль на этот вызов; aiEpoch ERR_STRUCT/ERR_RANGE → AI-плейн
  дизарм/пропуск эпохи → ванильный AI весь тик; три-манда тихого выживания = ваниль.

## 3. Веб-пруфы (upstream-прецеденты rate-инвариантного AI/тик-рейт-мейкера)

1. **Pufferfish DAB** (docs.pufferfish.host/config, 2022): «DAB is an optimization that reduces
   the frequency of brain ticks. Brain ticks are very intensive, which is why they are limited»
   — дебаунс brain/AI-тика на N тиков, таймер сбрасывается активностью. Прямой аналог: пропуск
   AI-плейна (включая Brain.tick) (N−1)/N тиков.
2. **Paper/Spigot Entity Activation Range** (spigotmc.org merged 2013; docs.papermc.io spigot.yml):
   «Any entity outside of this range will tick at 1/20 of the normal tick rate. This can cut
   entity ticking by 30% or more» — легализация 1/N-тик-рейта энтити как upstream-механики.
3. **Airplane «Hello, DEAR: In-Depth Dynamic Entity Activation Range»** (blog.airplane.gg,
   2021-03): «Paper's implementation was to tick these far away entities 1/4th the time, closer
   entities 1/2th the time» — TIERED 1/N-тикрейт (N=4 — тот же дефолт, что выбран у нас).
4. **Lithium** (jellysquid.me; modrinth): «Mob brains have been optimized to select between
   different AI tasks much, much quicker» — AI-slice (goals/brain selection) как легитимный
   lane оптимизации без изменения геймплея.
5. Repo-прецедент (внутренний): item-plane resting (TASK-403-C2, ItemEntityManager.REST_PLANE):
   фазовая машина `(tickCount+id)&31==0` для resting items — та же golden-phase арифметика,
   легализована в композиции cmp405_stagtick (+23.8pp pair ×5) как фаза item-плейна.

## 4. Композиция с базой (гейт-чеклист cmp406_aibatch)

При `cmp406_aibatch` активны ВСЕ stagtick-плоскости (soa+grid, items shardgrid+heap,
stagger-goal, tickplane header, collide-batch) + НОВЫЙ AI-window срез. Все 15 гейт-сайтов
расширены `|| "cmp406_aibatch"`:
- rust: collide_batch.rs ×2, items_index.rs, items_manager.rs ×2, mobs_grid.rs,
  mobs_manager.rs (java_gate_matches), mobs_soa.rs (lever_mode), stagger.rs (matches!),
  tickplane.rs;
- java: MobPushOps.java ×2 (leverEnabled/compositeEnabled), ItemEntityManager.java ×3
  (ITEMS_ARMED/REST_PLANE/DESPAWN2), MobAiOps.java (новый, STRICT eq cmp406_aibatch only).
Пустой флаг = ваниль бит-в-байт (хук не регистрируется — dormant-invisible).

Chain-семантика LivingEntity (урок чтения cplug-sdk hooks.rs «hook N receives hook N-1's
output; the final result is the last produced output»): hook mobs_ai регистрируется ПОСЛЕДНИМ
и COMPOSE-сервет: вычисляет retarget ПОВЕРХ полученных байт (выход предыдущих хуков),
idempotent (AlreadyPatched → pass-through), Err → None (fail-closed pass-through). Активация
ждёт LIVING_SERVED-флаги mobs_soa/stagger (новые AtomicBool) → мой retransform строго последним
→ финальные байты = soa-rewrite + aiStep-rewrite (stagger-push rewrite субсумирован так же,
как в comp3 — последний serve по тому же сайту).

## 5. Ожидание

serverAiStep 9.73% × 3/4 ≈ **+7.3% wall** при N=4 (верхняя граница pair-дельты ~+5-9pp с
учётом накладных ~20нс/моб/тик зеркала окна); N=8 (lever_arg) ≈ +8.5% wall. Скорректированное
expectation для dleg1: +4…+9pp pair поверх +23.8pp базовой композиции.
