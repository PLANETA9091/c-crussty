# RESEARCH-459-CX3 — chunk-serial LRU decompress-arena (WILD-агент закона 11, тик-459, СВОЯ идея C-X3)

Ветка: `round-459-cx3` @ origin/master 0d147876. Агент TASK-459-82 (закон 12e: анализ + wiring + dispatch + write-through, финал = {run id, ветка+SHA, вердикт-число}). Идея СВОЯ (не карточка): при серийном чтении чанков держать per-thread LRU-арену разжатых секций (последние N секций), повторные чтения (topup/соседние чанки) — без re-decompress; CRC32-контроль байт. STRICT dormant.

## Источники (≥2, факты по каждому)

1. **https://github.com/RelativityMC/C2ME-fabric/issues/419** — «Add Zstandard compression option for chunks» (C2ME). Факт: chunk-сжатие — признанный lane мировой генерации/загрузки (C2ME поднимает IO+decode как отдельный контур из воркеров); обсуждение подтверждает: decompress-стоимость в load-фазе существенна и её амортизируют переиспользованием контуров/буферов. Взято: целевая lane = region-read decompress на IO-воркерах, а не main-tick.
2. **https://github.com/facebook/zstd/blob/dev/doc/zstd_manual.html** — zstd manual. Факт: переиспользование decompression-контекста (ZSTD_createDCtx → ZSTD_decompressDCtx многократно) — канонический способ амортизировать setup/context-расход; контексты НЕ потокобезопасны → один DCtx на нить. Взято: per-thread арена (thread_local) как носитель, никакой синхронизации на hot-path.
3. **https://minecraft.wiki/w/Region_file_format** — формат региона (curl 403 bot-guard, страница живая). Факт: регион = 4KiB-сектора; loc-таблица 1024×(3B offset + 1B count); payload чанка = 4B длина + 1B compression-type (1 gzip / 2 zlib / 3 none / 4+ zstd) + паддинг. Взято: серийные соседние чтения бьют в ТОТ ЖЕ файл и близкие сектора → serial-паттерн реально повторяет (chunk → соседние) пары чтений; epoch = версия файла (mtime) для инвалидации.
4. **https://github.com/PrismarineJS/prismarine-provider-anvil** — независимый anvil-ридер. Факт: read-пайплайн везде один: loc-entry → read секторов → inflate → parse; инфляция — отдельный шаг ДО parse, значит кэшировать можно именно РАЗЖАТЫЕ байты между чтениями без риска несовпадения с parse-контрактом. Взято: граница кэша = post-inflate байты (не parse-результат) → паритет бит-в-байт тривиален.

## Механика (law 6: vanilla decompress остаётся; арена НАБЛЮДАЕТ put, HIT только с CRC)

1. **Арена (rust, `src/chunk_serial_arena.rs`)**: per-thread `thread_local!` LRU из `ARENA_SLOTS=12` слотов (бюджет 512KiB/нить, oversized → cap_dropped, не кэшируем). Ключ = pack(cx: i32, cz: i32, sy: u8) → u64; слот = {key, epoch, crc32IEEE, stamp, Vec<u8>}. Put — post-inflate java-стороной; Get — перед inflate; LRU = min-stamp evict, touch на hit.
2. **CRC-контроль байт**: put пишет CRC32IEEE(payload); get пересчитывает CRC по факту выдачи — mismatch → CRC_FAIL → слот отравлен → MISS (ваниль). Никаких доверий арене.
3. **Epoch-гейт**: epoch (версия региона-файла, java) не совпал → MISS — устаревшие байты не обслуживаются.
4. **Гейт/lever**: STRICT eq `cmp459_cx3`; natives регистрируются ТОЛЬКО rust-регистрацией (`register_arena`), java one-shot `arenaArmed()` — единственный путь включения (NCDFE-канон: define-ДО-arm). Lever пуст → класс не дефайнится → ваниль бит-в-байт.
5. **Fail-closed**: любая структурная ошибка (range/out-буфер меньше payload) → BROKEN one-shot latch → все ответы MISS → ваниль навсегда (mobs_soa ERR-лестница). Арена НИКОГДА не ломает read.
6. **Scaffold-консерватизм**: JNI `chunkSerialArenaGet` в скэфолде отвечает консервативным MISS (parity by default) — serve-путь из арены включается ARM-ногой вместе с retarget read-сайтов (javap-декомпиляция обязательна). `chunkSerialArenaPut` реален: кладёт байты, счётчики живые → DATA-PLAN первой ARM-ноги.

## Lane и capture-матем (числа)

- Тиковые профили (chkmono457-14 wall 63661 сэмплов) — grep `inflat|decompress|RegionFile` = **0 сэмплов** в steady-tick (есть только AABB.inflate — нерелевантно): в steady-состоянии region-IO lane пуста, эффект — в **load/topup-фазе** worldbench (boot radius 640 → population_target 150k, серийные region-чтения на IO-воркерах).
- Оценка lane (источники 1/3): decompress-часть load-фазы ≈ **2-4%** CPU load-окна на IO-нитях (zstd/сжатие чанков — признанная нагрузка загрузки; серийные повторные чтения соседей из того же региона реальны: 1024 loc-записей/файл, сосед по XZ = тот же файл в ~75% случаев).
- Capture: повторные чтения того же (chunk, section) в load-окне (topup-волны, повторные promote-попытки,/entity/poi довнгрузки соседей) — консервативно **20-40%** decompress-вызовов.
- **Δ-прогноз: lane 2-4% × capture 20-40% → Δ +0.4-1.6пп на load-фазу; на 300s-TPS-норме (разбавление boot-фазой) +0.1-0.4пп.** Потолок = вся decompress-lane load-фазы ≈ +2-4пп. Оценка до первой ARM-ноги; первая ARM-нога обязана снять hits/misses с DATA-PLAN счётчиков (GETS/HITS/MISSES в stats()) — превратить оценку в измерение.

## Риски / гейты (preregistered)

- R1 повторное чтение ≠ те же байты (файл перезаписан) → epoch-гейт + CRC-гейт: FN→0 по построению, FP невозможен (mismatch → MISS).
- R2 память: 12 слотов × ≤42KiB × число IO-нитей (4-8) ≈ ≤4MiB worst-case, кап 512KiB/нить — GC-нейтрально (Vec<u8> вне java-кучи).
- R3 спящий гейт (урок-408): ARM-маркер обязателен — stats() + stdout `cx3-arena: cmp459_cx3 ARMED`; jar-sha сверка как у roar-2.
- R4 NCDFE: define ChunkSerialArenaOps ДО RegisterNatives (канон); сам класс грузится в selftest без нативов → main() кидает только при ложном ARMED.
- G1 lockstep бит-в-байт: HIT-байты == ванильному decompress (selftest INV2/INV6 + оракул offline-харнесса на реальных region-файлах); G2 young/Full GC не изменён; G3 pop-паритет 140-165k; G4 min-of-3; G5 fail-closed disarm (BROKEN latch); G6 ARM-маркер в stdout.

## Статус scaffold (12e)

`src/chunk_serial_arena.rs` (арена+LRU+CRC32+JNI+selftest, lever STRICT eq) + `chunkparse/net/minecraft/world/level/chunk/storage/ChunkSerialArenaOps.java` (dormant-stub: гвард one-shot, нативы, wiring-план) + `src/lib.rs` wiring (mod). STRICT dormant: без lever cmp459_cx3 класс не дефайнится, natives не регистрируются, get отвечает MISS — ваниль бит-в-байт.
