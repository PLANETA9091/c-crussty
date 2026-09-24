#!/usr/bin/env python3
"""dispatch_437a_sscan2.py — TASK-437-A banked dispatcher (sscan2 plane).

BANKED — ЗАПУСК ТОЛЬКО main (тик-437 или golden-слот 02:08 +08). Агентский
контекст диспатчить НЕ имеет права (дисциплина TASK-436-A).

5 диспатчей big-interleaved:
  - 2 anchors @master lever=""                (round-437a-anchor-1/2)
  - 3 ss     @round-437-a-sscan2 cmp436_sscan2 (round-437a-ss-1/2/3)

INPUTS = РОВНО банк inputs (radius 640, seconds 300, fake_players 4,
fluid_guard 1, gc_tune 3, inside_cache 1, flush_diet 1, fluid_dirty 0,
fluid_bitmask 0, region_threads 4, batch_collector 1, inside_bitmask 0,
skip_store_bb 0, region_steal 0, bu_defer 0, population_target 150000,
population_seed 42, server_xmx 10G, server_xms 4G, cpu_band_min 6000000,
cpu_band_max 9500000, lever_flag, lever_arg=1). БЕЗ travel_diet/fluid_dirty_
ledger, БЕЗ concurrency-гвардов (канон TASK-436 dispatch_437.py).

Прегист пары (CLAIMS TASK-437): ARM ("sscan armed despawn+spawn") + EFFECT
("spawn-scan EFFECT armed", "despawn-scan EFFECT armed") + epoch ok
("epoch ok" / "spawn epoch ok") + scanned:N + selfTest=true ДО ARM + Retargeted
sites=1 (обе полуплоскости) + DATA-PLAN (mobSlots>0) + AIOOBE=0 + NCDFE=0 +
band 6.0-9.5M + pair Δ≤50k + окно ≤±5пп (депресс-гейт) + min-of-3.
"""
import json, re, subprocess, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BASE_LEGS = "round-437-a-sscan2"

BATCH = [
    ("anchor-1", "round-437a-anchor-1", "master",           ""),
    ("ss-1",     "round-437a-ss-1",     BASE_LEGS,          "cmp436_sscan2"),
    ("anchor-2", "round-437a-anchor-2", "master",           ""),
    ("ss-2",     "round-437a-ss-2",     BASE_LEGS,          "cmp436_sscan2"),
    ("ss-3",     "round-437a-ss-3",     BASE_LEGS,          "cmp436_sscan2"),
]

INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
}


def token_from_remote():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token in origin remote URL")
    return m.group(1)


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
        print(f"HTTP {e.code}: {e.read()[:200]}")
        raise
    return json.loads(body) if body else {}


def sha_of(tok, ref):
    return api(tok, f"/repos/{REPO}/git/ref/heads/{ref}")["object"]["sha"]


def ensure_branch(tok, branch, base):
    try:
        return sha_of(tok, branch)
    except urllib.error.HTTPError:
        base_sha = sha_of(tok, base)
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{branch}", "sha": base_sha})
        return base_sha


if __name__ == "__main__":
    tok = token_from_remote()
    for leg, branch, base, lever in BATCH:
        sha = ensure_branch(tok, branch, base)
        inputs = dict(INPUTS)
        inputs["lever_flag"] = lever
        inputs["lever_arg"] = "1"
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": branch, "inputs": inputs})
        print(f"dispatched {leg}: branch={branch} base={base} lever='{lever}' sha={sha[:8]}", flush=True)
        time.sleep(4)
