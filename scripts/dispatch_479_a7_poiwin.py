#!/usr/bin/env python3
"""dispatch_479_a7_poiwin.py — [479-A7] Commander: poi456-4 окно фиды ×4 (тик ×479, MEGA-SWARM v19.0).

CLAIM-ПРЕГИСТЕР (закон 14a/16, зафиксирован ДО диспатча этим коммитом):
  POI-окно [8907260,9007260] 2/3 → 3/3, порог norm_v5 ≤ −1.99 (v5-конверсия
  leg 18.01−20; pair = 18.01 − norm ≥ +20), страж a86@8978124 (m5 +19.0).
  B2-диагноз ×478: нужны fast-раннеры (пул давал 6.32–7.61M, окно не отрисовано
  0/6; W3-c5 in-window norm +6.85 MISS).

ПЛАН (0 код-дельт, vanilla @master, алиасы одного sha — прецедент Л-470-S52.1/Л188b):
  1) 4 фида vanilla @master 70d64190 (алиасы round-479-a7-f1..f4, runs_before=0
     GET-страж, 1 реф=1 диспатч Л188a).
  2) canon x466-C98 ЯВНЫМ JSON: 640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/
     10G/xms4G, lever=''/'', band 6.0–9.5M fast-fail (окно НЕ в гейте — cpu читаем
     из run-env.txt пост-фактум).
  3) hit = cpu(run-env.txt) ∈ POI ∧ norm_v5 ≤ −1.99 ∧ CLEAN M1 (STW≤23.0s,
     young_avg≤200ms) ∧ FIXTURE-VALID → normtool_478 (selftest до вердикта, урок B3)
     → pair = 18.01 − norm → board [479-A7] THIRD-HIT {run id, cpu, pair}.
     0 legal в-POI → REFUTED_CENS 0/N. Runs >15 мин → DISPATCHED run-id (18-iii).

Пороги/окна v5-FROZEN (BANK_V5_FREEZE §2/§5) НЕ двигаются. Закон-5 запреты чисты.

Usage: dispatch_479_a7_poiwin.py [dispatch|poll] [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN_SHA = "70d641907769d982497fafbb3c56fb387fe2b204"  # origin/master на момент прег-коммита (vanilla @master)

BRANCHES = [f"round-479-a7-f{i}" for i in range(1, 5)]

POI_MIN, POI_MAX = 8_907_260, 9_007_260  # v5-FROZEN, report-only (НЕ гейт)

# x466-C98 канон EXACT (как dispatch_478_b2_poiwin.py; окно НЕ в band-гейте)
INPUTS = {
    "world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
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
    return m.group(1) if m else open("/tmp/gh_token").read().strip()


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
            print(f"HTTP {e.code}: {e.read()[:300]}", flush=True)
        raise
    return json.loads(body) if body else {}


def ref_sha(tok, branch):
    try:
        return api(tok, f"/repos/{REPO}/git/ref/heads/{branch}")["object"]["sha"]
    except urllib.error.HTTPError as e:
        if e.code == 404:
            return None
        raise


def runs_on_branch(tok, br):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    return [(r["id"], r.get("status"), r.get("conclusion"), r.get("created_at"))
            for r in runs.get("workflow_runs", []) if r.get("head_branch") == br]


def main():
    args = sys.argv[1:]
    cmd = "dispatch" if "dispatch" in args else ("poll" if "poll" in args else "dispatch")
    if "--dry-run" in args and cmd == "poll":
        raise SystemExit("argv-guard: dry-run only with dispatch")
    dry = "--dry-run" in args
    tok = token()

    master = api(tok, f"/repos/{REPO}/git/ref/heads/master")["object"]["sha"]
    print(f"origin/master={master[:8]} pin={PIN_SHA[:8]} match={master == PIN_SHA}", flush=True)
    if master != PIN_SHA:
        print("WARN: master сдвинулся (конкуренция тик-агентов) — пин 70d64190 прегистерован", flush=True)

    for br in BRANCHES:
        live = ref_sha(tok, br)
        pre = runs_on_branch(tok, br)
        if live is None:
            if dry:
                print(f"DRY: would create ref {br} @ {PIN_SHA[:8]}", flush=True)
                continue
            api(tok, f"/repos/{REPO}/git/refs", method="POST",
                data={"ref": f"refs/heads/{br}", "sha": PIN_SHA})  # Л188a FULL-sha
            live = ref_sha(tok, br)
        if live != PIN_SHA:
            raise SystemExit(f"SHA MISMATCH: {br} live={live[:8]} pin={PIN_SHA[:8]}")
        if pre:
            raise SystemExit(f"RUN-SNAPSHOT DIRTY: {br} уже имеет runs {pre}")
        print(f"ref OK: {br} @ {live[:8]} runs_before=0", flush=True)
    print(f"inputs: {json.dumps(INPUTS)}", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatches", flush=True)
        return
    if cmd == "poll":
        do_poll(tok)
        return

    for br in BRANCHES:
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": br, "inputs": INPUTS})
        print(f"dispatched: {br} (HTTP 204)", flush=True)
        time.sleep(2)

    seen, deadline = {}, time.time() + 300
    while time.time() < deadline and len(seen) < len(BRANCHES):
        time.sleep(12)
        for br in BRANCHES:
            if br in seen:
                continue
            for rid, st, concl, ca in runs_on_branch(tok, br):
                if st in ("queued", "in_progress", "completed"):
                    seen[br] = (rid, st)
                    print(f"run-id discovered: {br} -> {rid} status={st}", flush=True)
                    break
    print("=== A7 DISPATCHED: " + " ".join(
        f"f{i}={seen.get(br, ('POLL-NEEDED',))[0]}" for i, br in enumerate(BRANCHES, 1)) + " ===", flush=True)


def do_poll(tok):
    for br in BRANCHES:
        hits = runs_on_branch(tok, br)
        if not hits:
            print(f"{br}: NO-RUN", flush=True)
            continue
        rid, st, concl, ca = hits[0]
        print(f"{br}: run {rid} status={st} concl={concl} created={ca}", flush=True)


if __name__ == "__main__":
    main()
