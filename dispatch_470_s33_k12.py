#!/usr/bin/env python3
"""dispatch_470_s33_k12.py — S33 [ЯКОРЯ] — chkclimb-12 0/3 K12-window re-rolls, gc6-preset
3 алиаса round-470-s33-k12{a,b,c} = ОДИН sha b3853246 (MERGE №11 vanilla, S01-a00a/a16a
прецедент; case "6" в run_world3.sh:587 живой) = 3 независимых concurrency-группы
(Л188b: 1 диспатч = 1 ветка). Ваниль: lever_flag="" lever_arg="".
Банк-v5 канон inputs (640/300s/fp4/gc6/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G,
band 6.0-9.5M fast-fail Л188c). gc_tune=3→6: Л184/Л183 re-grain — ценз-экономика
31.6%→~0 на 150k (HOST-ценз-хиты 26.4/27.1s съели window-слоты; gc6 = MD-Full=0
+ CC 6→2-3, javap-neutral 0 Java → норм-плоскость не тронута).
Гипотеза-дельта H-S33a/b/c (закон 14a preregister): Δnorm(gc6−gc3)≈0 (±CC-джиттер
σ5.6 Л182); STW 17.3-18.1s ≤23.0 CLEAN (Л217 s10base-gc6-аналог) → нога валидна;
hit: idx∈K12[8133686,8233686] ∧ norm_v5 ≤−2.57 → K12 pass 0→1 (chkclimb-12 B-якорь).
Usage: dispatch_470_s33_k12.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "b385324677ac76f4f9f8ef942a3835c8004aef78"  # MERGE №11 full sha (S20 урок: FULL-sha в refs, 422 иначе)
ALIASES = ["round-470-s33-k12a", "round-470-s33-k12b", "round-470-s33-k12c"]

INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "6", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    if m:
        return m.group(1)
    return open("/tmp/gh_token").read().strip()


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
        print(f"HTTP {e.code}: {e.read()[:200]}")
        raise
    return json.loads(body) if body else {}


def ensure_alias(tok, branch):
    """Л188a canon: direct POST /git/refs + GET verify object.sha ДО dispatch."""
    try:
        ref = api(tok, f"/repos/{REPO}/git/ref/heads/{branch}")
        live = ref["object"]["sha"]
        if live != PIN:
            raise SystemExit(f"SHA MISMATCH: {branch} live={live} pin={PIN}")
        return f"exists@{live[:8]}"
    except urllib.error.HTTPError as e:
        if e.code != 404:
            raise
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{branch}", "sha": PIN})
        ref = api(tok, f"/repos/{REPO}/git/ref/heads/{branch}")
        live = ref["object"]["sha"]
        if live != PIN:
            raise SystemExit(f"POST-VERIFY FAIL: {branch} live={live} pin={PIN}")
        return f"created@{live[:8]}"


def capture_run_id(tok, branch, attempts=8, sleep_s=20):
    """Run-id канон 12e: last workflow_dispatch run на ветке c head_sha==PIN."""
    for i in range(attempts):
        data = api(tok, f"/repos/{REPO}/actions/runs?branch={branch}&per_page=10")
        for run in data.get("workflow_runs", []):
            if run.get("head_sha") == PIN and run.get("event") == "workflow_dispatch":
                return run["id"], run["status"]
        if i < attempts - 1:
            time.sleep(sleep_s)
    return None, "not-visible-yet"


def main():
    args = sys.argv[1:]
    if any(a != "--dry-run" for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args
    tok = token()
    print(f"pin {PIN[:8]} = MERGE №11 vanilla (gc6 case alive, S01-a00a/a16a base)", flush=True)
    print(f"inputs: {json.dumps(INPUTS, ensure_ascii=False)}", flush=True)
    states = []
    for br in ALIASES:
        states.append(ensure_alias(tok, br))
    print(f"aliases: {list(zip(ALIASES, states))}", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatch", flush=True)
        return
    for br in ALIASES:
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": INPUTS})
        rid, st = capture_run_id(tok, br)
        print(f"dispatched: {br} HTTP 204 run_id={rid} status={st}", flush=True)


if __name__ == "__main__":
    main()
