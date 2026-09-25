#!/usr/bin/env python3
"""TASK-462-62 (swarx-3): STRICT-OR the SINGLE composite lever id
cmp458_swar_papaya into every gate site carrying the swar needle
"cmp458_swar" (mechanics = add_swar_gates_458.py canon; lineage
d22835bd/33939931/097def9d; mirror-drift урок ×451/×452: rust-гейты +
java flag-lists правятся СИНХРОННО; урок ×461: 4× дубль lever-id в строке
= AIOOBE — поэтому построчный счётчик вставок + idempotence-guard).

Компонент: papaya-lockfree shard-readers (round-460-chkswing-1 89f90d50)
graft на swar-носитель round-460-swarx-1 @33939931. Композит = ОДИН id
cmp458_swar_papaya: swar push-plane под-лейны (eqsnap drain, SoA upsert,
colpush, sscan, ai, goalquery, items) остаются LIVE, papaya sidecar
earмится строго на этот же флаг (papaya_arm::LEVER_ID).

CARVE-OUT (намеренный, base-parity): sense/SenseOps.java и
randomtick/src/BrainOps.java НЕ расширяются — их трекнутые блобы не несут
даже cmp458_swar (460-34 rebuild-set = 8 классов), лейны спали на +18.0
ноге (Brain tick2 = dead slice ×453); расширение источников без ребилда
блоба = новый F-1 (спящий гейт), ребилд = смещение дельты против гипотезы
ортогональности. STRICT-eq изоляция сохранена.
"""
import re, sys, glob

FLAG = "cmp458_swar_papaya"
ANCHOR = "cmp458_swar"

rust_files = sorted(glob.glob("src/*.rs"))
java_files = [
    "mobai/net/minecraft/world/entity/MobAiOps.java",
    "sscan/net/minecraft/world/entity/MobScanOps.java",
    "mobpush/net/minecraft/world/entity/MobPushOps.java",
    "queryplane/net/minecraft/world/entity/QueryPlaneOps.java",
    "colpush/net/minecraft/world/entity/ColpushOps.java",
    "goalops/net/minecraft/world/entity/ai/goal/GoalOps.java",
    "entitygoalquery/net/minecraft/world/entity/EntityGoalQueryOps.java",
    "entityinside/net/minecraft/world/entity/ItemEntityManager.java",
]
# base-parity carve-out: never widened, never rebuilt (см. докстринг)
CARVE_OUT = [
    "sense/net/minecraft/world/entity/SenseOps.java",
    "randomtick/src/BrainOps.java",
]

def needle_quoted(txt):
    return f'"{ANCHOR}"' in txt

fails = []
rust_touched = java_touched = 0
total_inserts = 0

# ---------------- RUST ----------------
for path in rust_files:
    with open(path) as f:
        lines = f.readlines()
    changed = False
    for i, line in enumerate(lines):
        if FLAG in line:
            continue  # idempotent (урок ×461)
        if not needle_quoted(line):
            continue
        last = None  # (kind, var, span_end)
        for m in re.finditer(r'\| (Ok|Some)\("' + ANCHOR + r'"\)', line):
            last = (m.group(1), None, m.end())
        for m in re.finditer(r'(\w+(?:\.trim\(\))?) == "' + ANCHOR + r'"', line):
            last = ("eq", m.group(1), m.end())
        if last is None:
            continue  # комментарий/док — не гейт
        kind, var, end = last
        if kind == "Ok":
            ins = f' | Ok("{FLAG}")'
        elif kind == "Some":
            ins = f' | Some("{FLAG}")'
        else:
            ins = f' || {var} == "{FLAG}"'
        lines[i] = line[:end] + ins + line[end:]
        changed = True
        total_inserts += 1
    if changed:
        with open(path, "w") as f:
            f.writelines(lines)
        rust_touched += 1
        print(f"rust ok: {path}")

# ---------------- JAVA ----------------
for path in java_files:
    with open(path) as f:
        lines = f.readlines()
    changed = False
    for i, line in enumerate(lines):
        if FLAG in line:
            continue  # idempotent
        if path.endswith("ColpushOps.java"):
            # FLAG-constant style (FLAG17 — следующий свободный слот после
            # FLAG16; цепь расширяется на сайте FLAG16 — та же механика,
            # что в add_swar_gates_458.py FLAG15->FLAG16)
            if 'f.trim().equals(FLAG16)' in line:
                lines[i] = line.replace(
                    'f.trim().equals(FLAG16)',
                    f'f.trim().equals(FLAG16) || f.trim().equals(FLAG17)')
                changed = True
                total_inserts += 1
            elif re.match(r'\s*private static final String FLAG16 = "' + ANCHOR + r'";', line):
                indent = re.match(r'(\s*)', line).group(1)
                lines[i] = (line.rstrip("\n") + "\n"
                            + f'{indent}/** TASK-462-62: swar⊕papaya composite '
                              f'(STRICT-OR, ОДИН id). */\n'
                            + f'{indent}private static final String FLAG17 = "{FLAG}";\n')
                changed = True
            continue
        if not needle_quoted(line):
            continue
        last = None
        for m in re.finditer(r'(\w+(?:\.trim\(\))?)\.equals\("' + ANCHOR + r'"\)', line):
            last = (m.group(1), m.end())
        for m in re.finditer(r'"' + ANCHOR + r'"\.equals\((\w+(?:\.trim\(\))?)\)', line):
            last = (m.group(1), m.end())
        if last is None:
            continue
        var, end = last
        ins = f' || {var}.equals("{FLAG}")'
        lines[i] = line[:end] + ins + line[end:]
        changed = True
        total_inserts += 1
    if changed:
        with open(path, "w") as f:
            f.writelines(lines)
        java_touched += 1
        print(f"java ok: {path}")

print(f"\n== union {FLAG} wired: {rust_touched} rust files, "
      f"{java_touched} java files, {total_inserts} inserts ==")

# ---------------- VERIFY ----------------
# 1) ×461 AIOOBE-guard: НИ одна строка не несёт >1 вставки композита.
dup = []
for path in rust_files + java_files:
    with open(path) as f:
        for n, line in enumerate(f.readlines(), 1):
            c = line.count(FLAG)
            if c > 1:
                dup.append(f"{path}:{n}: {c}x")
            # дубль anchor-вставки: композит не должен стоять внутри цепи
            # дважды даже через перенос — считаем и вхождения anchor-token
            # композита как такового (строки-продолжения уже покрыты count)
if dup:
    for d in dup:
        print(f"DUPLICATE {d}", file=sys.stderr)
    sys.exit(1)

# 2) каждый needle-несущий гейт несёт композит (кроме carve-out)
missing = []
for path in rust_files:
    txt = open(path).read()
    if needle_quoted(txt) and FLAG not in txt:
        missing.append(("rust", path))
for path in java_files:
    txt = open(path).read()
    if needle_quoted(txt) and FLAG not in txt:
        missing.append(("java", path))
if missing:
    for kind, p in missing:
        print(f"MISSING {kind}: {p}", file=sys.stderr)
    sys.exit(1)

# 3) carve-out нетронут: в carve-out файлах композита быть НЕ должно
for path in CARVE_OUT:
    txt = open(path).read()
    if FLAG in txt:
        print(f"CARVE-OUT VIOLATED: {path}", file=sys.stderr)
        sys.exit(1)

print(f"verify: all needle-carrying gates now accept {FLAG} (STRICT-OR union); "
      f"carve-out {CARVE_OUT} untouched")
