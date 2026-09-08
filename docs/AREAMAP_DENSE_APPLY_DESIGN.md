# AREAMAP DENSE APPLY — engineering design (diff-budget window, TASK-20-R / wave-3)

Agent: subagent-design (session S7-16, task 2-a) · 2026-09-09 · base: c-crussty master (research-only, no source modified)
Status: **DESIGN** (реализация не начата; всё ниже — доказательства + план)
Target lever: `docs/RESULTS_LEDGER.md` §5, строки 117–119: *«Area-map diff-budget window (~8·d bytes vs
cap=2·px, 64–256x fewer JNI bytes/call) … the fix needs a bridge+`.so` change»* + `docs/HOTSPOT_CANDIDATES_V2.md:55`
(TASK-20-R) + `bench/areamap/results/APPLY_BENCH.md` §Negative result.

---

## 0. TL;DR — вердикты

| Вопрос | Вердикт |
|---|---|
| **Byte-math** | «~8·d **bytes**» — **опровергнуто в буквальной формулировке, подтверждено как отношение элементов**: реальная формула today = `9·L·k` байт на вызов, где `L = GetArrayLength` (grow-удвоенный `cap = 2·px`, в проде `L/cap ≈ 1.13–1.15`), `k ∈ [1,2]` (copy-in only vs in+out). Полезная нагрузка = `9·n`, `n = 4d+2..8d+2` ops (1-chunk walk). Отношение окон: **64×/256×/512×** при d=63/255/511 (ledger цитирует 64–256 для d=63..255). |
| **Dense encoding** | Выбор: **2D run-length (построчные x-сегменты разности двух квадратов), вычисляемая в closed form** → **0 bulk-JNI байт** (нативный вызов исчезает целиком, не «ужимается»). Hard cap 0 байт ≪ 2·px. |
| **Body placement** | (a) **plugin-domain**: helper-класс, компилируемый в репо и `include_bytes!`+`define_class` в loader карты — точный прецедент `SingleUserAreaMapOps` (`src/area_map.rs:36-44,142-158`); wiring — same-descriptor `retarget_invokestatic` (прецедент G4: `src/improved_noise.rs:61-74`, `src/classfile.rs:725`). Для (опционального) нативного B-варианта — Rust fn в **libcrussty.so** + `RegisterNatives` локальных fn-указателей: прецедент `src/batch_api.rs:654-667`. **CRUSSTY engine .so НЕ нужен**; утверждение `HOTSPOT_CANDIDATES_V2.md:55` «unbuildable without the closed .so sources» — **опровергнуто** для этого lever'а. |
| **Projected win** | Бенч-гриды CHANGED: grid128 ≈ **8–30×**, grid512 ≈ **80–350×**, grid1024 ≈ **150–700×** (широкий band — см. §7 про неизвестную C = стоимость callback'а). MIX50 — те же ×2 к числителю. Прод-формы (d≤33, 6 карт/игрок): ≈ **3.5–15×** за событие пересечения границы, абсолютная экономия ≈ 12–16 µs на crossing player-tick. |
| **NO-GO** | 5 явных выходов (§9), включая честный главный: **на прод-дистанциях (d≤33) >100× НЕ получается** — >100x-class существует только на синтетических гридах d≥255. |

Позиционирование против `RESULTS_LEDGER.md` §4 limit 2/3 («pipeline пуст без ENGINE-TOUCH»): этот lever был
классифицирован как engine-domain потому, что фикc предполагался **внутри закрытого .so**. Дизайн ниже показывает,
что new dense body целиком живёт в plugin-domain (Java-class + при необходимости Rust fn в libcrussty.so), т.е.
lever **переоткрывается без ENGINE-TOUCH** — на бенч-гридах d≥255 он даёт >100x-class.

---

## 1. Контекст

### 1.1 Как работает apply сегодня (цепочка вызова)

1. Kernel `SingleUserAreaMap.update(III)` байт-патчится в `invokestatic SingleUserAreaMapOps.run(map, fromX, fromZ, oldD, toX, toZ, newD, param)`
   (`src/classfile.rs:397,420-421,436,490-491`; hook: `src/area_map.rs:51-74`, activation `:79-183`).
2. `SingleUserAreaMapOps.run` (исходник `area-map/ca/.../SingleUserAreaMapOps.java`, байты `include_bytes!` — `src/area_map.rs:36-44`):
   guard `fromX==MIN_VALUE` (:69-71), same-state fast path `from==to && oldD==newD` (:79-81, O(1), **не трогаем**),
   grow-only ThreadLocal scratch (:87-94, `INITIAL_CAP=578` :34, удвоение), затем
   `PaperNativeAreaMap.nativeUpdateOpsBatch(fromX, fromZ, oldD, toX, toZ, newD, ops[], keys[]) -> n` (:97).
3. Натив (закрытый `native/libpaper_native_jni.so`, экспорт `Java_ca_spottedleaf_moonrise_common_misc_PaperNativeAreaMap_nativeUpdateOpsBatch`
   — `native/JNI_EXPORTS.manifest:118`) пишет в `ops[i]` байт (0=Add, иначе Remove) и `keys[i]` long (`z<<32|x`)
   и возвращает n.
4. Java apply-loop декодирует пары и делает 1 виртуальный callback на op (`SingleUserAreaMapOps.java:101-110`).

В проде `PaperNativeAreaMap` доступен как bridge-класс, который сам плагин определяет в bootstrap loader и
биндит dlsym-символами закрытой библиотеки через `RegisterNatives` (`src/lib.rs:120-122,253-322`; таблица
`src/jni_table.rs:116-117`, `:294-299,:385`; манифест `native/JNI_EXPORTS.manifest:116-119`).

### 1.2 Производственный профиль вызовов (важно для честности проекции)

Единственный caller — `NearbyPlayers.tickPlayer` → 6 экземпляров `TrackedPlayer.update(III)` на игрока на тик,
дистанции **d = 33, 10, 3, 10, 10, 8** (javap-верифицировано: `docs/AREAMAP_COALESCING_FEASIBILITY.md:60-73`).
≤1 нативного вызова на map-instance на тик; idle-тик = 0 вызовов (fast path) (`:84-105`).
Гриды 128/512/1024 (d=63/255/511) в бенче — **синтетическая** нагрузка.

---

## 2. Q1 — Wire format today: точная арифметика байт

### 2.1 Что копируется

Сигнатура `(IIIIII[B[J)I` (`native/JNI_EXPORTS.manifest:118`) не содержит длины → натив работает с
`GetArrayLength(ops)`/`GetArrayLength(keys)`. Измерение (PROBE, `AreaMapApplyBench.java:193-227` →
`APPLY_BENCH.md:70-73`): при одном и том же diff (n0=254 @d=63, n0=2046 @d=511) и 4×-длине буферов:

```
PROBE d=63  len=32 258  n0=254   median 16.7 µs/call;  len=129 032 → 119.8 µs (7.2×)
PROBE d=511 len=2 093 058 n0=2046 median  3.02 ms/call;  len=8 372 232 → 34.7 ms (11.5×)
```

→ стоимость ∝ **длине массива**, НЕ ∝ n. `writes_beyond_n=0` (sentinel-check, `APPLY_BENCH.md:44`) → copy-out
ограничен n; O(L)-член — copy-in (или внутренний полный проход). Внутренность закрытого .so ненаблюдаема, поэтому
честная формула — с множителем:

```
bytes_touch(call) = 9 · L · k          k ∈ [1, 2]   (k=1: только copy-in; k=2: in+out, модель ledger'а)
L = GetArrayLength(ops) = GetArrayLength(keys)   (одна длина на оба буфера — Scratch удваивает их вместе)
cap(d,d) = (2·oldD+1)² + (2·newD+1)² = 2·px  при oldD=newD=d   (SingleUserAreaMapOps.java:53-58)
L = 578·2^i ≥ cap (grow-only doubling, :87-94) → L/cap ≈ 1.13–1.15 в проде
полезная нагрузка = 9·n байт;  n = |new∖old| + |old∖new| = 4d+2 (cardinal) .. 8d+2 (diagonal) для 1-chunk walk
```

Точные L прод-скрэтча подтверждены оракулом (`bench/areamap/results/TASK30_ORACLE.md:44-48,63-65`):
**d=63 → 36 992; d=255 → 591 872; d=511 → 2 367 488**.

### 2.2 Арифметика по d (k-обе оценки)

| d | px | cap=2px | L (факт) | 9·L (k=1) | 18·L (k=2) | полезная 9·n | избыточность (k=1 / k=2, по n max) |
|---|---|---|---|---|---|---|---|
| 63 | 16 129 | 32 258 | 36 992 | 325.4 KiB | 650.9 KiB | 2.3–4.6 KiB | 71× / 141× |
| 255 | 261 121 | 522 242 | 591 872 | 5.08 MiB | 10.2 MiB | 9.0–18.0 KiB | 283× / 566× |
| 511 | 1 046 529 | 2 093 058 | 2 367 488 | 20.3 MiB | 40.6 MiB | 18.0–36.0 KiB | 564× / 1129× |

(Модель ledger'а «in+out ≈ 37.6 MB @d=511/cap ⇒ ~12 GB/s» — это 18·cap без учёта удвоения; с L выходит 42.6 MB.
Эффективная BW непостоянна: 17→2.2 GB/s от L2 до DRAM+TLB — поэтому «12 GB/s» есть грубая средняя; инвариант
«cost ∝ L» от этого не зависит.)

### 2.3 Вердикт по «~8·d bytes»

- **Опровергнуто буквально**: «8·d bytes» не соответствует ни одной измеримой величине. 8·d — это **число ops**
  диагонального 1-chunk хода (8d+2), а байты окна = 9·(8d+2) ≈ 72·d.
- **Подтверждено по сути**: отношение «бюджетное окно / cap» по элементам = `2px/(8d+2) = (2d+1)²/(4d+2)` ≈
  **64.0× (d=63), 256× (d=255), 512× (d=511)** — ledger цитирует «64–256×» (диапазон d=63..255). С учётом
  doubling-overshoot (L) фактическое отношение 73×/290×/579×.
- **Ключевое следствие, которого нет в ledger**: уменьшить L **нельзя в принципе против закрытого либа** — мост
  обязан дать скрэтч `≥ cap` (контракт `maxOps`, `SingleUserAreaMapOps.java:52-58,99`; комментарий «buffer too small
  cannot happen for cap» и ветка `n<0` :98-100 прямо указывают, что натив валидирует длину). Значит «diff-budget
  window» против `nativeUpdateOpsBatch` мёртв; окно требует **новой нативной функции** (или отказа от натива — §4).

---

## 3. Q2 — Dense encoding: выбор представления

### 3.1 Домен состояний (из кода, не из догадки)

Пиксель карты имеет **ровно 2 состояния**: «внутри текущего квадрата» / «снаружи» — карта полностью описывается
тройкой (lastX, lastZ, lastD) (fast path корректен «по определению» — `SingleUserAreaMapOps.java:72-81`), а diff —
чистая функция шести int'ов. Алфавит op = 1 бит (Add/Remove), ключ ячейки = её координата (в dense-форме ключ
неявен — это индекс). Callback'и: `addCallback(param, x, z)` / `removeCallback(...)` — сетевые множества `new∖old`
и `old∖new` **дизъюнктны**, поэтому порядок доставки не влияет на итоговое состояние (канон паритета — multiset,
TASK-30: `TASK30_ORACLE.md:22-24,40-56`).

### 3.2 Кандидаты

| Кандидат | байт/вызов (bulk JNI) | hard cap | замечания |
|---|---|---|---|
| per-pixel packed state byte (байт на пиксель нового квадрата) | px | 1·px | всё ещё O(px) копия; требует натив + обход px в Java |
| bitmask adds+removes (2×px бит) | px/4 | 0.25·px | O(px/64) скан long'ов + popcount; хуже rect при sparse-diff (ход = 2–4 колонки) |
| **2D run-length (row-segments разности) — ВЫБРАН** | **0** (нет JNI-вызова) | **0** (128 B скаляров при B-варианте) | closed form; время O(n + side); нет переполнений буфера; нет скрэтча |
| budget-window ops+keys (B-вариант, §5.3) | 9·(8d+2) out | 9·(8d+2) | запасной вариант, если требуется нативная история |

### 3.3 Выбранный формат: row-run emission (rect-diff)

Разность двух осных квадратов `A = [ax0..ax1]×[az0..az1]`, `B = [bx0..bx1]×[bz0..bz1]` (в интах, с той же
семантикой переполнения, что и у kernel-циклов) считается построчно: для каждой строки z квадрата-источника
вычитается клип-интервал по x → 0..2 x-сегмента; по сегменту — прямой callback на ячейку. Это и есть run-length
(строки = runs), эквивалентно разложению на ≤4 прямоугольника на направление. Свойства:

- **0 bulk-байт через JNI** — нативный вызов не нужен вообще; заодно исчезает floor перехода 35–90 ns
  (`APPLY_BENCH.md:59-61`, canon `P500_REPORT_v2`/TASK-10 errata).
- **Worst case = teleport/полная смена d**: n ≤ 2·px callback'ов (семантический минимум, меньше нельзя — это
  полезная работа kernel'а), и всё равно 0 копируемых байт. Переполнение «бюджета» невозможно по построению →
  **не нужен overflow/fallback-путь по размеру** (в отличие от budget-window).
- O(n + side) времени: для 1-chunk хода ≈ 254..4090 итераций цикла + n callback'ов (те же n callback'ов, что и
  сегодня — семантическая нагрузка, не устраняется никаким encoding'ом).
- Bit-identical final state: эмитируется ровно multiset {(Add, cell) ∈ new∖old; (Remove, cell) ∈ old∖new} — тот же,
  что у закрытого натива (TASK-30: 268/268 multiset-parity REAL vs naive).

Псевдо-контракт нового класса (идентичен `run`-дескриптору моста, чтобы работал same-descriptor retarget):

```java
package ca.spottedleaf.moonrise.common.misc;   // тот же package: add/removeCallback — protected (RuntimeStubs.java:9-12)
final class SingleUserAreaMapDenseOps {
    private static volatile boolean ACTIVE;          // fail-safe: false → делегировать Ops.run (== today)
    static void arm(boolean on) { ACTIVE = on; }     // package-private: плагин-селфтест + бенч
    static long legacyFallbacks;                     // диагностика (читают oracle/bench), 0 в hot path
    static void run(SingleUserAreaMap map, int fromX, int fromZ, int oldD,
                    int toX, int toZ, int newD, Object param) {
        if (fromX == Integer.MIN_VALUE) return;                  // как Ops.run:69-71
        if (fromX == toX && fromZ == toZ && oldD == newD) return; // fast path — НЕ трогаем (Ops.run:79-81)
        if (!ACTIVE || overflowGuard(fromX, fromZ, oldD, toX, toZ, newD)) {
            legacyFallbacks++;
            SingleUserAreaMapOps.run(map, fromX, fromZ, oldD, toX, toZ, newD, param); // закрытый путь, байт-в-байт
            return;
        }
        // row-run emission: removes из old∖new, adds из new∖old (двумя проходами по строкам)
        ...
    }
}
```

`overflowGuard` — 4–6 сравнений через long-арифметику (ловит `fromX−oldD < INT_MIN` и т.п.): всё, что не влезает
в доказанный домен int-квадратов, уходит в легаси-путь без изменения семантики (поля lastX/lastZ/lastD уже
записаны патченным телом update до `run()` — `SingleUserAreaMapOps.java:76-78`, `AREAMAP_COALESCING:80-82`).

---

## 4. Q3 — Body placement (ключевое архитектурное решение)

**Решение: plugin-domain, вариант (a) — но в Java-половине плагина, не в Rust.** Три уровня кандидатов:

1. **Плагин-embedded Java helper class** (выбрано): исходник в `area-map/ca/`, компиляция `scripts/build_area_map.sh`
   (`--release 8`, major 52, ship-set guard — `scripts/build_area_map.sh:27-41`), `include_bytes!` в `src/area_map.rs`,
   `define_class` в **loader карты** (родительский bootstrap сломал бы резолв `SingleUserAreaMap` — докоментация
   `src/area_map.rs:12-15`), до `READY`/retransform (`:142-158,168-170`). Прецедент — сами `SingleUserAreaMapOps*`
   классы. Для ретаргета вызова: `classfile::retarget_invokestatic` (Variant R, G4) поверх `patch_update`-байтов —
   machinery готова и покрыта тестом на реальном fixture: patch → retarget `(OPS_CLASS,"run",RUN_DESC)` →
   `(DENSE_CLASS,"run",RUN_DESC)` с **тем же дескриптором** `src/classfile.rs:725,930-951`; живой прецедент
   same-descriptor retarget на helper-класс в kernel loader — `src/improved_noise.rs:61-74` (`ImprovedNoiseBatchOps`,
   «4th embed»).
2. **Rust fn в libcrussty.so + RegisterNatives** (опция B, §5.3): полный прецедент — `src/batch_api.rs:637-667`:
   плагин определяет `crussty/batch/PaperNativeBatchDispatch` и регистрирует **локальные** fn-указатели
   `Java_crussty_batch_PaperNativeBatchDispatch_run` (:658,:663) — не dlsym. Библиотека-плагин живёт столько же,
   сколько JVM (требование «raw fn pointers must outlive JVM» — `src/loader.rs:9`). Это опровергает
   `HOTSPOT_CANDIDATES_V2.md:55` («kernel-side change … unbuildable without the closed .so sources»): закрытый
   apply-kernel остаётся нетронутым — новая функция селф-хостится в собственном .so плагина.
3. **CRUSSTY engine .so** — НЕ требуется ни в каком варианте. ENGINE-TOUCH не задействован.

Почему Java, а не Rust, для выбранного варианта: dense-тело — это ~30 строк int-арифметики + n виртуальных
callback'ов, которые **в любом случае** делает JVM (натив не может вызвать `addCallback` дешевле обратного
JNI-upcall'а). Рust-вариант добавил бы JNI floor + copy-out 9n байт + скрэтч, т.е. строго хуже по всем осям,
сохраняя ту же семантическую нагрузку. Риск «JVM плохо JIT'ит» не состоит: цикл — тривиальные int-сравнения
(ориентир: FAKE-энумерация делает 32 258 cell-визитов за 15.8 µs = 0.49 ns/cell, `APPLY_BENCH.md:35-39` — на два
порядка быстрее необходимого).

---

## 5. Схема включения и два варианта тела

### 5.1 Wiring (общий для A и B)

- `area_map.rs::register`: hook-замыкание = `patch_update(bytes)` → при `DENSE_READY` — `retarget_invokestatic`
  на `(SingleUserAreaMapDenseOps, "run", RUN_DESC)`. Класс dense определяется в activation-потоке **до** retransform;
  порядок «define → arm-time self-test → READY → retransform» гарантирует, что retarget происходит только после
  PASS (см. §6).
- Env-gate: `CRUSSTY_AREAMAP_DENSE` = `off|on` (default **off**), fail-safe parse «всё прочее → off» — точный порт
  `parse_rollout`/`rollout_mode` (`src/batch_api.rs:393-437`, тесты `:1569-1576`), OnceLock + boot-marker.

### 5.2 Вариант A (выбран): Java dense body — 0 JNI

Описан в §3.3. Fail-safe-матрёшка: (1) env off → класс вообще не определяется, байты update идентичны today;
(2) class defined, `ACTIVE=false` → dense `run()` делегирует `SingleUserAreaMapOps.run` → поведение байт-в-байт
легаси (даже если retarget случился, а arm не произошёл); (3) `ACTIVE=true` только после arm-time 3-way
self-test; (4) `overflowGuard` → per-call легаси; (5) во всех ветках поля карты уже записаны патченным телом —
состояние карты не зависит от того, какая ветка исполнена.

### 5.3 Вариант B (запасной, спека для полноты): budgeted native в libcrussty.so

`nativeUpdateOpsBatchBudgeted(IIIIII[B[J)I` на плагин-классе (RegisterNatives, прецедент `batch_api.rs:654-667`):
Rust-энумератор той же разности пишет **только первые budget элементов** ops/keys (budget = 8d+2, окно берётся из
длины массива, но трогает ровно n ≤ budget) и возвращает n; n<0 = «не влезло» → Java растит окно и повторяет.
Байты: 9·n out ≈ 72·d (73×/290×/579× меньше элементов, чем сегодня). Минусы против A: JNI floor + copy-out +
скрэтч + overflow-путь + риск «RegisterNatives на классе kernel-loader в JVMTI-контексте не проверен живьём»
(batch-прецедент — null/bootstrap loader). Вариант B имеет смысл только если владелец требует «энумерацию в Rust»
или если A упадёт по JIT-причинам (маловероятно, см. §4).

---

## 6. Q4 — Parity strategy (bit-identical final map state)

Три контура, от дешёвого к дорогому:

1. **Headless Rust-модель (CI)**: расширить `area-map-fuzz` (`area-map-fuzz/src/lib.rs:1-29` — уже есть
   `FastPathMap`-модель контракта Java-половины + независимый `ReferenceMap` = naive set difference; CI-гейт
   TASK-36, `RESULTS_LEDGER.md:135`): добавить `DenseRectModel` (порт row-run алгоритма §3.3) и фаззить
   triple-equality `DenseRectModel == FastPathMap(native-модель) == ReferenceMap` на тех же сетах координат,
   включая MIN/MAX/zero edges (крейт уже генерит их — `lib.rs:28-29`). `cargo test -p area-map-fuzz` — gate.
2. **TASK-30 oracle, режим DENSE** (`bench/areamap/run_oracle.sh` + `OracleBench.java`): те же детерминированные
   стримы S1-MOVE/S2-RESIZE/S3-MIX d=63/255/511 (`TASK30_ORACLE.md:13-38`) + новые edge-стримы:
   S4-EDGE = {d=0; from==to c Δd; вложенные квадраты; полностью дизъюнктные; координаты у INT_MIN/INT_MAX
   (assert `legacyFallbacks` инкремент); MIN_VALUE sentinel}. Критерий: multiset-parity **3-way** на каждом вызове —
   dense vs закрытый натив (REAL) vs naive, 0 расхождений. Прецедент критерия — `TASK30_ORACLE.md:40-56,115-124`
   (268/268, «VERDICT: PARITY»); O(1)-проверка размера diff через square intersection уже реализована в REPLAY
   (`:35-38`).
3. **Arm-time runtime self-test** (в проде, до READY): существующий `bridge_selftest` паттерн
   (`src/area_map.rs:189-289`, naive_set_difference `:291-316`): ≥64 rects LCG + edge-кейсы, **3-way** — dense `run()`
   на RecordingMap vs `nativeUpdateOpsBatch` vs naive; PASS → `arm(true)` + retarget; FAIL → без retarget, маркер,
   лобби остаётся на закрытом пути.

Acceptance: (1) и (2) зелёные до мерджа; (3) PASS обязателен для arm'а в живом JVM; финальный вердикт —
A/B-бенч (§8) + живой E2E dormant/armed boot по runbook-дисциплине.

---

## 7. Q6 — Projected win (честно)

### 7.1 Модель

```
win = T_total(REAL) / T_total(DENSE),   T_total(REAL)   = T_native(L) + n·C + fixed
                                        T_total(DENSE)  = fixed' + n·C
C = стоимость одного Java-callback'а в apply-loop (неизвестна для реального kernel-callback'а!),
    в бенче (CountingMap) C ≈ 1–3 ns (выведено: REAL CHANGED d=63 = 21.8 µs ≈ T_native(36 992) ≈ 21.7 µs
    линейной интерполяцией PROBE-точек → residual ≈ 0.1 µs на ~380 ops);
T_native(L) — измерена PROBE (16.7 µs @L=32 258 … 3.02 ms @L=2 093 058, ∝L);
n = 4d+2..8d+2 (среднее по 8-dir walk 6d+2).
```

**C — главный источник неопределённости**: реальный `TrackedPlayer.add/removeCallback` мутирует TrackedChunk-состояния
(`AREAMAP_COALESCING:111-116`) и может стоить 5–100 ns; эта стоимость платится в ОБОИХ путях и сжимает win к
`1 + T_native/(n·C)`.

### 7.2 Проекция (CHANGED, средний n=6d+2, band C = 1–3 ns bench / до 40 ns kernel-callback)

| grid | d | today (measured, `APPLY_BENCH.md:18-30`) | dense (модель) | **win band** |
|---|---|---|---|---|
| 128 | 63 | 21 802 ns | 0.7–2.4 µs (C=1–3ns) … 15.3 µs (C=40ns) | **8–30×** (C bench) / до 2.5× (C=40ns) |
| 512 | 255 | 867 037 ns | 2.4–9.3 µs … 61 µs | **80–350×** / до 15× |
| 1024 | 511 | 3 588 585 ns | 4.7–18.5 µs … 123 µs | **150–700×** / до 30× |
| MIX50 (половина same-state ≈ 0) | — | 10 869 / 429 266 / 1 931 001 ns | ≈ CHANGED/2 | те же bands |

### 7.3 Прод-формы (обязательно, d≤33 — `AREAMAP_COALESCING:65-75`)

За событие пересечения границы (1 native call на каждую из 6 карт): today ≈ 17 µs копирования (GENERAL d=33 ≈ 13.2 µs)
→ dense ≈ Σ nᵢ·C ≈ 1.2–4.9 µs → **win ≈ 3.5–15×**, абсолютная экономия ≈ 12–16 µs на crossing player-tick
(≈ 0.02–0.03 % 50 ms tick на пересечение). Idle-тики не меняются (уже 0-cost fast path).

### 7.4 Честный итог и фальсификаторы

- >100x-class подтверждается **только на бенч-гридах d≥255** и только при C ≈ bench-значениях; на прод-дистанциях
  lever даёт единицы-десятки ×. Это надо проговорить с владельцем ДО реализации (тот же класс честности, что
  blend-cache NO-GO: `RESULTS_LEDGER.md:79` — «the probe worked, the premise failed»).
- Фальсификатор 1: A/B показывает dense CHANGED @d=511 > 100 µs/update → модель copy-bound неверна → стоп.
- Фальсификатор 2: live JFR не видит снижения на tick-бюджете при включённом dense (значит C≫bench) →
  переквалифицировать в «win на больших view-distance only» или закрыть.
- Фальсификатор 3 (дешёвый, шаг 0): прямой вызов закрытого натива с len<cap буферами — если НЕ вернёт n<0
  (а отработает), бюджетное окно возможно и против закрытого либа — тогда вариант B упрощается до смены скрэтча
  без нового тела. Ожидание по контракту моста (`SingleUserAreaMapOps.java:98-100`): вернёт ошибку.

---

## 8. Q7 — Implementation plan

| # | Файл | Действие | ~размер |
|---|---|---|---|
| 0 | `bench/areamap/` (одноразовый probe) | len<cap falsification probe (§7.4) | 20 строк, не коммитится обязательно |
| 1 | `area-map-fuzz/src/lib.rs` (+`tests/fuzz_parity.rs`) | `DenseRectModel` + triple-equality фазз, edge-сеты | +120 Rust |
| 2 | `area-map/ca/.../SingleUserAreaMapDenseOps.java` | NEW: dense run() по §3.3 (row-runs, guards, ACTIVE, legacyFallbacks) | ~120 Java |
| 3 | `scripts/build_area_map.sh` | добавить исходник в компиляцию + в SHIP-набор guard'а (major-52 проверка) | +6 |
| 4 | `src/area_map.rs` | `DENSE_*_BYTES` include_bytes, `DENSE_NAME`; env-gate `CRUSSTY_AREAMAP_DENSE` (порт `batch_api.rs:393-437`); define dense в activation-loop; arm-time 3-way self-test (переиспользует `naive_set_difference` `:293-316` + `nativeUpdateOpsBatch`); `DENSE_READY` + retarget в hook-замыкании (`classfile.rs:725`); маркеры | +110 Rust |
| 5 | `src/classfile.rs` | **БЕЗ изменений** (retarget_invokestatic готов, `:725`, тест `:930-951`) | 0 |
| 6 | `bench/areamap/ca/.../AreaMapApplyBench.java` + `run_apply_bench.sh` | третий arm DENSE (чистый JVM, без .so; `SingleUserAreaMapDenseOps.arm(true)`; те же SAME/MIX50/CHANGED + SANITY) — расширение паттерна `run_apply_bench.sh:18-37` | +60 Java, +12 sh |
| 7 | `bench/areamap/benchjava/.../OracleBench.java` + `run_oracle.sh` | режим DENSE: S1/S2/S3 + S4-EDGE, 3-way multiset + legacyFallbacks asserts | +80 Java, +10 sh |
| 8 | `docs/AREAMAP_DENSE_APPLY_RESULTS.md` | NEW (пишет исполнитель): A/B таблицы + oracle-вердикт | — |

Маркеры (grep-конвенция `[crussty-plugin] area_map: ...` — согласовано с `src/area_map.rs:61,94,150,170,284`):

```
[crussty-plugin] area_map: dense gate CRUSSTY_AREAMAP_DENSE=on (default off)   // boot, OnceLock
[crussty-plugin] area_map: dense defined <class> in map loader                 // activation
[crussty-plugin] area_map: dense self-test OK (N rects, dense == native == naive)
[crussty-plugin] area_map: dense self-test FAILED ... -> closed native path retained   // NO retarget
[crussty-plugin] area_map: dense path ARMED (retarget SingleUserAreaMapOps.run -> SingleUserAreaMapDenseOps.run)
[crussty-plugin] area_map: dense path OFF (gate off)                            // unarmed boot: ровно эта строка
```

Тест-лист: (T1) `cargo test -p area-map-fuzz` (dense-модель parity, edge-сеты); (T2) существующие classfile-тесты
retarget на REAL fixture (зелёные без изменений) + новый тест «patch_update+retarget на dense-дескриптор»;
(T3) unit-тест gate-parse (fail-safe: `""`/`1`/`garbage` → off); (T4) headless `run_apply_bench.sh` — SANITY d=32:
130 callbacks и в DENSE-arm; (T5) oracle DENSE: 0 расхождений на всех стримах; (T6) live E2E dormant (0 новых
маркеров) + armed (SELF-TEST OK + ARMED) — по `BATCH_ROLLOUT_RUNBOOK`-дисциплине.

Оценка суммарного диффа: ~450–500 строк (Java ~260, Rust ~120, bench ~120), 0 изменений classfile/engine/closed .so.

---

## 9. Q8 — Risks + NO-GO exits

| # | Риск | Триггер NO-GO | Митигация/статус |
|---|---|---|---|
| 1 | **Прод-вин < 100×** (C ≫ bench: реальный callback дорог) | владелец требует >100x-class именно на прод-формах → lever закрывается как «win только на d≥255» | честный band §7.3 уже посчитан; решение — бизнес-решение, не техническое |
| 2 | **Byte-math falsified**: cost ∝ L не подтвердится на новом стенде (противоречит PROBE `APPLY_BENCH.md:70-73`) | новый PROBE даст flat-cost vs len → вся premise умирает | маловероятно (два независимых PROBE-точки на d), но шаг 0 дешёвый |
| 3 | **Rect-семантика разошлась** на edge-формах (int-wrap координат, d-смены, дизъюнкт) | ЛЮБОЕ расхождение в (T1)/(T2)/(T5)/(self-test) | `overflowGuard` → легаси; fail-safe-матрёшка §5.2 гарантирует, что расхождение = «остались на today» |
| 4 | **Порядок callback'ов** | если владелец потребует bit-identical ПОРЯДКА (не multiset) — dense его не гарантирует | порядок ненаблюдаем (callback'и синхронны, читатели между update'ами — `AREAMAP_COALESCING:111-116`); shipped-путь уже отличается по порядку от исходного kernel-update; канон TASK-30 — multiset |
| 5 | **Retarget+define в kernel loader в live JVM** (VerifyError/лоадер-гонки) | P0-класс аварии на живом E2E | класс в ТОМ же loader, что Ops (резолв-риск снят — `area_map.rs:12-15`); `force_load_kernel_class` уже канарка VerifyError (`:318-324`); arm только после self-test; gate default off |
| 6 | **Вариант B**: RegisterNatives на класс kernel-loader не прецедентирован живьём (batch — bootstrap loader) | если B понадобится и упадёт | B — запасной; A регистрирует нативов вообще нет |
| 7 | **Параллельный рыночный контекст**: `bench/areamap/classes-*`, `applybench/` — FOREIGN WIP | — | не трогать (read-only), свой код — в отдельных файлах из §8 |

---

## 10. Evidence index (каждое утверждение)

- Контракт моста/wire: `area-map/ca/.../SingleUserAreaMapOps.java:34,52-58,69-71,79-81,87-94,97-110`;
  `area-map/ca/.../RuntimeStubs.java:9-20`; сигнатура — `native/JNI_EXPORTS.manifest:116-119`; соседние
  batch-экспорты `updateSummaryBatch`/`squareSummaryBatch` — там же (out of scope).
- O(L)-copy evidence: `bench/areamap/results/APPLY_BENCH.md:63-81` (PROBE 7.2×/11.5×, 12 GB/s, 37.6 MB),
  `:18-30` (headline), `:35-39` (FAKE 0.49 ns/cell), `:59-61` (JNI floor 35–90 ns);
  probe-код `bench/areamap/ca/.../AreaMapApplyBench.java:193-227`, `writes_beyond_n` `:200-206`.
- Скрэтч-размеры (факт): `bench/areamap/results/TASK30_ORACLE.md:44-48,60-68`; контракт maxOps:
  `SingleUserAreaMapOps.java:52-58`.
- Parity-канон (multiset, 268/268): `TASK30_ORACLE.md:22-24,40-56,115-124`; O(1) square-intersection: `:35-38`.
- Прод-дистанции и ≤1 native call/tick: `docs/AREAMAP_COALESCING_FEASIBILITY.md:60-75,84-105,139-148`.
- Body-placement прецеденты: `src/batch_api.rs:637-667` (локальные fn-указатели + RegisterNatives), `:393-437`
  (gate+marker), `:1569-1576` (fail-safe тесты); `src/lib.rs:120-122,253-322` (dlsym-поверхность — контраст);
  `src/loader.rs:9`; `src/improved_noise.rs:61-84` (same-descriptor retarget target + env-gate), `:812-887`
  (self-test); `src/classfile.rs:397-491` (patch_update), `:725+,:930-951` (retarget + тест); `src/area_map.rs:12-15,36-44,142-158,189-316,318-324`.
- Headless fuzz-модель: `area-map-fuzz/src/lib.rs:1-29`; CI TASK-36 `RESULTS_LEDGER.md:135`.
- Ledger-контекст: `RESULTS_LEDGER.md:52` (win #1 fast path), `:79` (blend NO-GO прецедент), `:89-102` (physical
  limits), `:95`, `:117-119` (этот lever), `docs/HOTSPOT_CANDIDATES_V2.md:55` (TASK-20-R, refuted по scope),
  `docs/BLEND_CACHE_PATCHER_DESIGN.md` §9 (NO-GO exit формат).
- Билд/бенч-риг: `scripts/build_area_map.sh:27-41`; `bench/areamap/run_apply_bench.sh:18-37`; `bench/areamap/run_oracle.sh`.

---

## 11. STEP-0 РЕЗУЛЬТАТ (исполнен 2026-09-08, probe `scripts/areamap_probe/`, mirror-of-record — worklog S7-16)

### 11.1 Фальсификатор 3 СРАБОТАЛ — вариант B упрощается до «вариант C»

Контракт закрытого натива **мягче**, чем предполагал §2.3: он валидирует `len ≥ n` (не `len ≥ cap`) и при недоборе
возвращает **-n0** (точный требуемый размер = встроенный оракул размера):

| d | cap | len | n (возврат) | вердикт |
|---|---|---|---|---|
| 63 | 32 258 | 32 258 | 254 | контроль: работает |
| 63 | 32 258 | 8 064 (cap/4) | 254 | **принят** |
| 63 | 32 258 | 300 / 260 / **254 (=n0)** | 254 | **принят** — граница ровно len≥n |
| 63 | 32 258 | 100 (<n0) | **-254** | отклонён, writes_past_n=0, side-effect-free |
| 511 | 2 093 058 | 523 264 / **2046 (=n0)** | 2046 | принят |
| 33 (прод) | 8 978 | 300 / **200 (≥n0=134)** | 134 | принят |

«Окно бюджета против закрытого либа» **возможно**: новое тело НЕ требуется. Достаточно изменить политику
скрэтча моста (`SingleUserAreaMapOps.java:87-94`, grow-only-до-cap → **budgeted + -n0-retry**).

### 11.2 Вариант C (НОВЫЙ, упрощённый, рекомендован к исполнению первым)

Изменить ТОЛЬКО скрэтч-политику моста: буфер размером с бюджет (адаптивный: per-thread budget =
max(2·последний n, floor)), при `n<0` — один retry с `len = -n`. Натив как делал энумерацию, так и делает;
parity не затронута (размер буфера — не семантика; reject side-effect-free, sentinel-проверено). Отпадают:
новый класс, retarget, self-test 3-way, dense-семантика. Остаётся: env-gate (тот же `CRUSSTY_AREAMAP_DENSE`
→ переименовать в `CRUSSTY_AREAMAP_BUDGET`, default off), unit-тесты retry-логики, TASK-30 oracle re-run,
A/B-бенч. Дифф ~40-60 строк Java + тесты вместо ~450-500.

**Warm median per-call, closed lib, идентичный workload** (`LenCapBench`, reps≥1000, BENCH-MUTEX):

| d | сегодня: len=L≈cap | budgeted: len=n | WIN |
|---|---|---|---|
| 63 | 16 760.6 ns | 1 420.7 ns | **11.8×** |
| 33 (прод-макс) | 5 361.1 ns | 899.2 ns | **6.0×** |
| 511 | 3 049 508.7 ns | 10 197.4 ns | **299.1×** |

### 11.3 Пересчёт проекций (честно)

- Бенч-гриды: d=511 → **299× измерено** (>100x-class, измерено на закрытом нативе, не модель); d=63 → 11.8×;
  d=255 → экстраполяция 40-120× (измерить на стадии A/B).
- Прод (d≤33, 6 карт/crossing): нативная нога ~5-6×; полный вызов (натив + n·C callback'и) ≈ 2-5.5× при
  C=1-40ns — скромнее варианта A в модели §7.3, НО с радикально меньшим риском и диффом.
- Вариант A (dense Java body) остаётся запасным усилителем (устраняет и нативный вызов, и JNI floor),
  если после вариант C понадобится больше. Порядок исполнения: **C → измерить → решить про A**.
- Фальсификаторы C: (1) -n0-retry шторм при пилообразном n (митигируется адаптивным budget'ом 2×);
  (2) скрытая семантика у натива при len между n и cap (oracle S1-S3 re-run на budgeted-скрэтче — обязательный
  gate перед arm'ом); (3) ThreadLocal-аллокация budget-буфера в hot path (пре-аллокация floor+рост, без alloc
  на steady-state вызове).

### 11.4 Исправление плана §8 под вариант C

| # | Файл | Действие |
|---|---|---|
| 1 | `area-map/ca/.../SingleUserAreaMapOps.java` | budgeted scratch + -n0-retry (вместо grow-only-до-cap), env-gate off→легаси байт-в-байт |
| 2 | `scripts/build_area_map.sh` | без изменений (класс тот же) |
| 3 | `src/area_map.rs` | env-gate `CRUSSTY_AREAMAP_BUDGET` (порт batch_api.rs:393-437), маркеры, селфтест budget-vs-cap parity на RecordingMap |
| 4 | `bench/areamap/` | A/B arm: легаси-скрэтч vs budgeted (тот же REAL натив) |
| 5 | `bench/areamap/.../OracleBench.java` | S1-S3 re-run на budgeted-скрэтче (multiset parity gate) |
