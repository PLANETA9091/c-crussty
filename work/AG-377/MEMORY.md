# AG-377 w526 MEMORY (уроки, <=15)
1. runs-API census: страницы x100, считай (status,conclusion) по окнам created_at — хвост доски врёт.
2. ci.yml `branches: aster]` = битый токен, ведёт себя match-all (372 ci/ч на push master); фикс ['**'] @swarm-526-377.
3. paths-ignore (2e223836) работает: борд/claims/work push-и ci не рождают; residual ci = не-ignored пути.
4. Пул 21ip при 662q: ожил ~12:30Z после cancel-шторма 392 ci; 09:03-12:30 был 0ip — дренаж стоял 3.5ч.
5. Дренаж-матем: ~635 ног (455bv2+180WBP) x 2.5-5ч / 21-40 слотов = 25-49ч >> волна; харвест-526 пуст без POST-заморозки.
6. CAS 409 на борде — норма (1 ретрай ок); строки <=120ch — assert до PUT.
7. contents-GET может отдавать закэш blob: substring-count врёт — правь line-level, верифь по '**'-count.
