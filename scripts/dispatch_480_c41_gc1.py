#!/usr/bin/env python3
"""dispatch_480_c41_gc1.py — COMMANDER 480-C41: gc1-стресс ячейка ×1, young-стена 638ev-класс.

CLAIM-ПРЕГИСТЕР (закон 14a/16, /home/z/rounds/ROUND-480/c41/PREREG_480_C41_GC1.md):
A4 ×478 якорь run 36358029075 — gc1 638ev/44.1s young-стена (pause-target-артефакт,
G1 MaxGCPauseMillis=40) vs gc2 151ev/19.4s; gc1-класс волн недобирал точек. Прогноз:
young ev ∈ [500,750] (гейт ≥400), ΣSTW [38,50]s, M1-гейт young≤200 ПРОГНОЗ FAIL
(FAIL = подтверждение gc1-класса; young ≤200 → REFUTED_CENS). Band GLOB [6.0,9.5]M
fast-fail; band-miss → 1 ре-ролл -r2 (W3/Л188c). Runs >15 мин → DISPATCHED run-id
(закон 18-iii/12e), абсорб ×481.

Usage: dispatch_480_c41_gc1.py [--dry-run] | poll
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # origin/master ×479-консолидация, vanilla lever ∅
BASE_CLAIM = "686f2258"

ALIAS = "round-480-c41-gc1"
REROLL = "round-480-c41-gc1-r2"
PAUSE_S = 15
BAND_DEAD_S = 180          # fast-fail окно band-dead (W3/Л188c)
COLLECT_TIMEOUT_S = 3300   # ~55 мин: bench 300s + boot + inject + teardown
IDLE_POLL_S = 30

# канон x466-C98 — ЯВНЫЙ JSON (yml-дефолты = merge-поверхность, урок C73);
# ЕДИНСТВЕННАЯ дельта от канона: gc_tune "3"→"1" (G1 pause-target-артефакт)
INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "1", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
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
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def drift_check(tok):
    """0-дельт-страж: live master == CLAIM-база 686f2258 (vanilla-зонд)."""
    br = api(tok, f"/repos/{REPO}/branches/master")
    live = br.get("commit", {}).get("sha", "")
    print(f"origin/master live = {live}")
    assert live, "no live master sha"
    assert live == PIN, f"master moved: {live} != PIN {PIN} — диспатч отменён"
    print(f"live == CLAIM-база {BASE_CLAIM}: 0 дельт")
    return live


def ensure_alias(tok, name, sha):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    cur = r.get("object", {}).get("sha")
    if cur == sha:
        print(f"{name}: GET-proof exists @ {sha[:8]} (Л188a)")
        return
    if cur:
        api(tok, f"/repos/{REPO}/git/refs/heads/{name}", method="PATCH",
            data={"sha": sha, "force": True})
        print(f"{name}: moved -> {sha[:8]}")
    else:
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{name}", "sha": sha})  # FULL sha (S20: short=422)
        print(f"{name}: created FULL-sha @ {sha[:8]}")
    v = api(tok, f"/repos/{REPO}/git/ref/heads/{name}").get("object", {}).get("sha")
    assert v == sha, f"GET-verify fail {name}: {v}"


def latest_run(tok, branch, min_created):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=40")
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] == branch and run["created_at"] > min_created:
            return {"id": run["id"], "status": run["status"],
                    "conclusion": run["conclusion"], "sha": run["head_sha"],
                    "created": run["created_at"]}
    return None


def fire(tok, alias, sha):
    ensure_alias(tok, alias, sha)
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": alias, "inputs": INPUTS})
    ok = (r == {})
    print(f"dispatch {alias} gc_tune={INPUTS['gc_tune']} "
          f"band[{INPUTS['cpu_band_min']},{INPUTS['cpu_band_max']}] -> "
          f"{'204 OK' if ok else r}")
    if not ok:
        time.sleep(20)
        r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
                method="POST", data={"ref": alias, "inputs": INPUTS})
        ok = (r == {})
        print(f"retry dispatch {alias} -> {'204 OK' if ok else r}")
    assert ok, f"dispatch failed ×2 ({alias})"
    time.sleep(PAUSE_S)
    run = latest_run(tok, alias, mc) or {"id": None, "status": "POLL-NEEDED", "created": mc}
    print(json.dumps({"alias": alias, **run}))
    return run


def wait_run(tok, alias, run, deadline_s):
    end = time.time() + deadline_s
    try:  # строгое `>` в latest_run: берём запас 300s до created
        base = time.mktime(time.strptime(run.get("created") or "", "%Y-%m-%dT%H:%M:%SZ"))
    except Exception:
        base = time.time()
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(base - 300))
    while time.time() < end:
        time.sleep(IDLE_POLL_S)
        fresh = latest_run(tok, alias, mc)
        if fresh:
            run = fresh
            print(f"{alias}: run {fresh['id']} status={fresh['status']} "
                  f"concl={fresh['conclusion']}")
            if fresh.get("status") == "completed":
                return run
        else:
            print(f"{alias}: no run yet, still polling")
    return run


def do_dispatch(dry):
    tok = token()
    live = drift_check(tok)
    if dry:
        ensure_alias(tok, ALIAS, live)
        print("DRY-RUN OK — ref GET-verified, 0 диспатчей")
        return
    run = fire(tok, ALIAS, live)
    # band-dead детект: fail в ≤BAND_DEAD_S от старта = free discard → 1 ре-ролл (W3)
    created = run.get("created")
    if run.get("id") is None:
        run = wait_run(tok, ALIAS, run, 600)
    if run.get("conclusion") == "failure" and run.get("status") == "completed":
        fast = (run.get("id") and
                (time.time() - _ts(run.get("created", ""))) < BAND_DEAD_S + 300)
        print(f"{ALIAS}: FAILURE (fast={fast}) → BAND-DEAD ре-ролл {REROLL} (W3/Л188c)")
        run2 = fire(tok, REROLL, PIN)
        run2 = wait_run(tok, REROLL, run2, COLLECT_TIMEOUT_S)
        print("C41-DISPATCH-JSON " + json.dumps(
            {"pin": PIN[:12], "primary": run, "reroll": run2,
             "inputs": {"gc_tune": INPUTS["gc_tune"]}}, default=str))
        return
    run = wait_run(tok, ALIAS, run, COLLECT_TIMEOUT_S)
    print("C41-DISPATCH-JSON " + json.dumps(
        {"pin": PIN[:12], "run": run,
         "inputs": {"gc_tune": INPUTS["gc_tune"]}}, default=str))


def _ts(s):
    try:
        return time.mktime(time.strptime(s, "%Y-%m-%dT%H:%M:%SZ")) - time.timezone
    except Exception:
        return time.time()


def do_poll():
    tok = token()
    for alias in (ALIAS, REROLL):
        run = latest_run(tok, alias, "2026-09-28T00:00:00Z")
        print(f"{alias}: " + (f"run {run['id']} status={run['status']} "
              f"concl={run['conclusion']}" if run else "NO-RUN"))


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "poll":
        do_poll()
    else:
        do_dispatch("--dry-run" in sys.argv)
