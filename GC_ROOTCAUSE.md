# GC_ROOTCAUSE — TASK-424-A (GC-ревизия brain2-носителя)

Артефакты: world3-bench ранов tick-422 round-b2l1/l2/l3 (re-скачаны из GH Actions:
35811223073/35811228420/35811233192), anchor422b из репо. Профили spark = SOAK-окно
(последние 300s), gc.log = весь ран (994s).

## 1. Главный факт — шторм = ФАЗА ИНЪЕКЦИИ, не soak
Бакеты young-GC (Allocation Failure) по минутам b2l1:
- t=60-660s (INJECTION 02:46:19→02:55:54, 575s, 150k): 271→542 GC/мин (young каждые ~116ms, eden 3.4GB)
- t=660-1019s (post-inject + soak): 5-15 GC/мин — СОАК СПОКОЕН
- LAST300 (soak): n=59, int_p50=4967ms, pause_p50=105.8ms
- anchor422b LAST300: n=58, int_p50=4919ms, pause_p50=116.4ms — ИДЕНТИЧНО
⇒ «young ×39 vanilla» из BOTTLENECK.md = whole-run счёт, штумующий в инъекции;
  soak-GC-профиль brain2 == anchor. Young-шторм = инъекционная фаза (растущая
  популяция + чанк-лоады + topup), где каждая молодая GC дешёвая (p50 5.9ms),
  т.к. eden почти весь мусор.

## 2. ВТОРОЙ факт — SoA-плоскости были ПУСТЫЕ весь ран b2l1
server-stdout b2l1: `cmp406_aibatch: epoch ok tick=N windowLen=0` ×895 и
`cmp406_sscan: epoch ok tick=N mobSlots=0` ×895 — КАЖДЫЙ ТИК (tick 14→908).
rc=0 весь ран: rust-плоскость не видит НИ ОДНОЙ мобы (idCount()=0) →
skipAi fail-closed → все мобы ваниль каждый тик. Окно AI (aibatch N=4) и
despawn-scan (sscan) = ИНЕРТНЫ. При этом epoch-машина (lock+JNI+LOG) гоняется
каждый тик вхолостую.

## 3. Третий факт — лог-спам = топ-1 аллокатор soak-профиля
alloc-collapsed (soak) b2l1 vs anchor422b, дельта фреймов:
- log4j AppenderControl.callAppender* +1465 сэмплов (RewriteAppender +969,
  PatternLayout +712, DatePatternConverter +570, Calendar +285) — САМЫЙ
  БОЛЬШОЙ позитивный дельта-источник soak-аллокаций
- источники (server-stdout b2l1): `items_subsys2: telemetry calls=N` ×3591
  (каждый 24000-й tickOne, счётчик 85.7M вызовов = 2M/s контеншн на static
  long с 4 region-воркеров), `epoch ok` ×1790 (см. п.2), `navpool EFFECT` ×845
- сам ARM-гвард «epoch ok» сломан для rc=0: ARM_LOGGED ставится только в
  skip/gate-hit ветках (windowLen=0 ⇒ never) → печать каждый тик

## 4. Кандидаты-не-подтверждены (мало сэмплов, профили = soak only)
- (1) mobs_soa fallback-read per-call: соа-плоскости инертны (rc=0) — не источник
- (3) per-mob JNI: не найден в профилях — гейты bulk с persistent-буферами
- (5) LongOpenHashSet/BlockPos churn: есть в обоих (ваниль), дельта не значима
- Шторм инъекции (до ~30GB/s TLAB-бамп почти без CPU-следа = огромные
  по-капасити буферы с частичным касанием) требует fresh-artifact волн-2:
  мой ран cmp423_brain3 даст чистый gc.log без лог-шума.

## 5. ФИКС-ВОЛНА-1 (cmp423_brain3, STRICT-OR поверх cmp422_brain2)
1. Empty-plane short-circuit: MobAiOps/MobScanOps/EntityGoalQueryOps —
   idCount()<=0 → vanilla БЕЗ lock/JNI/LOG на тик (895×2 холостых эпох умирают)
2. Once-гварды «epoch ok» в самой эпохе (ARM_LOGGED set on publish, не в skip)
3. Telemetry-счётчик/лог items_subsys2 удалён из tickOne (hot-path static++)
4. navpool EFFECT лог rate-limit 1/64 эпохи (rust)
5. Флаг cmp423_brain3 добавлен во ВСЕ 17 rust + 7 java STRICT-OR списков;
   блобы пересобраны все (×93 урок), javap flat==nested гейт
