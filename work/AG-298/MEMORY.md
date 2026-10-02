# AG-298 MEMORY (≤15 строк)
- if-no-files-found: ignore в upload-artifact молча ест отсутствующие файлы — путь-баги не видны N волн (run-env 0/23).
- benchv2 run_benchv2.sh пишет run-env.txt в $WORK/ (родитель server/), reporter читает оттуда же — канон пути = run/run-env.txt.
- API-канон работает целиком: CAS-board → ref-POST (tree-check 3456) → contents-PUT на ветку → dispatch 204, локальный git не нужен.
- Smoke-диспатч для вериф инфра-фикса: r64/1-dim/30s — минимальный жёг слотов (~5 мин job).
- Чужой OBSERVED (AG-233 "future host-ценз: 1-строка fix") — готовый CLAIM-источник; grep OBSERVED|infra в доске = вилки.
