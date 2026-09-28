#!/usr/bin/env python3
"""dispatch_483_a15_feed.py — COMMANDER A15 ROUND-483: STRICT-волна ×2 + pair-фид ×2.

CLAIM (board/CLM-A15.md, закон 14a/16): STRICT-волна ×2 (s1,s2) + рядь-фид ×2
(s3,s4) = 4 ваниль-драва @master acffa383. STRICT = пост-норм классификация
normtool (НЕ input); бимод 21.1% → E[хиты] ~0.8 на 4 драва; pair-пул расширение
burst-протокол ×481 (Л-481-BURST). 0 код-дельт, inputs = канон x466-C98
(burst73-ваниль). Band GLOB [6000000,9500000] fast-fail; band-miss → ре-ролл ≤2
(алиас -r2/-r3, закон W3/Л188c). 1 диспатч = 1 ветка (Л188b); refs FULL-sha
GET-verified (Л188a). Runs >15 мин → DISPATCHED run-id (закон 18-iii).

Usage: dispatch_483_a15_feed.py [--dry-run] | watch | poll
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "acffa3839b09a3388d4949d3767ab0a15b4fcd94"  # master ×482-учёт (BOTTLENECK.md)
BASE_CLAIM = "acffa383"
HOT = ("src/", ".github/", "Cargo", "pom", "native/")

ALIASES = [f"round-483-a15-s{i}" for i in range(1, 5)]
PAUSE_S = 15
RUNID_WAIT_S = 150          # окно появления run-id после диспатча (40-90с + запас)
BAND_DEAD_S = 240           # ранняя completed/failure = band-miss кандидат
IDLE_POLL_S = 15
GATE_HINT = ("band", "calibration")

# канон x466-C98 — ЯВНЫЙ JSON (идентичен VAN feed482/burst73); 0 дельт;
# STRICT = пост-норм классификация normtool, НЕ input
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
    else:
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


def latest_run(tok, branch, min_created):
    runs = api(tok, f"/repos/{REPO}/actions/runs?branch={branch}&per_page=20")
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] == branch and run["created_at"] > min_created:
            return {"id": run["id"], "status": run["status"],
                    "conclusion": run["conclusion"], "sha": run["head_sha"],
                    "created": run["created_at"]}
    return None


def dispatch_once(tok, ref, tag):
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    for attempt in (1, 2):  # 1 retry на transient (закон W3: только POST-фейл)
        r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
                method="POST", data={"ref": ref, "inputs": INPUTS})
        if r == {}:
            print(f"dispatch {ref} [{tag}] band[6000000,9500000] -> 204 OK", flush=True)
            return mc
        print(f"POST fail ({r}), retry in 20s", flush=True)
        time.sleep(20)
    sys.exit(2)


def get_run_id(tok, alias, mc, wait_s=RUNID_WAIT_S):
    """Л188-цикл: run-id появляется через 40-90с — поллим GET /actions/runs?branch="""
    deadline = time.time() + wait_s
    while time.time() < deadline:
        run = latest_run(tok, alias, mc)
        if run:
            print(f"{alias}: run {run['id']} status={run['status']}", flush=True)
            return run
        time.sleep(IDLE_POLL_S)
    print(f"{alias}: NO-RUN in {wait_s}s", flush=True)
    return None


def run_jobs(tok, run_id):
    return api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs?per_page=30").get("jobs", [])


def band_miss(tok, run):
    """Fast-fail-детектор: run completed(failure) рано + band-gate job failure."""
    if run["status"] != "completed":
        return False
    if run["conclusion"] != "failure":
        return False
    created = time.mktime(time.strptime(run["created"], "%Y-%m-%dT%H:%M:%SZ"))
    # updated_at недоступен в компакт-структуре — берём только ранние фейлы
    if time.time() - created > BAND_DEAD_S + 120:
        return False
    jobs = run_jobs(tok, run["id"])
    for j in jobs:
        name = j.get("name", "").lower()
        if any(h in name for h in GATE_HINT) and j.get("conclusion") == "failure":
            print(f"  band-gate job '{j['name']}' conclusion=failure", flush=True)
            return True
    return False


def do_dispatch(dry):
    tok = token()
    drift_check(tok)
    if dry:
        for a in ALIASES:
            ensure_alias(tok, a, PIN)
        print("DRY-RUN OK — refs GET-verified, 0 диспатчей")
        return
    results = {}
    for a in ALIASES:
        ensure_alias(tok, a, PIN)
        mc = dispatch_once(tok, a, "vanilla-STRICT/pair-feed")
        results[a] = {"alias": a, "min_created": mc}
        time.sleep(PAUSE_S)
    # фаза run-id: цикл по 4 (Л188b-рецепт, 40-90с на появление)
    for a in ALIASES:
        run = get_run_id(tok, a, results[a]["min_created"])
        results[a]["run"] = run or {"status": "POLL-NEEDED"}
    print("A15-DISPATCH-JSON " + json.dumps(
        {"pin": PIN, "inputs": INPUTS,
         "runs": {a: results[a].get("run", {}) for a in ALIASES}}, default=str))


def do_watch():
    """Band-watch + авто-ре-ролл (≤2 на ногу, алиас -r2/-r3) + poll до DISPATCHED."""
    tok = token()
    mc0 = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 900))
    legs = {a: {"alias": a, "roll": 0, "run": latest_run(tok, a, mc0)} for a in ALIASES}
    deadline = time.time() + BAND_DEAD_S * 2
    while time.time() < deadline:
        active = 0
        for a, L in legs.items():
            if L.get("dead") or L.get("band_miss"):
                continue
            run = L["run"]
            if not run:
                run = latest_run(tok, a, mc0)
                L["run"] = run
            if run and run["status"] == "completed":
                if band_miss(tok, run):
                    L["band_miss"] = True
                    print(f"{a}: BAND-MISS (fast-fail) run {run['id']}", flush=True)
                else:
                    L["done"] = True
                    print(f"{a}: completed run {run['id']} concl={run['conclusion']}", flush=True)
                continue
            active += 1
            if run:
                print(f"{a}: run {run['id']} status={run['status']}", flush=True)
        # ре-роллы для band-miss ног (≤2)
        for a, L in list(legs.items()):
            if L.get("band_miss") and L["roll"] < 2:
                L["roll"] += 1
                ra = f"{a}-r{L['roll'] + 1}"
                print(f"{a}: ре-ролл #{L['roll']} → {ra}", flush=True)
                ensure_alias(tok, ra, PIN)
                mc = dispatch_once(tok, ra, f"re-roll-{L['roll']}")
                L["band_miss"] = False
                L["run"] = None
                L["min_created"] = mc
                L["re_alias"] = ra
        if active == 0 and not any(
                L.get("band_miss") or (not L["run"]) or L.get("roll") for L in legs.values()):
            break
        time.sleep(IDLE_POLL_S * 2)
    print("A15-WATCH-JSON " + json.dumps(
        {a: {"run": L["run"], "roll": L["roll"], "re_alias": L.get("re_alias")}
         for a, L in legs.items()}, default=str))


def do_poll():
    tok = token()
    mc0 = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 7200))
    out = {}
    for a in ALIASES:
        r = latest_run(tok, a, mc0)
        out[a] = r or "NO-RUN"
        print(f"{a}: {json.dumps(r, default=str) if r else 'NO-RUN'}", flush=True)
    print("A15-POLL-JSON " + json.dumps(out, default=str))


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "poll":
        do_poll()
    elif len(sys.argv) > 1 and sys.argv[1] == "watch":
        do_watch()
    else:
        do_dispatch("--dry-run" in sys.argv)
