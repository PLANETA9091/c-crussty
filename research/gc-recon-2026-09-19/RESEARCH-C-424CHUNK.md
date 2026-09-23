# RESEARCH-C — ROUND-424-C chunk/boot axis (TASK-425-C, round-424-c-chunk @558fd1d)

Мандат: ≥+20% pair-stable min-of-3 ОБЯЗАТЕЛЬНО (владелец 2026-09-23). Ось:
chunk/worldgen (закон 8), игрок-видимая жалоба «че так медленно чанки грузятся».
База: master a8ef2fc + cherry-pick biomes-parse cache (round-423-c-wgen b582bc3).

## 1. Re-verified root-causes (CHUNK_ROOTCAUSE_423C + soak-логи 423-ног, артефакты)

- **RC-B (chunk-send lane = МЁРТВ в soak):** players_packets lane
  (ServerGamePacketListener|Clientbound|PlayerChunkSender) = 12/115655 = 0.01%
  soak-CPU. Fake-players fixture не тянет клиентские chunk-пакеты.
  → **R5b chunk-serialization/send slice в Rust = АНТИ-ПЛАЦЕБО-НАРУШЕНИЕ**
  (ретаргет с 0 горячих сайтов = sites>0 формально, эффект 0; закон
  анти-плацебо-гейта TASK-425(0)). НЕ имплементируем. Честная замена —
  измерение boot-Done (§3).
- **RC-A (GEN-ось инертна):** worldgen/noise bucket 0.03% soak-CPU (мир
  прегенерирован). noise_fill остаётся в юнионе ради boot worldgen tail.
- **RC-F (parse-бурст = boot-window):** 9216 чанков ≈ 2-3s ПОСЛЕ Done;
  biomes-parse cache (cherry-picked, lambda$parse$7) = единственный живой
  chunk-срез + chunk-parse cache (merged) — GC-debt relief в population/ramp.
- **КОРЕНЬ ПРОМАХА cmp423_wgen (+1.2% вместо ≥+20%):**carrier армил
  queryplane/chunk-parse/biomes/noise_fill, но **NOT colpush** —
  colpush.rs lever_flag_matches() STRICT-eq cmp420_colpush → pushEntities
  whole-body redirect DORMANT → MobPushOps.pushables/upsertSelf не зовутся →
  SoA-популяция пустая ×895 тиков (placebo-class #2 тика-423) → items-лейн
  31.17% остался ванильным. Мега-носитель cmp420_colpush (мерж 4ab7306) без
  этого дефекта: pair +25.0/+31.0/+29.5, медиана **+29.5% ≥ бар** (mg legs ×3).

## 2. Вектор раунда: cmp424_chunksend = композиция законов 6+7

**cmp424_chunksend = cmp420_colpush proven set (mega) ⊕ cmp423_wgen set
(queryplane ⊕ chunk-parse ⊕ biomes-parse ⊕ noise_fill).**

Ожидание по замерженным реплицированным числам: mega-медиана +29.5% (colpush ⊕
chunk-parse ⊕ queryplane; mg-ноги vs fresh-якоря 419) + biomes-parse
маргинал + parse-relief ранней рампы. Лейны: items 31.17→~0, nav_ai −4..−10,
broadphase −1.4..−6. STRICT-OR: пустой/чужой флаг = ваниль бит-в-байт (закон 4).

Сайты ретага (union + `|| v == "cmp424_chunksend"` / `f.trim().equals(...)`):
- Rust (21 гейт-сайт): colpush, queryplane(+lever_id), chunk_parse(+selftest
  needle), noise_fill, nav_plane, tickplane, entity_query, mobs_soa ×3,
  stagger, items_manager ×2, collide_batch, items_index, mobs_ai, mobs_sscan,
  mobs_grid, mobs_manager (+ARM-маркер ветка).
- Java (8 сайтов): ColpushOps.FLAG-union, MobPushOps ×2, MobAiOps, MobScanOps,
  EntityGoalQueryOps, QueryPlaneOps, ItemEntityManager ×3, ChunkParseOps
  CARRIER_UNION_424 raw-cp маркер.
- run_world3.sh arming case + check_blobs_sync.sh carrier-маркер.

## 3. BOOT-МЕТРИКА (игрок-видимая, без изменения харнесса)

absorb_boot424.py: парсит `Done (X.XXXs)!` из server-stdout.log абсорбнутого
run-dir + первые 3 soak-полла (ramp) → boot_done_s was→стал в RESULT.json.
was (vanilla anchor422b): **14.784s**; was (cmp423_wgen legs): 16.390/16.678/
12.513 (l1/l2/l3) — шум высокий, метрика фиксируется честно was→стал ×3.

## 4. Прегист вердикта

GREEN = ARM-маркер cmp424_chunksend + ЭФФЕКТ (parse-cache hit / bulk EFFECT /
items-plane) + Retargeted>0 + DATA-PLAN mobSlots>0 (placebo-класс #2 гейт) +
AIOOBE=0 + NCDFE=0 + band 6.0-9.5M + pair-by-runner vs FRESH якоря-424
(35825614835/35825628919/35825641995) + min-of-3 + selfTest==true + javap
flat==nested + polls + boot-Done was→стал. Δ≥+20% → GREEN (мерж не делаю).
Δ<+20% → цикл закон 3: следующий слайс noise-gen core (noise resampling
bulk-Rust per-chunk батч, крейт noise/) + boot-parse глубже.
