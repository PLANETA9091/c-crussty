#!/usr/bin/env python3
"""dispatch_481_c03_st.py — [481-C03] STRICT-БАНКЕР (тик ×481, банк 29→30/30).

CLAIM (пегист закон 14a/16, зафиксирован ДО диспатча):
  Миссия: STRICT-в-точка требует cpu_index ∈ [6.9M,7.2M] ∧ norm ∈ [−8.0,+1.5]
  ∧ M1 clean (STW_total ≤23.0s ∧ young_avg ≤200ms).
  Лотерея draw: 2 ваниль-диспатча (×2 лотерейных билета) на canon inputs
  x466-C98, lever_flag "" (ваниль, 0 код-дельт к базе 22919dfc),
  ветки round-481-c03-st1 / round-481-c03-st2 (1 диспатч = 1 ветка, Л188b).
  Ждём ≤15 мин; завершившиеся → normtool_478.py; незавершившиеся →
  финал DISPATCHED run-<id> (закон 12e/18-iii).

Вектор: canon x466-C98 ЯВНЫМ JSON, OMIT world_url → pregen afb3a0b3:
  640/300s/fp4/gc3/ic1/fd1/fd_dirty0/fd_bit0/rt4/bc1/ib0/ssbb0/rs0/bd0/
  pop150k/seed42/10G/xms4G, band [6.0,9.5]M fast-fail, lever_flag="".
ГЕЙТЫ (v5-FROZEN): band PASS; M1 STW_total≤23.0 ∧ young_avg≤200;
  NCDFE=0; AIOOBE=0 (биом-exempt Л-474-C88.2); FIXTURE-VALIDITY VALID;
  ваниль-коридор norm ∈ [−8.0,+1.5]; STRICT-зона cpu ∈ [6.9M,7.2M].
LEDGER: Л-481-C03.
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BASE = "22919dfc1ae0d91eb6d0d962bfed960c8d8cb884"  # master ×480 консолидация

BRANCHES = ["round-481-c03-st1", "round-481-c03-st2"]

# canon x466-C98 (RECIPE), OMIT world_url → pregen afb3a0b3; lever "" (ваниль)
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
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def ref_sha(tok, branch):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{branch}")
    return r.get("object", {}).get("sha") if "_http_error" not in r else None


def runs_on_branch(tok, br):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    if "_http_error" in runs:
        return []
    return [(r["id"], r.get("status"), r.get("created_at"), r.get("head_sha"))
            for r in runs.get("workflow_runs", []) if r.get("head_branch") == br]


def main():
    tok = token()

    live_master = ref_sha(tok, "master")
    pin = BASE
    print(f"origin/master live = {live_master} -> pin {pin[:8]}", flush=True)
    if live_master and live_master != pin:
        # база легов из RECIPE/BOTTLENECK — диспатчим строго от неё
        print("WARN: live master != BASE; пиним BASE по RECIPE (ветки от базы легов)", flush=True)

    rid_map = {}
    for br in BRANCHES:
        pre = runs_on_branch(tok, br)
        if pre:
            raise SystemExit(f"RUN-SNAPSHOT DIRTY: {br} has runs {pre}")
        cur = ref_sha(tok, br)
        if cur != pin:
            if cur is None:
                api(tok, "/repos/" + REPO + "/git/refs", method="POST",
                    data={"ref": "refs/heads/" + br, "sha": pin})  # FULL-sha (урок S20)
                print(f"ref CREATED {br} @ {pin[:8]}", flush=True)
            else:
                api(tok, f"/repos/{REPO}/git/refs/heads/{br}", method="PATCH",
                    data={"sha": pin, "force": True})
                print(f"ref PATCHED {br} -> {pin[:8]}", flush=True)
        got = ref_sha(tok, br)
        if got != pin:
            raise SystemExit(f"POST-CREATE VERIFY FAIL {br} (Л188a)")
        print(f"GET-verify OK {br} object.sha == {got[:8]}", flush=True)

        r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
                method="POST", data={"ref": br, "inputs": INPUTS})
        if r == {}:
            print(f"dispatch {br} 204-OK", flush=True)
        else:
            raise SystemExit(f"dispatch {br} failed: {r}")
        rid_map[br] = None

    # полл run-id до 4 мин на ветку
    deadline = time.time() + 240
    while time.time() < deadline and any(v is None for v in rid_map.values()):
        time.sleep(15)
        for br in BRANCHES:
            if rid_map[br] is not None:
                continue
            for i, st, ca, hs in runs_on_branch(tok, br):
                print(f"poll: {br} run {i} status={st} created={ca} sha={hs[:8]}", flush=True)
                if st in ("queued", "in_progress", "completed"):
                    rid_map[br] = i
                break

    print("C03-DISPATCH-JSON " + json.dumps(
        {"base": pin, "branches": BRANCHES, "run_ids": rid_map,
         "canon": "x466-C98 vanilla lever_flag=''", "strict_zone":
         "cpu[6.9M,7.2M] norm[-8.0,+1.5] M1 clean"}, indent=1), flush=True)


if __name__ == "__main__":
    main()
