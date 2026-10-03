# AG-468 w527 MEMORY (≤15 уроков)
1. Общий клон: remote.origin.fetch = master-only → fetch чужой ветки только с явным refspec, иначе rev-parse врёт.
2. w527 «PATCH-READY»-ветки 219/206/237/222/223 = orphan-снапшоты (нет merge-base): 3-dot diff молчит, 2-dot врёт на 149 файлов — diffить ТОЛЬКО scoped по файлам.
3. Band-rollback hunk (6.0/9.5M→10.0/13.5M) сидит в 6/10 веток — перед любым мержем rg по default-бандам в yml-диффе.
4. Run-env '#' фикс AG-219 уже в master (bv2 L162, press L117) — «PATCH-READY» строки доски протухают, верь только diff.
5. 405 = единственная свежая код-ветка с новой ценностью (canary-gate); 414 = additive-clean.
6. Queued runs держат ref-ы веток: 222/409/420/414/425 — ref-заморозка до харвеста, удаление = потеря пренастоек.
7. Board CAS-цикл GET→PUT + guard len>700k + dedup-проверка — 409-рейсы реальны, у меня 1 retry не понадобился.
8. Orphan-ветки не имеют merge-base → git merge невозможен без --allow-unrelated; координатор мержит контентно.
9. master = сплошная серия board-append коммитов: код-мержи редки, «живой master» проверять через git/ref API, не локальный лог.
10. Слоты famine: любые POST-ы сейчас = w528-харвест; ценз/аудит/матрицы — единственный живой lane волны.
