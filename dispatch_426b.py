#!/usr/bin/env python3
"""dispatch_426b.py — TASK-426-B: 3 inside-ноги l1/l2/l3 @round-424-b-inside aa55277, lever=cmp424_inside.
Рестарт оборвавшегося TASK-425-B. Шаблон: dispatch_anchors_424.py (ensure_alias + dispatch).
Диск-урок: explicit-path adds only."""
import json, re, subprocess, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

LEGS = {
    "l1": ("round-426-b-l1", "cmp424_inside", "origin/round-424-b-inside"),
    "l2": ("round-426-b-l2", "cmp424_inside", "origin/round-424-b-inside"),
    "l3": ("round-426-b-l3", "cmp424_inside", "origin/round-424-b-inside"),
}

# РОВНО банк (canon TASK-426); НЕ слать travel_diet/fluid_dirty_ledger
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


def local_sha(src):
    return subprocess.run(["git", "-C", "/home/z/c-crussty", "rev-parse", src],
                          capture_output=True, text=True).stdout.strip()


def ensure_alias(tok, name, src):
    sha = local_sha(src)
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    if r.get("object", {}).get("sha") == sha:
        print(f"{name}: exists @ {sha[:7]}")
        return True
    code = api(tok, f"/repos/{REPO}/git/refs", method="POST",
               data={"ref": f"refs/heads/{name}", "sha": sha})
    ok = bool(code)
    print(f"{name}: created @ {sha[:7]} ({'ok' if ok else 'FAIL'})")
    return ok


def dispatch(leg, ref, lever, tok):
    short = ref
    inputs = dict(INPUTS)
    inputs["lever_flag"] = lever
    inputs["lever_arg"] = "1"
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
        method="POST", data={"ref": ref, "inputs": inputs})
    print(f"{leg}: dispatch sent ref={ref} lever={lever!r}")
    for _ in range(12):
        time.sleep(5)
        d = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?per_page=40")
        for r in d.get("workflow_runs", []):
            if r["head_branch"] == short and \
               r["created_at"] > time.strftime("%Y-%m-%dT%H:%M", time.gmtime(time.time() - 300)):
                print(json.dumps({"leg": leg, "run_id": r["id"], "ref": ref,
                                  "status": r["status"], "sha": r["head_sha"][:7]}))
                return r["id"]
    print(f"{leg}: no run appeared", file=sys.stderr)
    return None


def main():
    tok = token_from_remote()
    out = {}
    for leg, (ref, lever, src) in LEGS.items():
        if not ensure_alias(tok, ref, src):
            continue
        time.sleep(2)
        rid = dispatch(leg, ref, lever, tok)
        if rid:
            out[leg] = rid
    print(json.dumps(out))


if __name__ == "__main__":
    sys.exit(main())
