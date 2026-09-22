#!/usr/bin/env python3
"""dispatch_round420_base.py — TASK-420: якоря ×3 fresh @master(41456af)
+ mc-композит дневная репликация ×3 @20c9fdc (cmp417_mcomp).
Якоря = vanilla (lever_flag=""), band 6.0-9.5M, банк inputs exact.
mc-ноги = protocol v2 конвой-фикс (ночь 2.50/2.70/3.20 ≈+15 provisional,
цель дневного min-of-3 ≥+20% → немедленный мерж, закон 2 v17).
"""
import json, re, subprocess, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

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

# tag -> (alias-ref, sha-to-point, lever_flag)
LEGS = {
    "anchora": ("round-420-anchora", "MASTER", ""),
    "anchorb": ("round-420-anchorb", "MASTER", ""),
    "anchorc": ("round-420-anchorc", "MASTER", ""),
    "mc4a":    ("round-420-mc4a",    "20c9fdc", "cmp417_mcomp"),
    "mc4b":    ("round-420-mc4b",    "20c9fdc", "cmp417_mcomp"),
    "mc4c":    ("round-420-mc4c",    "20c9fdc", "cmp417_mcomp"),
}


def token_from_remote():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token in origin remote URL")
    return m.group(1)


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


def dispatch(tok, ref, lever):
    inputs = dict(INPUTS)
    inputs["lever_flag"] = lever
    inputs["lever_arg"] = "1"
    data = {"ref": ref, "inputs": inputs}
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST", data=data)
    print(f"dispatch ref={ref} lever={lever!r} -> {'OK' if r == {} else r}")


def main():
    tok = token_from_remote()
    master_sha = subprocess.run(["git", "-C", "/home/z/c-crussty", "rev-parse", "origin/master"],
                                capture_output=True, text=True).stdout.strip()
    print(f"master = {master_sha[:7]}")
    resolved = {}
    for tag, (ref, sha_or_master, lever) in LEGS.items():
        sha = master_sha if sha_or_master == "MASTER" else sha_or_master
        resolved[tag] = (ref, sha, lever)
        ensure_alias(tok, ref, sha)
    min_created = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 60))
    for tag, (ref, sha, lever) in resolved.items():
        dispatch(tok, ref, lever)
        time.sleep(3)
    time.sleep(25)
    runs = api(tok, f"/repos/{REPO}/actions/runs?per_page=15")
    seen = []
    for run in runs.get("workflow_runs", []):
        if run["name"] == "world-bench-round" and run["head_branch"].startswith("round-420-") \
           and run["created_at"] > min_created:
            seen.append((run["id"], run["head_branch"], run["created_at"]))
    for rid, rb, ca in sorted(set(seen), key=lambda x: x[2]):
        print(f"RUN {rid} {rb} created {ca}")
    with open("/home/z/rounds/ROUND-420/BASE_DISPATCH.txt", "w") as f:
        f.write(f"master {master_sha}\nmc-sha 20c9fdc cmp417_mcomp\n")
        for rid, rb, ca in sorted(set(seen), key=lambda x: x[2]):
            f.write(f"{rid} {rb} {ca}\n")
    print("wrote /home/z/rounds/ROUND-420/BASE_DISPATCH.txt")


if __name__ == "__main__":
    main()
