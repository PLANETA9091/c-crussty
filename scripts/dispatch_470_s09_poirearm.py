#!/usr/bin/env python3
"""dispatch_470_s09_poirearm.py — ROUND-470 S09 (ЛАБ-poi) — POI re-arm leg.
S20-класс «забытая certified-плоскость под новым носителем» (Л201): POI-плоскость
(TASK-456-B, certified +14.53пп) молчит на всех пост-456-носителях — строгий-OR
гейт poi_plane::enabled() + PoiOps.leverEnabled заморожен на cmp453-эре.
Ветка round-470-s09-poirearm = master 40068dbe + cherry-pick 87c9ccf7
(re-arm cmp456_chunkmono/_p31snap/cmp466_c98ai + PoiOps census).
Канон диспатчера tick-466: argv-guard, sha-pin, canonical anchor inputs, band 6.0-9.5M,
ref-push = POST /git/refs + GET-верификация (Л188a).
Usage: dispatch_470_s09_poirearm.py [--dry-run] [--legs N]
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-470-s09-poirearm"
PIN_SHA = "9d02b133"  # round-470-s09-poirearm HEAD = 40068dbe + cherry 87c9ccf7 (1fe46c51) + board-docs 9d02b133

# Канонический anchor-сет (young-table corpus, band-канон [6.0,9.5]M):
# pop150k/seed42/fake4/r640/300s, gc_tune=6 (Л183 ценз-энаблер, пресет жив Л197),
# xmx10G/xms4G. lever_flag = пост-456-носитель cmp456_chunkmono_p31snap:
# на МАСТЕРЕ POI-плоскость под ним МОЛЧИТ (гейт cmp453-эры), на ветке — re-armed
# → пара leg(ветка) − carrier(мастер) = capture POI re-arm, прогноз +14.53пп (Л201).
INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "6", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "cmp456_chunkmono_p31snap", "lever_arg": "",
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
        if e.code != 404:
            print(f"HTTP {e.code}: {e.read()[:200]}")
        raise
    return json.loads(body) if body else {}


def sha_of(tok, ref):
    return api(tok, f"/repos/{REPO}/git/ref/heads/{ref}")["object"]["sha"]


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run", "--legs1", "--legs2") for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args
    legs = 2 if "--legs2" in args else 1

    tok = token()
    local = subprocess.run(["git", "-C", "/home/z/c-crussty", "rev-parse", "HEAD"],
                           capture_output=True, text=True).stdout.strip()
    if not local.startswith(PIN_SHA):
        raise SystemExit(f"LOCAL SHA MISMATCH: HEAD={local[:8]} pin={PIN_SHA}")
    # push локальную ветку (новая ветка = git push, НЕ PATCH-в-никуда),
    # затем GET-верификация object.sha (Л188a-канон).
    push = subprocess.run(["git", "-C", "/home/z/c-crussty", "push", "origin",
                           f"{BRANCH}:{BRANCH}"], capture_output=True, text=True)
    if push.returncode != 0:
        raise SystemExit(f"PUSH FAIL: {push.stderr[-400:]}")
    time.sleep(3)
    live = sha_of(tok, BRANCH)
    if not live.startswith(PIN_SHA[:7]):
        raise SystemExit(f"SHA MISMATCH: {BRANCH} live={live[:8]} pin={PIN_SHA}")
    print(f"preflight OK: {BRANCH} @ {live[:8]} (poi re-arm, cherry 87c9ccf7)", flush=True)
    print(f"inputs: {json.dumps(INPUTS, ensure_ascii=False)}", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatches", flush=True)
        return
    for i in range(legs):
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": BRANCH, "inputs": INPUTS})
        print(f"dispatched leg s09-poirearm-{chr(97+i)}: {BRANCH} "
              f"lever=cmp456_chunkmono_p31snap band=[6.0,9.5]M gc6", flush=True)
        time.sleep(4)
    print(f"=== ROUND-470 S09 BATCH COMPLETE: {legs} диспатчей ===", flush=True)


if __name__ == "__main__":
    main()
