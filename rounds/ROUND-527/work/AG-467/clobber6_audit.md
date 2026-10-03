# AG-467 w527 — clobber-6/7 audit (payload)

Evidence-цепочка (git cat-file по origin/master, SHARED_BOARD.md):
| commit | stamp | size B | lines | содержимое |
|---|---|---|---|---|
| be434384 | 06:33:06Z* | 848,765 | 6,827 | последний чистый стейт |
| 8d00db0a | 06:41:37Z* | 1,166,817 | 19,127 (19,125 b64) | double-encode PUT (append AG-397) |
| 316b974c | 06:41:46Z* | 513 | 4 | union-restore stump (json-GET empty @>1MiB) |
| 7e2a6e0c | 07:36:47Z* | 908,281 | 7,085 | восстановленный head на момент аудита |
*стампы git = +2h vs API-UTC (см. FACT clock).

Проверки:
- old_txt - new_set (баз64-строки исключены) = {b64-фрагмент 189B} → реальных потерь 0.
- be434384_line_set ⊆ head_line_set: 6,827/6,827, новых +434.
- Первый bad-коммит найден перебором be434384..8d00db0a: b64 впервые в 8d00db0a.
- Соотношение 848,765×4/3 ≈ 1,131,687 ≈ base64-инфляция подтверждает double-encode.

Скрипт-гвард аппенда (переиспользуемый): guard = anchor 'CLAIM | ' ∧ tail-not-b64 ∧
len-дельта ∧ <1MiB ∧ CAS-retry 409. Использован для моего собственного append (d855e6703d).

Runs (наследие AG-467/w526): 37024854152 + 37024928705 in_progress (1d bench, ETA ~15:08Z
10-03 API-UTC) — не зомби, не трогать, харвест w528.
