#!/usr/bin/env python3
"""dispatch_396F.py — TASK-396-F (MEGA-ROUND-1, вектор F): items_mono.

Архитектурная замена call-структуры горячего метода: единственный
megamorphic-диспетч entity-tick (invokevirtual Entity.tick внутри
ServerLevel.tickNonPassenger, javap bc80, fixture-кенсус ровно 1 site)
ретаргетится на RegionTickOps.entityTick type-test сплит — items
(~70% популяции 150k, 31.17% java = ТОП-1 лейн) идут monomorphic
invokevirtual ItemEntity.tick (C2 direct call + инлайн), остальные —
ванильный виртуальный диспетч. Порядок тиков/семантика ядра не тронуты.

Диспатч: world-bench-parallel.yml @ ref=round-396-f-items_mono (9d9b6e9+),
БАНК v4-флаги (inside_cache=1 + flush_diet=1 + region_threads=4 +
batch_collector=1 + fluid_guard=1, gc_tune=3, ParallelGC), lever_flag=
items_mono, lever_arg=1. Якорь банка v4: median5 TPS 2.6 @ 8551924 /
2.2 @ 6653417. Concurrency-гвардов НЕТ (per-ref concurrency в самом
workflow — до 10 параллельных бенчей агентов).

Запуск: python3 scripts/bench4_recon/dispatch_396F.py
"""
import json, re, subprocess, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
BRANCH = "round-396-f-items_mono"


def token_from_remote():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token in origin remote URL")
    return m.group(1)


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:200]}", file=sys.stderr)
        return {}


def main():
    tok = token_from_remote()

    local = subprocess.run(["git", "-C", "/home/z/rounds/ROUND-396/agent-f",
                            "rev-parse", "HEAD"],
                           capture_output=True, text=True).stdout.strip()
    remote = api(tok, f"{API}/repos/{REPO}/branches/{BRANCH}").get("commit", {}).get("sha", "")
    if remote[:12] != local[:12]:
        print(f"branch HEAD mismatch: local {local[:12]} != remote {remote[:12]} — dispatch BLOCKED")
        return 1

    inputs = {
        "radius": "640",
        "seconds": "300",
        "fake_players": "4",
        "fluid_guard": "1",
        "gc_tune": "3",
        "inside_cache": "1",
        "flush_diet": "1",
        "fluid_dirty": "0",
        "fluid_bitmask": "0",
        "region_threads": "4",
        "batch_collector": "1",
        "inside_bitmask": "0",
        "skip_store_bb": "0",
        "region_steal": "0",
        "bu_defer": "0",
        "population_target": "150000",
        "population_seed": "42",
        "server_xmx": "10G",
        "server_xms": "4G",
        "cpu_band_min": "6000000",
        "cpu_band_max": "9500000",
        "lever_flag": "items_mono",
        "lever_arg": "1",
    }
    api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench-parallel.yml/dispatches",
        method="POST", data={"ref": BRANCH, "inputs": inputs})
    print("dispatch POST sent; waiting for the run to appear...")
    for _ in range(12):
        time.sleep(5)
        d = api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench-parallel.yml/runs?per_page=30")
        for r in d.get("workflow_runs", []):
            if r["head_branch"] == BRANCH:
                print(json.dumps({"run_id": r["id"], "status": r["status"],
                                  "head_sha": r["head_sha"][:7], "created": r["created_at"]}))
                return 0
    print("no run appeared after 60s", file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main())
