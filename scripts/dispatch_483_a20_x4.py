#!/usr/bin/env python3
"""dispatch_483_a20_x4.py — COMMANDER 483-A20: РЯДЬ-ФИД ×4 (vanilla-канон).

CLAIM (Task 2-A20, тик ×483): 4 canon-ноги x466-C98 (0 код-дельт) → pair-пул банка v5,
burst-протокол ×481: банк 43/30 → цель 47/30. Все ноги идентичны канону.
PIN: acffa3839b09a3388d4949d3767ab0a15b4fcd94 (master ×482-учёт, BOTTLENECK.md).
1 диспатч = 1 ветка (Л188b). Алиасы: round-483-a20-v1..v4; ре-роллы -r2 (band-miss ≤2).
BAND: fast-fail "Runner calibration band gate" = band-miss; обе попытки ноги →
BAND-POOL-DEAD (не жечь бюджет). >15 мин = DISPATCHED run-id (закон 12e).
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "acffa3839b09a3388d4949d3767ab0a15b4fcd94"

# Канон x466-C98 ПОЛНЫЙ (задание 2-A20) — все 4 ноги идентичны
CANON_INPUTS = {
    "radius": "640", "seconds": "300",
    "fake_players": "4", "fluid_guard": "1", "gc_tune": "3",
    "inside_cache": "1", "flush_diet": "1", "fluid_dirty": "0",
    "fluid_bitmask": "0", "region_threads": "4", "batch_collector": "1",
    "inside_bitmask": "0", "skip_store_bb": "0", "region_steal": "0",
    "bu_defer": "0", "population_target": "150000", "population_seed": "42",
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
        try:
            msg = e.read()[:200].decode(errors="replace")
        except Exception:
            msg = ""
        print(f"HTTP {e.code} {url}: {msg}", file=sys.stderr)
        return {"_http_error": e.code}
    except urllib.error.URLError as e:
        print(f"URLERROR {url}: {e}", file=sys.stderr)
        return {"_http_error": "urlerror"}


def ensure_alias(tok, name, sha):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    cur = r.get("object", {}).get("sha")
    if cur == sha:
        print(f"{name}: GET-proof exists @ {cur[:12]}")
        return True
    if cur:
        print(f"{name}: EXISTS @ {cur[:12]} != PIN {sha[:12]} — NOT moving foreign branch")
        return False
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{name}", "sha": sha})
    v = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")  # GET-verify (Л188a)
    got = v.get("object", {}).get("sha", "")
    ok = got == sha
    print(f"{name}: created+verified {'OK' if ok else 'FAIL(' + got[:12] + ')'} @ {sha[:12]}")
    return ok


def dispatch(tok, ref, inputs):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": inputs})
    ok = r == {}
    print(f"dispatch {ref} -> {'204 OK' if ok else r}")
    return ok


def latest_run(tok, branch, min_created):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=60")
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] == branch and run["created_at"] > min_created:
            return {"id": run["id"], "status": run["status"],
                    "conclusion": run["conclusion"], "sha": run["head_sha"],
                    "created": run["created_at"]}
    return None


def job_facts(tok, run_id):
    """Jobs по ране: имена/выводы — детектор fast-fail 'Runner calibration band gate'."""
    jobs = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs?per_page=20")
    out = []
    for j in jobs.get("jobs", []):
        out.append({"name": j["name"], "status": j["status"], "conclusion": j["conclusion"],
                    "started": j.get("started_at"), "completed": j.get("completed_at")})
    return out


def band_miss(tok, run):
    """True, если ран fast-fail'нулся на 'Runner calibration band gate'."""
    if run["conclusion"] not in ("failure",):
        return False
    jobs = job_facts(tok, run["id"])
    if not jobs:
        return False
    # fast-fail: единственный(ые) job отработал failed/skipped с band-gate в имени
    names = " | ".join(j["name"] for j in jobs).lower()
    concls = [j["conclusion"] for j in jobs if j["conclusion"] not in (None, "skipped")]
    short = len([c for c in concls if c == "failure"]) <= 2 and all(
        (j["completed"] or "9") >= (j["started"] or "0") for j in jobs)
    hit = "calibration band" in names or "band gate" in names
    return hit or (short and not hit and len(jobs) <= 2 and concls and all(c == "failure" for c in concls))


def cmd_ensure(toks, args):
    tok = toks
    br = api(tok, "/repos/" + REPO + "/branches/master")
    live = br.get("commit", {}).get("sha", "")
    print(f"origin/master live = {live}")
    pin = live if live.startswith(PIN[:12]) else PIN
    aliases = args
    res = []
    for a in aliases:
        ok = ensure_alias(tok, a, pin)
        res.append({"alias": a, "ref_ok": ok})
    print("ENSURE-JSON " + json.dumps(res))
    return 0 if all(r["ref_ok"] for r in res) else 2


def cmd_dispatch(toks, args):
    tok = toks
    aliases = args
    res = []
    for a in aliases:
        mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
        ok = dispatch(tok, a, dict(CANON_INPUTS))
        res.append({"alias": a, "dispatched": ok, "min_created": mc})
        time.sleep(4)  # burst-протокол: короткая пауза против secondary-rate
    print("DISPATCH-JSON " + json.dumps(res))
    return 0 if all(r["dispatched"] for r in res) else 2


def cmd_poll(toks, args):
    tok = toks
    pairs = [a.split("@", 1) for a in args]  # alias@min_created
    pending = {a: mc for a, mc in pairs}
    found = {}
    deadline = time.time() + 420  # до 7 мин на ловлю run id (40-90с типично)
    while pending and time.time() < deadline:
        for a in list(pending):
            run = latest_run(tok, a, pending[a])
            if run:
                found[a] = run
                del pending[a]
                print(f"{a}: run {run['id']} status={run['status']} "
                      f"conclusion={run['conclusion']} created={run['created']} sha={run['sha'][:12]}")
        if pending:
            time.sleep(20)
    out = [{"alias": a, **found.get(a, {"id": None, "status": "NOT-FOUND"})} for a, _ in pairs]
    print("POLL-JSON " + json.dumps(out))
    return 0 if not pending else 3


def cmd_watch(toks, args):
    """watch alias@run_id ... — следит за финалом рана; band-miss → печать BAND-MISS."""
    tok = toks
    pairs = {a: rid for a, rid in (x.split("@", 1) for x in args)}
    finals = {}
    deadline = time.time() + int(ARGS.get("watch_secs", "540"))
    while pairs and time.time() < deadline:
        for a, rid in list(pairs.items()):
            r = api(tok, f"/repos/{REPO}/actions/runs/{rid}")
            st, cc = r.get("status", "?"), r.get("conclusion")
            if st == "completed":
                run = {"id": rid, "status": st, "conclusion": cc}
                bm = band_miss(tok, run) if cc == "failure" else False
                jobs = job_facts(tok, rid)
                print(f"{a}: COMPLETED conclusion={cc} band_miss={bm} "
                      f"jobs={json.dumps(jobs)[:400]}")
                finals[a] = {"run": run, "band_miss": bm}
                del pairs[a]
            else:
                print(f"{a}: {st} ({cc})")
        if pairs:
            time.sleep(45)
    for a, rid in pairs.items():
        finals[a] = {"run": {"id": rid, "status": "IN-PROGRESS>15min"}, "band_miss": None}
    print("WATCH-JSON " + json.dumps(finals))
    return 0


if __name__ == "__main__":
    cmd = sys.argv[1]
    ARGS = {}
    toks = token()
    if cmd == "ensure":
        sys.exit(cmd_ensure(toks, sys.argv[2:]))
    elif cmd == "dispatch":
        sys.exit(cmd_dispatch(toks, sys.argv[2:]))
    elif cmd == "poll":
        sys.exit(cmd_poll(toks, sys.argv[2:]))
    elif cmd == "watch":
        ARGS["watch_secs"] = sys.argv[2]
        sys.exit(cmd_watch(toks, sys.argv[3:]))
    else:
        print("usage: {ensure|dispatch|poll|watch} ...", file=sys.stderr)
        sys.exit(1)
