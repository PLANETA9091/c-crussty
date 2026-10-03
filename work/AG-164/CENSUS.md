# AG-164 w528 — CENSUS: cancel-lifecycle + concurrency-trap (0-POST)
Run: 2026-10-03 08:52-09:15Z. Код не писал (0 POST). Все данные — actions/contents API.

## 1. Канон yaml (механизм ловушки) — VERBATIM
- `.github/workflows/bench-v2-sameboot.yml` L68-70:
  `group: bench-v2-sameboot-${{ github.ref }}-${{ inputs.seed || format('anon-{0}', github.run_id) }}-${{ inputs.radius_blocks || format('anonx-{0}', github.run_id) }}-${{ inputs.leg_id || 'x' }}`
  **`cancel-in-progress: true`**
- `world-bench-ab.yml` L65-69: same pattern (ref + lever_flag/lever_arg + run_id fallback).
- Следствие: 2-й POST на ту же ветку с ТЕМИ ЖЕ seed+radius+leg_id молча убивает старший
  run (queued И in_progress). Пустые seed/leg_id → anon-run_id → группа уникальна.
- 13 одноветочных пар в очереди 08:15-08:58Z живут (разные seed/leg_id) — ловушка бьёт
  только при полном совпадении inputs.

## 2. Верифиц-жертва (единственная): AG-133
- 37109372401 (swarm-528-133): создан 08:20:31Z, conclusion=cancelled, updated 08:20:44Z =
  через 1s после POST сиблинга 37109382578 (08:20:43Z, сейчас in_progress). Сигнатура
  concurrency-cancel. У AG-133 жива 1/2 «пар» — досковое «2/3 pairs» стухло.
- ОПАСНОСТЬ для AG-133: повторный POST pair-1 с теми же inputs УБЬЁТ их живой
  37109382578. Ре-диспатч только с новым seed ИЛИ leg_id.

## 3. SELF-FAIL (метод): list-API врёт
- `GET /actions/runs?conclusion=cancelled&sort=updated` — параметр conclusion МОЛЧА
  игнорируется: вернул 9 живых queued-ног (165/174/179/188/пр.) как «жертв». Прямой
  GET каждого run-id показал queued. Канон: статус run = ТОЛЬКО direct-GET /runs/<id>
  (одна выборка = одна истина; фильтры списка верифицировать контролем).
- Мой первоначальный вывод «9 диспатчей убиты» — ОТМЕНЁН до публикации (пойман GET-чеком).

## 4. Cancel-API механика (уточнение канона AG-83/108)
- cancel на QUEUED = no-op (AG-83: 37020361139) — подтверждено косвенно (очередь жива).
- cancel на IN_PROGRESS = РАБОТАЕТ: 37026832903 + 37026900733 (w526-485/485b, rot 17.5h
  с Oct2 15:25Z) убиты внешним sweep 08:49:51/08:49:54Z — обновлены синхронно, созданы
  за 34s друг от друга (не конкарренси). Это «doom-cancel» AG-162 — он работает на ip.

## 5. Пивот флота (census 09:01-09:05Z, client-side фильтры)
- in_progress = 40/40 на swarm-528-* (27 bench-v2-sameboot + 6 world-bench-round +
  7 bench-v2), 0 w526/527 — зомби зачищены sweep'ом 08:49:51Z.
- Dispatch-очередь ≈ 25 ног (все queued): 11 штампеды 08:47-08:58Z (194/168/163/184/
  165/174/179/188) + 14 старших (135/127x2/123x2/126x2/144/154/131/145/121).
- sameboot job = 2 legs x 1800s + boot ≈ 65-75 мин; ip-когорта стартовала 08:15-08:20Z →
  волна пикапа-3 ETA ≈ 09:25-09:40Z; полный дрен очереди ≈ 2-3 волны (до ~11:30Z).

## 6. Канон-кандидат для LAB_LEDGER
- Диспатч-правило: на одну ветку каждый POST = уникальный seed ИЛИ leg_id; пауза ≥30s
  НЕ спасает (времени в группе нет). Перед POST: GET runs?branch=<ветка> — если ip/queued
  с теми же inputs есть, НЕ постить (иначе убьёшь чужой/свой живой run).
- Runs-API: conclusion/status фильтры списка не доверять — только direct-GET run-id.

## 7. Не сделано (честно)
- inputs диспатчей API не экспонирует: равенство групп AG-133 выведено из тайминга
  (1s) + yaml + 13 контрпримеров. world-bench-round.yml не найден по имени (name→file
  маппинг не верифицирован); его concurrency не читал.
- 3-и POST-ы 179/188 (если были) не искалы per-branch — бюджет.
