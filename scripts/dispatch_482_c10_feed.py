#!/usr/bin/env python3
"""dispatch_482_c10_feed.py — COMMANDER C10: chkclimb-5 фиды ×3, зонд-draw рецепт C24 (тик ×482).

CLAIM-ПРЕГИСТЕР (закон 14a/16, board/CLM-C10.md): окна v5-FROZEN —
chkclimb-5 [6427199,6527199] norm ≤ −1.60, состояние 2/3 (3-й хит = КЛИМБ-кандидат).
Дрейф-канон REFUTED +650k → −108k/ч OLS (Л-481-C23/C24, R² 0.09), бимод-пул
(slow 80.5-81.5% / fast 18.5-19.5%), микрозон НЕТ (iid) → С55-сдвиги ОТВЕРГНУТЫ,
рецепт = зонд-draw C24: ваниль @master, band GLOB [6.0,9.5]M fast-fail, окна В ГЕЙТ
НЕ СТАВЛЕНЫ — post-hoc cpu-фильтр run-env (Л-470-S31.1/Л195).
N-расчёт: локальная ценз тика n=82 → in-window 1/82=1.22% × P(norm≤−1.60|valid)=35%
→ 0.43%/ран → E[хитов на ×3] ≈ 0.013, P(≥1) ≈ 1.3% (честно: редкое попадание).

Usage: dispatch_482_c10_feed.py [--dry-run] | poll
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "3666a7931703e24a36af4887c41a585918667b7a"  # master МЕРЖ №19 (×481-консолидация), vanilla lever ∅
BASE_CLAIM = "3666a793"
HOT = ("src/", ".github/", "Cargo", "pom", "native/")

ALIASES = [f"round-482-c10-w{i}" for i in range(1, 4)]
PAUSE_S = 15
BAND_DEAD_S = 180
COLLECT_TIMEOUT_S = 420
IDLE_POLL_S = 15

# канон x466-C98 — ЯВНЫЙ JSON (идентичен VAN feed482/burst73); окна chkclimb-5
# НЕ в гейте → post-hoc фильтр (Л-470-S31.1)
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
WIN_CHK5 = (6427199, 6527199, -1.60)


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


def dispatch_wave(tok, live_sha):
    results = {}
    for alias in ALIASES:
        ensure_alias(tok, alias, live_sha)
        mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
        r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
                method="POST", data={"ref": alias, "inputs": INPUTS})
        ok = (r == {})
        print(f"dispatch {alias} band[{INPUTS['cpu_band_min']},{INPUTS['cpu_band_max']}] "
              f"-> {'204 OK' if ok else r}", flush=True)
        if not ok:
            results[alias] = {"error": r}
            continue
        time.sleep(PAUSE_S)
        run = latest_run(tok, alias, mc)
        results[alias] = run or {"id": None, "status": "POLL-NEEDED", "created": mc}
        print(json.dumps({"alias": alias, **(run or {})}), flush=True)
    return results


def collect_and_reroll(tok, results):
    """Дожидаемся run-id/band-dead; band-dead (fail ≤180s) → 1 ре-ролл wN-r2 (W3)."""
    deadline = time.time() + COLLECT_TIMEOUT_S
    while time.time() < deadline:
        time.sleep(IDLE_POLL_S)
        for alias in list(results):
            run = results.get(alias) or {}
            if run.get("id") and run.get("status") == "completed":
                continue
            mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 900))
            fresh = latest_run(tok, alias, mc)
            if fresh:
                results[alias] = fresh
                print(f"{alias}: run {fresh['id']} status={fresh['status']} "
                      f"concl={fresh['conclusion']}", flush=True)
    for alias in ALIASES:
        run = results.get(alias) or {}
        if run.get("conclusion") == "failure" and run.get("status") == "completed":
            rr = alias + "-r2"
            print(f"{alias}: BAND-DEAD fast-fail → ре-ролл {rr} (W3/Л188c)")
            ensure_alias(tok, rr, PIN)
            mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
            r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
                    method="POST", data={"ref": rr, "inputs": INPUTS})
            if r == {}:
                time.sleep(PAUSE_S)
                fresh = latest_run(tok, rr, mc)
                results[rr] = fresh or {"id": None, "status": "POLL-NEEDED"}
                print(json.dumps({"alias": rr, **(fresh or {})}), flush=True)
    return results


def do_dispatch(dry):
    tok = token()
    live = drift_check(tok)
    if dry:
        for alias in ALIASES:
            ensure_alias(tok, alias, live)
        print("DRY-RUN OK — refs GET-verified, 0 диспатчей")
        return
    # МИССИЯ-пин: refs строго @ 3666a793 full (burst73-канон Л-481-BURST);
    # drift a169b951 docs/scripts-only проверен выше — bench-поверхность идентична.
    results = dispatch_wave(tok, PIN)
    results = collect_and_reroll(tok, results)
    print("C10-DISPATCH-JSON " + json.dumps(
        {"pin": PIN[:12], "window_chk5": WIN_CHK5,
         "runs": results}, default=str))


def do_poll():
    tok = token()
    for alias in ALIASES + [a + "-r2" for a in ALIASES]:
        run = latest_run(tok, alias, "2026-09-28T11:00:00Z")
        if run:
            print(f"{alias}: run {run['id']} status={run['status']} "
                  f"concl={run['conclusion']} sha={run.get('sha','')[:8]}")
        else:
            print(f"{alias}: NO-RUN")


if __name__ == "__main__":
    if len(sys.argv) > 1 and sys.argv[1] == "poll":
        do_poll()
    else:
        do_dispatch("--dry-run" in sys.argv)
