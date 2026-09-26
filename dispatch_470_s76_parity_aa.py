#!/usr/bin/env python3
"""dispatch_470_s76_parity_aa.py — S76 [СТРЕСС-мир] — datapack-parity W1-канон-повтор A/A на №11-мастере (тик-470)
Две ваниль-ноги round-470-s76-aa1/aa2 = ref-push origin/master @b3853246 (МЕРЖ №11,
0 код-дельт, world_diff_parity_v2 ТЕПЕРЬ В МАСТЕРЕ), банк-v5 канон inputs.
Повтор канона S60 W1a/W1b A/A (697/697 чанков) на новом мастере: цель — подтверждение
parity-гейта после №11 (0 дельт ожидается). H-S76 (закон 14a/16, preregister):
обе ноги ваниль на идентичном носителе/векторе → P6 world_diff_parity_v2 лог-режим
должен дать WORLD-PARITY-OK 8/8 (0 FAIL); любой RED = дельта-хантинг на №11.
Канон-парность: |Δcpu|<=50k (Л195); CPU-BAND INFO-гейт. Кросс-чек: S15 aa1/aa2
(36269639744/36269641046) — независимая пара того же канона (drift-хантинг S79).
Usage: dispatch_470_s76_parity_aa.py [--dry-run]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
BRANCHES = ["round-470-s76-aa1", "round-470-s76-aa2"]
PIN_SHA = "b3853246"  # МЕРЖ №11 master head (parity-v2 в мастере)

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
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    if m:
        return m.group(1)
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code} {url}: {e.read()[:200]}")
        raise
    return json.loads(body) if body else {}


def sha_of(tok, ref):
    return api(tok, f"/repos/{REPO}/git/ref/heads/{ref}")["object"]["sha"]


def sha_of_commit(tok, sha):
    return api(tok, f"/repos/{REPO}/commits/{sha}")["sha"]


def is_ancestor(tok, base, head):
    cmp = api(tok, f"/repos/{REPO}/compare/{base[:12]}...{head[:12]}")
    # GitHub-семантика: status описывает head относительно base —
    # "ahead"/"identical" => base является предком head.
    return cmp.get("status") in ("ahead", "identical")


def ensure_branch(tok, branch, base_sha):
    """Л188a канон: прямой POST /git/refs + GET-верификация object.sha ДО dispatch."""
    try:
        live = sha_of(tok, branch)
        print(f"branch exists: {branch} @ {live[:8]}")
        return live
    except urllib.error.HTTPError:
        pass
    api(tok, f"/repos/{REPO}/git/refs", method="POST",
        data={"ref": f"refs/heads/{branch}", "sha": base_sha})
    live = sha_of(tok, branch)
    if not live.startswith(base_sha[:7]):
        raise SystemExit(f"SHA MISMATCH post-create: {branch} live={live} base={base_sha}")
    print(f"branch created+verified: {branch} @ {live[:8]}")
    return live


def poll_run_id(tok, branch, created_min, tries=10, delay=20):
    """Вернуть run id последнего workflow_dispatch рана ветки (создан после created_min)."""
    for i in range(tries):
        runs = api(tok, f"/repos/{REPO}/actions/runs?branch={branch}&event=workflow_dispatch&per_page=5"
                   ).get("workflow_runs", [])
        for r in runs:
            if r["head_sha"].startswith(PIN_SHA[:7]) and r["created_at"] >= created_min:
                return r["id"], r["status"]
        time.sleep(delay)
    return None, None


def main():
    args = sys.argv[1:]
    if any(a != "--dry-run" for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args
    tok = token()
    base = sha_of_commit(tok, PIN_SHA)  # база пар = МЕРЖ №11 head (как S15-пара)
    master = sha_of(tok, "master")
    if not is_ancestor(tok, base, master):
        raise SystemExit(f"MASTER ANCESTRY CHECK: master={master[:8]} base={base[:8]}")
    print(f"preflight OK: base @ {base[:8]} (МЕРЖ №11, parity-v2 в мастере; master HEAD {master[:8]})")
    print(f"inputs: {json.dumps(INPUTS, ensure_ascii=False)}")
    if dry:
        print("DRY-RUN OK — no dispatch")
        return
    stamp = time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 5))
    for br in BRANCHES:
        ensure_branch(tok, br, base)
        time.sleep(2)
    for br in BRANCHES:
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": INPUTS})
        print(f"dispatched: {br} (HTTP 204)")
    for br in BRANCHES:
        rid, st = poll_run_id(tok, br, stamp)
        print(f"run-id: {br} -> {rid} ({st})")


if __name__ == "__main__":
    main()
