#!/usr/bin/env python3
"""lineunion_harness.py — R468-S57 line-union test harness (Л141/Л145/Л171 canon).

WHAT: synthetic generator of line-union merge defects + full censor-chain
verifier. Proves NUMERICALLY that every defect class introduced by the
line-token-delta union strategy (MERGE #9 style) is caught by the merge
censor chain, and that each censor is NECESSARY (per-class expected-censor
matrix: the classes javac/rustc/bash-n are blind to are owned by CP-scan /
flagtok / canonline / sync_gate).

DEFECT CLASSES (ground truth = Л141-УРОК-3/4, Л145, Л171, ×461/×463):
  C1 dub_tail      — dangling duplicate tail `; || ...equals(...);` after `;`
                     (Л141: line-union java/rust порождает висящие дубли-хвосты)
  C2a flag_glue    — two lever flags pipe-glued into ONE quoted/CP utf8 token
                     "cmp457_paldelta|cmp457_eqsnap2" (×461/×463: merge
                     887c4641, 7/10 gate classes slept; substring-grep blind)
  C2b line_glue    — code line absorbed into preceding comment line
                     (REAL bee2b585 defect: `set -uo pipefail` glued after a
                     `# ====` separator => silently disabled pipefail)
  C3 marker_residue— `origin/<branch>` refs / `<<<<<<<` conflict markers left
                     in code or comments (Л141-УРОК-4: обязателен пост-ценз
                     branch-имён в коде)
  C4 case_slice    — eaten `;;` case-arm terminator: middle-arm (bash -n
                     syntax error, Л145 line 532) and before-esac (SILENT,
                     legal bash — only case_arm_scan WARN sees it)
  C5 stale_blob    — source edited, .class blob NOT rebuilt (rebuild no-op,
                     Л171/x93) — javac never sees it, only checksum/source-gate

CENSOR CHAIN (11): bash -n; case_arm_scan.py (vendored canon C77/gatev2);
javac; rustc --emit=metadata; cp_corrupt (classfile utf8 walk — port of
check_cp_exact.py CORRUPT rule, indy-recipe-proof raw CP truth, javap-gate
family); cp_standalone (required flags must be EXACT utf8 entries — port of
--require-standalone); cp_marker (branch strings in CP); marker_grep
(source text); flagtok (quoted cmp-token must be a single token);
canonline (manifest canon bare-lines, e.g. ^set -uo pipefail$); sync_gate
(checksums + flag-set source<->blob, check_blobs_sync source-gate canon).

FULL RUN corpus: 18 clean (6 sh + 6 java + 6 rs, FP baseline) + 53 defects
(12 dub_tail + 12 flag_glue [9 single-site + 3 all-sites java, the exact
x461 standalone-vanish signature] + 12 line_glue + 9 marker_residue + 6
case_slice + 2 stale) + 2 real-git-history fixtures (broken bee2b585 MERGE
#9 result with eaten `;;` @line 532 + glued `set -uo pipefail`; clean
parent bee2b585^).

Usage:
  python3 scripts/lineunion_harness.py                 # full run
  python3 scripts/lineunion_harness.py --selftest      # 6-file mini corpus, fast
  python3 scripts/lineunion_harness.py --corpus-dir D [--keep] [--json R.json]

Exit 0 iff: every defect detected by >=1 expected censor, zero censor hits
on clean corpus, both historical fixtures behave as recorded.
Deterministic: fixed injection enumeration, no RNG.
"""
import argparse
import hashlib
import json
import re
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

REPO = Path(__file__).resolve().parent.parent
CASE_ARM_SCAN = Path(__file__).resolve().parent / "case_arm_scan.py"

JDK = Path("/home/z/tools/jdk-21.0.12.1+1/bin")
JAVAC = str(JDK / "javac") if (JDK / "javac").exists() else shutil.which("javac")
RUSTC = str(Path.home() / ".cargo/bin/rustc") if (Path.home() / ".cargo/bin/rustc").exists() else shutil.which("rustc")

FIXTURES = {
    "broken_bee2b585": (REPO / "tests/lineunion/fixtures/run_world3_broken_bee2b585.sh",
                        ["bash_n", "case_arm_scan", "canonline"]),
    "clean_bee2b585p": (REPO / "tests/lineunion/fixtures/run_world3_clean.sh", []),
}
CANON_FIXTURE_LINES = [r"^set -uo pipefail$",
                       r"^# Benchmark 3.0 — REAL-WORLD no-player load benchmark in GitHub CI$"]

# ---------------------------------------------------------------- templates
SH_TMPL = """#!/usr/bin/env bash
# S57 synthetic lever-arm harness fixture {idx} (lineunion corpus)
set -uo pipefail
LEVER_FLAG="${{LEVER_FLAG:-cmp457_eqsnap2}}"
log() {{ printf '[S57] %s\\n' "$1"; }}
case "${{LEVER_FLAG}}" in
  cmp456_chunkmono)
    export CRUSSTY_KERNEL_POLICY="off"
    log "${{LEVER_FLAG}} armed: CHUNKMONO-CARRIER"
    ;;
  cmp457_eqsnap2)
    export CRUSSTY_KERNEL_POLICY="off"
    log "${{LEVER_FLAG}} armed: EQSNAP2"
    ;;
  cmp452_mega)
    log "${{LEVER_FLAG}} armed: MEGA"
    ;;
  *)
    log "no lever: ${{LEVER_FLAG}}"
    ;;
esac
log "harness done"
"""

JAVA_TMPL = """// S57 synthetic gate-class fixture {idx} (lineunion corpus)
public class {clsid} {{
    static final String LEVER_FLAG = System.getProperty("lever.flag", "cmp457_eqsnap2");
    static boolean gate() {{
        return "cmp456_chunkmono".equals(LEVER_FLAG)
            || "cmp457_eqsnap2".equals(LEVER_FLAG)
            || "cmp452_mega".equals(LEVER_FLAG)
            || "cmp434_chunkpl".equals(LEVER_FLAG);
    }}
    static boolean pairGate() {{
        return "cmp457_paldelta".equals(LEVER_FLAG)
            || "cmp457_eqsnap2".equals(LEVER_FLAG);
    }}
    public static void main(String[] a) {{ System.out.println(gate() + ":" + pairGate()); }}
}}
"""

RS_TMPL = """// S57 synthetic lever gate fixture {idx} (lineunion corpus)
pub fn gate(flag: &str) -> bool {{
    flag == "cmp456_chunkmono"
        || flag == "cmp457_eqsnap2"
        || flag == "cmp452_mega"
        || flag == "cmp434_chunkpl"
}}
pub fn probe(x: u64) -> u64 {{ x.wrapping_mul(3).wrapping_add(1) }}
"""

SH_CANON = [r"^set -uo pipefail$"]

# ------------------------------------------------------------- injectors
def _rep(text, old, new):
    assert old in text, "injector anchor missing: %r" % old[:60]
    return text.replace(old, new, 1)

def inj_dub_tail(t, lang):
    if lang == "java":
        return _rep(t, '|| "cmp434_chunkpl".equals(LEVER_FLAG);',
                       '|| "cmp434_chunkpl".equals(LEVER_FLAG); || "cmp424_mobfeed".equals(LEVER_FLAG);')
    if lang == "rs":
        return _rep(t, '        || flag == "cmp434_chunkpl"',
                       '        || flag == "cmp434_chunkpl"; || flag == "cmp424_mobfeed"')
    return _rep(t, '    log "${LEVER_FLAG} armed: MEGA"',
                   '    log "${LEVER_FLAG} armed: MEGA" ; || log "dub tail"')

def inj_flag_glue(t, lang, sub=None):
    glued = '"cmp457_paldelta|cmp457_eqsnap2"'
    if lang == "java":
        if sub == "all":  # x461 exact signature: EVERY eqsnap2 literal glued, standalone entry vanishes
            assert t.count('"cmp457_eqsnap2"') == 3  # gate() + pairGate() + getProperty default
            return t.replace('"cmp457_eqsnap2"', glued, 3)
        return _rep(t, '"cmp457_eqsnap2".equals(LEVER_FLAG)', glued + '.equals(LEVER_FLAG)')
    if lang == "rs":
        return _rep(t, 'flag == "cmp457_eqsnap2"', 'flag == ' + glued)
    return _rep(t, 'LEVER_FLAG="${LEVER_FLAG:-cmp457_eqsnap2}"',
                   'LEVER_FLAG="${LEVER_FLAG:-cmp457_paldelta|cmp457_eqsnap2}"')

def inj_line_glue(t, lang):
    if lang == "java":
        return _rep(t, '    static boolean gate() {\n        return "cmp456_chunkmono".equals(LEVER_FLAG)',
                       '    static boolean gate() { // gate chain: return "cmp456_chunkmono".equals(LEVER_FLAG)')
    if lang == "rs":
        return _rep(t, 'pub fn gate(flag: &str) -> bool {\n    flag == "cmp456_chunkmono"',
                       '// fn gate absorbed: pub fn gate(flag: &str) -> bool {\n    flag == "cmp456_chunkmono"')
    return _rep(t, "set -uo pipefail\n", "# ============================== set -uo pipefail\n")

def inj_marker_residue(t, lang, bare):
    if bare:
        return t + "\n<<<<<<< HEAD\n>>>>>>> round-466-poi-p22-1\n"
    if lang == "java":
        return _rep(t, "// S57 synthetic gate-class fixture",
                       "// S57 synthetic gate-class fixture (union of origin/round-466-poi-p22-1")
    if lang == "rs":
        return _rep(t, "// S57 synthetic lever gate fixture",
                       "// S57 synthetic lever gate fixture (union of origin/round-466-poi-p22-1")
    return _rep(t, "# S57 synthetic lever-arm harness fixture",
                   "# S57 synthetic lever-arm harness fixture (union of origin/round-466-poi-p22-1")

def inj_case_slice(t, middle):
    if middle:
        return _rep(t, '    ;;\n  cmp452_mega)', '  cmp452_mega)')
    return _rep(t, '    log "no lever: ${LEVER_FLAG}"\n    ;;\nesac',
                   '    log "no lever: ${LEVER_FLAG}"\nesac')

def inj_stale(t, sub):
    if sub == "add":
        return _rep(t, '            || "cmp434_chunkpl".equals(LEVER_FLAG);',
                       '            || "cmp434_chunkpl".equals(LEVER_FLAG)\n            || "cmp424_mobfeed".equals(LEVER_FLAG);')
    return _rep(t, '            || "cmp452_mega".equals(LEVER_FLAG)\n', '')

# ------------------------------------------------- classfile utf8 walker
def utf8_entries(path):
    """CONSTANT_Utf8 entries of a classfile (JVM spec §4.4; Long/Double = 2 slots)."""
    b = Path(path).read_bytes()
    if b[:4] != b"\xca\xfe\xba\xbe":
        raise ValueError("%s: not a classfile" % path)
    cp_count = int.from_bytes(b[8:10], "big")
    i, idx, out = 10, 1, []
    while idx < cp_count:
        tag = b[i]; i += 1
        if tag == 1:
            n = int.from_bytes(b[i:i + 2], "big"); i += 2
            out.append(b[i:i + n]); i += n
        elif tag in (7, 8, 16, 19, 20):
            i += 2
        elif tag == 15:
            i += 3
        elif tag in (3, 4, 9, 10, 11, 12, 17, 18):
            i += 4
        elif tag in (5, 6):
            i += 8; idx += 1  # two CP slots
        else:
            raise ValueError("%s: bad cp tag %d" % (path, tag))
        idx += 1
    return out

CP_GLUE_RE = re.compile(r"cmp[0-9a-z_]+\|cmp[0-9a-z_]+")

# ----------------------------------------------------------- censor prims
def run_cmd(cmd, timeout=120):
    p = subprocess.run(cmd, capture_output=True, text=True, timeout=timeout)
    return p.returncode, (p.stdout + p.stderr)

def c_bash_n(path):
    rc, out = run_cmd(["bash", "-n", str(path)])
    if rc == 0:
        return []
    first = out.strip().splitlines()[0] if out.strip() else "syntax error"
    return ["bash_n: %s" % first[:110]]

def c_case_arm_scan(path):
    rc, out = run_cmd([sys.executable, str(CASE_ARM_SCAN), str(path)])
    fails = [ln for ln in out.splitlines() if ": FAIL " in ln]
    if fails:
        return ["case_arm_scan:FAIL %s" % ln.split(": FAIL ", 1)[1][:90] for ln in fails]
    warns = [ln for ln in out.splitlines() if ": WARN " in ln]
    return ["case_arm_scan:WARN %s" % ln.split(": WARN ", 1)[1][:90] for ln in warns]

def c_javac(path, outdir):
    rc, out = run_cmd([JAVAC, "-d", str(outdir), str(path)])
    if rc == 0:
        return []
    first = out.strip().splitlines()[0] if out.strip() else "compile error"
    return ["javac: %s" % first[:110]]

def c_rustc(path, outdir):
    rc, out = run_cmd([RUSTC, "--edition", "2021", "--crate-type", "lib",
                       "--emit=metadata", "--out-dir", str(outdir), str(path)])
    if rc == 0:
        return []
    first = out.strip().splitlines()[0] if out.strip() else "compile error"
    return ["rustc: %s" % first[:110]]

def cp_scan_class(cls, require_standalone):
    """[censor:detail] hits from cp_corrupt / cp_standalone / cp_marker."""
    hits = []
    try:
        entries = [e.decode("utf-8", errors="replace") for e in utf8_entries(cls)]
    except ValueError as e:
        return ["cp_corrupt: %s" % e]
    for e in entries:
        for m in CP_GLUE_RE.finditer(e):
            s, f = m.start(), m.end()
            list_style = (s > 0 and e[s - 1] == "|") and (f < len(e) and e[f] == "|")
            if not list_style:
                hits.append("cp_corrupt: pipe-glued flag token %r in CP entry %r (x461 merge-glue signature)" % (m.group(0), e[:60]))
    for tok in require_standalone:
        if tok not in entries:
            hits.append("cp_standalone: flag %r not a standalone utf8 CP entry (glued or stale)" % tok)
    blob = "\x00".join(entries)
    for pat in ("origin/", "<<<<<<<", ">>>>>>>"):
        if pat in blob:
            hits.append("cp_marker: branch/conflict marker %r in classfile CP" % pat)
    return hits

def c_marker_grep(path):
    hits = []
    for ln_no, ln in enumerate(Path(path).read_text(errors="replace").splitlines(), 1):
        if re.match(r"^(<{7}|>{7})", ln):
            hits.append("marker_grep:%d conflict marker %r" % (ln_no, ln[:44]))
        for m in re.finditer(r"origin/[A-Za-z0-9._\-]+", ln):
            hits.append("marker_grep:%d branch residue %r" % (ln_no, m.group(0)))
    return hits

def c_flagtok(path):
    hits = []
    for m in re.finditer(r'"([^"\n]*)"', Path(path).read_text(errors="replace")):
        if re.search(r"cmp[0-9a-z_]+\|", m.group(1)):
            hits.append("flagtok: pipe-glued lever token %r (must be single token)" % m.group(1)[:64])
    return hits

def c_canonline(path, canon_lines):
    text = Path(path).read_text(errors="replace")
    return ["canonline: canon bare-line %r missing (line-glue/absorption)" % p
            for p in canon_lines if not re.search(p, text, re.M)]

def c_sync_gate(src, cls, baseline):
    """(a) every source flag token must live in blob bytes (source-gate canon);
    (b) source sha drifted + blob sha unchanged => STALE BLOB (rebuild no-op)."""
    hits = []
    cls_bytes = Path(cls).read_bytes() if cls and Path(cls).exists() else b""
    text = Path(src).read_text(errors="replace")
    for tok in sorted(set(re.findall(r'"(cmp[0-9a-z_]+)"', text))):
        if tok.encode() not in cls_bytes:
            hits.append("sync_gate: source flag %r missing from blob bytes (stale/no-op rebuild)" % tok)
    if baseline:
        src_sha = hashlib.sha256(Path(src).read_bytes()).hexdigest()
        cls_sha = hashlib.sha256(cls_bytes).hexdigest()
        if src_sha != baseline["src_sha"] and cls_sha == baseline["cls_sha"]:
            hits.append("sync_gate: source sha changed but blob sha identical -> STALE BLOB")
    return hits

# ------------------------------------------------------------- corpus spec
def build_spec(selftest=False):
    n = 1 if selftest else 6
    clean = [{"lang": lang, "idx": i} for lang in ("sh", "java", "rs") for i in range(1, n + 1)]
    if selftest:
        plan = [("dub_tail", None, "java", 1), ("flag_glue", None, "rs", 1),
                ("line_glue", None, "sh", 1), ("marker_residue", "comment", "rs", 1),
                ("case_slice", "middle", "sh", 1), ("stale", "drop", "java", 1)]
    else:
        plan = []
        for lang in ("sh", "java", "rs"):
            for i in range(1, 5):  plan.append(("dub_tail", None, lang, i))
            for i in range(1, 4):  plan.append(("flag_glue", None, lang, i))
            for i in range(1, 5):  plan.append(("line_glue", None, lang, i))
            for i in range(1, 4):  plan.append(("marker_residue", "comment" if i <= 2 else "bare", lang, i))
        for i in range(1, 4):  plan.append(("flag_glue", "all", "java", i))  # x461: standalone entry gone
        for i in range(1, 4):
            plan.append(("case_slice", "middle", "sh", i))
            plan.append(("case_slice", "esac", "sh", i))
        plan.append(("stale", "add", "java", 1))
        plan.append(("stale", "drop", "java", 2))
    return clean, plan

EXPECT = {  # family -> lang -> any-of expected censors (ground truth mapping)
    "dub_tail":       {"java": ["javac"], "rs": ["rustc"], "sh": ["bash_n"]},
    "flag_glue":      {"java": ["cp_corrupt", "cp_standalone", "flagtok"], "rs": ["flagtok"], "sh": ["flagtok"]},
    "line_glue":      {"java": ["javac"], "rs": ["rustc"], "sh": ["canonline"]},
    "marker_residue": {"java": ["marker_grep"], "rs": ["marker_grep"], "sh": ["marker_grep"]},
    "case_slice":     {"sh": ["case_arm_scan", "bash_n"]},
    "stale":          {"java": ["sync_gate"]},
}

# ---------------------------------------------------------------- runner
def entry_for(lang, clsid, kind, family=None, sub=None, extra=None):
    e = {"clsid": clsid,
         "require_standalone": ["cmp457_paldelta", "cmp457_eqsnap2"] if lang == "java" else [],
         "canonline": SH_CANON if lang == "sh" else [],
         "kind": kind}
    if family: e.update({"family": family, "sub": sub})
    if extra: e.update(extra)
    return e

def verify_file(path, entry, outdir):
    lang = {"sh": "sh", "java": "java", "rs": "rs"}[str(path).rsplit(".", 1)[1]]
    hits = []  # (censor, detail)
    if lang == "sh":
        hits += [("bash_n", h.split(": ", 1)[-1]) for h in c_bash_n(path)]
        hits += [("case_arm_scan", h.split(": ", 1)[-1]) for h in c_case_arm_scan(path)]
    if lang == "rs":
        hits += [("rustc", h.split(": ", 1)[-1]) for h in c_rustc(path, outdir)]
    if lang == "java":
        prebuilt = entry.get("prebuilt_class")
        cls = None
        if prebuilt:
            cls = Path(prebuilt)  # stale-blob simulation: javac never re-runs
        else:
            jh = c_javac(path, outdir)
            hits += [("javac", h.split(": ", 1)[-1]) for h in jh]
            if not jh:
                cand = outdir / ("%s.class" % entry["clsid"])
                cls = cand if cand.exists() else None
        if cls and cls.exists():
            for h in cp_scan_class(cls, entry.get("require_standalone", [])):
                cc, det = h.split(": ", 1)
                hits.append((cc, det))
            for h in c_sync_gate(path, cls, entry.get("baseline")):
                cc, det = h.split(": ", 1)
                hits.append((cc, det))
    hits += [("marker_grep", h.split(":", 1)[-1]) for h in c_marker_grep(path)]
    hits += [("flagtok", h.split(": ", 1)[-1]) for h in c_flagtok(path)]
    hits += [("canonline", h.split(": ", 1)[-1]) for h in c_canonline(path, entry.get("canonline", []))]
    return hits

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--corpus-dir", default=None)
    ap.add_argument("--keep", action="store_true")
    ap.add_argument("--selftest", action="store_true")
    ap.add_argument("--json", default=None)
    args = ap.parse_args()

    root = Path(args.corpus_dir) if args.corpus_dir else Path(tempfile.mkdtemp(prefix="lineunion_S57_"))
    root.mkdir(parents=True, exist_ok=True)
    clean_spec, defect_spec = build_spec(args.selftest)
    tmpl = {"sh": SH_TMPL, "java": JAVA_TMPL, "rs": RS_TMPL}

    report = {"clean": [], "defects": [], "fixtures": [], "totals": {}}
    matrix, clean_fp = {}, []

    def fname(stem, lang):
        return root / ("%s.%s" % (stem, lang))

    # ---- emit corpus ----
    clean_files = {}
    for c in clean_spec:
        stem = "clean_%s%d" % (c["lang"], c["idx"])
        fn = fname(stem, c["lang"])
        fn.write_text(tmpl[c["lang"]].format(idx="%s%d" % (c["lang"], c["idx"]), clsid=stem))
        clean_files[fn] = entry_for(c["lang"], stem, "clean")
    defect_files = {}
    for d in defect_spec:
        fam, sub, lang, idx = d
        stem = "def_%s%s_%s%d" % (fam, "_" + sub if sub else "", lang, idx)
        fn = fname(stem, lang)
        base = tmpl[lang].format(idx="%s%d" % (lang, idx), clsid=stem)
        entry = entry_for(lang, stem, "defect", fam, sub)
        text = base
        if fam == "stale":
            pre = root / ("_stale_base_%s" % stem); pre.mkdir(exist_ok=True)
            (pre / ("%s.java" % stem)).write_text(base)
            rc, _ = run_cmd([JAVAC, "-d", str(pre), str(pre / ("%s.java" % stem))])
            assert rc == 0, "stale baseline compile failed"
            stale_cls = root / ("stale_%s.class" % stem)
            shutil.copy(pre / ("%s.class" % stem), stale_cls)
            text = inj_stale(base, sub)
            entry["prebuilt_class"] = str(stale_cls)
            entry["baseline"] = {"src_sha": hashlib.sha256(base.encode()).hexdigest(),
                                 "cls_sha": hashlib.sha256(stale_cls.read_bytes()).hexdigest()}
        else:
            text = {"dub_tail": inj_dub_tail, "line_glue": inj_line_glue,
                    "flag_glue": lambda t, l: inj_flag_glue(t, l, sub),
                    "marker_residue": lambda t, l: inj_marker_residue(t, l, sub == "bare"),
                    "case_slice": lambda t, l: inj_case_slice(t, sub == "middle")}[fam](base, lang)
        fn.write_text(text)
        defect_files[fn] = entry

    # ---- verify clean (FP baseline; also builds sha baselines for sync) ----
    for fn, entry in clean_files.items():
        outdir = root / ("out_" + fn.stem); outdir.mkdir(exist_ok=True)
        if fn.suffix == ".java":
            assert not c_javac(fn, outdir), "CLEAN java failed to compile: %s" % fn
            cls = outdir / ("%s.class" % entry["clsid"])
            entry["baseline"] = {"src_sha": hashlib.sha256(fn.read_bytes()).hexdigest(),
                                 "cls_sha": hashlib.sha256(cls.read_bytes()).hexdigest()}
        hits = verify_file(fn, entry, outdir)
        report["clean"].append({"file": fn.name, "censors": sorted({h[0] for h in hits})})
        clean_fp += [{"file": fn.name, "censor": cn, "detail": det[:120]} for cn, det in hits]

    # ---- verify defects ----
    for fn, entry in defect_files.items():
        outdir = root / ("out_" + fn.stem); outdir.mkdir(exist_ok=True)
        hits = verify_file(fn, entry, outdir)
        censors = sorted({h[0] for h in hits})
        expected = EXPECT[entry["family"]][fn.suffix[1:] if fn.suffix != ".sh" else "sh"]
        detected = any(cn in expected for cn in censors)
        matrix.setdefault(entry["family"] + (":" + entry["sub"] if entry["sub"] else ""), {}) \
              .setdefault(fn.suffix[1:], []).append(
                  {"file": fn.name, "detected": detected, "censors": censors, "expected": expected})
        report["defects"].append({"file": fn.name, "family": entry["family"], "sub": entry["sub"],
                                  "lang": fn.suffix[1:], "detected": detected, "censors": censors,
                                  "expected": expected,
                                  "details": ["%s: %s" % (cn, det[:96]) for cn, det in hits]})

    # ---- historical fixtures (real git history) ----
    for key, (fpath, expect) in FIXTURES.items():
        outdir = root / ("out_fix_%s" % key); outdir.mkdir(exist_ok=True)
        entry = {"clsid": key, "canonline": CANON_FIXTURE_LINES, "require_standalone": []}
        hits = verify_file(fpath, entry, outdir)
        censors = sorted({h[0] for h in hits})
        ok = (any(cn in expect for cn in censors) if expect else len(censors) == 0)
        report["fixtures"].append({"file": fpath.name, "expect": expect, "censors": censors, "ok": ok,
                                   "details": ["%s: %s" % (cn, det[:96]) for cn, det in hits]})

    # ---- score ----
    n_def = len(report["defects"]); n_det = sum(1 for x in report["defects"] if x["detected"])
    n_fix_ok = sum(1 for x in report["fixtures"] if x["ok"])
    report["totals"] = {"clean_files": len(report["clean"]), "clean_fp": len(clean_fp),
                        "defects": n_def, "detected": n_det, "censors_in_chain": 11,
                        "fixtures_ok": "%d/%d" % (n_fix_ok, len(report["fixtures"]))}

    print("=" * 78)
    print("LINE-UNION HARNESS R468-S57 — corpus %s" % root)
    print("=" * 78)
    print("censors (11): bash_n, case_arm_scan(FAIL/WARN), javac, rustc, cp_corrupt,")
    print("              cp_standalone, cp_marker, marker_grep, flagtok, canonline, sync_gate")
    print("\n-- detection matrix (family:sub / lang -> censors that fired) --")
    for fam, langs in sorted(matrix.items()):
        for lang, items in sorted(langs.items()):
            det = sum(1 for i in items if i["detected"])
            cens = sorted({c for i in items for c in i["censors"]})
            print("  %-24s %-4s %d/%d  caught by: %s" % (fam, lang, det, len(items), ",".join(cens) or "-"))
    print("\n-- clean corpus FP --")
    print("  %d clean files, %d false positives" % (len(report["clean"]), len(clean_fp)))
    for fp in clean_fp:
        print("    FP %s [%s] %s" % (fp["file"], fp["censor"], fp["detail"]))
    print("\n-- fixtures (real git history: MERGE #9 bee2b585) --")
    for f in report["fixtures"]:
        print("  %-42s expect=%-32s got=%-28s %s" % (f["file"], ",".join(f["expect"]) or "NONE",
                                                    ",".join(f["censors"]) or "-", "OK" if f["ok"] else "MISMATCH"))
        if not f["ok"]:
            for d in f["details"]:
                print("      %s" % d)
    print("\nTOTAL: defects %d/%d detected (%.1f%%), clean FP %d, fixtures %s" %
          (n_det, n_def, 100.0 * n_det / max(n_def, 1), len(clean_fp), report["totals"]["fixtures_ok"]))
    jpath = Path(args.json) if args.json else root / "report.json"
    jpath.write_text(json.dumps(report, indent=1, ensure_ascii=False))
    print("report: %s" % jpath)
    ok = (n_det == n_def and not clean_fp and n_fix_ok == len(report["fixtures"]))
    if not args.keep and not args.corpus_dir:
        shutil.rmtree(root, ignore_errors=True)
    print("VERDICT: %s" % ("PASS" if ok else "FAIL"))
    return 0 if ok else 1

if __name__ == "__main__":
    sys.exit(main())
