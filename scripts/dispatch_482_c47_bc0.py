#!/usr/bin/env python3
"""dispatch_482_c47_bc0.py — COMMANDER C47 ROUND-482: WILD-bc0 прецедент-повтор.

CLAIM (board/CLM-C47.md, закон 14a/16): Л-480-C47 bc0 = −0.25 на носителе ×479
686f2258 (forEach-сбор суб-шум, НЕ банк-фид). МИССИЯ тика ×482: тот же
WILD-bc0 ×1 на НОВОМ master 3666a793 (МЕРЖ №19 c98ai-компо) — «−0.25 стабилен
или дрейфует на compo-носителе?». 0 код-дельт, алиас round-482-c47-bc0,
inputs = burst73-ваниль (канон x466-C98, идентичен VAN feed482/burst73 —
Л-481-BURST) НО batch_collector="0". Band GLOB [6000000,9500000] fast-fail,
band-miss → 1 ре-ролл (закон W3/Л188c). Норм = absorb_482_main.process
(канон-парсер банк v5). Runs >15 мин → DISPATCHED run-id (закон 18-iii).

Usage: dispatch_482_c47_bc0.py [--dry-run] | poll
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "3666a7931703e24a36af4887c41a585918667b7a"  # new-master МЕРЖ №19 (compo-носитель)
BASE_CLAIM = "3666a793"
HOT = ("src/", ".github/", "Cargo", "pom", "native/")

ALIAS = "round-482-c47-bc0"
PAUSE_S = 15
COLLECT_TIMEOUT_S = 1800
IDLE_POLL_S = 30

# канон x466-C98 = burst73-ваниль ЯВНЫЙ JSON; ЕДИНСТВЕННАЯ дельта = batch_collector "0"
INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "0",         # C47-ось: bc0-изоляция (canon bc1)
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
    """0-дельт-страж: CLAIM-база 3666a793 → live master не трогает bench-поверхности."""
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


def latest_run(tok, branch, min_created):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=40")
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] == branch and run["created_at"] > min_created:
            return {"id": run["id"], "status": run["status"],
                    "conclusion": run["conclusion"], "sha": run["head_sha"],
                    "created": run["created_at"]}
    return None


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
    if dry:
        ensure_alias(tok, ALIAS, PIN)
        print("DRY-RUN OK — ref GET-verified, 0 диспатчей")
        return
    # МИССИЯ-пин: refs строго @ 3666a793 full (new-master compo-носитель)
    ensure_alias(tok, ALIAS, PIN)
    mc = dispatch_once(tok, ALIAS)
    time.sleep(PAUSE_S)
    run = latest_run(tok, ALIAS, mc)
    print(json.dumps({"alias": ALIAS, **(run or {"status": "POLL-NEEDED"})}), flush=True)
    print("C47-DISPATCH-JSON " + json.dumps(
        {"alias": ALIAS, "pin": PIN, "bc": "0", "run": run or {}, "inputs": INPUTS}))


def do_poll():
    """Poll до completed; completion → финальный JSON (норм делает norm_482_c47_bc0.py)."""
    tok = token()
    deadline = time.time() + COLLECT_TIMEOUT_S
    while time.time() < deadline:
        run = latest_run(tok, ALIAS, "2026-09-28T11:00:00Z")
        if run:
            print(f"{ALIAS}: run {run['id']} status={run['status']} "
                  f"concl={run['conclusion']} sha={run.get('sha', '')[:8]}", flush=True)
            if run["status"] == "completed":
                print("C47-POLL-JSON " + json.dumps(run))
                return
        else:
            print(f"{ALIAS}: NO-RUN yet", flush=True)
        time.sleep(IDLE_POLL_S)
    print("C47-POLL-TIMEOUT (закон 18-iii: DISPATCHED run-id)")


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "poll":
        do_poll()
    else:
        do_dispatch("--dry-run" in sys.argv)
