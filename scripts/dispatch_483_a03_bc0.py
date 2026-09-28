#!/usr/bin/env python3
"""dispatch_483_a03_bc0.py — COMMANDER A03 ROUND-483: WILD-bc0 ×2 реплика.

CLAIM (board/CLM-A03.md, закон 14a/16): Л-480-C47 bc0 = −0.25 на ×479-носителе
686f2258; повтор ×482-C47 на 3666a793 → вердикт-реплика тика ×483. МИССИЯ:
WILD-bc0 ×2 (параллельная реплика, 1 диспатч = 1 ветка Л188b) на master
acffa383 (учёт ×482). 0 код-дельт, алиасы round-483-a03-bc0a/-bc0b, inputs =
канон x466-C98 (burst73-ваниль), ЕДИНСТВЕННАЯ дельта batch_collector="0"
(канон bc1). Band [6000000,9500000] fast-fail job "Runner calibration band
gate": band-miss → ре-ролл ≤2 (алиас -r2/-r3, закон W3/Л188c). Норм =
absorb_483_main.process (канон-парсер банк v5). Runs >15 мин → DISPATCHED
run-id (закон 18-iii).

Usage: dispatch_483_a03_bc0.py [--dry-run] | poll [HH]
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "acffa3839b09a3388d4949d3767ab0a15b4fcd94"  # master учёт ×482 (BOTTLENECK)
BASE_CLAIM = "acffa383"
HOT = ("src/", ".github/", "Cargo", "pom", "native/")

ALIASES = ["round-483-a03-bc0a", "round-483-a03-bc0b"]
PAUSE_S = 15
COLLECT_TIMEOUT_S = 1800
IDLE_POLL_S = 30
MIN_CREATED = "2026-09-28T14:00:00Z"
BAND_JOB = "Runner calibration band gate"

# канон x466-C98 = burst73-ваниль ЯВНЫЙ JSON; ЕДИНСТВЕННАЯ дельта = batch_collector "0"
INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "0",         # A03-ось: bc0-изоляция (canon bc1)
    "inside_bitmask": "0", "skip_store_bb": "0",
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
    """0-дельт-страж: CLAIM-база acffa383 → live master не трогает bench-поверхности."""
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
    v = api(tok, f"/repos/{REPO}/git/ref/heads/{name}").get("object", {}).get("sha")
    assert v == sha, f"GET-verify fail {name}: {v}"
    print(f"{name}: FULL-sha GET-verified @ {sha[:8]}")


def latest_run(tok, branch, min_created=MIN_CREATED):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=40")
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] == branch and run["created_at"] > min_created:
            return {"id": run["id"], "status": run["status"],
                    "conclusion": run["conclusion"], "sha": run["head_sha"],
                    "created": run["created_at"]}
    return None


def band_miss(tok, run_id):
    """band-miss детектор: job BAND_JOB failure = runner вне [6.0,9.5]M (Л188c)."""
    jobs = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs?per_page=20")
    for job in jobs.get("jobs", []):
        if BAND_JOB.lower() in job["name"].lower() and job["conclusion"] == "failure":
            return True
    return False


def dispatch_once(tok, ref):
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    for attempt in (1, 2):  # 1 retry на transient (закон W3: только POST-фейл)
        r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
                method="POST", data={"ref": ref, "inputs": INPUTS})
        if r == {}:
            print(f"dispatch {ref} bc={INPUTS['batch_collector']} "
                  f"band[{INPUTS['cpu_band_min']},{INPUTS['cpu_band_max']}] -> 204 OK")
            return mc
        print(f"POST fail ({r}), retry in 20s", flush=True)
        time.sleep(20)
    sys.exit(2)


def do_dispatch(dry):
    tok = token()
    live = drift_check(tok)
    for alias in ALIASES:
        ensure_alias(tok, alias, PIN)
    if dry:
        print("DRY-RUN OK — refs GET-verified, 0 диспатчей")
        return
    out = []
    for alias in ALIASES:  # 1 диспатч = 1 ветка (Л188b)
        dispatch_once(tok, alias)
        out.append(alias)
    time.sleep(PAUSE_S)
    runs = {a: latest_run(tok, a) for a in out}
    print(json.dumps({"aliases": out, "runs": runs}), flush=True)
    print("A03-DISPATCH-JSON " + json.dumps(
        {"aliases": out, "pin": PIN, "bc": "0", "runs": runs or {}, "inputs": INPUTS}))


def do_poll(hh):
    """Poll до completed; band-miss → авто-ре-ролл ≤2 (-r2/-r3); completion → финальный JSON."""
    tok = token()
    deadline = time.time() + COLLECT_TIMEOUT_S
    rerolled = set()
    results = {}
    active = {a: (a, None) for a in ALIASES}  # alias -> (branch, run_id)
    while time.time() < deadline and active:
        for alias in list(active):
            branch = active[alias][0]
            run = latest_run(tok, branch)
            if not run:
                print(f"{branch}: NO-RUN yet", flush=True)
                continue
            active[alias] = (branch, run["id"])
            print(f"{branch}: run {run['id']} status={run['status']} "
                  f"concl={run['conclusion']}", flush=True)
            if run["status"] != "completed":
                continue
            if run["conclusion"] == "failure" and alias not in rerolled and \
               band_miss(tok, run["id"]):
                rerolled.add(alias)
                tail = "-r2" if alias.count("-r") - alias.count("round") == 0 or \
                       not alias.endswith(("-r2", "-r3")) else "-r3"
                new_branch = branch + tail
                print(f"BAND-MISS {branch} (run {run['id']}) -> ре-ролл {new_branch}", flush=True)
                ensure_alias(tok, new_branch, PIN)
                dispatch_once(tok, new_branch)
                active[alias] = (new_branch, None)
                continue
            results[alias] = run
            del active[alias]
        time.sleep(IDLE_POLL_S)
    print("A03-POLL-JSON " + json.dumps(
        {"results": results, "active": {k: v[0] for k, v in active.items()},
         "rerolled": sorted(rerolled)}))
    for alias, run in results.items():
        print(f"FINAL {alias}: run {run['id']} concl={run['conclusion']}")
    for alias, (branch, _) in active.items():
        print(f"DISPATCHED {branch} (закон 18-iii, >15 мин = run-id в финале)")


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "poll":
        do_poll(sys.argv[2] if len(sys.argv) > 2 else None)
    else:
        do_dispatch("--dry-run" in sys.argv)
