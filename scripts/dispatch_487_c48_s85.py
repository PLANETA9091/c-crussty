#!/usr/bin/env python3
"""dispatch_487_c48_s85.py — [487-C48] Commander C48 vanilla-draw якорь [8.5,8.8]M #8 rep (тик ×487, Job 415026).

CLAIM (prereg, CLM-C48.md): vanilla-нога (lever_flag=∅) канон-вектора BANK-V5,
band-дельта = ЕДИНСТВЕННАЯ: cpu_band [8500000, 8800000] (узкое окно [8.5,8.8]M).
До 3 роллов: round-487-c48-s85r1/-r2/-r3 @ PIN 85a06f2f (1 ветка = 1 ран, Л188b).
Band fast-fail = бесплатный pairing-дискард (S7-96d) → ре-ролл (макс 3).
Поверхность: world-bench-parallel.yml (инфра-канон ×486, parallel per-ref).

Usage: dispatch_487_c48_s85.py [--dry-run]
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "85a06f2ffb9337f82b3c3851a99cbf020f6c431e"  # master ×486 учёт (код f0051e70)
GATE_STEP = "Runner calibration band gate (pair-hunter fast-fail, S7-96d pairing law)"

BRANCHES = ["round-487-c48-s85r1", "round-487-c48-s85r2", "round-487-c48-s85r3"]

# x466-C98 BANK-V5 канон; band [8.5,8.8]M = единственная дельта (узкая плоскость #8 rep)
INPUTS = {
    "radius": "640", "seconds": "300",
    "fake_players": "4", "fluid_guard": "1",
    "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0",
    "region_threads": "4", "batch_collector": "1",
    "skip_store_bb": "0", "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "8500000",   # ← дельта: плоскость [8.5,8.8]M
    "cpu_band_max": "8800000",   # ← дельта: плоскость [8.5,8.8]M
    "lever_flag": "", "lever_arg": "",  # vanilla path (vanilla-draw якорь)
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


def gate_verdict(tok, run_id):
    """None = ещё не решено; 'PASS' = гейт прошёл (бенч идёт); 'FAIL' = fast-fail."""
    jobs = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs?per_page=5")
    for j in jobs.get("jobs", []):
        for s in j.get("steps", []):
            if s.get("name") == GATE_STEP:
                c = s.get("conclusion")
                if c == "success":
                    return "PASS"
                if c in ("failure", "cancelled"):
                    return "FAIL"
    return None


def roll(tok, branch, dry=False):
    cur = ref_sha(tok, branch)
    if cur != PIN:
        if cur is None:
            api(tok, f"/repos/{REPO}/git/refs", method="POST",
                data={"ref": f"refs/heads/{branch}", "sha": PIN})  # FULL-sha (урок S20)
            print(f"ref CREATED {branch} @ {PIN[:8]}", flush=True)
        else:
            api(tok, f"/repos/{REPO}/git/refs/heads/{branch}", method="PATCH",
                data={"sha": PIN, "force": True})
            print(f"ref PATCHED {branch} -> {PIN[:8]}", flush=True)
    got = ref_sha(tok, branch)
    if got != PIN:
        raise SystemExit(f"POST-CREATE VERIFY FAIL {branch} (Л188a)")
    print(f"GET-verify OK {branch} object.sha == {got}", flush=True)

    if dry:
        print(f"DRY-INPUTS {branch}: " + json.dumps(INPUTS, sort_keys=True), flush=True)
        return None

    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": branch, "inputs": INPUTS})
    if r != {}:
        raise SystemExit(f"dispatch failed {branch}: {r}")
    print(f"dispatch 204-OK {branch}", flush=True)

    rid, hs = None, None
    deadline = time.time() + 300
    while time.time() < deadline and rid is None:
        time.sleep(10)
        for rid_, st, ca, hs_ in runs_on_branch(tok, branch):
            if hs_ == PIN:
                rid, hs = rid_, hs_
                break
    if rid is None:
        print(f"run-id not visible in 300s {branch} (204 принят)", flush=True)
        return {"branch": branch, "run_id": None, "verdict": "UNKNOWN"}
    print(f"RUN-ID {rid} ({branch})", flush=True)

    # Ждём вердикт band-гейта (fast-fail pre-download ~минута после старта job)
    deadline = time.time() + 900
    while time.time() < deadline:
        time.sleep(25)
        v = gate_verdict(tok, rid)
        if v == "PASS":
            print(f"GATE PASS {branch} run {rid} — бенч продолжается (hit)", flush=True)
            return {"branch": branch, "run_id": rid, "verdict": "HIT"}
        if v == "FAIL":
            print(f"GATE FAST-FAIL {branch} run {rid} — pairing discard", flush=True)
            return {"branch": branch, "run_id": rid, "verdict": "DISCARD"}
        st = api(tok, f"/repos/{REPO}/actions/runs/{rid}").get("status")
        if st == "completed":
            # ран завершился до фиксации шага в листинге — трактуем по conclusion
            concl = api(tok, f"/repos/{REPO}/actions/runs/{rid}").get("conclusion")
            print(f"run {rid} completed early conclusion={concl}", flush=True)
            return {"branch": branch, "run_id": rid,
                    "verdict": "DISCARD" if concl != "success" else "HIT"}
    print(f"gate verdict timeout 900s {branch} run {rid} — оставляем как in-flight", flush=True)
    return {"branch": branch, "run_id": rid, "verdict": "UNKNOWN"}


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args
    tok = token()

    live_master = ref_sha(tok, "master")
    print(f"origin/master live = {live_master} -> pin {PIN[:8]} (×486 учёт)", flush=True)

    for br in BRANCHES:
        if runs_on_branch(tok, br):
            raise SystemExit(f"RUN-SNAPSHOT DIRTY: {br} has runs")

    results = []
    for br in BRANCHES:
        res = roll(tok, br, dry=dry)
        if dry:
            continue
        results.append(res)
        json.dump({"pin": PIN, "inputs": INPUTS, "rolls": results},
                  open("/home/z/rounds/ROUND-487/c48_dispatch.json", "w"), indent=1)
        if res["verdict"] in ("HIT", "UNKNOWN"):
            break  # hit = бенч идёт; unknown = in-flight, ре-ролл не оправдан
        print(f"ре-ролл после дискард {br}", flush=True)

    if not dry:
        print("SUMMARY " + json.dumps(results), flush=True)


if __name__ == "__main__":
    main()
