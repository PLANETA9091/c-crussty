# AG-243 w529 — SWAR batch-AABB kernel iter-1 (cmp458_swar, DORMANT)
- Гипотеза: SWAR broadphase 8-wide + монотонный SAP-хвост снимет 40-70% java-перечисления pushCandidates → +5..8пп к ноге (lane 8.8-10.9% CPU).
- Ожидание: +5..8пп broadphase-лейн при 47k+ pop; superset-маска STRICT-безопасна.
- Сделал: src/mobs_swa.rs (411→429 строк, DORMANT, pub mod в src/lib.rs): (a) window_bounds — 2×lower_bound бинпоиск по xmin-отсортированному SoA (Box2 f32, f32_ordered_key — IEEE монотонный u32-маппинг); (b) swar_mask8/swar_window_mask — переносимый SWAR 8-wide на u64, x/z-verdicts AND-комбинятся в одном аккумуляторе, без intrinsics; (c) scalar_window_mask — тот же контракт, bit-for-bit; (d) insertion_sort_pass — O(n+inv), возвращает moves для self-heal триггера iter-2.
- Тесты: 4 passed — swar==fallback (100 боксов × 1000 боксов, chunk-for-chunk), superset vs brute-force (SAP-теорема: x-interval overlap необходим), insertion O(n+inv) на 50-дельта-дрейфе, NaN fail-closed (key-сортировка исключает NaN из окна).
- Итог: cargo check 0 errors; cargo test mobs_swa: 4/4 ok.
- iter-2: java-мост swarEpoch (InsideBatchOps-паттерн, третий bulk-JNI в EPOCH_LOCK-окне), NCDFE ARM-AFTER-DEFINE (early define ДО первого пуша, union-widen флага), scalar-fallback mandatory (Box2D канон: AVX2-runtime-detect opt-in только при том же контракте).
- Прим.: bench/p500/java/p500/groups.tsv отсутствует в sparse-checkout воркспейса → cargo test lib падал ДО моих правок; файл извлечён из HEAD только для прогона тестов, в коммит не включён (rm перед add).
