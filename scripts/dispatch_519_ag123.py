#!/usr/bin/env python3
"""dispatch_519_ag123.py — AG-123 verification leg: tectonic 3-space pin fix.

Ref = swarm-519-123 (624b3bf4) — my own branch, master NOT touched (canary = координатор).
Seed 519001: свободный диапазон волны-519 (519001..519099, SEED_REGISTRY_518 §C), коллизий нет.
Цель: доказать, что PIN-GATES проходит (tectonic.zip: OK) и ран доходит до boot/gates.
1 нога = 1 ветка; мой единственный диспатч этой волны (≤2).
"""
import json, urllib.request

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "bench-v2.yml"
REF = "swarm-519-123"  # leg-2: a6e4f52c (pins + DFCOMPILE)

inputs = {
    "seed": "519001",
    "run_seconds": "300",
    "radius_blocks": "1136",
    "server_xmx": "10G",
    "bench_dims": "minecraft:overworld,minecraft:the_nether,minecraft:the_end",
    "cpu_band_min": "",
    "cpu_band_max": "",
}
payload = json.dumps({"ref": REF, "inputs": inputs}).encode()
req = urllib.request.Request(
    f"{API}/actions/workflows/{WF}/dispatches", data=payload, method="POST",
    headers={"Authorization": f"Bearer {TOK}", "Accept": "application/vnd.github+json",
             "Content-Type": "application/json", "User-Agent": "swarm-519-ag123"})
try:
    with urllib.request.urlopen(req) as r:
        print("POST", r.status, "seed=519001 ref=" + REF,
              "rl-remaining:", r.headers.get("X-RateLimit-Remaining"))
except urllib.error.HTTPError as e:
    print("HTTP", e.code, e.read()[:300])
