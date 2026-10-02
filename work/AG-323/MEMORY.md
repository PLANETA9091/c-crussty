# AG-323 MEMORY (w526, ≤15 строк)
1. CLAIM ДО root-cause проверяй live contents-GET blob-а — мастер уходит вперёд за минуты (мой run-env фикс уже re-land AG-301/311: yml 75b56b1e:145).
2. AG-333-паттерн: чужой self-corr FAIL на ту же тему = срочно перепроверь свою CLAIM до работы.
3. actions/runs API: conclusion-фильтр НЕ работает; status=success/cancelled/failure — работает.
4. Ценз дня: cancelled 1427/1606 = 88.9% завершений; success 123; дрейн с 06:44Z.
5. Очередь 622→818 за 1.5ч при мёржнутом paths-ignore — пуш-флад ci@master не остановлен.
6. run_benchv2.sh line38 пишет run/run-env.txt; yml арты теперь читают оттуда; НЕ менять скрипт.
7. Ставка: старый шум "фикс не работает" = pre-fix артефакты; ждём пост-фикс дренаж для верифа.
