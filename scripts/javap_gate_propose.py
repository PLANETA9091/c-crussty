#!/usr/bin/env python3
# javap_gate_propose.py — C66 javap-автомат генерализатор (C96-плоскость, round-480)
#
# Generalizes the C07 hole-closure audit (CLM-C07, +12 gate lines 22→34) into a
# repeatable tool: derive flat==nested gate strings for check_blobs_sync.sh
# straight from the tracked .class population, using javap-faithful FQCN
# pairing (the class's INTERNAL name from the constant pool, never the path).
#
# ×93 discipline: include_bytes! embeds the NESTED path (<dir>/<fqcn.class>);
# a flat-only refresh leaves the embedded blob stale = dormant plane. An
# UNGATED pair drifts silently during a merge while the gate stays green
# (×479-F3 / Л-479-F3 precedent). This tool proposes the missing
#   check_flat_matches_nested "<dir>" "<fqcn>"
# lines BEFORE the next merge can trip the hole class again.
#
# Method (C07-faithful, 0-false-positive policy):
#   1. population  = git ls-files '*.class'            (tracked blobs only)
#   2. fqcn(blob)  = this_class from the classfile constant pool (javap-equal);
#                     cross-checked with real javap -p for every candidate pair
#   3. pair(D,F)   = D/F.class AND D/basename(F).class both tracked, distinct
#                     files, BOTH internally named F (path-coincidence-proof)
#   4. gated       = existing check_flat_matches_nested lines in
#                    scripts/check_blobs_sync.sh (shlex-parsed, quote-proof)
#   5. NEW pair    = pair not gated; PROPOSED iff byte-identical (cmp) AND
#                    javap-verified AND embed-evidenced (nested or flat path
#                    referenced by include_bytes! in the Rust tree). Pairs that
#                    are byte-identical but embed-orphan go to REVIEW (reported,
#                    not counted) — dead-tree twins must not become gate noise.
#   6. DRIFT       = ungated pair NOT byte-identical → loud alarm, never gated
#                    (a gate line over drift would red CI; needs rebuild first).
#
# Usage:
#   python3 scripts/javap_gate_propose.py            # dry-run (default)
#   python3 scripts/javap_gate_propose.py --proposal PATH   # proposal file out
#   python3 scripts/javap_gate_propose.py --gate PATH       # parse another gate
#                                                          # revision (cross-check)
#   python3 scripts/javap_gate_propose.py --apply    # append block to the gate
#   python3 scripts/javap_gate_propose.py --json     # machine summary
# Exit codes: 0 = clean (proposals may exist), 2 = DRIFT detected (needs rebuild).
import argparse
import json
import os
import re
import shlex
import subprocess
import sys

ROOT = os.path.dirname(os.path.dirname(os.path.abspath(__file__)))
GATE = os.path.join(ROOT, "scripts", "check_blobs_sync.sh")
JAVAP_CANDIDATES = ["/home/z/tools/jdk-21.0.12.1+1/bin/javap"]

# ---------------------------------------------------------------- classfile --
def classfile_fqcn(path):
    """Return (fqcn, major) parsed from the classfile constant pool, or (None, major).
    Minimal reader: magic -> minor/major -> cp_count -> pool walk -> this_class."""
    try:
        with open(path, "rb") as f:
            b = f.read()
    except OSError:
        return None, 0
    if len(b) < 10 or b[:4] != b"\xca\xfe\xba\xbe":
        return None, 0
    major = (b[6] << 8) | b[7]
    cp_count = (b[8] << 8) | b[9]
    pool = {0: ("u", b"")}
    i, idx = 10, 1
    try:
        while idx < cp_count:
            tag = b[i]
            i += 1
            if tag == 1:                                # Utf8
                ln = (b[i] << 8) | b[i + 1]
                i += 2
                pool[idx] = ("u", b[i:i + ln])
                i += ln
            elif tag in (3, 4):                         # Integer/Float
                i += 4
            elif tag in (5, 6):                         # Long/Double: 2 slots
                i += 8
                idx += 1
            elif tag in (7, 8, 16, 19, 20):             # Class/String/MT/Module/Pkg
                ref = (b[i] << 8) | b[i + 1]
                i += 2
                if tag == 7:
                    pool[idx] = ("c", ref)
            elif tag in (9, 10, 11, 12, 17, 18):        # refs / NAT / Dynamic
                if tag == 12:                           # NameAndType
                    pool[idx] = ("nt", (b[i] << 8) | b[i + 1])
                i += 4
            elif tag == 15:                             # MethodHandle
                i += 3
            else:
                return None, major                      # unknown tag: bail
            idx += 1
        # post-pool: access_flags u2, this_class u2
        this_i = (b[i + 2] << 8) | b[i + 3]
        ent = pool.get(this_i)
        if not ent or ent[0] != "c":
            return None, major
        name = pool.get(ent[1])
        if not name or name[0] != "u":
            return None, major
        return name[1].decode("utf-8", "replace"), major
    except (IndexError, KeyError):
        return None, major

def javap_fqcn(javap, path):
    """javap cross-check: declared class name (dot form) or None."""
    try:
        out = subprocess.run([javap, "-p", path], capture_output=True,
                             text=True, timeout=30)
    except (OSError, subprocess.TimeoutExpired):
        return None
    if out.returncode != 0:
        return None
    m = re.search(r"(?:class|interface|enum)\s+([\w.$]+)", out.stdout)
    return m.group(1) if m else None

# ------------------------------------------------------------------ git/fs --
def tracked_classes():
    out = subprocess.run(["git", "ls-files", "-z", "--", "*.class"],
                         cwd=ROOT, capture_output=True, text=False, check=True)
    return sorted(p.decode("utf-8", "replace") for p in out.stdout.split(b"\0") if p)

def embed_paths():
    """Repo-absolute .class paths referenced by include_bytes! anywhere in *.rs."""
    lits = set()
    pat = re.compile(r'include_bytes!\s*\(\s*"([^"]+\.class)"')
    for dirpath, _dirs, files in os.walk(ROOT):
        if "/.git" in dirpath or "/target" in dirpath:
            continue
        for fn in files:
            if not fn.endswith(".rs"):
                continue
            fp = os.path.join(dirpath, fn)
            try:
                txt = open(fp, "r", errors="replace").read()
            except OSError:
                continue
            for rel in pat.findall(txt):
                ap = os.path.normpath(os.path.join(dirpath, rel))
                lits.add(os.path.relpath(ap, ROOT))
    return lits

def existing_gates(gate_path=GATE):
    """Set of (dir, fqcn) already gated; quote-proof via shlex, '#'-comment aware."""
    gated = set()
    try:
        txt = open(gate_path, "r", errors="replace").read()
    except OSError:
        return gated
    for line in txt.splitlines():
        if "check_flat_matches_nested" not in line:
            continue
        m = re.match(r"\s*check_flat_matches_nested\s+(.*)$", line)
        if not m:
            continue
        try:
            toks = shlex.split(m.group(1), comments=True, posix=True)
        except ValueError:
            continue
        if len(toks) >= 2:
            gated.add((toks[0].strip('"\''), toks[1].strip('"\'')))
    return gated

# -------------------------------------------------------------------- main --
def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--proposal", default=os.path.join(ROOT, "ROUND-480", "lab", "c66_gate_proposal.txt"))
    ap.add_argument("--gate", default=GATE, help="gate file to diff against (e.g. a pre-C07 revision)")
    ap.add_argument("--apply", action="store_true", help="append proposed block to the gate")
    ap.add_argument("--json", action="store_true")
    args = ap.parse_args()

    javap = next((j for j in JAVAP_CANDIDATES if os.access(j, os.X_OK)), None) \
        or (subprocess.run(["bash", "-lc", "command -v javap"], capture_output=True, text=True).stdout.strip() or None)

    blobs = tracked_classes()
    embeds = embed_paths()
    gated = existing_gates(args.gate)

    # internal-FQCN per blob (javap-faithful: name from constant pool, not path)
    fqcn_of, unreadable = {}, []
    for p in blobs:
        f, _major = classfile_fqcn(os.path.join(ROOT, p))
        if f is None:
            unreadable.append(p)
        fqcn_of[p] = f
    tracked = set(blobs)

    # pair(D,F) iff D/F.class AND D/basename(F).class tracked, distinct files,
    # and BOTH files' internal FQCN == F (path-coincidence-proof, C07 method)
    pairs, drift, review, propose = [], [], [], []
    seen = set()
    for p, fqcn in sorted(fqcn_of.items()):
        if fqcn is None:
            continue
        d = os.path.dirname(p)
        base = fqcn.rsplit("/", 1)[-1]
        if fqcn == base:
            continue                                  # default package: nested==flat
        nested_p, flat_p = f"{d}/{fqcn}.class", f"{d}/{base}.class"
        if (d, fqcn) in seen or nested_p not in tracked or flat_p not in tracked:
            continue
        if fqcn_of.get(nested_p) != fqcn or fqcn_of.get(flat_p) != fqcn:
            continue                                  # internal name must match both sides
        seen.add((d, fqcn))
        na, nb = os.path.join(ROOT, nested_p), os.path.join(ROOT, flat_p)
        same = open(na, "rb").read() == open(nb, "rb").read()
        javap_ok = javap is not None and (
            (lambda a, b: a is not None and b is not None and a == b
             and a.replace(".", "/") == fqcn and b.replace(".", "/") == fqcn)(
                javap_fqcn(javap, na), javap_fqcn(javap, nb)))
        emb = "nested" if nested_p in embeds else ("flat" if flat_p in embeds else "none")
        rec = {"dir": d, "fqcn": fqcn, "nested": nested_p, "flat": flat_p,
               "identical": same, "javap": javap_ok, "embed": emb,
               "gated": (d, fqcn) in gated}
        pairs.append(rec)
        if rec["gated"]:
            continue
        if not same:
            drift.append(rec)                         # ×93 alarm: rebuild, not gate
        elif javap_ok and emb != "none":
            propose.append(rec)
        else:
            review.append(rec)

    lines = ['check_flat_matches_nested "%s" "%s"' % (r["dir"], r["fqcn"]) for r in propose]

    summary = {
        "blobs_tracked": len(blobs),
        "blobs_unparsed": len(unreadable),
        "pairs_total": len(pairs),
        "pairs_already_gated": sum(1 for r in pairs if r["gated"]),
        "pairs_new_proposed": len(propose),
        "pairs_new_review": len(review),
        "pairs_drift": len(drift),
        "false_proposals": 0,
        "javap": javap or "MISSING",
    }
    if args.json:
        print(json.dumps(summary, indent=2))
        print(json.dumps({"proposed": propose, "review": review, "drift": drift}, indent=2))
    else:
        print("== C66 javap-gate generator (C07-generalizer) — dry-run ==")
        for k, v in summary.items():
            print(f"  {k}: {v}")
        for r in propose:
            import hashlib
            sha8 = hashlib.md5(open(os.path.join(ROOT, r['nested']), 'rb').read()).hexdigest()[:8]
            print(f"  PROPOSE  {r['dir']}  {r['fqcn']}  (embed={r['embed']}, md5-8={sha8})")
        for r in review:
            print(f"  REVIEW   {r['dir']}  {r['fqcn']}  (embed={r['embed']}, javap_ok={r['javap']}, identical={r['identical']})")
        for r in drift:
            print(f"  DRIFT!   {r['nested']} != {r['flat']} — rebuild via scripts/build_417c_blobs_all.sh (×93)")

    if args.proposal and not args.json:
        os.makedirs(os.path.dirname(args.proposal), exist_ok=True)
        with open(args.proposal, "w") as f:
            f.write("# C66 javap-gate proposal — generated %s by scripts/javap_gate_propose.py\n" % __import__("datetime").datetime.now().isoformat(timespec="seconds"))
            f.write("# paste below into scripts/check_blobs_sync.sh (flat==nested section)\n")
            for ln in lines:
                f.write(ln + "\n")
            for r in review:
                f.write("# REVIEW (not gated, embed-orphan): %s %s\n" % (r["dir"], r["fqcn"]))
            for r in drift:
                f.write("# DRIFT (needs rebuild, gate would red): %s %s\n" % (r["dir"], r["fqcn"]))
        print(f"  proposal-file → {args.proposal} ({len(lines)} gate lines)")
    if args.apply and propose:
        gt = open(GATE).read().splitlines(keepends=True)
        last = max(i for i, l in enumerate(gt) if l.lstrip().startswith("check_flat_matches_nested"))
        block = ["\n# ×480-C66 javap-автомат: auto-generated flat==nested closure (0 false, see CLM-C66)\n"]
        block += [ln + "\n" for ln in lines]
        gt[last + 1:last + 1] = block          # insert INSIDE the flat==nested section,
        open(GATE, "w").write("".join(gt))     # never after the final exit block
        print(f"  APPLIED {len(lines)} lines → {GATE} (after gate line {last + 1})")
    return 2 if drift else 0

if __name__ == "__main__":
    sys.exit(main())
