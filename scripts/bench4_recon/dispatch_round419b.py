#!/usr/bin/env python3
"""dispatch_round419b.py — TASK-419-B: 3 ноги cmp419_sense @ round-419-b-sense (a6cf691).
ensure_alias-паттерн dispatch_round417_anchors.py; inputs exact (мандат тика).
"""
import json, re, subprocess, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
TOKEN = open("/tmp/gh_token").read().strip()
BRANCH_SHA = subprocess.run(
    ["git", "-C", "/home/z/rounds/ROUND-419/agent-b", "rev-parse", "HEAD"],
    capture_output=True, text=True).stdout.strip()

INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "cmp419_sense", "lever_arg": "1",
}

LEGS = {
    "sba": "round-419-b-sba",
    "sbb": "round-419-b-sbb",
    "sbc": "round-419-b-sbc",
}


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {}


def ensure_alias(tok, name, sha):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    if r.get("object", {}).get("sha") == sha:
        print(f"{name}: exists @ {sha[:7]}")
        return
    if r.get("object"):
        api(tok, f"/repos/{REPO}/git/refs/heads/{name}", method="PATCH",
            data={"sha": sha, "force": True})
        print(f"{name}: moved -> {sha[:7]}")
        return
    code = api(tok, f"/repos/{REPO}/git/refs", method="POST",
               data={"ref": f"refs/heads/{name}", "sha": sha})
    print(f"{name}: created @ {sha[:7]} ({'ok' if code else 'fail?'})")


def dispatch(tok, ref):
    data = {"ref": ref, "inputs": dict(INPUTS)}
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST", data=data)
    print(f"dispatch ref={ref} lever={INPUTS['lever_flag']!r} -> {'OK' if r == {} else r}")


def main():
    tok = TOKEN
    print(f"round-419-b-sense HEAD = {BRANCH_SHA[:7]}")
    for tag, ref in LEGS.items():
        ensure_alias(tok, ref, BRANCH_SHA)
    min_created = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 60))
    for tag, ref in LEGS.items():
        dispatch(tok, ref)
        time.sleep(3)
    time.sleep(20)
    runs = api(tok, f"/repos/{REPO}/actions/runs?per_page=15")
    seen = []
    for run in runs.get("workflow_runs", []):
        if run["name"] == "world-bench-round" and run["head_branch"].startswith("round-419-b-s") \
                and run["created_at"] > min_created:
            seen.append((run["id"], run["head_branch"], run["created_at"]))
    for rid, rb, ca in sorted(set(seen), key=lambda x: x[2]):
        print(f"RUN {rid} {rb} created {ca}")
    with open("/home/z/rounds/ROUND-419/B_DISPATCH_419B.txt", "w") as f:
        f.write(f"round-419-b-sense {BRANCH_SHA}\n")
        for rid, rb, ca in sorted(set(seen), key=lambda x: x[2]):
            f.write(f"{rid} {rb} {ca}\n")


if __name__ == "__main__":
    main()
