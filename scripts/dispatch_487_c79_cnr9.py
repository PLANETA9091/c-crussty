#!/usr/bin/env python3
"""dispatch_487_c79_cnr9.py — [487-C79] Commander C79 canary cnr9 (тик ×487, Job 415026).

CLAIM (prereg, /home/z/rounds/ROUND-487/board/CLM-C79.md):
  canary cnr9 — post-×486-мерж ваниль-коридор, 3-я точка коридора после 2 мержей.
  master PIN 85a06f2f = код f0051e70 (RAMP-мерж) + ee168ecb (LIMBO-GATE-мерж);
  cnr7 (C85) 12.9@6.99M + cnr8 (C86) 14.8@8.88M = CORRIDOR-PASS ×2.
ГИПОТЕЗА-ДЕЛЬТА (prereg): norm_v5 ∈ [-6,+6] (ci.yml canary-guard) — ваниль-семантика
  после 2 мержей интактна (0 деградации post-мерж).
ВЕКТОР = x466-C98 канон-ваниль бит-в-бит: 640/300/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/
  seed42/10G/xms4G, band [6000000,9500000], lever_flag=∅ lever_arg=∅, datapack ∅.
ПОВЕРХНОСТЬ: world-bench-parallel.yml (инфра-канон ×486: parallel per-ref — единственная
  поверхность). 1 ветка = 1 ран (Л188b).
LEDGER: «## ТИК-487 ЛАБ-C79».

Usage: dispatch_487_c79_cnr9.py [--dry-run]
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-487-c79-cnr9"
PIN = "85a06f2ffb9337f82b3c3851a99cbf020f6c431e"  # master ×486 учёт (код f0051e70, LIMBO-GATE ee168ecb in)

# x466-C98 канон-ваниль (640/300/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G,
# band [6.0,9.5]M), lever ∅ — всё СТРОКАМИ (урок C01: 422 на не-строках)
INPUTS = {
    "world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
    "radius": "640", "seconds": "300",
    "fake_players": "4", "fluid_guard": "1",
    "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "1", "fluid_bitmask": "0",
    "region_threads": "4", "batch_collector": "1",
    "skip_store_bb": "0", "region_steal": "0", "bu_defer": "0",
    "lever_flag": "", "lever_arg": "", "datapack_url": "",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
}


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def ref_sha(tok, branch):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{branch}")
    return r.get("object", {}).get("sha")


def runs_on_branch(tok, br):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    return [(r["id"], r.get("status"), r.get("created_at"), r.get("head_sha"))
            for r in runs.get("workflow_runs", []) if r.get("head_branch") == br]


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args
    tok = token()

    live_master = ref_sha(tok, "master")
    print(f"origin/master live = {str(live_master)[:8]} (НЕ трогаем) -> canary pin {PIN[:8]}", flush=True)

    pre = runs_on_branch(tok, BRANCH)
    if pre:
        raise SystemExit(f"RUN-SNAPSHOT DIRTY: {BRANCH} has runs {pre}")

    cur = ref_sha(tok, BRANCH)
    if cur != PIN:
        raise SystemExit(f"BRANCH SHA MISMATCH: remote {cur} != pin {PIN} (Л188a)")
    print(f"GET-verify OK {BRANCH} object.sha == {PIN}", flush=True)

    if dry:
        print("DRY-INPUTS: " + json.dumps(INPUTS, sort_keys=True), flush=True)
        print("DRY-RUN OK — no dispatch", flush=True)
        return

    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": BRANCH, "inputs": INPUTS})
    if r == {}:
        print("dispatch 204-OK", flush=True)
    else:
        raise SystemExit(f"dispatch failed: {r}")

    rid = None
    deadline = time.time() + 240
    while time.time() < deadline and rid is None:
        time.sleep(10)
        for rid_, st, ca, hs in runs_on_branch(tok, BRANCH):
            if hs == PIN:
                rid = rid_
                break
    if rid is None:
        print("run-id not visible in 240s (204 принят, discovery по head_sha позже)", flush=True)
        json.dump({"branch": BRANCH, "pin": PIN, "run_id": None,
                   "inputs": INPUTS}, open("/home/z/rounds/ROUND-487/c79_dispatch.json", "w"), indent=1)
        return
    print(f"RUN-ID {rid}", flush=True)
    json.dump({"branch": BRANCH, "pin": PIN, "run_id": rid,
               "inputs": INPUTS}, open("/home/z/rounds/ROUND-487/c79_dispatch.json", "w"), indent=1)


if __name__ == "__main__":
    main()
