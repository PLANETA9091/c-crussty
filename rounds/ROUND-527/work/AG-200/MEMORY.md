# MEMORY AG-200 (w527) — уроки
1. Контроль-ноги пары (ic1) могут быть все cancelled (famine 06:4xZ убил AG-96 pair) —
   живой GET ранов обязателен перед «A/B открыт» дедупом.
2. Л194 pair-cpu: ic0@pre-drift vs ic1@post-drilt несравнимы — reroll базы на ТОЙ ЖЕ когорте
   дешевле, чем спор о сопоставимости (2-й POST как A/A-якорь(kernel-drift)).
3. POST /git/refs c full-40 sha существующего объекта = ок для zero-code ветки;
   GET-верификация object.sha до dispatch (Л188a), 1 ветка = 1 ран (Л188b).
4. dp3v2 только full-URL releases/download/v484-dp3v2/stz3v2-fixture.zip (sha 16fa1a32),
   литерал-алиас = die@fetch (AG-121/CLAIM-513-6).
5. Band для pop50k dp50k-lane = 5.5-13.5M (вектор AG-141), не глобальный [6.0,9.5]M.
6. Board-CAS: content+sha из ОДНОГО GET (урок AG-158), retry 409.
