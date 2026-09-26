#!/usr/bin/env python3
"""linepin_audit.py — symbol-pin канон страж (TASK-R468-S52, канон Л180i / S43).

Канон: единственная легальная координата символа в class-файле = имя+дескриптор
(Owner.method(paramTypes)->retType). LINE-ПИНЫ (:NNN / "line NNN" / file:line)
ЗАПРЕЩЕНЫ в исполняемом тулинге: javap-LNT дрейфует (хроника канон-пина
MobPushOps.pushables: :463 (×452a, decl 442) -> :467 (×456, decl 446) ->
decl :448 + LNT touch :469@PC91 @master 04d58e6c = +19 строк от факта),
а шелл/питон-строки редактируются line-union'ами (урок Л171/C02).

Что сканируется: все git-tracked *.sh + *.py (тулинг, включая build_*/patch/
guard/dispatch семейства). docs/*.md исключены ПО УМОЛЧАНИЮ — там file:line =
исторический леджер-координат, дрейфует by design и пином не потребляется.

Классы:
  FAIL  P1   file:line координата   (Foo.java:467)
  FAIL  P2   method:line координата (MobPushOps.pushables:467, Ops-классы)
  FAIL  P2B  call-chain стрелка     (<- stealTick:352)
  FAIL  P4   sed IN-PLACE с числовым адресом (line-addressed edit)
  WARN  P3   текстовое "line NNN" (исторические ссылки на прошлые фейлы)
  WARN  P5   индексное чтение splitlines()/readlines()[N>=1]

Выход: exit 0 (0 FAIL), 1 (есть FAIL вне allowlist), 2 (инфра-ошибка).
Allowlist: scripts/linepin_allowlist.txt, формат
  <path>:<lineno>:<PATTERN>:<reason>
Каждая запись = задокументированный пин с планом конверсии; STALE-записи
(паттерн уже не матчится) репортятся и считаются ошибкой уборки.

Использование:
  python3 scripts/linepin_audit.py                 # скан, exit-код-гейт
  python3 scripts/linepin_audit.py --json          # машиночитаемый отчёт
  python3 scripts/linepin_audit.py --selftest      # позитив-фикстуры
  python3 scripts/linepin_audit.py --negative-test # чистые файлы -> 0 FAIL
"""
import json
import os
import re
import subprocess
import sys

P1 = re.compile(r"\b[A-Za-z0-9_.$/-]+\.(?:java|class|rs|py|sh|toml|c|h|cpp|json):[0-9]{2,5}\b")
P2 = re.compile(r"\b[A-Za-z][A-Za-z0-9_]*Ops\.[A-Za-z][A-Za-z0-9_]*:[0-9]{2,5}\b")
P2B = re.compile(r"<-\s*[A-Za-z][A-Za-z0-9_]*:[0-9]{2,5}")
P3 = re.compile(r"\bline [0-9]{2,5}\b")
P5 = re.compile(r"\b(?:splitlines|readlines)\(\)\[[0-9]{1,4}\]")
SED_RE = re.compile(r"\bsed\b")
SED_I_RE = re.compile(r"(^|\s)-iB?(^|\s)")
SED_NUM_ADDR_RE = re.compile(r"['\"]?[0-9]{1,5}(?:\s*,\s*[0-9]{1,5})?['\"]?[a-zA-Z$!]")

FAIL_CLASSES = ("P1", "P2", "P2B", "P4")
WARN_CLASSES = ("P3", "P5")


def tracked_scripts(repo_root):
    try:
        out = subprocess.run(
            ["git", "-C", repo_root, "ls-files", "*.sh", "*.py"],
            capture_output=True, text=True, check=True).stdout
        return sorted(l for l in out.splitlines() if l)
    except Exception:
        return []


def load_allowlist(repo_root):
    path = os.path.join(repo_root, "scripts", "linepin_allowlist.txt")
    entries = set()
    stale_src = []
    if not os.path.exists(path):
        return entries, stale_src
    with open(path) as f:
        for ln, raw in enumerate(f, 1):
            raw = raw.strip()
            if not raw or raw.startswith("#"):
                continue
            parts = raw.split(":", 3)
            if len(parts) != 4:
                continue
            entries.add((parts[0], int(parts[1]), parts[2]))
            stale_src.append((raw, ln))
    return entries, stale_src


def scan_line(path, lineno, line):
    """Возвращает [(class, matched_text)] для одной строки."""
    hits = []
    if SED_RE.search(line) and SED_I_RE.search(line) and SED_NUM_ADDR_RE.search(line):
        hits.append(("P4", "sed in-place numeric-address"))
    for cls, rx in (("P1", P1), ("P2", P2), ("P2B", P2B), ("P3", P3), ("P5", P5)):
        m = rx.search(line)
        if m:
            hits.append((cls, m.group(0)))
    return hits


def scan_repo(repo_root, include_docs=False, allow=frozenset()):
    results = []
    files = tracked_scripts(repo_root)
    for rel in files:
        if not include_docs and rel.startswith("docs/"):
            continue
        full = os.path.join(repo_root, rel)
        try:
            with open(full, encoding="utf-8", errors="replace") as f:
                lines = f.read().splitlines()
        except OSError:
            continue
        for i, line in enumerate(lines, 1):
            for cls, text in scan_line(rel, i, line):
                key = (rel, i, cls)
                if key in allow:
                    status = "ALLOWED"
                elif cls in FAIL_CLASSES:
                    status = "FAIL"
                else:
                    status = "WARN"
                results.append({"file": rel, "line": i, "class": cls,
                                "text": text, "status": status})
    return results


def report(results, stale, as_json=False):
    fails = [r for r in results if r["status"] == "FAIL"]
    warns = [r for r in results if r["status"] == "WARN"]
    allowed = [r for r in results if r["status"] == "ALLOWED"]
    if as_json:
        print(json.dumps({"fail": fails, "warn": warns, "allowed": allowed,
                          "stale_allowlist": [{"entry": e, "file_line": l}
                                              for e, l in stale]}, indent=2))
    else:
        print(f"linepin_audit: FAIL={len(fails)} WARN={len(warns)} "
              f"ALLOWED={len(allowed)} STALE-ALLOW={len(stale)}")
        for r in fails:
            print(f"  [FAIL {r['class']}] {r['file']}:{r['line']}: {r['text']}")
        for r in warns:
            print(f"  [warn {r['class']}] {r['file']}:{r['line']}: {r['text']}")
        for r in allowed:
            print(f"  [allow {r['class']}] {r['file']}:{r['line']}: {r['text']}")
        for e, l in stale:
            print(f"  [STALE-ALLOW] allowlist line {l}: {e}")
    return len(fails)


def selftest():
    import tempfile
    ok = True
    with tempfile.TemporaryDirectory() as td:
        # git-независимый путь: monkeypatch tracked_scripts
        orig = tracked_scripts
        fixture = {
            "f1.sh": "# pin test\nx=Foo.java:467\ny=MobPushOps.pushables:467\n",
            "f2.sh": "# chain\nz <- stealTick:352\n",
            "f3.sh": "sed -i '467s/a/b/' f.java\n",
            "ok1.sh": "sed -n '2,34p' \"$0\"\nurl=http://h:8080/x\ntxt=splitlines()[0]\n",
        }
        for name, body in fixture.items():
            with open(os.path.join(td, name), "w") as f:
                f.write(body)
        import linepin_audit
        linepin_audit.tracked_scripts = lambda root: sorted(fixture)
        results = linepin_audit.scan_repo(td, include_docs=True, allow=frozenset())
        fails = sorted((r["file"], r["class"]) for r in results if r["status"] == "FAIL")
        expect = sorted([("f1.sh", "P1"), ("f1.sh", "P2"), ("f2.sh", "P2B"),
                         ("f3.sh", "P4")])
        if fails != expect:
            print(f"selftest FAIL: got {fails}, expect {expect}")
            ok = False
        benign = [r for r in results if r["file"] == "ok1.sh" and r["status"] == "FAIL"]
        if benign:
            print(f"selftest FAIL: benign file flagged {benign}")
            ok = False
        allow = {("f1.sh", 2, "P1")}
        results2 = linepin_audit.scan_repo(td, include_docs=True, allow=allow)
        st = {r["status"] for r in results2 if r["file"] == "f1.sh" and r["line"] == 2}
        if st != {"ALLOWED"}:
            print(f"selftest FAIL: allowlist not applied, got {st}")
            ok = False
        linepin_audit.tracked_scripts = orig
    print("selftest:", "PASS" if ok else "FAIL")
    return 0 if ok else 1


def negative_test():
    import tempfile
    with tempfile.TemporaryDirectory() as td:
        fixture = {
            "a.sh": "sed -n '1,10p' file.csv\nhead -3 file\n",
            "b.py": "line = open(p).read().strip().splitlines()[0]\n",
            "c.sh": "grep -n 'descriptor' blobs.txt\njavap -p -c Foo.class\n",
            "d.sh": "# история: URL https://x.y/z и порт :8080, не пин\n",
        }
        for name, body in fixture.items():
            with open(os.path.join(td, name), "w") as f:
                f.write(body)
        import linepin_audit
        orig = linepin_audit.tracked_scripts
        linepin_audit.tracked_scripts = lambda root: sorted(fixture)
        results = linepin_audit.scan_repo(td, include_docs=True, allow=frozenset())
        linepin_audit.tracked_scripts = orig
        fails = [r for r in results if r["status"] == "FAIL"]
        if fails:
            print(f"negative-test FAIL: {fails}")
            return 1
    print("negative-test: PASS (0 FAIL)")
    return 0


def main(argv):
    if "--selftest" in argv:
        return selftest()
    if "--negative-test" in argv:
        return negative_test()
    root_override = None
    if "--root" in argv:
        root_override = argv[argv.index("--root") + 1]
    own_root = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
    repo_root = root_override or own_root
    # allowlist всегда из СВОЕГО репо (ветки), скан может идти по чужому root
    allow, stale_src = load_allowlist(own_root)
    # STALE-детект: allowlist-запись, под которой паттерна больше нет
    # (файлы читаются из СВОЕГО репо — там, где живёт allowlist)
    stale = []
    for raw, ln in stale_src:
        parts = raw.split(":", 3)
        path, lineno, cls = parts[0], int(parts[1]), parts[2]
        full = os.path.join(own_root, path)
        try:
            with open(full, encoding="utf-8", errors="replace") as f:
                line = f.read().splitlines()[lineno - 1]
        except (OSError, IndexError):
            stale.append((raw, ln))
            continue
        if not any(c == cls for c, _ in scan_line(path, lineno, line)):
            stale.append((raw, ln))
    results = scan_repo(repo_root, "--include-docs" in argv, allow)
    nfail = report(results, stale, "--json" in argv)
    if nfail:
        print("linepin_audit: FAIL — канон S43/Л180i: координата = имя+дескриптор "
              "(ncdfe_guard.sh --pin), line-пины в тулинге запрещены")
        return 1
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
