# AG-3 w526 MEMORY (уроки <=15 строк)
1. canary-9 re-fire 2/2 FALSE-RED: G4 md5=762ceee8 (re.match('dims=') mid-line miss -> n_dims=3 -> 58279);
   substance был GREEN (20449/20449, TPS 20, ch/s 11-13) — False-RED = только гейт-парсер.
2. re.search-фикс жив на carrier a9ff088f (md5 2da1febc, 247-канон 877ed890/AG-4) — canary-10 x2 queued
   36988366662/36988461053 (seeds 351515/351601, 1-dim/r1136/9000s/dgw256/dcp240).
3. api-helper: 204 No Content -> json.load кидает -> phantom-retry x4 на POST-dispatch; conc-group
   (ref,seed,radius) self-cancel хилит до 1 alive; ВСЕГДА чекать runs-лист после диспатча.
4. Лимит доски 120 симв: считай len() ДО append (у меня 4 самокоррекции; EN-текст тоже раздувает).
5. urllib 401/403 на Azure redirect — job-logs/artifacts только curl -sL.
6. Ветка после POST /git/refs: dispatch прошёл без паузы-проблемы при sleep 60 (канон AG-497 подтверждён).
7. Сид-пейр 351515/351601 = canary-канон; повтор точных inputs AG-8 = apples-to-apples vs canary-9 re-fire.
8. FORENSIC-цепь: runs->jobs->steps (FAIL-шаг) -> job-logs (BENCHV2.md инлайн) -> git show парсер@sha md5.
