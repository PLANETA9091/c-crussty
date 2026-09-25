#!/usr/bin/env python3
"""dispatch_457a.py — TASK-457-A chunkmono monster-hunt legs (agent-a).
Carrier cmp456_chunkmono @d73758a3 (NCDFE-fix, validated x4 NCDFE=0), law-8
axis (ServerChunkCache scheduling mono-plane chunk6-sched on cert stack,
STRICT-OR). Legs 6/7/8 dispatched by main; agent-a continues 9+ per law-3
cycle. Band fast-fail legs (like leg 7 run 36131765237) = infra, not verdict,
re-roll <=2 per leg slot.
Canon dispatch_456c_reroll.py shape: argv-guard, ancestry sha pin
(d73758a3 HARD), ensure_branch, world-bench-parallel.yml canon inputs.
Usage: scripts/dispatch_457a.py [--dry-run] [--no-batch] [--leg N]"""
import json, re, sys, time, urllib.request, urllib.error, subprocess

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

CARRIER_BRANCH = "round-456c-chunkmono"
EXPECTED_SHA = "d73758a3bdbfd1e4c9c11fc0d7cfc9f76a4b7bc0"  # HARD sha-pin

# canon inputs (TASK-457-A brief, verbatim)
INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "cmp456_chunkmono", "lever_arg": "1",
}
# НЕ слать travel_diet/fluid_dirty_ledger (freed slots, pinned in workflow).


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


def ensure_branch(tok, branch, base):
    try:
        return sha_of(tok, branch)
    except urllib.error.HTTPError:
        base_sha = sha_of(tok, base)
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{branch}", "sha": base_sha})
        return base_sha


def dispatch_leg(tok, leg):
    branch = f"round-456c-chunkmono-{leg}"
    sha = ensure_branch(tok, branch, CARRIER_BRANCH)
    if not sha.startswith(EXPECTED_SHA[:8]):
        raise SystemExit(f"SHA MISMATCH {branch}: live={sha[:12]} expected={EXPECTED_SHA[:12]}")
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
        data={"ref": branch, "inputs": dict(INPUTS)})
    print(f"dispatched chunkmono-{leg}: {branch} <- {CARRIER_BRANCH} "
          f"lever='cmp456_chunkmono' sha={sha[:8]}", flush=True)
    time.sleep(4)


def main():
    argv = sys.argv[1:]
    dry = "--dry-run" in argv
    no_batch = "--no-batch" in argv
    legs = []
    if "--leg" in argv:
        legs = [int(argv[argv.index("--leg") + 1])]
    else:
        raise SystemExit("argv-guard: pass --leg N (single leg per call; "
                         "--no-batch honoured, --dry-run for preflight)")

    tok = token_from_remote()
    live = sha_of(tok, CARRIER_BRANCH)[:8]
    if not EXPECTED_SHA.startswith(live):
        raise SystemExit(f"SHA MISMATCH: {CARRIER_BRANCH} live={live} expected={EXPECTED_SHA[:8]}")
    print(f"preflight OK: {CARRIER_BRANCH} @ {live} (NCDFE-fix carrier)", flush=True)
    if dry:
        print(f"DRY-RUN OK — would dispatch legs {legs}", flush=True)
        return
    if not no_batch:
        print("note: batch mode not used by agent-a (one leg per call)", flush=True)
    for leg in legs:
        dispatch_leg(tok, leg)
    print(f"=== TASK-457-A dispatch complete: legs {legs} ===", flush=True)


if __name__ == "__main__":
    main()
