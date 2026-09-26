#!/usr/bin/env python3
"""dispatch_463_navmath.py — TASK-463-69a: navmath MovePlaneOps leg
round-463-navmath-1 (P44 move plane, lever cmp463_move, STRICT eq).
Bank-canon v5 inputs WITHOUT cpu_band (band-gate физика ×463: no-band ран =
бесплатный pre-download discard, банк-v5 нормировка) — lever_flag=cmp463_move.
Canon: dispatch_456c.py shape (argv-guard, sha pin, world-bench-parallel.yml).
Usage: scripts/dispatch_463_navmath.py [--dry-run]"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-463-navmath-1"
LEVER = "cmp463_move"
EXPECTED_SHA = "c9fcb154"  # navmath-1 impl commit (oracle 10^5x3 GREEN, blobs in sync)

INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    # банк-канон v5: БЕЗ cpu_band_min/max (band снят ×463 решёткой)
}


def token_from_remote():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
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


def main():
    dry = "--dry-run" in sys.argv[1:]
    tok = token_from_remote()
    print("=== TASK-463-69a NAVMATH LEG ===", flush=True)
    live = sha_of(tok, BRANCH)[:8]
    if not live.startswith(EXPECTED_SHA):
        raise SystemExit(f"SHA MISMATCH: {BRANCH} live={live} expected={EXPECTED_SHA}")
    print(f"preflight OK: {BRANCH} @ {live}", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatch", flush=True)
        return
    inputs = dict(INPUTS)
    inputs["lever_flag"] = LEVER
    inputs["lever_arg"] = "1"
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
        data={"ref": BRANCH, "inputs": inputs})
    print(f"dispatched: {BRANCH} lever='{LEVER}' sha={live} (bank-canon, no band)", flush=True)
    # run id: the latest dispatch of the workflow on this branch
    time.sleep(12)
    runs = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?branch={BRANCH}&per_page=1")
    if runs.get("workflow_runs"):
        r0 = runs["workflow_runs"][0]
        print(f"run id={r0['id']} url={r0['html_url']} status={r0['status']}", flush=True)


if __name__ == "__main__":
    main()
