#!/usr/bin/env python3
"""dispatch_342_bandfix.py — AG-342 волна-522: вериф-нога band-gate drift-2 recal.

Классификация canary-5 RED ×2 (36832349428/36832346586): НЕ #16a/#16b —
step-3 'Runner calibration band gate' fast-fail, idx 7073387/6977844 < 10.0M
floor (pool -35% с рекалибровки AG-318 x521). Бенч ни разу не стартовал.

Нога = первая bench-v2 верификация волны-522 на фиксе swarm-522-342 (bf9078c3):
seed 522298 (gate 522001..522299, уникален vs claims/), явное окно 6.5-13.5M.
1 POST (лимит ≤2/агента соблюдён). concurrency group = ref+seed+radius →
sibling-cancel невозможен.
"""
import json, urllib.request

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "bench-v2.yml"
REF = "swarm-522-342"  # своя ветка (ref=master запрещён агентам)

inputs = {
    "seed": "522298",
    "run_seconds": "300",
    "radius_blocks": "1136",
    "server_xmx": "10G",
    "bench_dims": "minecraft:overworld,minecraft:the_nether,minecraft:the_end",
    "cpu_band_min": "6500000",   # drift-2 floor (covers observed 6977844/7073387)
    "cpu_band_max": "13500000",
}
payload = json.dumps({"ref": REF, "inputs": inputs}).encode()
req = urllib.request.Request(
    f"{API}/actions/workflows/{WF}/dispatches", data=payload, method="POST",
    headers={"Authorization": f"Bearer {TOK}", "Accept": "application/vnd.github+json",
             "Content-Type": "application/json", "User-Agent": "ag342-bandfix-x522"})
try:
    with urllib.request.urlopen(req) as r:
        print("POST", r.status, "seed=522298", "rl-remaining:", r.headers.get("X-RateLimit-Remaining"))
except urllib.error.HTTPError as e:
    print("HTTP", e.code, e.read()[:300])
