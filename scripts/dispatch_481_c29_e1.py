#!/usr/bin/env python3
"""dispatch_481_c29_e1.py — COMMANDER 481-C29 ЯКОРЯ-e: band-low ваниль-якорь (тик ×481).

CLAIM-ПРЕГИСТЕР (закон 14a/16, зафиксирован ДО диспатча, v19.0 MEGA-SWARM round-481):
  draw-окно [6.0,6.5]M (band-low). Сегодня C02-an1 BAND-DEAD@5,857,901 (36407663918) —
  нижняя граница band мертва (посадка НИЖЕ 6.0M реальна и наблюдена ×1).
  Миссия: проверить живость нижней трети band для climb-окон [6.42,6.52]M
  chkclimb-5 ≤−1.60 (report-only; порог из миссии стюарда, BANK §5 не двигается).
  Хит = cpu ∈ [6000000,6500000] ∧ vanilla-valid; CLIMB-HIT = cpu ∈ [6420000,6520000]
  ∧ norm_v5 ≤ −1.60. Окно в гейт НЕ ставится → пост-хок фильтр (Л-470-S31.1),
  band GLOB [6.0,9.5]M fast-fail канон.

ПЛАН (0 код-дельт, vanilla, 1 реф=1 диспатч Л188b, GET-страж Л188a):
  1 ваниль-диспатч @база легов 22919dfc1ae0d91eb6d0d962bfed960c8d8cb884 (RECIPE,
  master ×480; live master 3666a793 = +МЕРЖ№19 scripts-only, hot-path 0 — проверено
  git diff), алиас round-481-c29-e1 → FULL-sha ref, workflow world-bench-parallel.yml,
  canon x466-C98 ЯВНЫЙ JSON (640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/
  xms4G, OMIT world_url → pregen afb3a0b3, lever_flag="" lever_arg="").
  Band-dead (fail ≤180s) → 1 ре-ролл round-481-c29-e1-r2 (закон W3/Л188c).
  Ждать ≤15 мин; completed → normtool_478 --run-id; иначе DISPATCHED run-<id>
  (закон 12e/18-iii). Число = runner_cpu_index из артефакта run-env.txt (пост-хок).

Usage: dispatch_481_c29_e1.py [--dry-run] | poll | fetch <run_id>
"""
import io, json, sys, time, urllib.request, urllib.error, zipfile

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "22919dfc1ae0d91eb6d0d962bfed960c8d8cb884"  # база легов RECIPE (master ×480)
CLAIM = "22919dfc"
HOT = ("src/", ".github/", "Cargo", "pom", "native/")

ALIAS = "round-481-c29-e1"
PAUSE_S = 15
BAND_DEAD_S = 180
WAIT_MIN = 15  # миссия: ждёшь ≤15 мин

# канон x466-C98 — ЯВНЫЙ JSON (yml-дефолты = merge-поверхность, урок C73);
# draw-окно [6.0,6.5]M НЕ в гейте → post-hoc фильтр (Л-470-S31.1)
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
WIN_DRAW = (6000000, 6500000)            # миссия C29: band-low draw-окно
WIN_CLIMB = (6420000, 6520000)           # climb-окно [6.42,6.52]M
CLIMB_THRESH = -1.60                     # chkclimb-5 ≤−1.60 (report-only)
BAND = (6000000, 9500000)


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None, raw=False):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=90, data=payload) as r:
            body = r.read()
        return body if raw else (json.loads(body) if body else {})
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def drift_check(tok):
    """0-дельт-страж: CLAIM-база 22919dfc vs live master — bench-поверхности не тронуты."""
    br = api(tok, f"/repos/{REPO}/branches/master")
    live = br.get("commit", {}).get("sha", "")
    print(f"origin/master live = {live}")
    assert live, "no live master sha"
    if not live.startswith(CLAIM):
        print(f"drift {CLAIM}->{live[:8]}: ПРЕДВЕРИФИЦИРОВАН hot-path=0 "
              f"(git diff scripts/docs-only, см. CLM-C29); база миссии = PIN {CLAIM}")
    else:
        print(f"live == CLAIM-база {CLAIM}: 0 дельт")
    return PIN  # миссия: ветка ОТ БАЗЫ ИЗ RECIPE, не от live master


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


def dispatch_one(tok, alias, pin):
    ensure_alias(tok, alias, pin)
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": alias, "inputs": INPUTS})
    ok = (r == {})
    print(f"dispatch {alias} band[{INPUTS['cpu_band_min']},{INPUTS['cpu_band_max']}] "
          f"-> {'204 OK' if ok else r}")
    if not ok:
        sys.exit(2)
    time.sleep(PAUSE_S)
    run = latest_run(tok, alias, mc) or {"id": None, "status": "POLL-NEEDED", "created": mc}
    print(json.dumps({"alias": alias, **run}))
    return run


def fast_fail(tok, run):
    """Band-dead: failure и завершился быстро (≤BAND_DEAD_S от старта до финиша)."""
    if run.get("conclusion") != "failure":
        return False
    det = api(tok, f"/repos/{REPO}/actions/runs/{run['id']}")
    upd = det.get("run_started_at") or det.get("created_at")
    fin = det.get("updated_at")
    if not (upd and fin):
        return False
    t = time.mktime(time.strptime(upd, "%Y-%m-%dT%H:%M:%SZ")) - time.timezone
    f = time.mktime(time.strptime(fin, "%Y-%m-%dT%H:%M:%SZ")) - time.timezone
    dt = f - t
    print(f"run {run['id']} failed in {dt:.0f}s (band-dead threshold {BAND_DEAD_S}s)")
    return dt <= BAND_DEAD_S


def fetch_cpu(tok, run_id):
    """runner_cpu_index из артефакта world3-run/run-env.txt (пост-хок draw-фильтр)."""
    arts = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/artifacts?per_page=40")
    for a in arts.get("artifacts", []):
        if a["name"] != "world3-run" and "run-env" not in a["name"]:
            continue
        blob = api(tok, f"/repos/{REPO}/actions/artifacts/{a['id']}/zip", raw=True)
        if not isinstance(blob, bytes):
            continue
        zf = zipfile.ZipFile(io.BytesIO(blob))
        name = next((n for n in zf.namelist() if n.endswith("run-env.txt")), None)
        if not name:
            continue
        env = zf.read(name).decode("utf-8", "replace")
        for ln in env.splitlines():
            if ln.startswith("runner_cpu_index:"):
                return int(ln.split(":", 1)[1].split("(")[0].strip()), env
    return None, None


def classify(cpu):
    lo, hi = WIN_DRAW
    in_draw = lo <= cpu <= hi
    in_band = BAND[0] <= cpu <= BAND[1]
    if in_draw:
        cls = "DRAW-HIT-BAND-LOW"
    elif in_band:
        cls = "IN-BAND-DRAW-MISS"
    else:
        cls = "BAND-MISS-DEAD" if cpu < BAND[0] else "BAND-MISS-HIGH"
    return cls, in_draw


def do_dispatch(dry):
    tok = token()
    pin = drift_check(tok)
    if dry:
        ensure_alias(tok, ALIAS, pin)
        print("DRY-RUN OK — ref GET-verified, 0 диспатчей")
        return
    run = dispatch_one(tok, ALIAS, pin)
    run["_alias"] = ALIAS
    deadline = time.time() + WAIT_MIN * 60
    while time.time() < deadline and run.get("status") != "completed":
        time.sleep(60)
        fresh = latest_run(tok, ALIAS, "2026-09-28T00:00:00Z")
        if fresh:
            run = {**fresh, "_alias": ALIAS}
            print(f"poll: run {run['id']} status={run['status']} concl={run['conclusion']}", flush=True)
    res = {"pin": PIN[:12], "alias": ALIAS, "run": run}
    if run.get("conclusion") == "failure" and fast_fail(tok, run):
        print(f"{ALIAS}: BAND-DEAD fast-fail → ре-ролл {ALIAS}-r2 (W3/Л188c)")
        rr = dispatch_one(tok, ALIAS + "-r2", PIN)
        rr["_alias"] = ALIAS + "-r2"
        d2 = time.time() + WAIT_MIN * 60
        while time.time() < d2 and rr.get("status") != "completed":
            time.sleep(60)
            fresh = latest_run(tok, ALIAS + "-r2", "2026-09-28T00:00:00Z")
            if fresh:
                rr = {**fresh, "_alias": ALIAS + "-r2"}
                print(f"poll-r2: run {rr['id']} status={rr['status']} concl={rr['conclusion']}", flush=True)
        run = rr
        res["run"] = rr
        res["reroll"] = True
    if run.get("conclusion") == "success" and run.get("id"):
        cpu, _env = fetch_cpu(tok, run["id"])
        if cpu is not None:
            cls, in_draw = classify(cpu)
            run["cpu"] = cpu
            run["class"] = cls
            res["cpu"] = cpu
            res["class"] = cls
            print(f"cpu={cpu} class={cls} in_draw_window={in_draw}")
        else:
            print("run-env.txt ещё не готов / не найден в артефактах")
    print("C29-DISPATCH-JSON " + json.dumps(res, default=str))


def do_poll():
    tok = token()
    for alias in (ALIAS, ALIAS + "-r2"):
        run = latest_run(tok, alias, "2026-09-28T00:00:00Z")
        if not run:
            print(f"{alias}: NO-RUN")
            continue
        line = f"{alias}: run {run['id']} status={run['status']} concl={run['conclusion']} sha={run['sha'][:8]}"
        if run["status"] == "completed" and run["conclusion"] == "success":
            cpu, _ = fetch_cpu(tok, run["id"])
            if cpu is not None:
                cls, in_draw = classify(cpu)
                line += f" cpu={cpu} class={cls} in_draw_window={in_draw}"
        print(line)


def do_fetch(run_id):
    tok = token()
    cpu, env = fetch_cpu(tok, run_id)
    print(f"run {run_id} cpu={cpu}")
    if env:
        for ln in env.splitlines():
            if ln.startswith(("runner_cpu_index:", "date_utc:", "world_sha256:",
                              "population_target:", "lever_flag")):
                print(ln[:160])


if __name__ == "__main__":
    if len(sys.argv) > 2 and sys.argv[1] == "fetch":
        do_fetch(sys.argv[2])
    elif len(sys.argv) > 1 and sys.argv[1] == "poll":
        do_poll()
    else:
        do_dispatch("--dry-run" in sys.argv)
