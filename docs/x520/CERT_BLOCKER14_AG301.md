# CERT: BLOCKER #14 — DimForceload sync-ticket hang → Paper watchdog kill (AG-301, волна-520)

## Метод
Харвест 5 completed-failure ног ×520 + 3 валид-леги фикс-веток (52/70/107). Скачаны job-логи
+ артефакты benchv2-ag433 (server-stdout.log) в work/AG-301/ (job_*.log, art70/, art107/).

## Вердикты 5 ног (быстрые)
- run-36806846385 (249), 36806683248 (435), 36807531651 (438), 36807204643 (445b): exit 1,
  BAND-DISCARD — runner_cpu_index {11558383, 10975247, 11493012} вне 6.0–9.5M (эпидемия пула
  10.2–12.5M подтверждена ×3 новых точки). Remedy: runs-on-pin ubuntu-24.04 (6.95M).
- run-36808544224 (358): exit 43 G-DFLOAD «Initialized 0 plugins» — блокер #8 (ветка без plugin.yml фикса).

## ГЛАВНОЕ: валид-леги #8-#12 фиксов ВСЁ РАВНО падают — следующий блокер цепи = #14
- AG-70 valleg run-36805200406 (swarm-520-70): `Initialized 1 plugin` 02:23:09 → Server thread
  dump с 02:24:20 каждые 5с (≥13 шт) — Server thread ЗАВИС в
  `CraftWorld.addPluginChunkTicket(CraftWorld.java:582)` ← `getChunkAt` → `syncLoad` →
  `managedBlock` ← `DimForceloadPlugin.lambda$onEnable$0(DimForceloadPlugin.java:83)` →
  `Stopping server` 02:25:12 → сервер МЁРТВ на 02:25:15, харнесс жёг ещё 34 мин →
  marked=0, G-DIM пуст, spark n=0, DRAIN-TIMEOUT 1200s, G-HB «PASS» 1097 (ЛОЖНО-ЗЕЛЁНЫЙ —
  hb пишет сэмплер харнесса, не сервер).
- AG-107 valleg run-36805421425 (swarm-520-107): ИДЕНТИЧНАЯ сигнатура (dumps 02:25:10,
  Stopping 02:26:22) — детерминировано ×2 ветки.
- AG-52 valleg run-36803689619: G3 1/4 (нет report-фикса #9) + тот же zombie-профиль.
- Прогноз: AG-422 valleg run-36806981830 (стек #8+#9+#10) упадёт ТАК ЖЕ (#14 не в стеке).

## Механика #14
Мастер-плагин (onEnable runTaskTimer 40L/60L) при наличии dimload.start добавляет ВСЕ
20449×3 тикетов в ОДИН тик; CraftWorld.addPluginChunkTicket синхронно грузит чанк на
главном потоке → тик >60s → watchdog kill. Баг невидим на canary-3 (плагин мёртв #8),
вскрывается ЛЮБЫМ стеком с фиксом #8 → canary-4 на мастере без #14 обречён.

## Фикс (ветка swarm-520-301, commit 9eb4f7ee, дерево 3233 == master, javac 20 err == base)
- Батчирование: ≤128 тикетов/вызов (env DIM_MARK_BATCH), период 10t, курсор на мир,
  row-major порядок → ~1.3s синк-лоадов/тик worst-case, watchdog не задеть.
- Marked-emitter: ровно одна строка `[DimForceload] Marked 20449 chunks world=<w>` на мир
  (3×20449=61347 ≥ 58272 → G4 PASS) — закрывает и #12 (эмиттер).
- Шампион-стек для canary-4 = #8 (plugin.yml) + #9 (g3 occurrences) + #10 (spark tps
  «Tick durations») + #11 (финальный dimchunks) + #12 (Marked) + **#14 (батчирование, эта
  ветка)** + сервер-liveness гейт харнесса (fail-fast exit 45 при «Stopping server» в stdout
  — иначе zombie-профиль жжёт 30+ мин и G-HB false-green).
