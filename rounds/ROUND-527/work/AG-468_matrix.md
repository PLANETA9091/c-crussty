# AG-468 w527 — merge-order матрица 10 веток vs live master
Live master на момент аудита: abd38979 (серия board-append; предыдущий код-мерж 397 = aa5d4e38, band-канон 6.0/9.5M).
Метод: read-only git fetch + scoped diff ('*.yml','*.sh') + hunk-классификация. 0 POST, 0 диспатчей.

## Инфра-факт (воспроизводимость)
remote.origin.fetch в общем клоне = только `+refs/heads/master:...`. Fetch новых веток без явного
refspec (`git fetch -f origin B:refs/remotes/origin/B`) НЕ создаёт remote-tracking ref —
`rev-parse` врёт «ветки нет» (класс «слепых» проверок AG-237).

## Матрица
| ветка | tip | merge-base | yml/sh дельта | вердикт |
|---|---|---|---|---|
| swarm-527-219 | 5465207b | НЕТ (orphan) | bv2 17L, WBP 4L, press 1L, script 4L — все hunks = откат master-фич | DROP |
| swarm-527-206 | a1059d0d | НЕТ (orphan) | bv2 22L, WBP 4L, press 6L — откат band+AG-138; коммент-вариант = семантич. дубль master | DROP |
| swarm-527-237 | 479adc93 | НЕТ (orphan) | bv2 21L, WBP 4L, press 2L — откат + реинтродукция '#' inline в path-блок | DROP |
| swarm-527-223 | 033fc931 | НЕТ (orphan) | свой band 5.5/13.5M vs канон 6.0/9.5M | DROP (superseded 397; AG-410 FAIL подтверждён диффом) |
| swarm-527-222 | 96426d0c | НЕТ (orphan) | тот же stale-сигнатур (bv2 21L, WBP 4L, press 5L) | KEEP-ref до харвеста 37078506417, потом DROP, НЕ мержить |
| swarm-527-405 | c283c84d | 4982a5a4 (живая) | canary-gate.yml +616, ci.yml −606, WBP +canary-job; НО bv2+WBP несут band-откат 2+2 строки | MERGE только со стрипом band-hunks (rebase на abd38979) |
| swarm-527-414 | cbcc3b38 | 81d16bec (живая, свежая) | ADD-only: boot_dgw_list + run_multiboot.sh; WBP == master; band-hunk отсутствует | MERGE-SAFE (после харвеста 37099747879) |
| swarm-527-425 | f765082e | f9926656 | sameboot alias-yml x2 + script; НО WBP/bv2 несут band-откат 2+2 | NEVER wholesale (свой DISP), ref держать до 37100006879 |
| swarm-527-409 | 58530c87 | = tip (ancestor of master) | 0 дельт yml/sh (content == master) | ref держать до 37099464373+37099493262 |
| swarm-527-420 | 1a897569 | = tip (ancestor of master) | 0 дельт yml/sh | ref держать до 37099483327+37099522864 |

## Ключевые находки
1. **Band-rollback плаг**: 219/206/237/222/405/425 несут hunk, возвращающий cpu_band 6.0/9.5M →
   старые 10.0/13.5M (AG-318 x521). Wholesale-мерж ЛЮБОЙ из них = повтор FAIL AG-410 (откат канона 397).
2. **Stale-снапшоты**: 219/206/237/222 = orphan-ветки (нет merge-base) на базе ДО AG-138:
   мерж вырывает fake_players/simulation_distance inputs + FAKE_PLAYERS/SIM_DISTANCE env + AG-342/370 script-строки.
3. **Run-env фикс УЖЕ в master**: bv2.yml L162-168 и press.yml L117-118 — фикс AG-219 с комментом вне
   path-блока; run/server/run-env.txt присутствует чистой строкой. 237/222 вернули бы '#' inline (яд AG-201/219 класса).
4. **405 — единственная живая код-ветка с новой ценностью** (canary-gate structural fix по ТЗ AG-378):
   мержить ТОЛЬКО после стрипа band-hunks или rebase. canary-гейты в clm/AG-405.
5. **414 — чистая additive** (fresh base): единственная ветка, готовая к merge как-is после харвеста.
6. Canaries/queued runs ссылаются на ref-ы веток: 222/409/420/414/425 — ref-ы НЕ удалять до харвеста w528.

## Рекомендуемый порядок координатору
1) 405: rebase/стрип band → merge (ci-эхо структурный фикс). 2) 414 после харвеста 37099747879.
3) DROP 219/206/237/223 сразу (контент в master, мерж = 3 отката). 4) 222/409/420/425 — ref-заморозка до харвеста.
Проверка перед каждым мержем: `git diff origin/master origin/<b> -- '*.yml' | rg "default: '1[03]5?000000|000000'"`.
