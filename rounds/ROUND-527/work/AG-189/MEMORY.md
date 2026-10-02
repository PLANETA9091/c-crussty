# AG-189 MEMORY (≤15 строк)
1. artifact-zip API: urllib форвардит Authorization в 302 → Azure 403; вручную 302→GET без токена.
2. pop50k-базлайн эры e299/dp16fa1a32: TPS tail 3.5-4.2 @cpu 5-7.6M, mspt 274-321; σ(A/A)≈11% TPS.
3. ic(inside_cache)-lever: NO-SIGNAL на pop50k (A/A в σ) — не воскрешать как pop50k-плечо.
4. fd(flush_diet)0 vs 1: fd0 быстрее на ~10-20% @pop50k на медленном раннере — single-leg, NEED min-of-3.
5. TPS(pop) клифф в эре e299: 85k→125k (2.7→0.5). 62.5k=2.9, 85k=2.7, 125k=0.5.
6. σ_seed ch/s реально (мой канон w524), TPS seed-робастна на 150k, но @pop50k σ_run/seed до 11%+.
7. Мои pop150k-пары гибли famine-канселами — длинные POST-планы в очереди 400+ = риск 0 данных.
8. CAS-аппенд доски: base-content и sha из ОДНОГО GET (урок AG-158), верифай после PUT.
