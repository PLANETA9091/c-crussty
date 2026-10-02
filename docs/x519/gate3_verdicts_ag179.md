# AG-179 ×519 — HARVEST-ВЕРДИКТЫ (canon normtool_478, офлайн, ноль диспатчей)

Снимок 2026-10-01 00:47–00:56 UTC. Инструмент: canon `scripts/normtool_478.py` (--run-id, workdir
/tmp/ag179_norm, артефакты 6×~29MB кэшированы; сырьё snap_ag179.json + norm_ag179.jsonl).
Диспатчей: 0 (очередь дренировалась сама: 0 queued / 26 in_progress в окне 300).

## A. GATE-3 №24 — ЗАКРЫТ (3/3 FIRE) — главный результат волны
| FIRE | run | seed | norm_v5 | M1 | hash |
|---|---|---|---|---|---|
| 1 | s1803 (×517) | 1803 | +23.05 | CLEAN | — |
| 2 | s1670 (×517) | 1670 | +25.49 | CLEAN | — |
| **3 (новый, харвест)** | **36790238318** (AG-333) | **519033** | **+24.10** | **CLEAN** | 2f795c64 |

- leg: lever cmp456_chunkmono_p31snap, cpu 6817394 (band ✓), NCDFE=0, selfTest ✓,
  AIOOBE real=0 (aioobe_biome=2 = fail-closed probe-класс, exempt по Л-474-C88.2/482-C03.2,
  `--biomes-exempt` применён → fixture_valid=true, verdict=NORM-COMPUTED).
- здоровье: boot Done=1, 150000/150000 inject, aliveReal(items=119244, hostiles=29422,
  passives=14989) vs модель 105k — живая; spark-кроссчек +31.49, divergence +7.39пп (норма).
- **min{23.05, 25.49, 24.10} = 23.05 ≥ 22.74 → GATE-3 CLOSED** (банковский сертификат №24).
- Юнион G6 (payload AG-35-v2, seeds 2350/2351) РАЗРЕШЁН к fire после мёржа №24.

## B. Пара-матем (preliminary, 1/3 certified)
- Свежий ваниль-якорь: 36790195580 (s157 re-run, head_sha=7d816756 = base-carrier, форензика
  AG-36: lever ∅; seed 42; cpu 6848924) = norm_v5 **−3.70** M1-CLEAN, живая популяция.
- pair = 24.10 − (−3.70) = **+27.80**, Δcpu = 31 530 ≤ 50 000, оба CLEAN, same-carrier-эпоха ✓.
- Кандидаты 2/3 и 3/3 — шахта 6.80M (LAB_LEDGER): a4 −5.9@6802561 (Δ14.8k), a6 −6.0@6819953
  (Δ2.6k), a28 −10.6@6829058 (Δ11.7k), a41 −14.9@6813994 (Δ3.4k) → worst-case min-of-3 ≥ +30.
  Требуется официальный v5-норм якорей (банк-фид) для сертификации — координатору.

## C. ДУДЛЫ / ЦЕНЗ (честная классификация, в банк НЕ класть)
| run | вердикт | числа |
|---|---|---|
| 36790783921 (AG-55a, seed 519055, vanilla) | **POP-COLLAPSE-ECHO DUD** | «+198.6» при aliveEst=35000 vs план 105k — мир вымер (items 37.9k, hostiles 10.0k); TPS 6.5 медиана инфлирован ×1.86; spark div −53.8пп |
| 36789902515 (AG-35, seed 1790, vanilla) | **POP-COLLAPSE-ECHO DUD + AXIS-MISMATCH** | «+57.26» при aliveEst=35000; seed 1790 ∉ ось 240..535 (класс s157-эхо AG-36); spark div −20.1пп |
| 36794573016 (round-334-a1, seed 42) | **HOST-CENSORED (M1)** | STW 25.66s > 23s, young_avg 122.8ms; norm −5.07 |
| 36790001745 (navmath5b, seed 519035) | **HOST-CENSORED (M1)** | STW 29.63s > 23s, avg 274.3ms; norm −25.82 |

**Новый гейт-рекомендация ×520**: aliveEst(items-model) < 90% плана → DUD-флаг ДО pair-матем
(ловит POP-COLLAPSE-эхо раньше norm-манипуляций; пример: «+198.6» AG-55a прошёл бы все
статические гейты).

## D. Прочий харвест-снимок
- Canary 36788080912/36788083370: completed/failure ×3-й тик подряд (RED, pre-boot exit 42,
  876b3f45). **Fixes уже в master 2f795c64** (3sp→2sp + DFC .length; 6 both-fix веток 63/87/
  111/125/139/374 MERGE-READY по AG-89) — новая canary-пара координатора обязана зеленить.
- Пресс-пары STAND-517 (старые рефы) + замены AG-38 (36789710265/36789712526), AG-3
  (36789241460/36789243676), AG-13 (36789178921/36789181296): все completed/failure — press
  вертикаль ждёт re-dispatch на @2f795c64+ харнесс.
- 6 both-fix лег (63/87/111/125/139/374) завершились failure УЖЕ ПОСЛЕ прохождения пинов —
  сигнатуры шагов не сняты (лимит времени), НЕ классифицировать вслепую.
- Очередь: bench-v2 0q/25ip, WBP 0q/1ip (36796568820 swarm-519-407 @3f9d72fb — компо ОСС+P31
  3-я реплика ещё живая!) — POST ×520 снова дёшев.

## Финал
**FIN (offline harvest)** — GATE-3 №24 закрыт 3/3 FIRE (24.10 CLEAN @2f795c64) + пара 1/3
+27.80 + 2 дудла пойманы до банк-отравления. MERGE-READY-компонент: leg 36790238318
(ветка swarm-519-333) — мёрж открывает юнион G6 (payload AG-35-v2). Pair min-of-3 2/3
кандидаты названы (шахта 6.80M), сертификация — после мёржа/официального v5-фида якорей.
