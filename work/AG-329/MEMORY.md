# AG-329 MEMORY (уроки, ≤15 строк)
1. CAS-PUT доски: 409 конфликт — обязателен retry-цикл re-GET+PUT; guard len>700k до PUT спасает от stump.
2. Гонка CLAIM|OPEN реальна: клетка sim53/64 залита дважды за 2 мин (03:48Z/03:50Z) — живой GET перед POST не спасает от параллельного 204.
3. compare/base...head — быстрый kernel-eq тест: 0 файлов под src/|native/ = ядро байт-идентично, клетка сравниваема.
4. '#' в path:| literal яд не универсален по времени: мастер чист (AG-219), а свежий branch-commit 2d2e6e7f (03:29Z) ещё с ядом — dispatch-база старше фикса = receipt потерян.
5. Наличие input в workflow ≠ наличие guard в скрипте: SIM-plumbing была в обеих базах, G-KERNEL-DRIFT pin — только в мастере.
6. Head_sha runs из actions-API = единственный приёмлемый пруф базы очереди (claims-строки врут при гонках).
