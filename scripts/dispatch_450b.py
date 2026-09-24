#!/usr/bin/env python3
"""dispatch_450b.py — TASK-450-B (tick-450, agent-B) — items cycle-4 нога.
УРОК ×447: argv-guard ПЕРЕД любым действием. --dry-run = только префлайт, БЕЗ диспатчей.
Диспатчит РОВНО ОДНУ ногу: ветка round-450b-items-1 (ensure_branch от round-450b-items),
lever_flag=cmp446_items, банк-inputs канона v17."""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

# (leg, branch, base_branch, lever)
BATCH = [
    ("items-c4-1", "round-450b-items-1", "round-450b-items", "cmp446_items"),
]

EXPECTED_SHA = {
    "round-450b-items": "5e11dbef",  # cycle-4 repair 30d80c3e + dispatch-script commit (live head)
}

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


def main():
    args = sys.argv[1:]
    if "--help" in args or "-h" in args:
        print("usage: dispatch_450b.py [--dry-run]")
        print("  --dry-run  preflight only, NO dispatch (always safe)")
        raise SystemExit(0)
    dry = "--dry-run" in args
    tok = token_from_remote()
    print("=== TASK-450-B DISPATCH (items cycle-4) ===", flush=True)
    for _, branch, base, _ in BATCH:
        live = sha_of(tok, base)[:8]
        exp = EXPECTED_SHA.get(base)
        if exp is None:
            raise SystemExit(f"NO PIN for base {base} (argv-guard canon)")
        if not live.startswith(exp):
            raise SystemExit(f"SHA MISMATCH: {base} live={live} expected={exp} — PUSH not visible? abort")
        print(f"preflight OK: {base} @ {live}", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatches", flush=True)
        return
    for leg, branch, base, lever in BATCH:
        sha = ensure_branch(tok, branch, base)
        if sha[:8] != EXPECTED_SHA[base]:
            # ветка уже существует на ДРУГОМ SHA — не переиспользуем вслепую
            raise SystemExit(f"BRANCH {branch} exists at {sha[:8]}, expected {EXPECTED_SHA[base]} — pin update required")
        inputs = dict(INPUTS)
        inputs["lever_flag"] = lever
        inputs["lever_arg"] = "1"
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": branch, "inputs": inputs})
        print(f"dispatched {leg}: {branch} <- {base} lever='{lever}' sha={sha[:8]}", flush=True)
        time.sleep(4)
    print("=== TASK-450-B DISPATCH COMPLETE ===", flush=True)


if __name__ == "__main__":
    main()
