#!/usr/bin/env python3
"""dispatch_round421_c.py — TASK-421-C: chunk/worldgen axis (law 8) legs ×3
@round-421-c-chunk (base 2d23f45), lever_flag=cmp421_chunk.
Pair = vs БЛИЖАЙШИЙ якорь-421 по runner_cpu_index:
  anchora 35800920567 / anchorb 35800926548 / anchorc 35800931828 @2d23f45
  (bank-ожидание ~2.75/2.20/2.10, /home/z/rounds/ROUND-421/ANCHOR_DISPATCH.txt).
Inputs = банк РОВНО (ничего не слайдить: travel_diet/fluid_dirty_ledger не трогаем).
Concurrency-гвардов не ставить (world-bench-parallel per-ref discipline).
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
    "lever_flag": "cmp421_chunk", "lever_arg": "1",
}

# leg -> alias-ref; each alias points at the round-421-c-chunk branch sha.
BRANCH = "round-421-c-chunk"
LEGS = ["round-421-c-chunk-l1", "round-421-c-chunk-l2", "round-421-c-chunk-l3"]


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


def dispatch(tok, ref):
    data = {"ref": ref, "inputs": dict(INPUTS)}
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST", data=data)
    print(f"dispatch ref={ref} lever={INPUTS['lever_flag']!r} -> {'OK' if r == {} else r}")


def main():
    tok = token_from_remote()
    sha = subprocess.run(["git", "-C", "/home/z/c-crussty", "rev-parse", f"origin/{BRANCH}"],
                         capture_output=True, text=True).stdout.strip()
    if not sha:
        sha = subprocess.run(["git", "-C", "/home/z/rounds/ROUND-421/agent-c", "rev-parse", BRANCH],
                             capture_output=True, text=True).stdout.strip()
    if not sha:
        raise SystemExit(f"cannot resolve {BRANCH}")
    print(f"{BRANCH} = {sha}")
    min_created = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 60))
    for leg in LEGS:
        ensure_alias(tok, leg, sha)
    for leg in LEGS:
        dispatch(tok, leg)
        time.sleep(3)
    time.sleep(25)
    runs = api(tok, f"/repos/{REPO}/actions/runs?per_page=15")
    seen = []
    for run in runs.get("workflow_runs", []):
        if run["name"] == "world-bench-round" and run["head_branch"].startswith("round-421-c-chunk-l") \
           and run["created_at"] > min_created:
            seen.append((run["id"], run["head_branch"], run["created_at"]))
    for rid, rb, ca in sorted(set(seen), key=lambda x: x[2]):
        print(f"RUN {rid} {rb} created {ca}")
    with open("/home/z/rounds/ROUND-421/agent-c/CHUNK_DISPATCH_421.txt", "w") as f:
        f.write(f"{BRANCH} {sha} lever=cmp421_chunk\n")
        for rid, rb, ca in sorted(set(seen), key=lambda x: x[2]):
            f.write(f"{rid} {rb} {ca}\n")
    print("wrote /home/z/rounds/ROUND-421/agent-c/CHUNK_DISPATCH_421.txt")


if __name__ == "__main__":
    main()
