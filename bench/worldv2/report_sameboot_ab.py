#!/usr/bin/env python3
"""report_sameboot_ab.py — SAME-BOOT A/B verdict merger (AG-361, wave-527).

Parses leg-A and leg-B BENCHV2.md (report_benchv2.py output), computes Delta%
per metric, emits BENCHV2_AB.md with the preregistered same-boot verdict.

Prereg (SHARED_BOARD 2026-10-03, AG-361 w527, claims/AG-361.md):
  - AB-NULL (AB_NULL=1): harness sanity canary. Both legs are byte-identical
    env on the same VM => any delta = within-VM boot-to-boot noise.
      PASS: |D(ch/s)| <= 10% AND |D(mspt_med)| <= 10%  (ch/s cross-runner
            spread 6.8% (AG-216 dgw256 n=6) is the cross-runner floor; the
            same-boot null must not exceed it materially)
      WARN: |D| <= 25% (boot-to-boot drift elevated; same-boot pairs still
            judgeable at the >=20пп bar, but record sigma)
      FAIL: |D| > 25% (within-VM drift swamps the +20пп cert bar => harness
            unusable for certs, FACT to board)
  - AB-LEV (lever delta): no pass/fail; publishes D(ch/s), D(mspt_med),
    D(tps_last) as a same-boot pair delta with |dIdx|=0. Cert = min-of-3
    same-boot pairs, FIN bar +20пп (pair math canon LAB_LEDGER).

Exit 0 iff both leg reports parsed and (AB-NULL not FAIL) and (G1 echo-audit
not MISMATCH).

G1 echo-audit (AG-167 w528; trap found by AG-129 w528): argv-trusting mode
labels let A/A pairs masquerade as AB-LEV (empty leg_b_vars) or AB-NULL legs
carry lever echoes. The run-env.txt `ab_vars=` echoes (written by the wrapper)
are the byte truth; on mismatch the verdict is FAIL (fail-closed, dud-gate
canon x519). Missing env files => audit skipped (pre-AG-167 artifacts).
"""
import os, re, sys


def read_echo(p):
    """Return (ab_null_str, ab_vars) from a run-env.txt echo line, else None."""
    if not p or not os.path.exists(p):
        return None
    for ln in open(p, encoding="utf-8", errors="replace"):
        if "ab_leg=" in ln and "ab_vars=" in ln:
            m_n = re.search(r"ab_null=([0-9]+)", ln)
            m_v = re.search(r"ab_vars=(.*)", ln)
            return (m_n.group(1) if m_n else "?", (m_v.group(1) if m_v else "").strip())
    return None


def parse_leg(d):
    p = os.path.join(d, "BENCHV2.md")
    if not os.path.exists(p):
        return None
    txt = open(p, encoding="utf-8", errors="replace").read()
    leg = {"dir": d}
    m = re.search(r"ch/s \(drain-def[^\n]*\*\*([0-9.]+)\*\*", txt)
    leg["ch_s"] = float(m.group(1)) if m else None
    m = re.search(r"MSPT: idle≈([0-9.]+), sustain-median≈([0-9.]+)", txt)
    leg["mspt_idle"] = float(m.group(1)) if m else None
    leg["mspt_med"] = float(m.group(2)) if m else None
    m = re.search(r"TPS samples[^\n]*last=([0-9.]+)", txt)
    leg["tps_last"] = float(m.group(1)) if m else None
    m = re.search(r"forceload-marked chunks total: \*\*([0-9]+)\*\*", txt)
    leg["marked"] = int(m.group(1)) if m else None
    leg["g4"] = "G4 marked≥95%: PASS" in txt
    leg["g5"] = "G5 drain: PASS" in txt
    leg["ncdfe0"] = "NCDFE=0" in txt
    return leg


def delta(a, b):
    if a is None or b is None or a == 0:
        return None
    return (b - a) / a * 100.0


def fmt(x, nd=2, suffix="%"):
    return ("n/a" if x is None else f"{x:+.{nd}f}{suffix}")


a = parse_leg(sys.argv[1])
b = parse_leg(sys.argv[2])
outdir = sys.argv[3]
a_rc, b_rc = sys.argv[4], sys.argv[5]
ab_null = sys.argv[6] == "1"
env_a = sys.argv[7] if len(sys.argv) > 7 else None   # AG-167: leg A run-env echo
env_b = sys.argv[8] if len(sys.argv) > 8 else None   # AG-167: leg B run-env echo

rep = ["# BENCHV2-AB — same-boot A/B verdict (AG-361 w527)", ""]
rep.append(f"- mode: {'AB-NULL (A/A harness sanity)' if ab_null else 'AB-LEV (lever delta)'}; leg_rc: A={a_rc} B={b_rc}")
rep.append("- same-boot property: 1 job = 1 VM = 1 runner_cpu_index => |dIdx|=0 (AG-210/212 w527 sigma census)")
rep.append("")
rep.append("| metric | leg A | leg B | D(B-A) |")
rep.append("|---|---|---|---|")
rep.append(f"| ch/s (drain-def) | {a['ch_s'] if a else 'n/a'} | {b['ch_s'] if b else 'n/a'} | {fmt(delta(a['ch_s'], b['ch_s']) if a and b else None)} |")
rep.append(f"| mspt sustain-median | {a['mspt_med'] if a else 'n/a'} | {b['mspt_med'] if b else 'n/a'} | {fmt(delta(a['mspt_med'], b['mspt_med']) if a and b else None)} |")
rep.append(f"| tps last | {a['tps_last'] if a else 'n/a'} | {b['tps_last'] if b else 'n/a'} | {fmt(delta(a['tps_last'], b['tps_last']) if a and b else None)} |")
rep.append(f"| marked chunks | {a['marked'] if a else 'n/a'} | {b['marked'] if b else 'n/a'} | n/a |")
rep.append(f"| G4/G5/NCDFE=0 | {a['g4'] if a else '?'}/{a['g5'] if a else '?'}/{a['ncdfe0'] if a else '?'} | {b['g4'] if b else '?'}/{b['g5'] if b else '?'}/{b['ncdfe0'] if b else '?'} | n/a |")
rep.append("")

ok = a is not None and b is not None
verdict = "FAIL"
if ok and ab_null:
    d_ch = abs(delta(a["ch_s"], b["ch_s"]) or 999.0)
    d_ms = abs(delta(a["mspt_med"], b["mspt_med"]) or 999.0)
    gates_ok = a["g4"] and b["g4"] and a["ncdfe0"] and b["ncdfe0"]
    if d_ch <= 10.0 and d_ms <= 10.0 and gates_ok:
        verdict = "PASS"
    elif d_ch <= 25.0 and d_ms <= 25.0 and gates_ok:
        verdict = "WARN"
    rep.append(f"- AB-NULL prereg deltas: |D(ch/s)|={d_ch:.2f}% |D(mspt_med)|={d_ms:.2f}% (bars PASS 10 / WARN 25)")
elif ok:
    d_ch = delta(a["ch_s"], b["ch_s"])
    d_ms = delta(a["mspt_med"], b["mspt_med"])
    verdict = "REPORT"  # lever mode: no prereg pass/fail, cert = min-of-3
    rep.append(f"- AB-LEV pair delta: D(ch/s)={fmt(d_ch)} D(mspt_med)={fmt(d_ms)} — same-boot |dIdx|=0 pair; cert = min-of-3, bar +20пп")

# --- G1 echo-audit (AG-167 w528): byte-truth of the A/B split, AFTER mode
# verdicts so a mismatch cannot be clobbered back to PASS/REPORT -------------
g1 = "skipped (no run-env echo files)"
ea, eb = read_echo(env_a), read_echo(env_b)
if ea is not None and eb is not None:
    g1_ok = True
    if ea[1] != "base":
        g1_ok, g1 = False, f"MISMATCH: leg A ab_vars={ea[1]!r} expected 'base'"
    elif ab_null and eb[1] != "none":
        g1_ok, g1 = False, f"MISMATCH: AB_NULL=1 but leg B ab_vars={eb[1]!r} (vars never applied)"
    elif (not ab_null) and (eb[1] in ("", "none", "base") or eb[1] == ea[1]):
        g1_ok, g1 = False, f"MISMATCH: leg B ab_vars={eb[1]!r} => A/A pair mislabeled AB-LEV"
    else:
        g1 = f"OK: legA ab_vars={ea[1]!r} legB ab_vars={eb[1]!r}"
    if not g1_ok:
        verdict = "FAIL"
rep.append(f"- G1 echo-audit (AG-167): {g1}")
rep.append("")
rep.append(f"- VERDICT: {verdict}")
rep.append("- usage: cert path for pending pairs (dgw-axis ch/s ghost +24.5пп n=1 AG-216; fd/ic lever pairs w527) — 3 same-boot jobs = min-of-3")

open(os.path.join(outdir, "BENCHV2_AB.md"), "w").write("\n".join(rep) + "\n")
print("\n".join(rep))
sys.exit(0 if ok and verdict != "FAIL" else 1)
