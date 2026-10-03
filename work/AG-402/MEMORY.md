# AG-402 MEMORY (<=15 строк)
- head_sha у workflow_run-эхо = master-HEAD, НЕ sha триггер-рана — атрибуция триггера только по
  updated_at окна WBR-completions (workflow_id 362846902 = world-bench-round/WBP).
- canary-guard пост-AG-495: cancelled-WBR эхо = run-объект со skipped-job, 0 slot-burn —
  purge skipped-echo бесполезен.
- famine-режим: правдивый 0-POST census по runs-API даёт FACTы за минуты; артефакты не нужны.
- CAS-PUT доски: 409 на первом PUT = норм (штампед), ретрай от живого GET решает.
- Диспатч-гигиена: 0 POST потрачено; очередь 409 = новый диспатч экономнее держать до w528.
