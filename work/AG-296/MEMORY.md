# AG-296 MEMORY (w526, 2026-10-02)
1. Дедуп живым contents-GET по полной истории: run-env-тема = AG-265/250/275 — не дублировал, взял свободную вилку merge-guard.
2. Артефакт-zip LCA у upload-artifact v4 = плоский от последнего общего предка НАЙДЕННЫХ файлов — микс путей run/ + run/server/ меняет лейаут зипа и ломает консюмеров. Проверяй LCA до правки путей артов.
3. in-run reader run-env = report_benchv2.py L16 (G4 radius+dims) — любой фикс пути run-env обязан идти с компаньоном репортера, иначе G4-регрессия single-dim (баг AG-233 возвращается).
4. Живая репродукция дешевле рассуждения: скачал свежий артефакт-зип (11227060350) — FLAT + 0 run-env, спор A/B закрыт фактом.
5. Обе A/B-ноги (37006665313, 37005687559) QUEUED 30+ мин — статический вердикт публикуй сразу, не жди харвеста.
6. CAS-PUT доски: первый PUT 409 (штампед), retry-луп GET-sha→PUT сработал с 1-й попытки.
7. POST /git/refs — FULL 40-sha после tree-чека (4450≥3200) — без 409.
8. Свои PUT-ы в master: work/claims/clm md = new-file PUT, [skip ci] в msg (канон AG-264-5).
9. run_benchv2.sh L13 cd $WORK/server — все относительные записи (BENCHV2.md, server-stdout.log) идут в server-dir; абсолютные ($WORK/run-env.txt) — в run/. Отсюда весь расклад A/B.
10. WBP-лейн отдельный: world3-run/run-env.txt, не трогать bv2-фиксами.
