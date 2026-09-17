#!/usr/bin/env python3
"""pair_hunter.py — dispatch bench runs until a PAIRED leg pair exists (S7-96d law).

Runner-variance law: cross-run MSPT deltas are noise-dominated unless legs share
(world_sha256, runner_cpu_index). GitHub-hosted runners can't be pinned, but the
harness echoes `run-env: world_sha256=... runner_cpu_index=...` into the workflow
log — so we dispatch runs, scrape each run's log, and pair completed legs by
matching cpu_idx within tolerance. Result: legal min-of-2 A/B infrastructure
without owner hardware (self-served path 3).

Usage:
  pair_hunter.py [--want-pairs 1] [--max-dispatch 4] [--tol 0.02]
                 [--fp 4] [--sweeps 0] [--seconds 900] [--repo PLANETA9091/c-crussty]
                 [--index PATH] [--no-dispatch]

  --no-dispatch: only re-scan already-known runs (from --index cache) for pairs.
Index cache (JSONL, one run per line) avoids re-fetching logs on later ticks;
set --index to a persistent path to accumulate across ticks.
"""
import argparse, json, os, re, sys, time, urllib.request

API = "https://api.github.com"

def api(token, url, method="GET", data=None):
    req = urllib.request.Request(url, method=method,
        headers={"Authorization": f"Bearer {token}",
                 "Accept": "application/vnd.github+json"})
    payload = None
    if data is not None:
        req.add_header("Content-Type", "application/json")
        payload = json.dumps(data).encode()
    try:
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"  HTTP {e.code} on {url}: {e.read()[:200]}", file=sys.stderr)
        return {}

def token_from_creds(path="~/.git-credentials"):
    p = os.path.expanduser(path)
    line = open(p).read().strip().splitlines()[0]
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com$", line)
    return m.group(1) if m else None

def log_for_run(token, repo, run_id):
    """Download run log zip via redirect to the archive endpoint."""
    url = f"{API}/repos/{repo}/actions/runs/{run_id}/logs"
    req = urllib.request.Request(url, headers={
        "Authorization": f"Bearer {token}", "Accept": "application/vnd.github+json"})
    with urllib.request.urlopen(req, timeout=180) as r:
        data = r.read()
    import io, zipfile
    zf = zipfile.ZipFile(io.BytesIO(data))
    txt = []
    for name in zf.namelist():
        if name.endswith(".txt"):
            try:
                txt.append(zf.read(name).decode("utf-8", errors="replace"))
            except Exception:
                pass
    return "\n".join(txt)

def scrape_run_env(logtxt):
    out = {}
    m = re.search(r"run-env: world_sha256=([0-9a-f]{8,64}) runner_cpu_index=(\d+) fake_players=(\d+)", logtxt)
    if m:
        out["world_sha"] = m.group(1); out["cpu_idx"] = int(m.group(2)); out["fake_players"] = int(m.group(3))
    m = re.search(r"spark tick-monitor MSPT: avg \*\*([\d.]+)ms\*\*", logtxt)
    if m:
        out["mspt"] = float(m.group(1))
    m = re.search(r"FIXTURE-VALIDITY: (VALID|INVALID)", logtxt)
    out["fixture"] = m.group(1) if m else "N/A"
    return out

def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("--want-pairs", type=int, default=1)
    ap.add_argument("--max-dispatch", type=int, default=4)
    ap.add_argument("--tol", type=float, default=0.02)
    ap.add_argument("--fp", type=int, default=4)
    ap.add_argument("--sweeps", type=int, default=0)
    ap.add_argument("--seconds", type=int, default=900)
    ap.add_argument("--repo", default="PLANETA9091/c-crussty")
    ap.add_argument("--index", default=os.path.expanduser(
        "~/my-project/scripts/bench4_recon/runs_index.jsonl"))
    ap.add_argument("--no-dispatch", action="store_true")
    ap.add_argument("--band-min", default="")
    ap.add_argument("--band-max", default="")
    a = ap.parse_args()

    tok = token_from_creds()
    if not tok:
        print("FATAL: no token"); return 2

    # load index cache
    idx = []
    if os.path.exists(a.index):
        for line in open(a.index):
            line = line.strip()
            if line:
                idx.append(json.loads(line))
    known = {r["run_id"] for r in idx}

    def persist():
        os.makedirs(os.path.dirname(a.index), exist_ok=True)
        with open(a.index, "w") as f:
            for r in idx:
                f.write(json.dumps(r) + "\n")

    def find_pairs():
        legs = {}
        for r in idx:
            if r.get("cpu_idx") and r.get("fixture") == "VALID" and r.get("fake_players") == a.fp \
               and r.get("conclusion") == "success":
                legs.setdefault(r["world_sha"], []).append(r)
        pairs = []
        for sha, rs in legs.items():
            rs = sorted(rs, key=lambda x: x["cpu_idx"])
            for i in range(len(rs)):
                for j in range(i + 1, len(rs)):
                    ci, cj = rs[i]["cpu_idx"], rs[j]["cpu_idx"]
                    if abs(ci - cj) / min(ci, cj) <= a.tol:
                        pairs.append((sha, rs[i], rs[j]))
        return pairs

    def fetch_and_index(wf_runs):
        added = 0
        for r in wf_runs:
            rid = r["id"]
            if rid in known or r["status"] != "completed":
                continue
            print(f"  fetching log for run {rid} ({r['conclusion']})...")
            env = scrape_run_env(log_for_run(tok, a.repo, rid)) or {}
            rec = {"run_id": rid, "conclusion": r["conclusion"],
                   "created_at": r["created_at"], "head_sha": r["head_sha"][:7], **env}
            idx.append(rec); known.add(rid); added += 1
            time.sleep(1)
        return added

    # 1. harvest existing completed world-bench runs
    d = api(tok, f"{API}/repos/{a.repo}/actions/workflows/world-bench.yml/runs?per_page=30")
    fetch_and_index(d.get("workflow_runs", []))

    # 2. dispatch loop
    dispatched = []
    if not a.no_dispatch:
        for i in range(a.max_dispatch):
            if len(find_pairs()) >= a.want_pairs:
                break
            print(f"[dispatch {i+1}/{a.max_dispatch}]")
            ok = api(tok, f"{API}/repos/{a.repo}/actions/workflows/world-bench.yml/dispatches",
                     method="POST",
                     data={"ref": "master", "inputs": {"radius": "640",
                           "seconds": str(a.seconds), "summon_sweeps": str(a.sweeps),
                           "fake_players": str(a.fp),
                           **({"cpu_band_min": a.band_min} if a.band_min else {}),
                           **({"cpu_band_max": a.band_max} if a.band_max else {})}})
            time.sleep(25)
            d = api(tok, f"{API}/repos/{a.repo}/actions/workflows/world-bench.yml/runs?per_page=1")
            run = d.get("workflow_runs", [{}])[0]
            rid = run.get("id")
            print(f"  dispatched run {rid}")
            dispatched.append(rid)
            # poll to completion (cap 50 min)
            t0 = time.time()
            while time.time() - t0 < 50 * 60:
                s = api(tok, f"{API}/repos/{a.repo}/actions/runs/{rid}")
                if s.get("status") == "completed":
                    break
                time.sleep(60)
            d = api(tok, f"{API}/repos/{a.repo}/actions/workflows/world-bench.yml/runs?per_page=3")
            fetch_and_index(d.get("workflow_runs", []))
            persist()

    persist()
    pairs = find_pairs()
    print(f"\n=== PAIR HUNTER: {len(idx)} indexed runs, {len(pairs)} paired pair(s) "
          f"(tol {a.tol:.0%}, fp={a.fp}) ===")
    for sha, ra, rb in pairs:
        ms = [ra.get("mspt"), rb.get("mspt")]
        spread = (abs(ms[0] - ms[1]) / min(ms) * 100) if all(ms) and min(ms) else None
        print(f"  PAIR world={sha[:12]} runA={ra['run_id']} cpu={ra['cpu_idx']} mspt={ms[0]} | "
              f"runB={rb['run_id']} cpu={rb['cpu_idx']} mspt={ms[1]}"
              + (f" | spread {spread:.1f}%" if spread else ""))
    if not pairs:
        print("  no paired legs yet — dispatch more next tick (index cache accumulated)")
    return 0 if pairs else 1

if __name__ == "__main__":
    sys.exit(main())
