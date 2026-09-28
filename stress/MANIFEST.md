# stress/MANIFEST.md — СТЗ-фикстуры (закон 20a) — Реестр ×479-A10

Реестр стресс-ТЗ фикстур (stress-TZ scenes) для world-bench-parallel через input `world_url`.
Общие правила: ваниль-семантика (закон-5 запреты: пороги/окна v5-FROZEN не трогать, lever ∅
если сцена сама является нагрузкой), zip dirs обязаны быть 0755 (A9-урок Л-478-F9: v3-зип DOA,
dirs 0o600 → unzip нетраверсибельные dirs → preflight «FATAL: level.dat parent has no region/»),
sha256 pin + download-верификация обязательны, прегист закон 14a/16 до диспатча.

---

## СТЗ-1: item-storm «weak-chunks» (Paper #13783) — 260k item-дропов → 1000+ MSPT

- CLAIM (F6-наследие, веб-разведка ×478-F6): 260k item-дропов в одном чанке → 1000+ MSPT
  (Paper #13783 weak-chunks: дропы копятся в weak-loaded чанках, item-tick/merge-search не
  масштабируется). Проверка in-vivo: сцена поверх канона pop150k → MSPT-профиль vs канон
  (canon pop150k spark-avg ≈ 392.89 Y5 / 415.84 Y1 @6.9–7.6M).
- Фикстура: `world479-a10-stz-v1.zip` — sha256
  `d88978f16e0b0de5eae7f3a05e87469c597ff98ebf2496b5143bbb992da76b9c` (71552 B),
  релиз `v479-a10-stz-world` (asset world479-a10-stz-v1.zip), download-верифицирован (sha match).
- База: level.dat из world478-bn-stress-v4.zip (510c8be3, download-verified) — server-born
  DataVersion 4556 / 1.21.10, seed 42, region/ ПУСТОЙ → полный r640 fresh-gen; DIM-1/DIM1
  пустые; datapacks/bukkit/pack.mcmeta сохранён; тяжёлые паки BN/Terralith/BACAP/Structory/
  tectonic УДАЛЕНЫ (осознанно: сцена = item-ось, а не C63 gen-шторм — иначе MSPT не атрибуцировать).
- Датапак `datapacks/stz1/` (pack.mcmeta dual-declaration 48+[48,88] — C61-урок, единственная
  форма, парсящаяся 1.21.10):
  - `tick` → `stz:tick`: gametime → score `#stz_t`; фаза `#stz_p = (#stz_t − 300) % 600`;
    burst активен при `#stz_p ∈ [0,199]` И маркер `stz_probe` жив.
  - `load` → `stz:init`: scoreboard + `summon marker ~ ~ ~ {Tags:["stz_probe"]}` — маркер
    суммонится в world spawn (tick-функции исполняются в позиции world spawn); fail-safe:
    если позиция исполнения невалидна/чанк не загружен — маркера нет → шторм молчит → ран
    вырождается в canon-pop150k (не FAILURE).
  - `stz:burst` → `stz:wave0..9` (цикл 10 волн × 1300 строк = 1300 summons/тик):
    `execute in minecraft:overworld positioned ~-6 ~ ~-6 run summon minecraft:item <dx> <dy> <dz>
    {Item:{id:"minecraft:stone",Count:1b},PickupDelay:32767s}`.
- Числа сцены: инжект **1300 item/тик**, burst **200 тиков × 1300 = 260,000 дропов/бурст**,
  период 600 тиков (200 шторм + 400 холостые; макс. без-штормовое окно 400 тиков < bench-окна
  ~723 тика @canon MSPT 415 → ≥1 полный бурст гарантированно попадает в 300s-окно).
- Ваниль-семантика дропов: Count 1b камень (обычный дроп), гравитация включена (падают),
  Age 0 → despawn 6000т (не достижим в окне), merging — ваниль (pickup-delay 32767s —
  штатный vanilla «неподбираемый» флаг; если блокирует merge — легитимно имитирует
  weak-chunk-накопление без подбора; если нет — merge-чурн сам по себе ось #13783).
  Кастомных NBT вне Item/PickupDelay — 0.
- Геометрия: окно 13×13 блоков вокруг world spawn (offsets −6..+6, y +8..+40; LCG 7919 по
  решётке 13×13×33). Худший случай 2×2 чанка (граница), ожидание — 1 чанк (spawn chunk);
  после падения — до 169 наземных куч. Это концентрация «один чанк» в мере #13783.
- Самоограничение: живые дропы ≈ тики_шторма × 1300; при MSPT→1000+ окно сжимается в тиках →
  alive ≤ ~1M (heap ~0.4–0.5 GB из 10G — безопасно); despawn-хвост после 6000т — ваниль.
- Zip-гигиена (A9-урок): все dir-энтрии `0o40755<<16` (0755 + traverse), файлы `0o100664<<16`,
  testzip CLEAN, 33 энтрии; preflight `[ -d region ]` проходим.
- Диспатч (прегист закон 14a/16: `scripts/dispatch_479_a10_stz.py`): алиас `round-479-a10-stz`
  = HEAD мастера на тик (дельта docs/stress/scripts-only, 0 src/Java/Rust), canon x466-C98
  ЯВНЫМ JSON (640/300s/fp4/gc3/ic1/fd1/rt4/bc1/**pop150k**/seed42/10G/xms4G), band [6.0,9.5]M
  (fast-fail pre-download калибровка — защита от узла, не от сцены), lever ∅ (нагрузка = сцена),
  `world_url` = релиз-ассет v479-a10-stz-world.
- Гейты чтения результата: FIXTURE-VALID (preflight + boot Done), NCDFE=0, AIOOBE=0,
  маркер/шторм: item-entity counts из entity-recon артефакта (>0 = шторм жил; 0 = fail-safe
  canon-ран → REFUTED_CENS по доставке сцены), MSPT-профиль vs канон 392.89/415.84.
  Band-miss по workload-cpu ОЖИДАЕМ (сцена добавляет CPU — это и есть claim) — не ре-ролл-триггер
  (Л201-урок: пост-хок STRICT-фильтр, окно не в гейте). Wall >15 мин → DISPATCHED run-id
  (закон 18-iii).
- H-479-A10 (prereg): burst 260k → item-merge-search + entity-tick поверх 150k-популяции →
  MSPT заметно выше канона (≥1.5× канона = сцена жива); если MSPT ≈ канон при alive>100k —
  находка против #13783 (report честно, вердикт по числам, пороги не трогаем).

| # | world_url | sha256 | bytes | scene |
|---|---|---|---|---|
| СТЗ-1 | https://github.com/PLANETA9091/c-crussty/releases/download/v479-a10-stz-world/world479-a10-stz-v1.zip | d88978f16e0b0de5eae7f3a05e87469c597ff98ebf2496b5143bbb992da76b9c | 71552 | item-storm 260k/burst @spawn-chunk, Paper #13783 |

История: round-477-c94c97-stress — внешние паки (Terralith/tectonic/Structory/towers,
20a-ось worldgen); round-478-f9-bn — BN 6.9.8 стенд (v3-DOA → v4 dirs-0755 фикс, Л-478-F9);
×479-A10 — СТЗ-1 item-ось (этот реестр).

---

## СТЗ-2: chunk-load churn «scheduler×TPS-coupling» (C2ME #457) — forceload 4096-чанковый цикл

- CLAIM (веб-разведка ×479-F2, закон 20b): chunk-loading привязан к TPS/скедулеру — C2ME #457
  (closed, 10c, 2025-07-18): loading rate растёт при `/tick sprint` при том же I/O ⇒ чанк-лоад
  привязан к tick-бюджету, при деградации TPS лоад-рейт падает (feedback-петля). In-vivo:
  периодический load/unload churn сильной загрузки → MSPT-профиль vs канон pop150k
  (canon spark-avg ≈ 392.89 Y5 / 415.84 Y1 @6.9–7.6M).
- Репро-класс: **chunk-load-churn/scheduler** (ось chunk_sched × worldgen-io) — отличен от
  СТЗ ×478 ×3 (weak-chunks item-drops / Folia global-lock / villagers POI) и от СТЗ-1
  (item-ось). Runner-up того же класса: C2ME #423 «Extreme Slowdown with High Chunk
  Loading» (spread-players) — покрыт той же сценой консервативно (без spread).
- Фикстура: `world479-f2-stz-v1.zip` — sha256
  `9b3a3f091941e90dbd22cbf4fc34ee3a2345e2655f108b2c1c14e29fc4c0bb9f` (19861 B),
  релиз `v479-f2-stz-world` (asset world479-f2-stz-v1.zip), download-верифицирован (sha match).
- База: level.dat из v479-a10-stz-world (d88978f1, download-verified) — server-born
  DataVersion 4556/1.21.10, seed 42, region/ ПУСТОЙ → полный r640 fresh-gen; DIM-1/DIM1
  пустые; datapacks/bukkit/pack.mcmeta сохранены.
- Датапак `datapacks/stz2/` (namespace stz2; pack.mcmeta dual-declaration 48+[48,88] —
  C61-урок; stz1 УДАЛЕН из зипа — one-scene-per-run, иначе MSPT не атрибутировать):
  - `load` → `stz2:init`: scoreboard stz2 + `summon marker` Tags:["stz2_probe"] в world spawn
    (fail-safe A10-класса: маркер мёртв → шторм молчит → ран = canon-pop150k, не FAILURE).
  - `tick` → `stz2:tick`: gametime → `#stz2_t`; фаза `#stz2_p = (#stz2_t − 300) % 600`.
  - `stz2:dispatch` (p∈[0,119], маркер жив): 64 ячейки 8×8 чанков, ячейка k при p∈{2k,2k+1}
    → `forceload add <x0> <z0> <x0+7> <z0+7>`; координаты k: ix=k%8, iz=k//8,
    x0=ix*8−32, z0=iz*8−32 (сетка ±32 чанк-коорд = ±512 блоков ⊂ r640 — сгенерировано,
    НЕ worldgen-ось); каждая команда 64 чанка < 256-лимит, forceload idempotent.
  - p=120: `forceload remove all` ( unload 4096 чанков → save/unload-чурн);
    p∈[121,599]: idle 480 тиков (макс. фазовая длина 600 < bench-окно ~723 тика @canon
    MSPT 415 → ≥1 полный цикл в 300s-окне гарантирован).
- Числа сцены: 4096 чанков сильной загрузки на цикл (64×64-чанковая сетка вокруг спавна),
  load-burst 120 тиков (≈34 чанка/тик пик), период 600 тиков. Ваниль-команды only
  (forceload = штатная ваниль-механика strong-loading,datapack-функции исполняются с perm lvl 2).
- Zip-гигиена (A9-урок): dirs `0o40755<<16`, files `0o100664<<16`, testzip CLEAN, 86 энтрии.
- Диспатч (прегист закон 14a/16: `scripts/dispatch_479_f2_stz.py`): алиас `round-479-f2-stz`
  = HEAD мастера на тик (дельта docs/stress/scripts-only, 0 src/Java/Rust), canon x466-C98
  ЯВНЫМ JSON (640/300s/fp4/gc3/ic1/fd1/rt4/bc1/**pop150k**/seed42/10G/xms4G), band [6.0,9.5]M
  fast-fail, lever ∅ (нагрузка = сцена), `world_url` = релиз-ассет v479-f2-stz-world.
- Гейты чтения: FIXTURE-VALID, NCDFE=0, AIOOBE=0, маркер жив + forceload-след в stdout
  (0 → fail-safe canon-ран → REFUTED_CENS сцены), MSPT-профиль vs канон 392.89/415.84;
  band-miss workload-cpu ожидаем (сцена = claim), не ре-ролл-триггер (Л201).
  Wall >15 мин → DISPATCHED run-id (закон 18-iii).

## СТЗ-3: redstone×chunk-load (Lithium #37) — materialization-pending (одна сцена на ран)

- CLAIM (веб-разведка ×479-F2): Lithium #37 (closed, 16c, P-high/S-confirmed/A-vanilla-issue,
  2020-05-01): heavy mechanisms (72k ice-farm) при нагрузке ломают чанк-лоадинг в том же
  измерении — грузятся только близкие к игрокам чанки, соседние выгружаются. Репро-класс:
  **redstone-mechanism-mass × chunk-load-coupling** (ось blockupd × chunk_sched, lever
  BlockUpdateOps/chunk_sched — оба в master). Командный класс (setblock-наброс observer-часов
  в load-функции + forceload-цикл СТЗ-2 как чанк-катализатор), НО самостоятельная сцена —
  отдельный zip след. тиком (смешение со СТЗ-2 в одном ране запрет one-scene-per-run).
- Moonrise-сноска (mandate «Moonrise Optimized TPS»): серверных TPS-конвертируемых issue нет —
  #27 lag-spikes = client-render (не сервер-ось), #44 section-status entity-removal (2c,
  thin), #182 spectator chunk-load (фикстура без игроков неприменима). Топ-2 ⇒ C2ME #457 +
  Lithium #37; C2ME #464 lighting-desync (38c, топ по комментариям) — parity-класс, не
  стресс-ось (вне формата СТЗ, фикс-тура тяжёлая) — задокументирован здесь как runner-up.
