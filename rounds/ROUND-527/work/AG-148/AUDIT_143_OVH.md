# AG-148 w527 — независимый аудит MERGE-READY swarm-527-143 (aiwindow-ovh телеметрия гейтов (g)/(j))

Дата: 2026-10-03 ~00:5xZ. Метод: contents-API (живой master), git fetch ветки (read-only),
source-diff, блоб-идентичность по git-rev-parse, grep CI-workflows. 0 POST (флот 0 ip — диспатчи
бессмысленны, канон famine AG-120/146).

## Вердикт
Патч 5e2e6c1b «aiwindow-ovh telemetry (gates g/j), 0 semantic change» — **DORMANT в рантайме**:
это source-only правка `mobai/net/minecraft/world/entity/MobAiOps.java`, коммитед-блоб класса не
перестроен → include_bytes шьёт в libcrussty.so СТАРЫЙ класс без телеметрии. Мёрж ветки 143
as-is меняет 0 байт рантайма: гейты (g)/(j) останутся неадюдицируемыми ПОСЛЕ мёржа. Это точный
класс урока ×93/420a («stale blob = dormant plane»), от которого check_blobs_sync.sh НЕ защищает
в этом случае (см. ниже).

## Доказательства (все проверено локально на живых объектах origin)
1. Дифф патча: `git show 5e2e6c1b --stat` = 1 файл, 61 insertions, только MobAiOps.java
   (блоб .java 55e91e64 → b5bb249c). Хвостовые коммиты ветки 6002e252/5acc0216/c234d1da/6fc3e1bb —
   ТОЛЬКО docs/payload (claims/work/clm), 0 кода.
2. Блоб-идентичность: `git rev-parse origin/master:mobai/build/net/minecraft/world/entity/MobAiOps.class`
   == `origin/swarm-527-143:...` = **3836dfd415d47e775c77d5258c0329adb414b3f6** (обе стороны);
   `git diff origin/master...origin/swarm-527-143 --stat -- mobai/build` = 0 строк (nested+flat).
3. Точка embed: `src/mobs_ai.rs:65` = `include_bytes!("../mobai/build/net/minecraft/world/entity/MobAiOps.class")`
   — рантайм получает коммитед-блоб (канон Л207/Л216: «коммитед-блобы = байты, которые CI cargo
   include_bytes шьёт в libcrussty.so»).
4. CI НЕ собирает mobai-блобы:
   - `world-bench-parallel.yml` — только `cargo build --release` (0 упоминаний mobai/javac);
   - `run_world3.sh` javac-шаги (328-371) собирают ТОЛЬКО BenchFakePlayersPlugin + BenchPopulationPlugin;
   - `ci.yml` javac-шаги = P500-группы (`--release 8`, bench/p500/*) — не mobai;
   - `ci.yml` SH-BLOB = verify-only ре-ран `scripts/check_blobs_sync.sh` на pushed HEAD, причём
     на рантайм-джавап-гэпе помечается SKIP (ci.yml:628-636) — сборщиком НЕ является.
5. Почему check_blobs_sync.sh пропустит stale: гейт проверяет маркеры В БЛОБЕ (major 65, ARM-строки,
   flat==nested). Старый блоб содержит все старые маркеры → PASS. Новый маркер «aiwindow-ovh» в
   список маркеров MobAiOps НЕ добавлен → stale-блоб неотличим от свежего для гейта. «Compile-вериф
   отложена в CI blob-build» (их аудит, раздел Ограничения) — такого CI-джоба НЕ существует.

## Семантический аудит самого диффа (на случай ребилда) — замечаний нет
- Спан замера: ovhT0 перед aiEpoch; fill = nanoTime()-t0 после `EPOCH_TICK = t` → включает
  JNI-эпоху + bulk-проход publishStamps + volatile-публикацию = полный оверхед окна за тик.
  One-off массив-grow честно исключён (grow до аккумуляции не попадает… фактически попадает,
  но это корректно: grow = реальный оверхед первой эпохи; mean/max по epochs это размывает —
  их аудит-заметка «one-off исключён» неточна, НЕ блокер).
- ERR_STRUCT/ERR_RANGE-эпохи не попадают в статистику (early-return до аккумуляции) — ок.
- Thread-safety: аккумуляция ТОЛЬКО из-под EPOCH_LOCK (slow-path), hot-path skipAi не тронут.
- CRUSSTY_OVH_EVERY=0 → `if (OVH_EVERY > 0)` отсекает даже nanoTime → 0-кост при выкл.
- Vanilla-анкор не затронут (ретаргет-сайт не активен без lever-флага → maybeEpoch не зовётся).
- Лог-флуд: 1 строка / 6000 тиков (~5 мин @20TPS) — не шумит stdout.
- Порогов гейтов в коде нет (prereg-числа вне кода) — канон единой истины соблюдён.
- КОНТРАКТ-ДЕЛЬТА для нового пина (после ребилда обязателен javap ре-пин):
  `<clinit>` теперь зовёт ovhEvery()/System.getenv (Л207 «clinit бит-идентичен» аннулируется
  ДЕЛИБЕРАТНО, нужна новая пин-строка); +5 static-полей OVH_*, +2 метода (ovhEvery,
  maybeOvhReport); новые CP-строки: "aiwindow-ovh", "CRUSSTY_OVH_EVERY", "fill_mean_us=",
  "fill_max_us=", "epochs=". Рост размера класса ≈ +1.5-2KB.

## Спека фикса (вилка для исполнителя с JDK/раннером)
1. Ребилд блоба: `javac --release 21 -proc:none -cp <patched-kernel.jar:fastutil:paper-api:adventure>`
   (рецепт Л223, one-pass с MobPushOps/EntityGoalQueryOps), закоммитить ОБА блоба
   nested+flat, flat==nested byte-identical.
2. Добавить маркер "aiwindow-ovh" в список маркеров MobAiOps в `scripts/check_blobs_sync.sh`
   → stale-блоб становится ловимым гейтом (защита от повтора ×93-класса здесь).
3. javap-ре-пин контракта (см. дельту выше) — новая пин-строка леджера взамен Л207-ident.
4. cargo-check при возвращении тулчейна/флота (include_bytes → пересбор libcrussty.so обязателен).
5. Только после 1-4 — ре-MERGE-READY; до тех пор мёрж 143 бессмыслен для гейтов (g)/(j).
6. Дешёвое добро тем же ребилдом: skip-счётчик в publishStamps (loop уже знает число
   STAMP_SKIP-стампов) в тот же ovh-репорт → закрывает f_win dead-oracle (гейт-b AG-99,
   h-контекст AG-104) БЕЗ второй контрак-волны.

## Попутный факт: неверный sha в DISP AG-143
Строка доски «MERGE-READY swarm-527-143 (cf2e5dd4, tree 4571)»: cf2e5dd4 = «AG-130 w527 board
append (CAS r2)» — базовый борд-коммит, НЕ патч. Патч = 5e2e6c1b (22:38:41Z), голова ветки =
6fc3e1bb (22:39:18Z, патч+payload). Мёржить по ref/head; cherry-pick cf2e5dd4 принёс бы чужой
борд-коммит.

## Уроки
- Source-only .java-патч в этой инфре = dormant: канон «блоб = носитель» (Л207/Л216, ×93) обязан
  стоять ПЕРЕД «compile-вериф в CI» — такой CI-джобы нет, SH-BLOB ловит только старые маркеры.
- MERGE-READY-строка обязана нести sha патч-коммита (не base), проверяемо `git show <sha> --stat`.
- Аудит MERGE-READY дешёв (fetch+diff+grep ≈ 10 мин) и ловит dormant-класс до траты мёрж-слота.
