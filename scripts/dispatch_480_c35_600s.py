#!/usr/bin/env python3
"""dispatch_480_c35_600s.py — COMMANDER 480-C35: стресс-плоскость 19a, чистая ячейка-повтор 600s/640.

CLAIM-ПРЕГИСТЕР (закон 14a/16, зафиксирован ДО диспатча, v19.0 MEGA-SWARM round-480):
  W7-базовая 600s/640: run 36369278270 (gc3, vanilla), cpu_index 7022396,
  norm_v5 +17.64, плато 2.6 достигнуто к poll 9-11 ≈ 450-550s → t_stab [250,500]s
  подтверждён (канон 300s undersamples — A4/W7 консистентно). Ячейка-повтор = 3-я
  точка 600s-лестницы (t_stab-лестница: 300s < t_stab < 600s-плато) — стабилизирует
  ступень против host-дрейфа. 0 код-дельт: vanilla @686f2258 (×479 MAIN-консолидация).

ПЛАН (1 реф=1 диспатч Л188b, GET-страж Л188a): алиас round-480-c35-600s → refs
  @ FULL-sha 686f225830a40570fd7dddcf77c2e1e64e4ecb88, canon x466-C98 ЯВНЫЙ JSON
  (640/600s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/xms4G, lever=""), band GLOB
  [6000000,9500000] fast-fail. Band-miss (fail ≤180s) → 1 ре-ролл round-480-c35-600s-r2
  (закон W3). Runs >15 мин → DISPATCHED run-id (закон 12e/18-iii) — бенчи не ждать,
  число = cpu/norm пост-хок на абсорбе.

Usage: dispatch_480_c35_600s.py [--dry-run] | poll
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # origin/master ×479-консолидация, vanilla lever ∅
BASE_CLAIM = "686f2258"
HOT = ("src/", ".github/", "Cargo", "pom", "native/")

ALIAS = "round-480-c35-600s"
REROLL = ALIAS + "-r2"
PAUSE_S = 20
BAND_DEAD_S = 180
BAND_PROOF_S = 240      # in_progress после этого = band-гейт пройден (C24: band-dead пал на 39-41s)
COLLECT_TIMEOUT_S = 330

# канон x466-C98 — ЯВНЫЙ JSON (yml-дефолты = merge-поверхность, урок x466-C73);
# дельта от 300s-канона ТОЛЬКО seconds=600 (стресс-ячейка W7-класса)
INPUTS = {
    "radius": "640", "seconds": "600", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
}
BASE_W7 = {"run": 36369278270, "cpu_index": 7022396, "norm_v5": 17.64, "t_stab": "[250,500]s"}


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
    """0-дельт-страж: CLAIM-база 686f2258 → live master не трогает bench-поверхности."""
    br = api(tok, f"/repos/{REPO}/branches/master")
    live = br.get("commit", {}).get("sha", "")
    print(f"origin/master live = {live}")
    assert live, "no live master sha"
    if not live.startswith(BASE_CLAIM):
        stat = subprocess.run(
            ["git", "-C", "/home/z/c-crussty", "diff", "--name-only",
             BASE_CLAIM, live[:12]], capture_output=True, text=True).stdout
        hot = [ln for ln in stat.splitlines() if any(h in ln for h in HOT)]
        if hot:
            print("DRIFT-HOT (bench surface touched):", *hot, sep="\n", file=sys.stderr)
            sys.exit(3)
        print(f"drift {BASE_CLAIM}->{live[:8]}: docs/scripts-only OK")
        return live
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


def dispatch_one(tok, ref):
    ensure_alias(tok, ref, PIN)
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": INPUTS})
    ok = (r == {})
    print(f"dispatch {ref} seconds={INPUTS['seconds']} "
          f"band[{INPUTS['cpu_band_min']},{INPUTS['cpu_band_max']}] -> {'204 OK' if ok else r}")
    if not ok:
        sys.exit(2)
    time.sleep(PAUSE_S)
    run = latest_run(tok, ref, mc) or {"id": None, "status": "POLL-NEEDED", "created": mc}
    print(json.dumps({"alias": ref, **run}))
    return run


def band_watch(tok, ref, run):
    """Ждём band-вердикт: failure ≤180s = band-dead (→ ре-ролл W3); in_progress ≥
    BAND_PROOF_S = band-гейт пройден, бенч в полёте → DISPATCHED (закон 12e/18-iii)."""
    t0 = time.time()
    while time.time() - t0 < COLLECT_TIMEOUT_S:
        if run.get("id") and run.get("status") == "completed":
            return run
        time.sleep(30)
        fresh = latest_run(tok, ref, time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 1200)))
        if fresh:
            run = fresh
            el = int(time.time() - t0)
            print(f"{ref}: run {run['id']} status={run['status']} concl={run['conclusion']} (+{el}s)")
            if run["status"] == "completed":
                return run
    return run


def do_dispatch(dry):
    tok = token()
    live = drift_check(tok)
    if dry:
        ensure_alias(tok, ALIAS, live)
        print("DRY-RUN OK — ref GET-verified, 0 диспатчей")
        return
    run = dispatch_one(tok, ALIAS)
    run = band_watch(tok, ALIAS, run)
    if run.get("conclusion") == "failure":
        print(f"{ALIAS}: BAND-MISS fast-fail → 1 ре-ролл {REROLL} (закон W3)")
        rr = dispatch_one(tok, REROLL)
        rr = band_watch(tok, REROLL, rr)
        run = {REROLL: rr}
    print("C35-DISPATCH-JSON " + json.dumps(
        {"pin": PIN[:12], "seconds": INPUTS["seconds"], "base_w7": BASE_W7,
         "runs": {ALIAS: run}}, default=str))


def do_poll():
    tok = token()
    for ref in (ALIAS, REROLL):
        run = latest_run(tok, ref, "2026-09-28T00:00:00Z")
        if run:
            print(f"{ref}: run {run['id']} status={run['status']} concl={run['conclusion']}")
        else:
            print(f"{ref}: NO-RUN")


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "poll":
        do_poll()
    else:
        do_dispatch("--dry-run" in sys.argv)
