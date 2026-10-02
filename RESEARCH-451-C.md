# RESEARCH-451-C — chunk parse/send/network срез, cycle-3 (закон 3)

TASK-451-C, ветка round-451c-chunk = origin/master (b5baf548) ⊕ round-450c-chunk
(201d9d66, юнион cmp450_chunk) → merge 02426ed5. Гейты merged-дерева:
check_blobs_sync ALL IN SYNC (flat==nested, cmp450_chunk needles ×2 в явном
списке + raw-cp ×3: ChunkSendOps/ChunkPacketEncodeOps/ChunkParseOps),
cargo check OK, cargo test **317/0** (target /tmp/c451-target удалён немедленно).

## 0. Банк линии к cycle-3 (6 ног cmp450_chunk, все threw=0, AIOOBE=0, ARM ×3 плоскости)

| нога | run | runner | TPS | norm | вердикт |
|---|---|---|---|---|---|
| chunk-1 | 36053633635 | 7338218 | 2.70 | +15.2 | банк (Δ176k) |
| chunk-2 | 36053645579 | 6774396 | 2.60 | +16.8 | **ПАРА +13.0** ↔a4 Δ49k |
| chunk-2r1 | 36053671041 | 8608618 | 2.60 | −0.5 | parity (high-зона) |
| chunk-3r1 | 36054213318 | 7141084 | 2.50 | +8.6 | **ПАРА +6.7** ↔a20 Δ20k |
| chunk-4 | 6563211 | 2.50 | +14.6 | банк, пара +4.4 ↔a8 (абсорб главного агента) |
| chunk-5 | 7113409 | 2.55 | +11.0 | банк, пара +9.1 ↔a20 (абсорб главного агента) |

Best pair **+13.0 < +20** — закон 3: цикл продолжается.

## 1. Что осталось в срезе после 5 отсечённых техник (cycle-2 инвентарь, неизменно)

Отсечено цензами ×450 (RESEARCH-450-C §5/§9, не ре-открывать):
RegionFile-buffering REFUTED (one-shot reads + off-main), write-batch PLACEBO,
cap-raise REFUTED (working set 400-600 ≪ 2048), network deflate-once REFUTED
(off-main + hash≈deflate), serialization residues LOW-POTENTIAL (<1-2пп).
Законом 5 запрещено: всё из ban-списка (ZGC, alloc_diet, …) — вне среза.

Остаток лейна на зелёных ногах: chunk system (kernel) 8.7% CPU, network 2.2%,
paletted 6.41→5.2-6.3 (parse-плоскость уже сняла ~1пп). Остаток ядра лейна =
moonrise scheduling + IO-воркеры (PrioritisedQueueExecutorThread, off-main) —
т.е. то, что НЕ сидит на main-тике и не может дать TPS-пару в этой фикстуре.

## 2. Кандидаты cycle-3 (мандат) — вердикты

### (а) send-snapshot revision-окно диета — REFUTED (parity/players-visible)
Механика: HIT пути chunk4 = `snap != null && !chunk.isUnsaved()`. Любая диета
окна (сужение частоты rebuild / расширение переиспользования на unsaved-чанк /
батчинг sendNextChunks MIN/MAX_CHUNKS_PER_TICK) либо (i) отдаёт устаревший
пейлоад изменённому чанку = players-visible стейл (парити-стена, vanilla
строит свежий пакет на КАЖДУЮ отправку), либо (ii) трогает пакет-пейсинг =
players-visible ось, запрещено javap-цензом ×450. HIT-статистика ног
(fixture-normal, идентично зелёным chunk4-ногам ×443): first-hit маркер в
join-бурсте не печатается → окно и так исчерпано тем, что парити позволяет.

### (б) encode-cache hit-rate диета — REFUTED (hit-rate == chunk4 hit-rate; MISS-диета ≈ плацебо)
chunk5 ключ = packet INSTANCE; попадает ровно тогда, когда chunk4 отдал тот же
инстанс (hit-path требует !isUnsaved — см. (а)). Расширение ключа на контент
требует хэша пейлоада ≈ самому encode (та же арифметика, что REFUTED
deflate-once ×450: cost ≥ benefit, C-JNI без профита). Оставшаяся MISS-диета
(кодировать сразу в channel-буфер + один read-back вместо scratch+replay,
−2 копии ~КБ на свежий send) упирается в тот же (а): MISS-объём в фикстуре =
окно unsaved, а HIT-доминантный путь уже copy-минимален. Потолок < 1пп ≪ шум
окна ±5-8пп — пара недоказуема (канон анти-плацебо ×445/×450), ре-серт
носителя ради недоказуемого = риск без мандатной отдачи. LOW-POTENTIAL.

### (в) section-cache parse расширение покрытия — AT CEILING
Юнион уже покрывает ОБЕ section-лямбды (block_states deep cache cap 16384 +
biomes-parse cache — ChunkParseOps, общая CACHE с per-codec inner). Остаток
parse = heightmaps/entity-NBT/light (REFUTED ×450 как <1-2пп) и
PalettedContainer.copy() на HIT (общий инстанс = коррупция: vanilla отдаёт
свежий мутируемый контейнер, чанк-система мутирует после парса). Полный
Rust-парс SerializableChunkData = подсистема СЛЕДУЮЩЕГО тика (закон 6:
per-function вектор запрещён как основной). Расширять нечего.

### (г) плотность носителя — ПРИНЯТО (canon ×449/×450)
6 ног кластеризуются 6.56-8.61M с банками +11.0..+16.8 raw. Якорная решётка
здоровых (×450: 26 валидных) кластеризуется 6.4-6.8M и 8.3-8.9M; решётка
×451 (round-451-anchor-*, 12 ног + ins4-хвост) идёт параллельно прямо сейчас.
**Арифметика потолка pair**: лучший norm линии +16.8 (chunk-2), лучший
здоровый якорь решётки −1.5 (a44)/−0.4 (a1) → предельный pair из ТЕКУЩЕГО
банка = +18.3 < +20. Следствие: сертификация возможна ТОЛЬКО через
монстр-хвост распределения (legs ≥ +18.5 norm при якоре ≤ −1.5, или ≥ +21.5
при якоре ~+1.5; у ins4-семьи хвосты +20.5..+22.9 наблюдаются регулярно, у
chunk-линии хвост пока +16.8 — хвосты существуют: chunk-1 raw +15.2 в дыре
решётки 7.2-8.2M, chunk-2 +16.8). Каждая доп. нога = (1) шанс монстр-ролла,
(2) шанс Δ≤50k к растущей решётке (×451-якоря падают в т.ч. в mid-зону).
Нулевой код-дельта = нулевой ре-серт риск на 6×зелёном носителе (урок ×449:
лотерея бьётся плотностью; ×450: ≥12 якорей/волну, окна-прицелы).

## 3. Решение cycle-3
**batch-3 = 3 density-ноги СЕРТИФИЦИРОВАННОГО носителя cmp450_chunk
@02426ed5 (код = 201d9d66 бит-в-байт по kernel/блобам: master-хвост b5baf548
= docs/hygiene, нулевой код-дрейф — гейты merged-дерева это подтверждают)**:
round-451c-chunk-6/-7/-8 ← round-451c-chunk, lever=cmp450_chunk lever_arg=1,
inputs = банк-канон (radius 640, seconds 300, fake_players 4, fluid_guard 1,
gc_tune 3, inside_cache 1, flush_diet 1, region_threads 4, batch_collector 1,
population 150k/42, xmx 10G/xms 4G, band 6.0-9.5M). Диспатч —
dispatch_451c.py (argv-guard + --dry-run + --no-batch канон ×447, ancestry-пин
201d9d66), ноги в хвосте очереди CI (решётка главного агента 16 ранов 06:5x
идёт впереди). Пары считаю против round-450-anchor-* (26 валидных) И
round-451-anchor-* (свежие, абсорб по мере прилёта), Δ≤50k pair-fresh,
депресс-гейт norm≥−2.

Если batch-3 тоже <+20 pair — закон 3 исчерпывает бюджет тика: ветка
round-451c-chunk + ноги передаются ×452 вместе с двумя подсистемами
следующего тика: (1) полный Rust-парс SerializableChunkData (parse NBT→
ProtoChunk целиком, закон-6-чистая подсистема), (2) moonrise scheduling
резервы (ServerChunkCache 4.6-5.2% сэмплов). Честный потолок среза в этой
фикстуре фиксируется как: банки +11..+17, пары +4.4..+13.0, хвост +16.8.

## 4. Cycle-3 вердикты (заполняется по абсорбу — см. §5)
