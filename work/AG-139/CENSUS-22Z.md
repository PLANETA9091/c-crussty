# AG-139 w527 — famine re-census 22:28-22:35Z + dgw-нижний-край harvest (0-POST)

## CLAIM
Пост-мёрж ревизия 107/103 stale (69+110 в master) + famine re-cens 22Z (fork AG-9 "re-cens ≥22:00Z").

## 1. КВ-ЦЕНЗ 22:31:57Z (jobs-api, вилка AG-9)
- QUEUED=449, IN_PROGRESS=40. Динамика: 554q@17:07Z (AG-112) → 449q@22:31Z = дрейн ~21/ч (5ч).
- ci-эхо продолжает спавн: 30 последних ci-ранов 21:02-22:29Z ВСЕ queued (guard fff60bf1 не режет спавн-исток).
- WBP-смоуки голодают: smoke69 37037064852 queued с 16:54Z (5.6ч), parity27 37031297573 queued с 16:03Z (6.5ч).
- КЛЮЧЕВОЕ: слоты БЫЛИ: bv2-ноги стартовали 17:21:21Z (r576-71), 18:10:59Z (dgw128), 19:21:49Z (dgw64).
  Значит «0 success с 14:36Z» (AG-96/AG-120) stale: r576-71 завершился SUCCESS @18:17:49Z.
  Флот не терминально мёртв — он медленный: длинные bv2-ноги (3ч+) съедают слоты, WBP-смоуки ждут.
- Слот-модель w528: не «0 ip», а «ip заняты 3ч-ногами»; POST-ы смоуков имеют шанс при drain.

## 2. HARVEST r576-71: run 36990722717 = SUCCESS, job 17:21:21→18:17:49Z (56м, slot живой)
- Арт benchv2-ag433 313051B id=11243949945 скачан (r576/BENCHV2.md + server-stdout.log 5.3MB).
- ch/s drain-def 21.40 — FALSE-DRAIN suspect: drain window 249s < pregen floor 254s @21ch/s (канон AG-116).
- forceload-marked 5329 (expect ≥5062 = 0.95×1×5329; side=73, 1-dim), G4 PASS, G5 PASS, NCDFE=0, AIOOBE=0.
- MSPT idle 3.3 / sustain-median 6.6; TPS n=227 min 12.74 last 20.0. G-DIM ov=5929 total 5929 (<60000).
- ВЕРДИКТ: нога VALID-job, но ch/s=21.40 в FALSE-DRAIN-классе → НЕ валид для r-кривой AG-71/AG-119
  (там же AG-116 phantom 730.32). Для S_BV2 ch/s-оси не использовать.

## 3. HARVEST dgw128 (МОЯ нога s528139): run 36995272375 = FAILURE, job 18:10:59→21:18:33Z (3ч07м!)
- Арт benchv2-ag433 3378394B id=11251659515 скачан (dgw128/BENCHV2.md + server-stdout.log 67MB).
- ch/s 9.83; forceload-marked 20449 при expect ≥58279 = 0.95×3×20449 → G4 marked≥95%: FAIL (33% mark).
- G-DIM: overworld 21609 OK, world_nether loaded=0, world_the_end loaded=0 — nether/end НЕ сгенерились вовсе.
- MSPT sustain-median 46.7 (vs 6.6 у r576-ноги) — ген-пайплайн задушен окном 128.
- BENCH-V2 step failure = гейт-выход по G4; лог чистый (0 Exception) — смерть не крашем, а недогеном.
- ВЕРДИКТ: dgw128 = death-march класс: 3ч07м, 1/3 стенда, G4 FAIL. Нижний край w-кривой = узкое место
  генерации (window ≤128 душит 3-dim pregen в cap). Согласуется с w1024 клифф 2.27 (AG-58 cap-trunc):
  кривая w не-монотонна с ОБОИХ краёв, пик в миде (w512 11.69).

## 4. dgw64 (МОЯ нога s527139): run 36995191941 in_progress 19:21:49Z → 22:33Z = 3ч10м > смерть-точки dgw128
- Предсказуемо мёртв (тот же класс). POST /cancel → 202 @22:33Z — слот освобождён очереди
  (cancel in_progress = реальное освобождение раннера; dead-letter AG-101 касается только queued).
- SELF-FAIL: гипотеза «нижний край 64/128 даст валидные точки» REFUTED — обе ноги G4/смерть-класс.
- w528 ЗАПРЕТ: re-POST dgw≤128 в 3-dim стенде без dcp/урезания dims — G4-класс гарантирован.

## 5. Merge-ревизия (master ушёл вперёд: 8184f1e0 → 61dd7452 → 978305b7)
- Master поглотил: swarm-527-69 (e53c2f01), swarm-527-110 (5ca5df3a), 27 (de6001c7), 59 (b51019c1),
  46 (8184f1e0), 526-500, 526-370. merge-base проверен локально: 69 IN master, 110 IN master, 64 NOT.
- swarm-527-107 (union 64+69 @ddc8c7f7dc) STALE: 69-хунки уже в master → мёрж 107 задублирует.
- swarm-527-103 (stack master+69+59+27 @25826eb9) VOID: все члены поглощены.
- swarm-527-110 (POP_TIMEOUT) МЁРЖЕН — MERGE-READY флаг закрыт.
- Остаток merge-очереди: только 12a577a9 (64 soak-START, +1-1 run_world3.sh) — но owner SKIP_CONFLICT 64/43
  (61dd7452). Аг-105: 64-START и 69-маркер избыточны вместе; 69 в master → 64 отдельно ревизить арбитром.

## Артефакты
- /tmp/ag139_h/r576/ + /tmp/ag139_h/dgw128/ (локально); job-ids 110786052579 / 110800455745.
- Census script /tmp/ag139_census.py, CAS-хелпер /tmp/ag139_api.sh (contents-API, retry-409).
