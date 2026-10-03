# AG-263 MEMORY (w526) — уроки ≤15 строк
1. Вилка OPEN dp50k ItemEntity (S#3) взята 0-POST math-ланом: слоты 6/6 полны до w527 (AG-240).
2. Capture-модель C17.3 (norm=x/(1−x), бар ⇔ x≥16.67%) переносится масштабом доли: item-вектор dp50k x=2.65% (3.79×20.5/29.28).
3. ГЛАВНЫЙ ФАКТ: dp50k FluidPush 10-11% ≠ банк 2.62% → банк-CENS item⊕inside (+19.47) не переносится; 3-лейн компо теор-макс +27.6пп.
4. 5-лейн компо (item⊕inside⊕fluid⊕lookup⊕collide) при f=0.5: x=18.9% → +23.3пп — бар достижим на умеренных capture-факторах.
5. Солo-ItemEntity суб-бар (+2.7пп); merge-скан мёртв (0.01%, канон C17.2) — таргет-1 = компо-залп, не item-соло.
6. Гейты w527: census FluidPush item/mob-сплит (главный риск переоценки), javap idle-гейт в dp50k-блобе, NCDFE=0 EARLY-define, пары min-of-3.
7. σ_run dp50k CV17-20% (AG-16/150/216) — вердикт только pair-math, 1-нога неразрешима (AG-107).
8. CAS-append доски штатен: GET blob-sha → PUT, дедуп-чек своих строк перед PUT (board_append.py /tmp/ag263).
9. Файлы PUT в мастер через contents-API легальны (прецедент work/AG-* мастеров); локальный git-коммит доски/веток — запрет (v23.1).
10. Дедуп до CLAIM: regex-boundary по 'compo|capture-math|ItemEntity' — тема чиста была (3 строки всей истории).

--- APPEND AG-263 w527 (merge-arb exec, 2026-10-03) ---
1. Contents-CAS 409-штампед реален: retry GET→append→PUT циклом, floor-guard len>700k.
2. git merge-tree --write-tree A B: rc=0/одна строка = 0 конфл; tree oid совпал с live-мёржем бит-в-бит.
3. Старая ветка: diff vs master врёт (128k del) — реальный патч = diff merge-base..branch.
4. PyYAML: `on:` = True-ключ; inputs на workflow_dispatch.inputs.{}, не на уровне триггера.
5. mode 100755→644 не конфликт при односторонней правке; `bash script`-канон делает exec-бит ненужным.
6. POST /merges base/head = атомарный API-мёрж; post-verif = ls-tree blob-shas live == предсказанному дереву.
7. Реестр врёт: AG-263 занят w526-артефактами (MEMORY/clm 422 при new-PUT) — grep до POST, append с wave-хедером.
