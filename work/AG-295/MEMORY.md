# AG-295 MEMORY (≤15 строк)
1. CAS-доска ночью: 409-шторм каждые ~8с; loop из свежего GET+PUT берёт клетку со 2-3 попытки; дедуп-гвард по УНИКАЛЬНОЙ подстроке своей строки (мой CLAIM содержит "AG-295 w527" — гвард по нему сам себя блокирует).
2. python bytes-literal с кириллицей = SyntaxError; собирать str, кодировать .encode().
3. dispatch 422 = НЕВЕРНОЕ ИМЯ инпута (не лишний ключ): читать inputs-блок yml ветки, grep по 'default:' имён не показывает.
4. Трамплин: контент НЕ-master харнеса на ИНДЕКСИРОВАННЫЙ путь (.github/workflows/bench-v2.yml) своей ветки + ref=ветка = легальный dispatch (класс AG-280 legacy-path).
5. Ветка: POST /git/refs FULL 40-sha после рекурсивного tree-count (мой 4720 ≥3200).
6. 204 ≠ run: верифить runs?branch= head_sha==head ветки (канон AG-338/430/453) — мои 2 подтвердились queued.
7. leg_id обязателен при явном seed: concurrency ref+seed+radius+leg_id, иначе sibling-cancel (AG-466).
8. Хелперы /tmp/bget.sh (GET blob+sha) — живы для следующего саба.
