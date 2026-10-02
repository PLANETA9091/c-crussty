#!/usr/bin/env python3
"""dispatch_470_s35_window.py — S35 / ЯКОРЯ (round-470): ре-ролл окна [6741667,6841667]
для БАНКА — ваниль-якоря rev-b5..b8 (b1-b4 band-dead), Л196/Л188b.
Канон: argv-guard, sha-pin, vanilla-anchor inputs (x466-C98 бан-EXACT),
band 6.0-9.5M fast-fail, Л188a (POST /git/refs FULL-sha + GET-верификация),
Л188b (1 диспатч = 1 ветка; алиасы одного sha = независимые concurrency-группы).
ЦЕЛЬ: чистая ваниль (flag=""/arg="") на №11-пине b3853246 → (а) в-точки банка v5
в плотной 6.7-6.8M-щели (Л201); (б) 3-й якорь МЕРЖа №11: hit = norm_v5 ≤+4.33
(leg1 +24.33−20 Л196) при bench runner_cpu_index ∈ [6741667,6841667] (Δcpu≤50k);
(в) резерв к leg2-порогу ≤−4.84 @6787302.
Usage: dispatch_470_s35_window.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCHES = ["round-470-s35-b5", "round-470-s35-b6", "round-470-s35-b7", "round-470-s35-b8"]
PIN_SHA = "b385324677ac76f4f9f8ef942a3835c8004aef78"  # МЕРЖ №11 (n16) — 0 код-дельт, чистая ваниль
WINDOW = (6741667, 6841667)
THRESHOLD = "+4.33"  # leg1 +24.33 (Л196) − 20

# x466-C98 бан-EXACT vanilla-anchor vector (канон S01-a00 / dispatch_s01.sh / Л-466-C41.1)
INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
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
        if e.code != 404:
            print(f"HTTP {e.code}: {e.read()[:300]}")
        raise
    return json.loads(body) if body else {}


def ref_sha(tok, branch):
    try:
        return api(tok, f"/repos/{REPO}/git/ref/heads/{branch}")["object"]["sha"]
    except urllib.error.HTTPError as e:
        if e.code == 404:
            return None
        raise


def create_ref(tok, branch, full_sha):
    """Л188a: прямой POST /git/refs FULL-sha (короткий sha = 422, S20-микро-урок)."""
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{branch}", "sha": full_sha})


def find_run_id(tok, full_sha):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&head_sha={full_sha}&per_page=20")
    return runs.get("workflow_runs", [])


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args

    tok = token()
    master = ref_sha(tok, "master")
    if not master.startswith(PIN_SHA[:8]):
        print(f"note: origin/master={master[:8]} != pin {PIN_SHA[:8]} — якоря на пине №11 (0 код-дельт)")
    print(f"preflight: pin={PIN_SHA[:8]} window={WINDOW} threshold<={THRESHOLD} "
          f"vanilla (flag=''/arg='') gc3 pop150k", flush=True)
    print(f"inputs: {json.dumps(INPUTS, ensure_ascii=False)}", flush=True)

    for br in BRANCHES:
        live = ref_sha(tok, br)
        if live is None:
            if dry:
                print(f"DRY: would create ref {br} @ {PIN_SHA[:8]}", flush=True)
                continue
            create_ref(tok, br, PIN_SHA)
            live = ref_sha(tok, br)
        if live != PIN_SHA:
            raise SystemExit(f"SHA MISMATCH after create: {br} live={live} pin={PIN_SHA}")
        print(f"ref OK: {br} @ {live[:8]} (Л188a GET-верификация)", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatches", flush=True)
        return

    for br in BRANCHES:
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": INPUTS})
        print(f"dispatched: {br} vanilla-anchor (HTTP 204)", flush=True)
        time.sleep(2)

    print("polling run-ids (Л188a discovery by head_sha)...", flush=True)
    seen = {}
    deadline = time.time() + 300
    while time.time() < deadline and len(seen) < len(BRANCHES):
        time.sleep(15)
        for r in find_run_id(tok, PIN_SHA):
            name = r.get("head_branch")
            if name in BRANCHES and name not in seen:
                seen[name] = r["id"]
                print(f"run-id {name}: {r['id']} status={r.get('status')} {r['html_url']}", flush=True)
    missing = [b for b in BRANCHES if b not in seen]
    if missing:
        print(f"run-ids NOT discovered for {missing} — poll actions/runs?head_sha={PIN_SHA}", flush=True)
    print("=== S35 DISPATCHED: " + " ".join(f"{b.replace('round-470-s35-','')}={seen.get(b)}" for b in BRANCHES) + " ===", flush=True)


if __name__ == "__main__":
    main()
