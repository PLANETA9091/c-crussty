# SYMBOL-PIN КАНОН — ПАТЧ-ПЛАН И РИСКИ (TASK-R468-S52, ROUND-468)

Канон: Л180i (S43) — line-пин в class-файле = пин-в-никуда; javap-LNT дрейфует;
единственная легальная координата = **имя+дескриптор**
(`Owner.method(paramTypes)->retType`, гейт `ncdfe_guard.sh --pin`, ветка
round-467-s43-pin@a15a2d91). Этот документ закрепляет канон на ВСЕМ тулинге
репо: аудит, конверсия, риски, enforcement.

## 1. АУДИТ (числа, сканер scripts/linepin_audit.py @round-468-s52-symbolpin)

Периметр: все git-tracked `*.sh`+`*.py` (436 файлов), включая семейства
build_* (49 скриптов), patch_*, guard, dispatch, absorb. docs/*.md исключены
(леджер-координаты дрейфуют by design и пином не потребляются).

| метрика | число |
|---|---|
| tracked sh+py (периметр) | 436 |
| из них build_*/patch-скрипты (sh/py) | 49 |
| FAIL-хиты (P1+P2+P2B+P4 паттерны) | 19 |
| уникальных FAIL-сайтов (file:line) | 11 |
| из них канон-пин-класса `:467` (MobPushOps.pushables) | 2 |
| исторические javap-аттрибуции (s7158/s7182-7188/432b) | 9 |
| P4 (sed in-place line-addressed edit) | 0 |
| P5/WARN (индексные чтения splitlines()[N], все header/top-N) | 8 |
| P3/WARN (текстовые "line NNN" — ссылки на прошлые фейлы) | 5 |
| allowlist-записей (задокументированные пины) | 19 |
| STALE-ALLOW после allowlist | 0 |
| LIVE-PIN-ов, требующих немедленной конверсии | 0 (все 2 канон-класса уже конвертированы на S43) |

Дрейф-хроника канон-пина (почему line-пин запрещён):
`:463` (×452a, decl 442) → `:467` (×456, decl 446) → `decl :448` +
javap-LNT touch `:469@PC91` (master 04d58e6c; c98 31a2b4a8 блоб
byte-identical 14397B) = **+19 строк дрейфа** при неизменной семантике.

## 2. ПАТЧ-ПЛАН КОНВЕРСИИ (имя+дескриптор)

Фаза A (уже сделано, принять): merge round-467-s43-pin@a15a2d91 —
(a) `git mv ncdfe_guard_tmp.sh → scripts/ncdfe_guard.sh` + `--pin` режим
(symbol-pin форма `SYMBOL-PIN <blob> <Owner.method(desc)> -> <Target> @pc N`;
LNT печатается только как PIN-INFO, грепать запрещено M01 §3);
(b) docstring dispatch_456c_reroll.py перепинут. После мержа: убрать 2 записи
allowlist (помечены `drop on merge`) → 17 записей.

Фаза B (convert-on-touch, 9 записей): исторические javap-аттрибуции в
docstring'ах dispatch_s7158/s7182-7188.py и build_432b_blobs.sh:105
заменяются при следующем касании файла на symbol-форму, например:
  BlockUpdateOps.vanilla:117 →
  BlockUpdateOps.vanilla(Lnet/minecraft/server/level/ServerLevel;...)Z
(точный дескриптор берётся `ncdfe_guard.sh --pin <blob>` из javap, не с листа).
Пока файл не тронут — координата задокументирована в allowlist с тегом
`convert-on-touch`; STALE-детект гейта следит, чтобы записи не сгнивали.

Фаза C (enforcement, эта ветка): `scripts/linepin_audit.py` входит в
пред-вердикт/merge-гейты рядом с case_arm_scan.py (шелл-армы) и
check_blobs_sync.sh (javap-маркеры): FAIL = DELIVERY-FAIL по аналогии с
NCDFE-гейтом. Режимы: скан (exit 0/1), `--json`, `--selftest`,
`--negative-test`, `--root <path>` (скан чужого worktree при allowlist своего
репо).

## 3. РИСКИ (что могло пойти не так и почему низко)

1. **Мыслемысленный grep-гейт**: P1/P2 могут поймать невинные строки (URL:порт,
   версии). Контроль: негатив-фикстуры `--negative-test` PASS (0 FAIL на
   URL/портах/CSV-хедерах); P5-класс `[0]`-индексы = WARN, не FAIL.
2. **Allowlist-гниль**: записи переживают код → тихий вечный allowlist.
   Контроль: STALE-ALLOW репорт (запись без паттерна под ней = сигнал уборки);
   сейчас 0 stale.
3. **Конфликт с S43**: если конвертировать 2 живых сайта на этой ветке,
   merge S43 даст rename/edit-конфликт (файл переносится). Поэтому здесь
   ТОЛЬКО allowlist-пометка `drop on merge`, правка файлов — веткой S43.
4. **Ложное спокойствие по rust-плоскости**: этот гейт сканирует sh/py;
   rust/java пины отсутствуют конструктивно (src/classfile.rs::find_method
   @:348 резолвит по name_idx/desc_idx = имя+дескриптор; гейты- needles
   cmp456_* строковые; check_blobs_sync.sh = javap-маркер-строки), что
   зафиксировано аудитом как «уже канон», не как «не проверял».
5. **Дрейф самих регексов**: шаблоны зашиты в гейт; при новых формах пинов
   (например `@pc N` как канон-координата — это ЛЕГАЛЬНО, PC от дескриптора)
   гейт расширяется фикс-туром с selftest-фикстурой, не тихим правками.
6. **PC-пины ≠ line-пины**: `@pc N` в symbol-pin выводе S43 — INFO-поле,
   дрейфует при пересборке; гейт НЕ требует PC-равенства, только дескриптор.

## 4. СТАТУС

- Ветка: round-468-s52-symbolpin (от origin/master c1196321, scripts/docs
  только — 0 java/rust/kernel дельт, CI-инертно; диспатч ваниль-ноги для
  BANK-точки сделан отдельным ref-push этой ветки, см. ROUND-468/S52.md).
- Гейт на полный мастер-скан: FAIL=0 / WARN=13 / ALLOWED=19 / STALE=0.
- Selftest PASS, negative-test PASS, py_compile OK.
