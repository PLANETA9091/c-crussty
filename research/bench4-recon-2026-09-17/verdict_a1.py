#!/usr/bin/env python3
"""verdict_a1.py — §125-AMENDMENT-1 verdict calculator (S7-119).

Inputs (all read-only):
  - leg_b_v4.json (written by hunt_leg_b_v4.py on BASELINE COMPLETE /
    PACK COMPLETE): {baseline: [arm1, arm2], window: [lo, hi], pack_legs: [...]}
  - GitHub Actions logs of each run (zip of .txt) -> headline metric
    `MSPT: avg **X.XXms**` (the SAME metric the bank uses: run#18 85.24,
    run#17 76.98, run#21 76.01 are all `avg` headlines).

Computation (preregistered §125-A1):
  threshold   = median(baseline_mspts) * 0.97   (>=3.0% MSPT gate)
  pack_median = median(pack_mspts)              (min-of-2 in-window legs)
  verdict     = LANDS if pack_median <= threshold else REFUTED
  Also prints per-leg pairing sanity (cpu in window, world pin match).

Usage: python3 verdict_a1.py            # uses leg_b_v4.json as-is
       python3 verdict_a1.py --check    # only extract+print, no verdict file
"""
import io, json, os, re, statistics, sys, urllib.request, zipfile

API = "https://api.github.com"
REPO = "PLANETA9091/c-crussty"
WORLD_SHA = "afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5"
HERE = os.path.dirname(os.path.abspath(__file__))
RESULT = os.path.join(HERE, "leg_b_v4.json")


def token_from_creds(path="~/.git-credentials"):
    line = open(os.path.expanduser(path)).read().strip().splitlines()[0]
    return re.match(r"^https://[^:]+:([^@]+)@github\.com$", line).group(1)


def run_logs_txt(tok, run_id):
    req = urllib.request.Request(
        f"{API}/repos/{REPO}/actions/runs/{run_id}/logs",
        headers={"Authorization": f"Bearer {tok}",
                 "Accept": "application/vnd.github+json"})
    zf = zipfile.ZipFile(io.BytesIO(urllib.request.urlopen(req, timeout=180).read()))
    return "\n".join(zf.read(n).decode("utf-8", "replace")
                     for n in zf.namelist() if n.endswith(".txt"))


def extract(txt, run_id):
    """Returns dict with mspt_avg, cpu, world_sha, validity markers."""
    out = {"run_id": run_id}
    m = re.search(r"MSPT: avg \*\*([\d.]+)ms\*\*", txt)
    out["mspt_avg"] = float(m.group(1)) if m else None
    m = re.search(r"run-env: world_sha256=([0-9a-f]+) runner_cpu_index=(\d+)", txt)
    if m:
        out["world_sha"], out["cpu"] = m.group(1), int(m.group(2))
    out["fixture_valid"] = bool(re.search(r"FIXTURE-VALIDITY[^\n]*VALID", txt)) and \
        not bool(re.search(r"FIXTURE-VALIDITY[^\n]*INVALID", txt))
    return out


def main():
    check_only = "--check" in sys.argv
    tok = token_from_creds()
    st = json.load(open(RESULT))
    arms, legs = st["baseline"], st.get("pack_legs", [])
    lo, hi = st["window"]

    print(f"window=[{lo},{hi}] baseline_arms={[a['run_id'] for a in arms]} "
          f"pack_legs={[l['run_id'] for l in legs]}")
    rows = []
    for arm in arms:
        info = extract(run_logs_txt(tok, arm["run_id"]), arm["run_id"])
        rows.append(("BASE", info))
    for leg in legs:
        info = extract(run_logs_txt(tok, leg["run_id"]), leg["run_id"])
        rows.append(("PACK", info))

    base_ms, pack_ms = [], []
    for kind, info in rows:
        cpu_ok = info.get("cpu") is not None and lo <= info["cpu"] <= hi
        world_ok = info.get("world_sha") == WORLD_SHA
        print(f"  {kind} run {info['run_id']}: mspt_avg={info['mspt_avg']} "
              f"cpu={info.get('cpu')} in_window={cpu_ok} world_ok={world_ok} "
              f"valid={info['fixture_valid']}")
        if info["mspt_avg"] is None:
            print("    !! no MSPT headline found — absorb artifacts instead")
            return 1
        if kind == "BASE":
            base_ms.append(info["mspt_avg"])
        else:
            if not cpu_ok:
                print("    !! pack leg OUT of window — illegal, abort")
                return 2
            pack_ms.append(info["mspt_avg"])

    if len(pack_ms) < 2:
        print(f"pack legs complete: {len(pack_ms)}/2 — no verdict yet")
        return 4
    threshold = statistics.median(base_ms) * 0.97
    pack_median = statistics.median(pack_ms)
    delta = (pack_median - statistics.median(base_ms)) / statistics.median(base_ms) * 100
    verdict = "LANDS" if pack_median <= threshold else "REFUTED"
    print(f"\nthreshold   = median({base_ms}) x 0.97 = {threshold:.2f}ms")
    print(f"pack_median = median({pack_ms}) = {pack_median:.2f}ms "
          f"({delta:+.2f}% vs baseline median)")
    print(f"VERDICT §125-A1: {verdict}")
    if not check_only:
        json.dump({"verdict": verdict, "threshold": threshold,
                   "pack_median": pack_median, "delta_pct": delta,
                   "baseline_mspts": base_ms, "pack_mspts": pack_ms},
                  open(os.path.join(HERE, "verdict_a1.json"), "w"))
    return 0


if __name__ == "__main__":
    sys.exit(main())
