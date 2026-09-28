#!/usr/bin/env python3
"""dispatch_480_c30_a1.py — COMMANDER 480-C30: ваниль-якорь ×1 [7.2,7.5]M (тик ×480).

CLAIM-ПРЕГИСТЕР (закон 14a/16, зафиксирован ДО диспатча, v19.0 MEGA-SWARM):
  Миссия: ваниль-якорь ×1 в полосе [7.2, 7.5]M cpu — дополнение сэмпл-пула
  ×480 (drift-вектор C24: медианы ~6.8-6.96M, верхняя масса утекает в
  band-top; якорь 7.2-7.5M закрывает середину коридора gate→bench
  −163k/5.2мин, C48). Окна v5-FROZEN в ГЕЙТ НЕ СТАВЛЕНЫ — пост-хок
  cpu-фильтр run-env (Л-470-S31.1/Л195), норм-гейт пост-хок (закон 16).
  VANILLA-НОГА: lever_flag="" lever_arg="" — 0 код-дельт, чистота закона-5.

ПЛАН (1 реф=1 диспатч Л188b, GET-страж Л188a, FULL-sha S20):
  алиас round-480-c30-a1 → ref @ FULL-sha 686f2258 (origin/master live,
  ×479-консолидация), canon x466-C98 ЯВНЫМ JSON (640/300s/fp4/gc3/ic1/fd1/
  rt4/bc1/pop150k/seed42/10G/xms4G, lever=""), band GLOB [7200000,7500000]
  fast-fail; band-dead (fail ≤180s) → 1 ре-ролл a1-r2 (закон W3/Л188c).
  Runs >15 мин → DISPATCHED run-id (закон 12e/18-iii) — бенчи не ждать.

Usage: dispatch_480_c30_a1.py [--dry-run] | poll
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "686f225830a40570fd7dddcf77c2e1e64e4ecb88"  # origin/master ×479-консолидация, vanilla lever ∅
BASE_CLAIM = "686f2258"
HOT = ("src/", ".github/", "Cargo", "pom", "native/")

ALIAS = "round-480-c30-a1"
PAUSE_S = 15
BAND_DEAD_S = 180
COLLECT_TIMEOUT_S = 420
IDLE_POLL_S = 15

# канон x466-C98 — ЯВНЫЙ JSON (yml-дефолты = merge-поверхность, урок C73);
# окна пост-хок → НЕ в гейте (Л-470-S31.1); ваниль-якорь: lever ∅
INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "7200000", "cpu_band_max": "7500000",
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


def do_dispatch(dry):
    tok = token()
    live = drift_check(tok)
    ensure_alias(tok, ALIAS, live)
    if dry:
        print("DRY-RUN OK — ref GET-verified, 0 диспатчей")
        return
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ALIAS, "inputs": INPUTS})
    ok = (r == {})
    print(f"dispatch {ALIAS} band[{INPUTS['cpu_band_min']},{INPUTS['cpu_band_max']}] "
          f"-> {'204 OK' if ok else r}")
    if not ok:
        sys.exit(2)
    time.sleep(PAUSE_S)
    run = latest_run(tok, ALIAS, mc) or {"id": None, "status": "POLL-NEEDED", "created": mc}
    print(json.dumps({"alias": ALIAS, **run}))
    # сбор run-id / band-dead → 1 ре-ролл a1-r2 (W3/Л188c)
    deadline = time.time() + COLLECT_TIMEOUT_S
    while time.time() < deadline and run.get("status") != "completed":
        time.sleep(IDLE_POLL_S)
        fresh = latest_run(tok, ALIAS, mc)
        if fresh:
            run = fresh
            print(f"{ALIAS}: run {fresh['id']} status={fresh['status']} "
                  f"concl={fresh['conclusion']}")
    if run.get("conclusion") == "failure" and run.get("status") == "completed":
        rr = ALIAS + "-r2"
        print(f"{ALIAS}: BAND-DEAD fast-fail ≤{BAND_DEAD_S}s → ре-ролл {rr} (W3/Л188c)")
        ensure_alias(tok, rr, PIN)
        mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
        r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
                method="POST", data={"ref": rr, "inputs": INPUTS})
        if r == {}:
            time.sleep(PAUSE_S)
            fresh = latest_run(tok, rr, mc)
            print(json.dumps({"alias": rr, **(fresh or {"id": None, "status": "POLL-NEEDED"})}))
            run = fresh or run
    print("C30-DISPATCH-JSON " + json.dumps(
        {"pin": PIN[:12], "alias": ALIAS, "band": [7200000, 7500000],
         "window": "post-hoc (S31.1, не в гейте)", "runs": run}, default=str))


def do_poll():
    tok = token()
    for alias in (ALIAS, ALIAS + "-r2"):
        run = latest_run(tok, alias, "2026-09-28T00:00:00Z")
        if run:
            print(f"{alias}: run {run['id']} status={run['status']} "
                  f"concl={run['conclusion']}")
        else:
            print(f"{alias}: NO-RUN")


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "poll":
        do_poll()
    else:
        do_dispatch("--dry-run" in sys.argv)
