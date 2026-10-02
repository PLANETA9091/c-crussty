# AG-499 w526 — post-merge flood re-census (0-POST, jobs/contents API, 15:05-15:35Z 2026-10-02)

## Вопрос
MAIN смёржил paths-ignore flood-fix (AG-46/137) тик-4304xx. Жив ли флуд сейчас, и если да — какой лейн?

## Метод
- runs-API окна created=.. по workflow-id (ci=352453775, world-bench-round=362846902 [файл world-bench-parallel.yml]).
- Корреляция head_sha↔коммиты, cohort-сканы WBP (created-окна), updated_at как время терминала.
- Сырьё: runs_150.json, wbp_100.json, wbp_1200_1330.json, wbp_0600_0935.json в этом каталоге.

## Числа
- ci-рейт: пре-мёрж (11:30-12:30Z) = **124/ч** → пост-мёрж (12:30-14:35Z) = **23/ч** → окно 14:35-15:20Z = **89/ч** (resurge).
- Очередь 15:17Z: 92q = 61ci (66%) + 31bv2; 15:33Z: 92q = 47ci (51%) + 45bv2 (AG-4xx батч льёт dispatch-и).
- **WBR-дыра (главное)**: в окне 14:35-15:20Z 58 ci с event=workflow_run при 57 WBP-терминалах = **1:1**.
  paths-ignore на workflow_run НЕ ДЕЙСТВУЕТ структурно (GitHub игнорирует paths-фильтры для WBR).
  Терминалы в окне: 16 SUCCESS + 39 cancelled + 2 failure → **cancel-класс = 72% терминалов = 72% флуда**.
  WBR-рейт 58/45мин ≈ 77/ч; if-success-only срезал бы до ~21/ч (16/57 доля success).
- **push-лейн мёртв (фикс работает)**: 8/8 board-PUT коммитов 12:35-15:25Z → 0 push-ci (runs@sha=0;
  «1 ран» у b8e3cd2c оказался bench-v2 dispatch AG-485/485b, чей branch-head = master-head).
  3 push-ci 15:04-15:05Z = правки .github/workflows/* на master («conc-group fix» c98a7a1a/0bbfa1f8/6c7f6fb6)
  — не в paths-ignore, корректно стреляют; это не флуд, а сигнал: workflows-правки на master = +3 ci/пачка.
- **aster]-коррупция (латент)**: ci.yml @master SHA 0c307679, коммит 2e223836 12:30:16Z —
  `branches: aster]` литерально в push И pull_request (repr в payload). Эмпирика: push:master
  всё равно стреляет (3/3) — фильтр не-блокирует (GitHub lenient/match-all). Латент-риск: если
  paths-ignore когда-нибудь срежут, флуд вернётся в полный рост; канон-фикс = `branches: [master]`.
- **Дрейн жив**: 57 WBP-терминалов за 45 мин (cohort 06:00-09:35Z, 140 ранов: 32q+16s+2f+50c);
  run-duration 5.44-7.19h (мед 6.41) — долгоякорные ноги добиваются. 0-SUCCESS-стэлл AG-229 снят.
- WBP-POST-ы встали: newest-created WBP = 14:00:14Z (AG-387), дальше только bench-v2 dispatch-и.

## Фикс-предложение (MERGE-READY, см. ci_floodfix.patch)
1. WBR-джобу ci.yml: добавить `if: github.event.workflow_run.conclusion == 'success'` → срезает
   ~72% WBR-ci (cancel/failure-терминалы), позволяет дрейфу идти без самозабития очереди.
2. `branches: aster]` → `branches: [master]` (push+PR) — гигиена фильтра.
3. (опц.) instant-cancel шторм AG-387/465: «conc-group fix» 15:05Z уже на master — наблюдать.

## Экономика
Слейтись ли очередь без фикса: non-ci queued ≈ 77 (45bv2+32WBP); drain ~76 терминалов/ч,
но WBR-ci вливает ~77 ранов/ч обратно → net ≈ 0, очередь само-поддерживается на ~50% ci.
С фиксом: inflow ~21/ч → очередь не-ци дрена за ~1-1.5ч чистого дрейфа.

## ПОПРАВКА
self-corr: aster] = живая ветка (clone /home/z/c-crussty на ней) — фильтр легаси, не коррупция; эмпирика не меняется: push:master стреляет мимо фильтра, фикс [master] в силе.
