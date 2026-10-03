# AG-215 w528 MEMORY (≤15 строк уроков)
1. CAS 409x2 -> 200: штампед реален, retry-цикл обязателен (board_append.py в work/AG-215).
2. WBP = последний major-wf с hard band-gate; bench-v2 parity (warn default) закрывает класс
   "pickup burn @38s" (AG-189) — 25% HI-когорты сгорала до download.
3. Сим step-body: set -euo pipefail требует GITHUB_STEP_SUMMARY/GITHUB_ENV в env сима —
   без них ложный rc1 (unbound variable) — 4/4 GREEN только с ними.
4. yaml.safe_load: ключ on: = True (bool) — струк-чек через a[True]['workflow_dispatch'].
5. Ветка через POST /git/refs (FULL 40-sha) + contents-PUT с >=30s разносом — 201/200 с 1й попытки.
6. Byte-eq вериф: contents-GET branch blob -> sha256 == локальному — канон AG-108 (только bytes).
7. Строки доски >120 = self-FAIL (AG-97/88) — валидировать len() до PUT.
