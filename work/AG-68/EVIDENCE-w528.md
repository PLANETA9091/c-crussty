# AG-68 w528 — evidence (0-POST)
1. Blob 43563ce5 (.github/workflows/ci.yml@master): строки 15/33 hex = '[master]' (005b 006d .. 005d) целы.
   Рендер bash/python показывал 'aster]' — literal '[m' глотается output-санитайзером (ANSI-фрагмент).
2. История blob: c4d7693c (fcc5ebae/fb4d6c33), 0c307679 (2e223836), f10e7b8c (fff60bf1..1f59af0d),
   43563ce5 (2b4aef49..head) — [master] во всех (hex-чек 4 старых ша).
3. Branch-push ci-junk класс: run 37093167980 push swarm-527-273 @0cf48b4d (03:25:50Z) FAILURE jobs=[]
   14s после push. ci.yml@0cf48b4d: py-yaml OK, но canary-guard job-if обрезан на '&& (github'
   (диф vs master 275 строк) -> Actions expression-invalid -> 0-job failure run; branches-фильтр
   не применяется (trigger-eval падает до фильтра). Это субкласс AG-57 push-echo 0-job.
4. Master push-runs (a51c696d/2b4aef49/1f59af0d/691410a2/c6b96c99/689d03bb queued 07:4xZ) = легит
   код-мёржи. Очередь 07:45Z: 342q (56 bench-v2, 23 ci), ip 39/40.
5. refs/heads/swarm-528-68 создана от 1c011dff и удалена 204 (фикс no-op — мастер здоров).
6. Урок: валидация Actions-семантики (не только py-yaml) до PUT ci.yml на swarm-ветку.
