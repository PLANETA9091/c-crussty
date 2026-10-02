#!/usr/bin/env python3
"""check_cp_exact.py — CP-EXACT gate (TASK-463-88a, lesson ×461/×463).

LEDGER-46 §3: merge 887c4641 union-glued two flag terms INSIDE one
equals() string — "cmp457_paldelta|cmp457_eqsnap2" (pipe inside the
constant-pool utf8 entry). A substring-grep for "cmp457_eqsnap2" is happy
(the token IS present) while the gate never matches a real flag: 7/10 java
gate classes slept and carried an NCDFE landmine. The canonical gate must be
CP-EXACT:

  1. CORRUPT  — a utf8 constant-pool entry carrying the combined pipe literal
                "cmp457_paldelta|cmp457_eqsnap2" UNLESS it is a segment of a
                longer pipe-separated flag LIST (preceded AND followed by '|',
                e.g. BrainOps TICK2_FLAGS whose mechanics is list-style and is
                preserved per file style). The merge-glue signature is the pair
                alone or a pair glued into a single equals() argument.
  2. STANDALONE (--require-standalone) — for gate classes that must arm on
                the eqsnap2 family, BOTH standalone utf8 entries
                "cmp457_paldelta" and "cmp457_eqsnap2" must exist in the
                constant pool (the ItemEntityManager ground-truth shape).

Parsing is a real constant-pool walk (not raw bytes): indy bootstrap recipes
(x93) hide concat constants from javap -c, so the utf8 entries are the truth.

Usage:
  python3 scripts/check_cp_exact.py                 # repo-wide CORRUPT scan (exit 1 on any)
  python3 scripts/check_cp_exact.py --all           # same, explicit
  python3 scripts/check_cp_exact.py FILE.class...   # CORRUPT scan of given classes
  python3 scripts/check_cp_exact.py --require-standalone FILE.class...
"""
import sys
from pathlib import Path

PAIR = b"cmp457_paldelta|cmp457_eqsnap2"
FLAG_A = "cmp457_paldelta"
FLAG_B = "cmp457_eqsnap2"

# --- constant pool walker -------------------------------------------------

def utf8_entries(path):
    """Yield every CONSTANT_Utf8 entry of a classfile as decoded str."""
    b = Path(path).read_bytes()
    if b[:4] != b"\xca\xfe\xba\xbe":
        raise ValueError(f"{path}: not a classfile (bad magic)")
    cp_count = int.from_bytes(b[8:10], "big")
    i = 10
    out = []
    idx = 1
    while idx < cp_count:  # Long/Double occupy TWO slots (JVM spec §4.4.5)
        tag = b[i]
        i += 1
        if tag == 1:  # Utf8
            n = int.from_bytes(b[i:i + 2], "big")
            i += 2
            out.append(b[i:i + n])
            i += n
        elif tag in (7, 8, 16, 19, 20):  # Class/String/MethodType/Module/Package
            i += 2
        elif tag in (15,):  # MethodHandle
            i += 3
        elif tag in (17, 18):  # Dynamic/InvokeDynamic
            i += 4
        elif tag in (3, 4, 9, 10, 11, 12):  # Integer/Float/refs/NameAndType
            i += 4
        elif tag in (5, 6):  # Long/Double (two slots)
            i += 8
            idx += 1
        else:
            raise ValueError(f"{path}: unknown cp tag {tag} at offset {i - 1}")
        idx += 1
    return [e for e in (x.decode("utf-8", "replace") for x in out if x is not None)]


def is_glue_entry(entry):
    """The ×463 merge-glue signature: a utf8 entry that is EXACTLY the two
    flag tokens glued by a pipe (in either order) — the union-resolve artifact
    of two standalone equals() terms. Longer pipe-separated lists (e.g.
    BrainOps TICK2_FLAGS, list-style mechanics preserved per file style) are
    NOT glue: their tokens are separate list members."""
    tokens = entry.split("|")
    return len(tokens) == 2 and set(tokens) == {FLAG_A, FLAG_B}


def audit(path, require_standalone=False):
    """Return list of problems for one classfile."""
    problems = []
    try:
        entries = utf8_entries(path)
    except (ValueError, OSError) as e:
        return [str(e)]
    corrupt = [e for e in entries if is_glue_entry(e)]
    for e in corrupt:
        problems.append(f"CORRUPT cp-exact: utf8 entry '{e}' carries the merge-glue "
                        f"pipe literal (was it union-resolved into standalone terms?)")
    if require_standalone:
        for flag in (FLAG_A, FLAG_B):
            if flag not in entries:
                problems.append(f"STANDALONE missing: utf8 entry '{flag}' not in "
                                f"constant pool (gate cannot arm — stale/glued blob)")
    return problems


# --- cli -------------------------------------------------------------------

def main(argv):
    require = False
    files = []
    for a in argv:
        if a == "--require-standalone":
            require = True
        elif a == "--all":
            root = Path(__file__).resolve().parent.parent
            files = [str(p) for p in root.rglob("*.class") if ".git" not in p.parts]
        else:
            files.append(a)
    if not files:
        root = Path(__file__).resolve().parent.parent
        files = [str(p) for p in root.rglob("*.class") if ".git" not in p.parts]
    bad = 0
    for f in sorted(files):
        probs = audit(f, require_standalone=require)
        if probs:
            bad += 1
            for p in probs:
                print(f"FAIL {f}: {p}", file=sys.stderr)
        else:
            if require:
                print(f"OK {f}: cp-exact clean, standalone '{FLAG_A}'+'{FLAG_B}' present")
            # plain mode stays quiet per-file; summary below
    if require:
        # in require mode failures already printed; summary only
        if bad:
            print(f"check_cp_exact: {bad} FILE(S) VIOLATE", file=sys.stderr)
            return 1
        return 0
    if bad:
        print(f"check_cp_exact: {bad} class file(s) carry the merge-glue pipe literal", file=sys.stderr)
        return 1
    print(f"check_cp_exact: CLEAN — '{PAIR.decode()}' merge-glue literal in 0 of {len(files)} class files")
    return 0


if __name__ == "__main__":
    sys.exit(main(sys.argv[1:]))
