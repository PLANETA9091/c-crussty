#!/usr/bin/env python3
"""dispatch_round420_mega.py — TASK-420: мега-композит colpush⊕chunk2 @77b22b7,
lever_flag=cmp420_colpush (мега-носитель; chunk-parse+noise ride-on по юниону).
Банк inputs exact. 3 ноги mg{a,b,c}."""
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

MEGA_SHA = "77b22b7a2821e1df8288abc3dd7ca210565f0ac2"
LEGS = {
    "mga": ("round-420-mga", MEGA_SHA),
    "mgb": ("round-420-mgb", MEGA_SHA),
    "mgc": ("round-420-mgc", MEGA_SHA),
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
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{name}", "sha": sha})
    print(f"{name}: created @ {sha[:7]}")


def main():
    tok = token_from_remote()
    for tag, (ref, sha) in LEGS.items():
        ensure_alias(tok, ref, sha)
    for tag, (ref, sha) in LEGS.items():
        inputs = dict(INPUTS)
        inputs["lever_flag"] = "cmp420_colpush"
        inputs["lever_arg"] = "1"
        r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
                method="POST", data={"ref": ref, "inputs": inputs})
        print(f"dispatch {ref} -> {'OK' if r == {} else r}")
        time.sleep(3)
    time.sleep(25)
    runs = api(tok, f"/repos/{REPO}/actions/runs?per_page=15")
    seen = []
    for run in runs.get("workflow_runs", []):
        if run["name"] == "world-bench-round" and run["head_branch"].startswith("round-420-mg") \
           and run["created_at"] > time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120)):
            seen.append((run["id"], run["head_branch"], run["created_at"]))
    for rid, rb, ca in sorted(set(seen), key=lambda x: x[2]):
        print(f"RUN {rid} {rb} created {ca}")
    with open("/home/z/rounds/ROUND-420/MEGA_DISPATCH.txt", "w") as f:
        f.write(f"mega {MEGA_SHA} lever cmp420_colpush\n")
        for rid, rb, ca in sorted(set(seen), key=lambda x: x[2]):
            f.write(f"{rid} {rb} {ca}\n")


if __name__ == "__main__":
    main()
