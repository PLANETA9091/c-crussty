# LEVER-A — TASK-400-A: КОМПОЗИТ B+F (cmp399_bfcomp)

## Семантика
`CRUSSTY_LEVER_FLAG=cmp399_bfcomp` = одновременное армирование ДВУХ ортогональных векторов поверх J-base (86807b6):

1. **B — shard-grid** (round-399-b-shardgrid @ 22f1982, ранее +8.7%/+9.8% реплицировано):
   item-индекс `src/items_index.rs` в SHARDED-режиме — 64 шарда по `mix64(cell_key)&63`, lock-free per-cell read (v1→данные→v2 Acquire-пара, cell-local retry, ERR_RANGE per-call fallback), писатели под глобальным writer-mutex (линейность = legacy RwLock), фиксированная память (16384/шард, 1<<20 id, fail-closed, без realloc).
2. **F — lifetime-heap batch-despawn** (round-399-f-despawn2 @ 53945af, ранее +6.0%/+6.0% реплицировано):
   `src/items_lifetime.rs` — rust min-heap `(due,id)`, natives `lifetimePush([JI)I` (батч-пуш, 1/тик) + `lifetimeDue(J[J)I` (drain, 1/тик), java-флоу деспавна строго ванильный (callItemDespawnEvent → cancel? re-push : discard), stale-early re-push после merge.

## Гейты (единый флаг, семейство cmp399_*)
- rust `items_manager.rs`: family-gate `items_subsys2 || starts_with("cmp399_")` → cmp399_bfcomp входит; ARM-маркер `cmp399_bfcomp: ARMED shards=64 … heap=lifetime-minheap(rust,vec) … (composite B+F)` + post-define `{flag}: ARMED heap=…`.
- rust `items_index.rs::shard_mode()`: exact `cmp399_shard || cmp399_bfcomp`.
- rust lifetime-нативы регистрируются всегда при define (gating java-стороной DESPAWN2 — маркер heap= печатается только при despawn2-флагах: cmp399_despawn2/cmp399_bfcomp).
- java `ItemEntityManager`: `ENABLED = "items_subsys2".equals || startsWith("cmp399_")`, `DESPAWN2 = "cmp399_despawn2".equals || "cmp399_bfcomp".equals`. Класс ПЕРЕСОБРАН javac --release 21 из merged-исходника (19142 байт) и закоммичен; runtime CP-патч (patch_utf8_gate) остаётся только на legacy-пути cmp399_shard (байт-паритет A/B); для cmp399_bfcomp класс self-armed, патч не требуется (проверено тестом `gate_patch_swaps_exactly_one_utf8`: "items_subsys2" в пуле ровно 1).
- J-подсистема (items_subsys2 family-gate) активируется автоматически — композитный флаг ∈ семейства cmp399_*.

## Паритет
Структура/порядок/кандидат-сеты merge = ванильные (B), despawn/pickup/ItemDespawnEvent = ванильные (F). Оба суб-вектора A/B-двухрежимны по построению: `items_subsys2` → legacy-путь байт-в-байт, без флага → 100% ваниль. Никаких новых retarget'ов kernel-классов; регион-фаза/мерджи/деспавн/пикап — те же, что в B и F по отдельности.

## Проверки этого рана
- merge origin/round-399-f-despawn2 → 2 конфликта (src/items_manager.rs, ItemEntityManager.java) решены вручную: сохранены ОБА поведения (B: GATE_*+lever_flag_matches_for+CP-патч; F: DESPAWN2-гейт+heap-маркер).
- javac --release 21 PASS (classpath: patched-kernel.jar, fastutil.jar, paper-api.jar, adventure-api/key, examination-api-1.3.0).
- cargo check --lib PASS; cargo test --lib gate_patch PASS (202 отфильтровано).
