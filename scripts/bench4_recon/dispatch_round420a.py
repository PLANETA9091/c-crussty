#!/usr/bin/env python3
"""dispatch_round420a.py — TASK-420-A: colpush FIX legs ×3 on the colpush-fix
carrier (round-420-a-colpush2). Lever cmp420_colpush (renamed from
cmp419_colpush), band 6.0-9.5M, bank inputs EXACT (anchors-420 template;
NO travel_diet/fluid_dirty_ledger additions).

FIX МАНДАТ (ROOTCAUSE-NCDFE.md): define(ColpushOps)+selfTest ДО арма
RegionTickOps.bulkTick (COLPUSH_ON volatile flip) + one-shot NCDFE guard
(eprintln ×1 → vanilla tail → DISARM) + javap-гейт flat==nested.
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

LEGS = {
    "cpa": ("round-420-a-cpa", "cmp420_colpush"),
    "cpb": ("round-420-a-cpb", "cmp420_colpush"),
    "cpc": ("round-420-a-cpc", "cmp420_colpush"),
}


def token():
    try:
        with open("/tmp/gh_token") as f:
            t = f.read().strip()
            if t:
                return t
    except OSError:
        pass
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token")
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
        api(tok, f"/repos/{REPO}/git/refs/heads/{name}", method="PATCH", data={"sha": sha, "force": True})
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
    tok = token()
    head_sha = subprocess.run(["git", "-C", "/home/z/rounds/ROUND-420/agent-a", "rev-parse", "HEAD"],
                              capture_output=True, text=True).stdout.strip()
    if len(head_sha) != 40:
        raise SystemExit(f"bad head sha: {head_sha!r}")
    print(f"head = {head_sha} (round-420-a-colpush2, 40-char)")
    for tag, (ref, lever) in LEGS.items():
        ensure_alias(tok, ref, head_sha)
    min_created = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 60))
    for tag, (ref, lever) in LEGS.items():
        dispatch(tok, ref, lever)
        time.sleep(3)
    time.sleep(20)
    runs = api(tok, f"/repos/{REPO}/actions/runs?per_page=15")
    seen = []
    for run in runs.get("workflow_runs", []):
        if run["name"] == "world-bench-round" and run["head_branch"].startswith("round-420-a-cp") \
           and run["created_at"] > min_created:
            seen.append((run["id"], run["head_branch"], run["created_at"]))
    out = [f"head {head_sha} (round-420-a-colpush2, lever cmp420_colpush)"]
    for rid, rb, ca in sorted(set(seen), key=lambda x: x[2]):
        print(f"RUN {rid} {rb} created {ca}")
        out.append(f"{rid} {rb} {ca}")
    with open("/home/z/rounds/ROUND-420/A_DISPATCH_420A.txt", "w") as f:
        f.write("\n".join(out) + "\n")


if __name__ == "__main__":
    main()
