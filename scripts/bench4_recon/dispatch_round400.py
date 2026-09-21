#!/usr/bin/env python3
"""dispatch_round400.py — TASK-400: парный A/B вердикт B-shardgrid (min-of-3)
+ F leg3 + 3 свежих якорных ноги банк-v4 (lever_flag="" — ванильный items,
т.к. композит B = J+shardgrid без items_oss; base comparable).
Alias-refs round-400-anchor{a,b} создаются на текущем master sha для per-ref параллели.
"""
import json, re, subprocess, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

LEGS = {
    "anchor1": ("master", ""),
    "anchor2": ("round-400-anchora", ""),
    "anchor3": ("round-400-anchorb", ""),
    "bleg3":   ("round-399-b-shardgrid", "cmp399_shard"),
    "fleg3":   ("round-399-f-despawn2", "cmp399_despawn2"),
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


def ensure_alias(tok, sha):
    for name in ("round-400-anchora", "round-400-anchorb"):
        r = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
        if r.get("object", {}).get("sha") == sha:
            print(f"{name}: exists @ {sha[:7]}")
            continue
        code = api(tok, f"/repos/{REPO}/git/refs", method="POST",
                   data={"ref": f"refs/heads/{name}", "sha": sha})
        print(f"{name}: created @ {sha[:7]} ({'ok' if code else 'fail?'})")


def dispatch(leg, ref, lever, tok):
    d = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?per_page=30")
    clash = [r for r in d.get("workflow_runs", [])
             if r["head_branch"] == ref.split("/")[-1] and
             r["status"] in ("in_progress", "queued", "waiting")]
    if clash:
        print(f"{leg}: per-ref clash on {ref} (run {clash[0]['id']}) — SKIP")
        return None
    inputs = dict(INPUTS)
    inputs["lever_flag"] = lever
    inputs["lever_arg"] = "1"
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
        method="POST", data={"ref": ref, "inputs": inputs})
    print(f"{leg}: dispatch sent ref={ref} lever={lever!r}")
    for _ in range(12):
        time.sleep(5)
        d = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?per_page=30")
        for r in d.get("workflow_runs", []):
            if r["head_branch"] == ref.split("/")[-1] and \
               r["created_at"] > time.strftime("%Y-%m-%dT%H:%M", time.gmtime(time.time() - 300)):
                print(json.dumps({"leg": leg, "run_id": r["id"], "ref": ref,
                                  "status": r["status"], "sha": r["head_sha"][:7]}))
                return r["id"]
    print(f"{leg}: no run appeared", file=sys.stderr)
    return None


def main():
    tok = token_from_remote()
    sha = subprocess.run(["git", "-C", "/home/z/c-crussty", "rev-parse", "origin/master"],
                         capture_output=True, text=True).stdout.strip()
    ensure_alias(tok, sha)
    legs = sys.argv[1].split(",") if len(sys.argv) > 1 else list(LEGS)
    out = {}
    for leg in legs:
        if leg not in LEGS:
            print(f"unknown leg {leg}", file=sys.stderr)
            continue
        ref, lever = LEGS[leg]
        rid = dispatch(leg, ref, lever, tok)
        if rid:
            out[leg] = rid
    print(json.dumps(out))


if __name__ == "__main__":
    sys.exit(main())
