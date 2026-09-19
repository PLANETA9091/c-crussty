# RECON-13 — absorb FREE-HOST 2.5GB лега s7167 (run 35430216073, head d19bbd0)

- heap: xms=2G xmx=2G (пегистер: оба 2G)
- PG-A доставка: pop INVALID; ARMED no-sbb: да; NCDFE: 0
- ВЕРДИКТ: **INFEASIBLE-BY-MEMORY** (окно (a)) — conclusion=failure, OOM-маркеры: [('server-stdout.log', 'OutOfMemoryError: (\\w+ ?\\w*)', [('Java heap', 121)])]
- Следствие: сцена 150k не помещается в класс контейнера 2.5GB; варианты владельцу: сниженная популяция free-host трека ИЛИ 10G-класс хостинга. FREE-HOST A/B база = недоступна.
## Детали смерти (тик 16:2x)
- **2,611 завершённых Pause Full GC** (10G-база: 0) + **121× OutOfMemoryError: Java heap space** в stdout = GC-спираль смерти, контейнер-класс 2.5GB не переваривает inject 150k
- POPULATION INJECT НЕ завершён (смерть до/во время инъекции → pop INVALID в PG-A — ожидаемо при OOM)
- Подтверждение прегистера: окно (a) INFEASIBLE-BY-MEMORY — валидный результат RECON-лега, не отказ инструмента (PG-A доставка ARMED/NCDFE=0 чиста)
