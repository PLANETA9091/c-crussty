#!/usr/bin/env python3
"""dispatch_457b.py — TASK-457-B step-5: monster-hunt re-rolls of the cmp456_poi
carrier (POI subsystem whole in Rust). Base pin = NCDFE-fix commit
5ecd841a128aa97c633e8b005479c5dcf09781aa (branch round-456b-poi head 0fa13d72
contains fix + pre-flight; legs are created EXACTLY at 5ecd841a).

Canonical inputs identical to dispatch_456b_fix.py (canon tick-456/457).

Usage: dispatch_457b.py [--dry-run] [--no-batch] [--leg NAME]
  --dry-run   preflight only, no dispatches
  --no-batch  dispatch only the FIRST leg of the batch (extra guard)
  --leg NAME  dispatch ONLY the leg whose short-name matches (e.g. 5r2)
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

# (leg, branch, base, lever)
BATCH = [
    ("poi457-5r2", "round-456b-poi-5r2", "round-456b-poi", "cmp456_poi"),
    ("poi457-8",   "round-456b-poi-8",   "round-456b-poi", "cmp456_poi"),
    ("poi457-9",   "round-456b-poi-9",   "round-456b-poi", "cmp456_poi"),
]

BASE_SHA = "5ecd841a128aa97c633e8b005479c5dcf09781aa"  # HARD pin (NCDFE fix)
LEVER = "cmp456_poi"

INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
}


def token_from_remote():
    url = subprocess.run(["git", "-C", "/home/z/rounds/ROUND-457/agent-b",
                          "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    return re.match(r"^https://[^:]+:([^@]+)@github.com/", url).group(1)


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        if e.code != 404:
            print(f"HTTP {e.code}: {e.read()[:200]}")
        raise
    return json.loads(body) if body else {}


def sha_of(tok, ref):
    return api(tok, f"/repos/{REPO}/git/ref/heads/{ref}")["object"]["sha"]


def ensure_branch_exact(tok, branch):
    """Create branch at BASE_SHA if missing; HARD-fail if exists but drifted."""
    try:
        live = sha_of(tok, branch)
        if live != BASE_SHA:
            raise SystemExit(f"BRANCH DRIFT {branch}: live={live[:8]} pin={BASE_SHA[:8]}")
        return live
    except urllib.error.HTTPError:
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{branch}", "sha": BASE_SHA})
        return BASE_SHA


def main():
    args = sys.argv[1:]
    if "--help" in args or "-h" in args:
        print("usage: dispatch_457b.py [--dry-run] [--no-batch] [--leg NAME]")
        raise SystemExit(0)
    allowed = ("--dry-run", "--no-batch", "--leg")
    if any(a.startswith("--") and not a.startswith(allowed) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    if "--leg" in args:
        i = args.index("--leg")
        leg_filter = args[i + 1]
        del args[i:i + 2]
        batch = [t for t in BATCH if t[0] == leg_filter or t[0].endswith("-" + leg_filter)]
        if not batch:
            raise SystemExit(f"argv-guard: no leg matching {leg_filter} in BATCH")
    else:
        batch = list(BATCH)
    if "--no-batch" in args:
        batch = batch[:1]

    tok = token_from_remote()
    print(f"=== TASK-457-B POI CARRIER DISPATCH (pin {BASE_SHA[:8]}, lever={LEVER}) ===", flush=True)
    base = "round-456b-poi"
    live = sha_of(tok, base)
    if live != BASE_SHA:
        raise SystemExit(f"SHA MISMATCH: {base} live={live} expected={BASE_SHA}")
    print(f"preflight OK: {base} @ {live[:8]} (exact pin)", flush=True)
    if "--dry-run" in args:
        for leg, branch, _, _ in batch:
            print(f"DRY-RUN would dispatch {leg}: {branch} @ {BASE_SHA[:8]}", flush=True)
        print("DRY-RUN OK — no dispatches", flush=True)
        return
    for leg, branch, _, lever in batch:
        sha = ensure_branch_exact(tok, branch)
        inputs = dict(INPUTS)
        inputs["lever_flag"] = lever
        inputs["lever_arg"] = "1"
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": branch, "inputs": inputs})
        print(f"dispatched {leg}: {branch} @ {sha[:8]} lever='{lever}'", flush=True)
        time.sleep(4)
    print(f"=== TASK-457-B BATCH COMPLETE: {len(batch)} dispatches ===", flush=True)


if __name__ == "__main__":
    main()
