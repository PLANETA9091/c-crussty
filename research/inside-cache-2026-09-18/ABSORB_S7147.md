# S7-147 — ABSORB leg #2' INSIDE-CACHE (run 35314220731, head 53d14d1) + fixture parity fix

## Конфиг-девиация диспатча → A/B невалиден как §156-лег

run-env.txt (run 35314220731): `fake_players=0`, `seconds=900` — дефолты workflow;
preregistered leg #2' (dispatch_s7135.py/§156) требует **fp=4 / 300s** (база-ценз
35275967738 = fp4/300s). Плюс закон равных окон S7-96d нарушен (alloc-окно 90s vs 60s,
cpu 495s vs 165s). Ран не может дать §156-вердикт.

## Что ран ДОКАЗАЛ (позитив, вердикт-градации вердикта не даёт)

- **ARMED-цепочка ПОЛНОСТЬЮ живьём** (гейт §156-2 PASS): pristine sighting Entity
  205458B (major 65) + второе sighting 204078B (известный dual-copy факт leg1,
  не тикает) → forcing kernel load → defined InsideBlockOps + $Recorder →
  computed patch Retargeted {sites: 1} 205458→205522B → hook serve 205522B →
  **retransform rc=0**. Hardened-мост (S7-136) обслужен end-to-end на живой сцене.
- Fixture-инъекция: INJECT DONE 150000/150000, FIXTURE-VALIDITY: VALID.
- Сцена пережила 900s soak, 0 tick-behind warnings, spark-репорт цел.
- ap.log чист (профайлер v3 healthy).
- **TPS ВЫШЕ базы**: crawl после инъекции 1.2 → 3.3 монотонно (база 0.6-0.8 на 148k).

## ГЛАВНАЯ НАХОДКА: «коллапс сцены» = паритет-артефакт фикстуры, не самовзрыв рычага

- entity totals: **71267/71199/71108** (база 148402) — та же сигнатура, что leg2
  (67-74k). Харденинг ping-pong (S7-136) коллапс не устранил ⇒ гипотеза ping-pong
  была вторична.
- Траектория: TPS рос 1.2→3.3 РОВНО по мере дрейфа популяции вниз. Накоплено
  ~2970 тиков (900s × ~3.3 TPS) против ~210 тиков базы (300s × 0.7 TPS) — 14×.
- Механика распада: item-merge (105k→54.7k items), дневное горение/смерти мобов
  (hostiles ~28k→~15k) — ВСЁ ванильное, просто тикает в 14× больше раз на стенку.
- Topup-эстиматор слеп: `deficit=0 aliveEst=105000` (константа инъекции; модель
  покрывает ТОЛЬКО age-despawn 6000t). В базе topup вообще не стрелял: 210 тиков <
  600-периода. **Парадокс: чем ЛУЧШЕ работает рычаг, тем «невалиднее» ран по
  wall-time паритету популяции.**
- fp=0 девиация дополнительно нарушает спеку сцены («спавн/деспавн как будто
  игроки есть» → fp=4 обязателен).

## Фикс S7-147 (bench/world3/population/BenchPopulationPlugin.java, коммит 4c8f029)

- TOPUP-SCAN каждые 600 тиков: реальный счёт w.getEntities() по лейнам
  (Item/Monster/Animals), маркеры aliveReal/deficit/aliveEst/topupSpawnedTotal.
- Непрерывный drain: 20 spawn/тик (~14ms, profile-invisible), largest-lane-first
  (items→hostiles→passives), miss-guard 64, детерминизм seed^(ft*1_000_003)^total.
- Compile-OK: javac --release 21 против kernel 29386794B + purpur-api + 125
  libraries (CI-эквивалент classpath).
- Следствие для протокола: **база обязана быть перезапущена с фикстурой-фиксом**
  (base-b, ран 35317176927, head 4c8f029, все рычаги 0) — все A/B пары дальше
  сравниваются с base-b, не с до-фиксным цензом 35275967738.

## Лейны leg2' vs база (SHARES, НЕ равные окна — только ориентир, не гейт)

| лейн | база (7369 сэмплов) | leg2' (25724) | Δ share |
|---|---|---|---|
| inside-blocks path | 42.39% | 35.08% | −17.3% rel |
| movement-geom path | 22.50% | 21.86% | −2.8% rel |
| flushStep path | 3.91% | 4.29% | +9.7% rel |
| PalettedContainer.get (cpu, 56113/136798) | 3.06% | 3.54% | +16% rel |
| young GC (gc,start, весь ран) | 146 | 182 | +25% |

Не гейт-градация: окна/тики/популяция несопоставимы. Единственный честный вывод —
самовзрыва моста нет (bridge не виден в топах, TPS вырос).

## Вердикт

- leg2' = **A/B-НЕВАЛИДЕН (девиация конфига + паритет-артефакт), ARMED-СОК-ЭВИДЕНС
  ЗАБАНКОВАН** (ARMED full chain, 900s soak, TPS ↑, 0 crash).
- §156-вердикт INSIDE-CACHE **ПЕРЕНОСИТСЯ на leg #2''** (fp4/300s, inside_cache=1,
  head ≥4c8f029) vs **base-b** (ран 35317176927) по preregistered гейтам §156.
- Инфра-урок фиксируется в каноне: любые будущие ноги — только fp=4 (спека сцены)
  и только после base-b.

## Артефакты

- research/inside-cache-2026-09-18/run-s7136-leg2prime/ (артефакт рана 35314220731,
  world3-bench 32.3MB: BOTTLENECKS_3/cpu/wall/alloc-collapsed/stdout/gc/ap/run-env/kernel)
- research/inside-cache-2026-09-18/run-s7134b-base/ (артефакт базы-ценза
  35275967738 для повторных сравнений; kernel 29386794B байт-в-бит e2992d63)
- scripts: /home/z/my-project/scripts/{fetch_artifact_s7147,analyze_absorb_s7147,dispatch_s7147}.py
- sha256: artifact_hashes_s7147.txt
