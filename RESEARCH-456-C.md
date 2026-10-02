# RESEARCH-456-C — chunk моно-плоскости маржа-аудит (закон 8) + ServerChunkCache scheduling-плейн

Task ID: TASK-456-C · Agent: C2 · Base: origin/master fd790a17 (= f44a831e chunk-composition cert + docs/hygiene) · Дата: 2026-09-25

## (а) Плоскости серт-мержа f44a831e и их индивидуальные дельты (ABSORB/GOAL ×444–×455)

Серт ×455 = diet-carrier 47ea8b20 [ins4 ⊕ senseins ⊕ chunk4-send] ⊕ chunk-union 797ae4f0 [chunk5-encode ⊕ chunkparse ⊕ noise-GEN], STRICT-OR.

| Плоскость | Lever | Индивидуальные дельты (история) | Маржа в юнионе master |
|---|---|---|---|
| chunk4-send-snapshot (PlayerChunkSender) | cmp437_chunk4 | соло-банки +24.7/+7.3/+9.1, пары +8.0 (golden ×449), +13.0 (юнион ×450); cycle-4 банк +11.7/+9.4 | **+0.5–1.5пп** (send-path diet; перекрытие с chunk5 hit) |
| chunk5-encode (ClientboundLevelChunkWithLightPacket) | cmp444_chunk5 | пара +2.2 (×444); encode-hit-rate REFUTED (hit==chunk4-hit), MISS-diet <1pp плацебо (×451-C), players_packets lane 0.00–0.01% FLAT на всех ногах ×452 | **≈0** (STRICT-OR пассажир; не тянет вниз — серт ×455 доказал) |
| chunkparse (SerializableChunkData section-codec) | cmp420_chunk2 | parse AT CEILING (оба горячие лямбды покрыты, 63ea5451); диета-ноги ×454: broadphase −5.7..−6.0пп, лейн-сигнатура ×8 | **+1–3пп** TPS (через диету/broadphase-эффект), в мастере |
| noise-GEN (improved_noise + noise_fill) | on-by-default | fillFromNoise **0.0%** в соаке ×17 ранов ×3 раунда (фикстура прегенерирована); boot Done 16.6–17.2s PARITY; wgen-ось ЗАКРЫТА честно (×445) | **0 в soak-TPS**; закон-8 boot-маржа (уже в мастере) |

Субаддитивность ×3 канон подтверждена: серт-композит +20.0 min-of-3 (chk455-2 +18.7 norm ↔ a19/a21/a32), при этом диета-сертификат (+22.6) субсумирован тем же мержем. **Ни одна моно-плоскость chunk-семьи индивидуально не даёт ≥+20%**: максимум chunk4 (+24.7 банк непарно, пары +8.0/+13.0), chunk5 ≈ 0, parse в потолке, noise = 0 в соаке. Ре-соло моно-плоскостей = строго доминируется мастером → **REFUTED как основной вектор**.

## (б) ServerChunkCache scheduling профиль на новом мастере

Ваниль-профиль ×455/×456 (round-a3/a7/a13/a25/a32, band 6.0–9.5M):
- bucket `chunk system (kernel)`: **8.0–8.5%** self (ваниль) → **7.6%** на серт-ноге chk455-2 (−0.4..−0.9пп — съедено мержем).
- Состав scheduling-слайса (ваниль): `ServerChunkCache.getChunkNow` 0.6–0.8% self + `ConcurrentLong2ReferenceChainedHashTable.getNode` 1.2% (разделяется с holder-lookup) + `ServerChunkCache$$Lambda` 0.8–1.0% (JIT-лямбда) + фазы `chunk system (off-main worker)` 0.9–1.4% + scheduling-хвосты tickChunks/broadcast → **суммарно 4.6–5.2% wall** (канон BOTTLENECK ×456).
- Entity-slices часть (getEntities 3.2–3.7 + getHardColliding 0.7) — senseins/eindex территория, не scheduling.
- Потолок идеального устранения scheduling-слайса: **≤+5% TPS**; реалистичный (fast-path getChunkNow, ~50–70% устранения probe-ветки): **+0.7–1.5пп маржинально на носителе**. Per-call JNI на getChunkNow (~10⁵–10⁶ вызовов/тик от 150k сцен) = закон-6 дизайн-ошибка → единственный честный механизм = body-redirect getChunkNow→ChunkSchedOps (машина `redirect_method_body_to_static`, прецеденты BLOCKUPD/NAVPLANE/NAVPOOL) + direct-mapped shadow (Java) + Rust-зеркало ключей (chunk-граничный JNI на setFullChunk — event-granular, как parse/send; 0 событий/тик в соаке).

## (в) GO/NO-GO

- **Моно-плоскость** (ре-соло chunk4/5/parse/noise): **NO-GO** — суб-бар индивидуально (см. таблицу), юнионы едят дельту.
- **Scheduling-плейн соло**: **NO-GO для бара ≥+20%** — потолок ≤+5% TPS. Не_REFUTED как ЭФФЕКТ: положительная маржа есть (+0.7–1.5пп), значит ветка ЗАКОННА (BOTTLENECK ×456: ветка только при положительной марже).
- **STRICT-композиция на новом мастере**: **GO** — канон ×455: leg с carrier-флагом несёт полный стек мастера (серт +20.0 min-of-3: пары {+20.4, +20.4, +20.0}) ⊕ маржинальный scheduling-плейн (+0.7–1.5пп). Master-серт был бордлайн (+20.0), добавка двигает пары в +20.5–22 → min-of-3 ≥+20 надёжнее. Исполнение: `cmp456_chunkmono` carrier, STRICT-OR иглы во все гейты (ins4/senseins/diet/chunk4/5/parse/noise + prod-гейты и тест-хелперы синхронно — mirror-drift урок ×452).

## Имплементация (cmp456_chunkmono — chunk6-sched)

1. `chunksched/net/minecraft/server/level/ChunkSchedOps.java`: `getNow(ServerChunkCache,int,int)` = javap-вербатим ваниль `getChunkNow` (fullChunks.get + PlatformHooks.hasCurrentlyLoadingChunk ветка + NewChunkHolder getCurrentlyLoadingChunk путь) + direct-mapped shadow fast-path (release/acquire, fail-open при drift) + mirror-инструмент `onSetFullChunk` (shadow put/del + JNI mirrorPut/mirrorDel) + selfTest + EFFECT-маркеры.
2. `src/chunk_sched.rs`: гейт CRUSSTY_LEVER_FLAG STRICT-OR (мастер-список ∪ cmp456_chunkmono); patch = `redirect_method_body_to_static` ×2 (getChunkNow → Ops.getNow; moonrise$setFullChunk → Ops.onSetFullChunk); Rust-зеркало u64-ключей (open-address), drift-guard, маркеры.
3. STRICT-OR иглы: rust ~20 файлов + java Ops-флаг-листы + run_world3.sh + check_blobs_sync needles (sync с тест-хелперами).
4. Пустой lever = ваниль бит-в-байт (гейты не регистрируются, пачки не применяются).
