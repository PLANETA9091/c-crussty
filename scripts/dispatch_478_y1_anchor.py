#!/usr/bin/env python3
"""dispatch_478_y1_anchor.py — COMMANDER 478-Y1: ваниль-якорь STRICT-зоны, мишень cpu ~7.0M.

CLAIM (банк §3 Л201-рефит): STRICT-зона [6.9,7.2]M, банк n=26/30, дефицит 4.
Задача: 1 ваниль-рун @master 0 код-дельт (алиас round-478-y1-anchor, pin FULL-sha
origin/master 386887a82f6647311a1d2005172507af09821256; дрейф 848d8f14→386887a8
docs/scripts-only верифицирован diff-name-only: 13 файлов, 0 src/Java/Rust),
canon x466-C98 ЯВНЫМ JSON (урок C66-C72), band GLOB [6000000,9500000] fast-fail;
norm A1-метод (polls-median C55, tps_exp interp BANK_V5_FREEZE §2, HOST-ценз M1
STW≤23/avg≤200); в-точка в банк §3, пороги НЕ двигать. Band-miss → 1 ре-ролл (закон W3).
Прегист закон 14a/16 — этот docstring; LEDGER Л-478-Y1.
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
ALIAS = "round-478-y1-anchor"
PIN = "386887a82f6647311a1d2005172507af09821256"  # origin/master FULL sha

# canon x466-C98 — ЯВНЫЙ JSON, yml-дефолты = merge-поверхность (урок C73)
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


def ensure_alias(tok, name, sha):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{name}")
    cur = r.get("object", {}).get("sha")
    if cur == sha:
        print(f"{name}: GET-proof exists @ {sha} (Л188a)")
        return
    if cur:
        api(tok, f"/repos/{REPO}/git/refs/heads/{name}", method="PATCH",
            data={"sha": sha, "force": True})
        print(f"{name}: moved -> {sha}")
        return
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{name}", "sha": sha})
    print(f"{name}: created FULL-sha @ {sha}")


def dispatch(tok, ref):
    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": ref, "inputs": INPUTS})
    print(f"dispatch {ref} -> {'204 OK' if r == {} else r}")
    return r == {}


def latest_run(tok, branch, min_created):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    for run in runs.get("workflow_runs", []):
        if run["head_branch"] == branch and run["created_at"] > min_created:
            return {"id": run["id"], "status": run["status"],
                    "conclusion": run["conclusion"], "sha": run["head_sha"],
                    "created": run["created_at"]}
    return None


def main():
    tok = token()
    # верификация пина против живого origin/master
    br = api(tok, f"/repos/{REPO}/branches/master")
    live = br.get("commit", {}).get("sha", "")
    print(f"origin/master live = {live}")
    pin = live if live.startswith(PIN[:12]) else PIN
    ensure_alias(tok, ALIAS, pin)
    mc = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 120))
    ok = dispatch(tok, ALIAS)
    if not ok:
        sys.exit(2)
    time.sleep(30)
    run = latest_run(tok, ALIAS, mc)
    print(json.dumps(run, indent=1))


if __name__ == "__main__":
    main()
