# TASK-400 VERDICT (тик 12:08→14:1x +08, cron 401462, 2026-09-21)

МЕГА-РАУНД-4: 10 векторов + ВВЕДЕНА ПАРНАЯ МЕТОДИКА (урок TASK-399 ±14пп).

## МЕТОДОЛОГИЧЕСКОЕ ЯДРО ТИКА
6 якорных ног same-day (master 54806ad, банк v4 чистый, lever=""):
2.40@7071566 / 2.30@6940803 / 2.40@8344070 / 2.35@7112096 / 2.20@6811622 (+fleg3 контекст).
- **anchor-vs-anchor шум ≈ ±2пп** (a1 2.40@7071 vs anchor4 2.35@7112 — почти один runner).
- **Линия TPS_exp завышает дельты ~+5пп** (ваниль сегодня бьёт линию: a1 +4.9%).
- Вердикты отныне ТОЛЬКО pair-by-runner (ближайший индекс) + min-of-3.

## КАРТА ВЕКТОРОВ (pair-вердикты)

| Вектор | Флаг | Ноги (tps@runner) | Pair-вердикт |
|---|---|---|---|
| A bfcomp (B+F композит) | cmp399_bfcomp | 2.20@6191083 | line +4.6%, близкого якоря нет → КАРТА (не доказан) |
| B navfix navstagger | cmp399_navstag | 2.30@6636044 | pair ≈0..+4.5%; nav_ai 14.16→10.14pp РЕАЛЕН, но поглощён другими лейнами → КАРТА |
| C devirtfix | cmp399_devirt | 2.60@8780288 | line −1.8%, pair-relative ≈+4.4% → КАРТА |
| D wakefix | cmp399_wakeup | 2.20@6616394 | pair 0.0% → PARITY (но крэш спавна ИСправлен — инфра-материал) |
| E sensebatch | cmp399_sensebatch | 2.60@7250 / 2.15@6658 / 2.65@7199 | pair +8.3/−2.3/+10.4 → **min-of-3 = −2.3% REFUTED** (высокая дисперсия) |
| F collidesweep | cmp399_coll | 2.20@6548779 | pair 0.0% → PARITY |
| G typesec | — | — | DROP (research 11KB сохранён; имплементация не начата) |
| H actrange DAB | cmp399_dab | 2.30@7023958 | ARMED, pair −4.2% → PARITY **bench-структурно**: все 150k внутри activation-боксов radius 640 — подавлять нечего |
| I jnibulk | cmp399_jnib | — | INFRA-DROP (java-драфт 1a3b37c: raw-arena + coarse stamps; rust-сторона не успела — unsafe не спешим, урок J SIGSEGV) |
| J mobpush grid | cmp399_mobpush | 2.35@6410427 / 2.10@5946251 / fixture-INVALID | best pair +6.8%, лейны НЕ подтвердили механизм (broadphase flat) → UNPROVEN, min-of-3 не достигнут |

**МЕРЖ НЕТ** (бар 10% не взят никем; директива владельца 14:0x: бар теперь **80%**).

## ROOT-CAUSES ТИКА (материал)
1. **navstagger NOT-ARMED (фейковые +12.6% TASK-399)**: matcher сканировал iconst_2/iconst_4 как 0x06/0x08 (JVM-таблица: 0x05/0x07) → ноль совпадений на реальном kernel Mob.class. Фикс 16d7cdb + roundtrip + 202 теста.
2. **wakeup pop=INVALID**: полу-вооружённый item-мост — rust items_manager gate расширен до cmp399_* при java ENABLED=false (CP-патч не покрывает cmp399_wakeup) → каждый addEntity кидал AIOOBE(-1, 16385) во время inject → население не дошло до 150k. Фикс 67084fb: rust gate → eq(items_subsys2), двойные гейты обязаны переключаться синхронно.
3. **sensebatch NoClassDefFoundError mid-population**: unconditional beginBucket lazy-resolve; фикс b37ab70: SenseBatchOps в bridge_prelist ДО RegionTickOps (флаг-гейт).
4. **Adapter-timeout 10 Task-вызовов (3-й тик подряд)**: агенты доделали пайплайны фоном (10 worktrees, 8 research, 6 веток push, 6 собственных диспатчей!) — верхний агент собрал артефакты и дотянул. Вывод в промпт крона v13: волны ≤3, «мёртвый вызов ≠ мёртвый агент», рестарт с контекстом, давление на слабых.

## ДИРЕКТИВА ВЛАДЕЛЬЦА (2026-09-21 14:0x +08)
- **БАР 10% → 80%**: проект выходит из маленьких изменений; сабагент не может закончить работу ниже 80%; цикл сабагента: бенч → Δ<80% → новый рисёрч → имплементация → снова бенч.
- Верхний агент рестартит и давит на слабых сабагентов (не подсказывая техник).
- Оптимизации обязаны чувствоваться как ванильный Minecraft.
- Промпт крона обновлён (v13, job 402447, взамен 401462).

## NEXT (TASK-401) — под бар 80%
1. **Композиции — единственный честный путь к 80%**: B(+4.2pp) + F(+2.2) + navstagger(0..+4.5) + devirtfix(+4.4) + mobpush(?) — собрать общий композит на базе round-400-a-bfcomp и реплицировать pair-методом. Каждая ступень — карта, мерж только на 80%.
2. mobpush min-of-3 доделать (fixture-INVALID root-cause: spawnable chunks/churn/alive-check — сверить с wakeup-уроком: не ломает ли grid alive-check при inject).
3. jnibulk rust-сторона (stamp-table + arena native) — скрытая цена 12-17% wall, самый большой нетронутый горшок.
4. actrange непомеряем на этом харнессе — либо расширить bench-мир (требует владельца), либо списать вектор.
5. typesec имплементация (research готов).
6. Якорная база: 6 ног same-day уже в банке; новым тикам — 2-3 свежих якоря + pair-law.
