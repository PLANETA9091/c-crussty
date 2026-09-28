#!/usr/bin/env python3
"""dispatch_479_y1_anchor.py — COMMANDER 479-Y1: ваниль-якорь STRICT-зоны, мишень cpu ~7.0M.

CLAIM (банк §3 Л-479-A4): STRICT-зона [6.9,7.2]M (Л201-узел 2.1293), банк n=27/30,
дефицит 3. Задача: 1 ваниль-рун @master 0 код-дельт (алиас round-479-y1-anchor,
pin FULL-sha origin/master ed705c5df431c54a84e9899baa3ef8f06fe88cb4; дрейф
c10e6395→ed705c5d docs/scripts-only верифицирован diff-name-only: 5 файлов, 0
src/Java/Rust), canon x466-C98 ЯВНЫМ JSON (урок C66-C72), band GLOB
[6000000,9500000] fast-fail; norm A1-метод (polls-median C55, tps_exp interp
BANK_V5_FREEZE §2, HOST-ценз M1 STW≤23/avg≤200, гейт m1_clean [479-G1] в
normtool_478); STRICT-hit ⇔ cpu(run-env) ∈ [6.9,7.2]M ∧ CLEAN ∧ ваниль-VALID ∧
norm ∈ ваниль-коридор [−8.0,+1.5] (Л143) → банк n→28; CLEAN in-band вне STRICT →
в-точка банк §3 (банк-фид полным band, пост-хок idx-фильтр — канон Л-470-S31.1);
пороги НЕ двигать. Band-miss → 1 ре-ролл (закон W3).
Прегист закон 14a/16 — этот docstring; LEDGER Л-479-Y1.
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
ALIAS = "round-479-y1-anchor"
PIN = "ed705c5df431c54a84e9899baa3ef8f06fe88cb4"  # origin/master FULL sha
BASE_CLAIM = "c10e6395"  # база из CLAIM командира — дрейф до PIN проверяем

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


def drift_check(tok, pin):
    """0-дельт-страж: diff-stat BASE_CLAIM→pin не должен трогать bench-поверхности."""
    br = api(tok, f"/repos/{REPO}/branches/master")
    live = br.get("commit", {}).get("sha", "")
    print(f"origin/master live = {live}")
    if live.startswith(PIN[:12]):
        pin = live
    stat = subprocess.run(
        ["git", "-C", "/home/z/c-crussty", "diff", "--name-only",
         BASE_CLAIM, pin[:12]],
        capture_output=True, text=True).stdout
    hot = [ln for ln in stat.splitlines()
           if ("src/" in ln or ".github/" in ln or "Cargo" in ln or "pom" in ln
               or "native/" in ln)]
    if hot:
        print("DRIFT-HOT files (bench surface touched!):", *hot, sep="\n", file=sys.stderr)
        sys.exit(3)
    print(f"drift {BASE_CLAIM}->{pin[:8]}: docs/scripts-only OK "
          f"({len(stat.strip().splitlines()) if stat.strip() else 0} files)")
    return pin


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
        data={"ref": f"refs/heads/{name}", "sha": sha})  # FULL sha (S20: short=422)
    # GET-верификация ДО диспатча (Л188a)
    v = api(tok, f"/repos/{REPO}/git/ref/heads/{name}").get("object", {}).get("sha")
    assert v == sha, f"alias verify failed: {v} != {sha}"
    print(f"{name}: created FULL-sha @ {sha}, GET-verified")


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
    pin = drift_check(tok, PIN)
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
