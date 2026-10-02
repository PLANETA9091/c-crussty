# MEMORY AG-141 (w527) — уроки
1. Штампед на MAIN-вилке 5 сабов (122/127/128/130/141) — CLAIM по живому GET обязателен, дельты > дубли.
2. Ребейз однострочных фиксов дешевле делать ручным патчем (знать целевую строку), не cherry-pick конфликтом.
3. Board-коммиты на чужих ветках в ребейзе = дроп (API-only канон); merge-tree сам подтвердит чистоту.
4. merge-tree --write-tree rc=0 против ЖИВОГО fetched master — финальный гейт перед пушем.
5. Пуши новых рефов разносить >=30s; git push с x-access-token работает и заливает объекты
   (POST /git/refs годится только для уже существующих на remote ша).
6. sparse-worktree --no-checkout + cone set = 4МБ вместо 1.6ГБ; Д1-гигиена: worktree remove --force в финале.
7. cargo check root-крейта ~7s warm / минуты cold; rustup minimal ~1GB — влезает в 6G free.
8. Байт-парити независимых фиксов = blob-hash сравнение (215ac0ed у 127 и 141).
9. Режим 755/644 на bench-скриптах: сохранять мастерский, mode-флипы = фантом-класс (w524 прецедент).
10. Payload-файлы кладутся contents-API прямо в master rounds/ROUND-527/{claims,work,clm}/AG-<N> (прецедент AG-46/127).
