#!/usr/bin/env python3
"""flame_diff.py — self-time delta between two JFR execution-sampling runs.

Compares a BASELINE run against a PROFILE run and prints the top-N
regression / improvement tables of *self time* (leaf frames of
jdk.ExecutionSample stacks — the classic quick triage after profiling the
Purpur+CRUSSTY server with scripts/profile_server.sh).

Accepted inputs (auto-detected per file, can be mixed):

  1. CSV emitted by scripts/profile_server.sh (*.hot_methods.csv):
       method,samples
       net.minecraft.server.level.ChunkMap$1.compute,812
       ...
     '#' comments and an optional header are tolerated; signature is already
     stripped by the shell script.

  2. Raw `jfr print --events jdk.ExecutionSample <file.jfr>` text dumps
     (profile_server.sh --keep-raw, or produce manually):
       jdk.ExecutionSample {
         startTime = ...
         stackTrace = [
           com.example.Hot.worker() line: 10
           ...
         ]
       }
     The FIRST frame of every jdk.ExecutionSample stackTrace block is counted
     as that sample's self time. Method names are normalized (signature and
     "(Native Method)" stripped, hidden lambda ids like $$Lambda+42/0x7f...
     collapsed) so runs stay comparable.

  3. A .jfr file directly — if a `jfr` binary is available (searched via
     --jdk, $JAVA_HOME, /home/z/jdk21, PATH) it is fed through
     `jfr print --events jdk.ExecutionSample` (subprocess).

A `jfr summary` dump alone contains no per-method data and is rejected with a
clear message.

Because two runs rarely collect the same number of samples, deltas are
reported both as raw samples and as share of total (percentage points, pp).
Ranking uses the share delta, which makes runs of different durations
comparable.

Usage:
  python3 scripts/flame_diff.py BASELINE PROFILE [--top N] [--out FILE.md]
                                [--jdk PATH] [--no-color]
  python3 scripts/flame_diff.py --self-test

Exit codes: 0 ok (self-test passed) | 1 usage/parsing error | 2 self-test failed.
Stdlib only.
"""

from __future__ import annotations

import argparse
import os
import re
import shutil
import subprocess
import sys
from collections import Counter

DEFAULT_JDK_CANDIDATES = ("/home/z/jdk21",)


# --------------------------------------------------------------- parsing ----

def normalize_frame(frame: str) -> str:
    """Normalize a jfr-print stack frame to a stable `package.Class.method` key."""
    f = frame.strip()
    f = re.sub(r"\s+line:.*$", "", f)          # trailing " line: 1234 [bci: N]"
    f = f.split("(", 1)[0].strip()             # signature + "(Native Method)" etc.
    f = re.sub(r"/0x[0-9a-fA-F]+", "", f)      # hidden class address suffix
    f = re.sub(r"\$\$Lambda\+\d+", "$$Lambda", f)  # lambda instantiation index
    return f


def parse_jfr_print(text: str, source: str = "<text>") -> Counter:
    """Count leaf frames of every jdk.ExecutionSample stackTrace block."""
    counts: Counter = Counter()
    cur_event = None
    in_stack = False
    leaf_taken = False
    samples = 0
    for lineno, raw in enumerate(text.splitlines(), 1):
        line = raw.strip()
        m = re.match(r"^(jdk\.[A-Za-z0-9_]+)\s*\{", line)
        if m:
            cur_event = m.group(1)
            in_stack = False
            continue
        if re.search(r"stackTrace\s*=\s*\[", line):
            in_stack = cur_event == "jdk.ExecutionSample"
            leaf_taken = False
            continue
        if not in_stack:
            continue
        if line == "]":
            in_stack = False
            continue
        if not leaf_taken:
            method = normalize_frame(line)
            if method:
                counts[method] += 1
                samples += 1
            leaf_taken = True
    if not samples:
        raise ValueError(
            f"{source}: no jdk.ExecutionSample stacks found — "
            "is this a `jfr print --events jdk.ExecutionSample` dump?"
        )
    return counts


def parse_csv(text: str, source: str = "<text>") -> Counter:
    """Parse the `method,samples` CSV emitted by profile_server.sh."""
    counts: Counter = Counter()
    for lineno, raw in enumerate(text.splitlines(), 1):
        line = raw.strip()
        if not line or line.startswith("#") or line.startswith("method,"):
            continue
        if "," not in line:
            raise ValueError(f"{source}:{lineno}: expected 'method,samples', got: {line!r}")
        method, _, num = line.rpartition(",")
        method = method.strip()
        num = num.strip()
        if not re.fullmatch(r"\d+", num):
            raise ValueError(f"{source}:{lineno}: samples column is not an integer: {num!r}")
        if not method:
            raise ValueError(f"{source}:{lineno}: empty method name")
        counts[normalize_frame(method)] += int(num)
    if not counts:
        raise ValueError(f"{source}: no method,samples rows found")
    return counts


def looks_like_summary(text: str) -> bool:
    # real `jfr summary` header is "Event Type   Count   Size (bytes)"
    return "Event Type" in text and "stackTrace" not in text


def parse_auto(path: str, jdk: str | None = None) -> Counter:
    """Read a file and dispatch to the right parser (CSV / jfr-print / .jfr)."""
    if path.endswith(".jfr"):
        return parse_jfr_file(path, jdk)
    try:
        with open(path, "r", encoding="utf-8", errors="replace") as fh:
            text = fh.read()
    except OSError as exc:
        raise ValueError(f"{path}: cannot read ({exc})") from exc
    if not text.strip():
        raise ValueError(f"{path}: empty file")
    return parse_auto_text(text, path)


def find_jfr_binary(jdk: str | None) -> str | None:
    candidates = []
    if jdk:
        candidates.append(os.path.join(jdk, "bin", "jfr"))
    if os.environ.get("JAVA_HOME"):
        candidates.append(os.path.join(os.environ["JAVA_HOME"], "bin", "jfr"))
    for cand in DEFAULT_JDK_CANDIDATES:
        candidates.append(os.path.join(cand, "bin", "jfr"))
    for cand in candidates:
        if os.path.isfile(cand) and os.access(cand, os.X_OK):
            return cand
    return shutil.which("jfr")


def parse_jfr_file(path: str, jdk: str | None = None) -> Counter:
    """Run `jfr print --events jdk.ExecutionSample` on a .jfr and parse it."""
    jfr_bin = find_jfr_binary(jdk)
    if not jfr_bin:
        raise ValueError(
            f"{path}: .jfr given but no `jfr` binary found. Pass --jdk /path/to/jdk21 "
            "or convert first: `jfr print --events jdk.ExecutionSample FILE.jfr > FILE.txt` "
            "(or use the *.hot_methods.csv emitted by profile_server.sh)."
        )
    try:
        proc = subprocess.run(
            [jfr_bin, "print", "--events", "jdk.ExecutionSample", path],
            capture_output=True, text=True, timeout=900, check=False,
        )
    except (OSError, subprocess.TimeoutExpired) as exc:
        raise ValueError(f"{path}: failed to run {jfr_bin} ({exc})") from exc
    if proc.returncode != 0:
        raise ValueError(f"{path}: jfr print failed: {proc.stderr.strip()[:400]}")
    return parse_jfr_print(proc.stdout, path)


# ------------------------------------------------------------------ diff ----

class Row:
    __slots__ = ("method", "base", "prof", "base_pct", "prof_pct", "dsamp", "dpp")

    def __init__(self, method, base, prof, base_pct, prof_pct):
        self.method = method
        self.base = base
        self.prof = prof
        self.base_pct = base_pct
        self.prof_pct = prof_pct
        self.dsamp = prof - base
        self.dpp = prof_pct - base_pct

    @property
    def status(self) -> str:
        if self.base == 0:
            return "new"
        if self.prof == 0:
            return "gone"
        return ""


def compute_diff(base: Counter, prof: Counter) -> tuple[list[Row], int, int]:
    tb, tp = sum(base.values()), sum(prof.values())
    rows = []
    for method in set(base) | set(prof):
        b, p = base.get(method, 0), prof.get(method, 0)
        rows.append(Row(method, b, p, 100.0 * b / tb, 100.0 * p / tp))
    return rows, tb, tp


def top_rows(rows: list[Row], top: int) -> tuple[list[Row], list[Row]]:
    regressions = sorted((r for r in rows if r.dpp > 1e-9),
                         key=lambda r: (-r.dpp, r.method))[:top]
    improvements = sorted((r for r in rows if r.dpp < -1e-9),
                          key=lambda r: (r.dpp, r.method))[:top]
    return regressions, improvements


def _fmt_row(r: Row) -> str:
    return (f"{r.dpp:+8.2f}  {r.dsamp:+6d}  {r.base:6d} ({r.base_pct:5.1f}%)  "
            f"{r.prof:6d} ({r.prof_pct:5.1f}%)  {r.status:>4}  {r.method}")


HEADER = (f"{'D pp':>8}  {'D samp':>6}  {'base':>13}  {'profile':>13}  {'note':>4}  method")


def print_table(title: str, rows: list[Row]) -> None:
    print(f"\n{title}")
    if not rows:
        print("  (none)")
        return
    print("  " + HEADER)
    for r in rows:
        print("  " + _fmt_row(r))


def markdown_table(title: str, rows: list[Row]) -> str:
    out = [f"## {title}", "", "| D pp | D samp | base | base % | profile | profile % | note | method |",
           "|---:|---:|---:|---:|---:|---:|---|---|"]
    for r in rows:
        out.append(f"| {r.dpp:+.2f} | {r.dsamp:+d} | {r.base} | {r.base_pct:.2f}% | "
                   f"{r.prof} | {r.prof_pct:.2f}% | {r.status} | `{r.method}` |")
    return "\n".join(out)


# ------------------------------------------------------------- self-test ----

SELFTEST_BASELINE = """\
jdk.ExecutionSample {
  startTime = 12:00:00.000 (2026-09-07)
  tid = 0x1
  javaThread = "main" (osThreadId = 1)
  stackTrace = [
    com.example.Hot.worker() line: 10
    com.example.Main.run() line: 2
  ]
}

jdk.ExecutionSample {
  startTime = 12:00:00.010 (2026-09-07)
  tid = 0x2
  javaThread = "Worker-1" (osThreadId = 2)
  stackTrace = [
    com.example.Hot.worker() line: 11
    com.example.Alloc.make() line: 5
  ]
}

jdk.ExecutionSample {
  startTime = 12:00:00.020 (2026-09-07)
  tid = 0x1
  javaThread = "main" (osThreadId = 1)
  stackTrace = [
    java.lang.Thread.sleep(long) line: 1838
    com.example.Main.idle() line: 9
  ]
}
"""

SELFTEST_PROFILE = """\
jdk.ExecutionSample {
  startTime = 12:01:00.000 (2026-09-07)
  tid = 0x1
  javaThread = "main" (osThreadId = 1)
  stackTrace = [
    net.minecraft.server.level.ChunkMap$$Lambda+42/0x00007f8a1b2c3d40.tick() line: 7
    net.minecraft.server.level.ChunkMap.tickChunks() line: 900
  ]
}

jdk.ExecutionSample {
  startTime = 12:01:00.010 (2026-09-07)
  tid = 0x2
  javaThread = "Worker-1" (osThreadId = 2)
  stackTrace = [
    net.minecraft.server.level.ChunkMap$$Lambda+42/0x00007f8a1b2c3d40.tick() line: 7
    net.minecraft.server.level.ChunkMap.tickChunks() line: 900
  ]
}

jdk.ExecutionSample {
  startTime = 12:01:00.020 (2026-09-07)
  tid = 0x1
  javaThread = "main" (osThreadId = 1)
  stackTrace = [
    com.example.Hot.worker() line: 10
    com.example.Main.run() line: 2
  ]
}

jdk.ExecutionSample {
  startTime = 12:01:00.030 (2026-09-07)
  tid = 0x1
  javaThread = "main" (osThreadId = 1)
  stackTrace = [
    java.lang.Thread.sleep(long) line: 1838
    com.example.Main.idle() line: 9
  ]
}

jdk.ExecutionSample {
  startTime = 12:01:00.040 (2026-09-07)
  tid = 0x3
  javaThread = "Worker-2" (osThreadId = 3)
  stackTrace = [
    jdk.internal.misc.Unsafe.park(boolean, long) (Native Method)
    java.util.concurrent.locks.LockSupport.park() line: 371
  ]
}
"""

SELFTEST_CSV = """\
# emitted by profile_server.sh
method,samples
com.example.Hot.worker,2
java.lang.Thread.sleep,1
"""

SELFTEST_SUMMARY = """\
 Version: 2.1
 Chunks: 1
 Start: 2026-09-07 12:00:00.000 (UTC)
 Duration: 12 s

 Event Type                              Count  Size (bytes) 
=============================================================
 jdk.ExecutionSample 3 70
 jdk.JavaMonitorEnter 1 23
"""


def self_test() -> int:
    failures = []

    def check(name, cond):
        print(f"  [{'PASS' if cond else 'FAIL'}] {name}")
        if not cond:
            failures.append(name)

    print("flame_diff.py self-test")

    # 1. jfr-print text parsing + normalization
    base = parse_jfr_print(SELFTEST_BASELINE, "selftest-baseline")
    check("baseline leaf frames parsed", dict(base) == {"com.example.Hot.worker": 2,
                                                        "java.lang.Thread.sleep": 1})
    prof = parse_jfr_print(SELFTEST_PROFILE, "selftest-profile")
    check("profile lambda+native frames normalized",
          dict(prof) == {"net.minecraft.server.level.ChunkMap$$Lambda.tick": 2,
                         "com.example.Hot.worker": 1,
                         "java.lang.Thread.sleep": 1,
                         "jdk.internal.misc.Unsafe.park": 1})

    # 2. CSV parsing matches text parsing of the same run
    base_csv = parse_csv(SELFTEST_CSV, "selftest-csv")
    check("CSV parser agrees with jfr-print parser", dict(base_csv) == dict(base))

    # 3. summary-only input is rejected with a clear error
    try:
        parse_auto_text(SELFTEST_SUMMARY, "selftest-summary")
        check("jfr summary dump rejected", False)
    except ValueError as exc:
        check("jfr summary dump rejected ('%s')" % str(exc)[:60], True)

    # 4. diff math
    rows, tb, tp = compute_diff(base, prof)
    check("totals 3 -> 5", (tb, tp) == (3, 5))
    by_m = {r.method: r for r in rows}
    lam = by_m["net.minecraft.server.level.ChunkMap$$Lambda.tick"]
    check("new lambda method is a regression (+40pp, status new)",
          lam.dpp > 0 and abs(lam.dpp - 40.0) < 1e-9 and lam.status == "new" and lam.dsamp == 2)
    worker = by_m["com.example.Hot.worker"]
    check("worker improved (2/3 -> 1/5 = -46.67pp)",
          abs(worker.dpp - (20.0 - 100.0 * 2 / 3)) < 1e-6 and worker.dpp < 0)
    park = by_m["jdk.internal.misc.Unsafe.park"]
    check("Unsafe.park regression (+20pp)", abs(park.dpp - 20.0) < 1e-9 and park.status == "new")
    sleep = by_m["java.lang.Thread.sleep"]
    check("sleep share dropped (33.3 -> 20.0 = -13.33pp)", abs(sleep.dpp + 13.333333) < 1e-4)

    regs, imps = top_rows(rows, top=30)
    check("regressions ranked: lambda first, Unsafe.park second",
          regs[0].method == "net.minecraft.server.level.ChunkMap$$Lambda.tick"
          and regs[1].method == "jdk.internal.misc.Unsafe.park")
    check("improvements ranked: worker first",
          imps[0].method == "com.example.Hot.worker")

    # 5. tables render
    print_table("TOP REGRESSIONS (self-test)", regs)
    print_table("TOP IMPROVEMENTS (self-test)", imps)
    md = markdown_table("SELF-TEST", regs)
    check("markdown table renders", "| method |" in md and "ChunkMap" in md)

    # 6. bad input fails loudly
    try:
        parse_csv("com.example.Foo\n", "bad")
        check("CSV without samples column rejected", False)
    except ValueError:
        check("CSV without samples column rejected", True)

    print()
    if failures:
        print(f"SELF-TEST FAILED: {len(failures)} check(s): {failures}")
        return 2
    print("SELF-TEST PASSED (6 groups, 13 checks)")
    return 0


def parse_auto_text(text: str, source: str) -> Counter:
    """Dispatch on in-memory text (shared by parse_auto and self-test)."""
    if looks_like_summary(text):
        raise ValueError(
            f"{source}: this is a `jfr summary` dump — it has no per-method data. "
            "Use a *.hot_methods.csv, a `jfr print --events jdk.ExecutionSample` "
            "dump, or the .jfr file itself."
        )
    if re.search(r"^jdk\.[A-Za-z0-9_]+\s*\{", text, re.M) or "stackTrace = [" in text:
        return parse_jfr_print(text, source)
    return parse_csv(text, source)


# ------------------------------------------------------------------ main ----

def main(argv: list[str] | None = None) -> int:
    ap = argparse.ArgumentParser(
        prog="flame_diff.py",
        description="Self-time delta (regressions/improvements) between two JFR runs. "
                    "Inputs: *.hot_methods.csv, `jfr print --events jdk.ExecutionSample` "
                    "dumps, or .jfr files (needs a jfr binary).",
    )
    ap.add_argument("baseline", nargs="?", help="baseline dump/CSV/.jfr")
    ap.add_argument("profile", nargs="?", help="profile dump/CSV/.jfr")
    ap.add_argument("--top", type=int, default=30, metavar="N",
                    help="rows per table (default: 30)")
    ap.add_argument("--out", metavar="FILE.md", help="also write a markdown table")
    ap.add_argument("--jdk", metavar="PATH", help="JDK home for the `jfr` binary (.jfr inputs)")
    ap.add_argument("--no-color", action="store_true", help="disable ANSI colours")
    ap.add_argument("--self-test", action="store_true",
                    help="run embedded self-test (no arguments needed)")
    args = ap.parse_args(argv)

    if args.self_test:
        return self_test()
    if not args.baseline or not args.profile:
        ap.error("need BASELINE and PROFILE (or --self-test)")

    try:
        base = parse_auto(args.baseline, args.jdk)
        prof = parse_auto(args.profile, args.jdk)
    except ValueError as exc:
        print(f"ERROR: {exc}", file=sys.stderr)
        return 1

    rows, tb, tp = compute_diff(base, prof)
    regs, imps = top_rows(rows, args.top)

    use_color = sys.stdout.isatty() and not args.no_color
    green = "\033[32m" if use_color else ""
    red = "\033[31m" if use_color else ""
    reset = "\033[0m" if use_color else ""

    print(f"self-time diff: {args.baseline} ({tb} samples) -> {args.profile} ({tp} samples)")
    print("deltas ranked by share of total samples (pp = percentage points; "
          "self time = jdk.ExecutionSample leaf frames)")

    print(f"\n{red}TOP REGRESSIONS (profile worse, max {args.top}){reset}")
    if regs:
        print("  " + HEADER)
        for r in regs:
            print("  " + _fmt_row(r))
    else:
        print("  (none)")
    print(f"\n{green}TOP IMPROVEMENTS (profile better, max {args.top}){reset}")
    if imps:
        print("  " + HEADER)
        for r in imps:
            print("  " + _fmt_row(r))
    else:
        print("  (none)")

    print(f"\ntotals: baseline {tb} samples, profile {tp} samples, "
          f"{len(rows)} distinct methods "
          f"({sum(1 for r in rows if r.status == 'new')} new / "
          f"{sum(1 for r in rows if r.status == 'gone')} gone)")

    if args.out:
        with open(args.out, "w", encoding="utf-8") as fh:
            fh.write(f"# flame_diff: {args.baseline} -> {args.profile}\n\n"
                     f"baseline {tb} samples, profile {tp} samples.\n\n"
                     + markdown_table(f"Top {args.top} regressions", regs)
                     + "\n\n" + markdown_table(f"Top {args.top} improvements", imps)
                     + "\n")
        print(f"markdown written: {args.out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
