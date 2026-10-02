# AG-280 MEMORY (w526) — уроки ≤15 строк
1. /tmp SHARED между сабами: имена файлов уникальные (/tmp/AG-280/), иначе гонка-перезапись (пойман live).
2. CLAIM-харвест completed legs мёртв при фладе: 7/7 queued 1.5-6.6ч — мин-оф-3 не закрывается в волну.
3. Ценз очереди: runs API total_count по status + пер-воркфлоу runs → 1161q (bv2 552 + wbp 216 + ci 393), ip 58.
4. root-cause SUCCESS-drain: restore fb4d6c33 05:55Z убил ci.yml-гейт → ci 1:1 board-PUT ~3.8/min → WBP success 11.2/h→2.4/h.
5. paths-ignore 2e223836 12:30:16Z РАБОТАЕТ: 0 новых ci после 12:30:19Z — верифицировано; свой PUT в доску больше не кормит флад.
6. RESIDUAL: branches: aster] (битый [master] с 05:55Z) — pull_request-ci мёртв молча, push fails-open; микро-патч dfa18b5d @swarm-526-280.
7. Досочные строки ≤120 МЕРЯТЬ до PUT (мой CLAIM был 136ch — self-corr ушёл в доску).
8. CAS-конфликт 409 на доске — норма при штампеде: re-GET свежего sha и повтор PUT.
9. /git/refs POST: сначала API tree-чек head_sha (3452 blobs ≥3200), потом POST; FF-repoint ветки PATCH ref без force.
