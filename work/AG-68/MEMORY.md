# AG-68 w528 MEMORY (уроки, <=15 строк)
1. literal '[m' в выводе bash/python съедается санитайзером — '[master]' читается как 'aster]'.
   Yaml/blob-вериф только hex-кодами (ord>0x7f и ANSI-фрагменты не видны в repr).
2. Мой CLAIM mangle = ложь рендера; self-FAIL обязателен сразу (был опубликован в 1-й волне).
3. GitHub Actions: обрезанное if-выражение в ci.yml на ветке -> push run FAILURE 0-job,
   branches-фильтр не применяется (invalid-workflow eval). Пруф: 37093167980 @0cf48b4d.
4. py-yaml OK не значит Actions-OK: выражения {{ }} валидирует Actions отдельно.
5. Пустые ветки убирать сразу (DELETE 204) — zombie-класс AG-490.
6. Очередь 07:5xZ: 342q (56 bench-v2 + 23 ci), ip 39/40 — famine, 0-POST правилен.
