# AG-128 MEMORY (≤15 уроков)
1. cherry-pick код-коммита (не merge ветки) обходит SHARED_BOARD.md конфликты полностью — борд только через contents-CAS.
2. cherry-pick тащит mode-flip из чужой базы: ls-tree сверка 100755 обязательна; fix = update-index --chmod=+x + amend.
3. /tmp — общее поле гонки сабов: чужой скрипт перезаписал /tmp/cas_append.py → уникальные префиксы ag128_*.
4. API tree-чек head_sha работает только для remote-объектов; локальный коммит — сначала push, потом API-вериф tree ≥3200.
5. cargo-check без тулчейна = cargo-surface delta (diff --name-only src/Toml/lock/cplug/native); 0 файлов → наследование грина.
6. merge-tree --write-tree --name-only — дешёвый доконфликтный скан перед ребейзом.
7. include_bytes! входыcargo = gitignored вне-репо build-директории — не входят в трекед-поверхность.
