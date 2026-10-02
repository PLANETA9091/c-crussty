#!/usr/bin/env python3
"""dispatch_519_canary.py — CANARY pair MAIN-infra (волна-519, координатор).

Canary = гейт мёржа (закон 8), НЕ саб-диспатч: ref=master валидирует канон
8bab7a6 + инфра-фикс 876b3f45 (RUNNER_CPU_INDEX default + dl retry x3).
Пара на разных canon-осях seed-gate-легально:
  canary-A seed 351601 (FP4-ось, re-run убитого run-36773277359)
  canary-B seed 351515 (sanity-ось, реестр: «только sanity/canon-A/B» — canary им и является)
Разные seeds → per-leg concurrency (seed+radius) → sibling-cancel невозможен.
"""
import json, urllib.request

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "bench-v2.yml"
REF = "master"  # MAIN canary gate: валидация канона в master (закон 8), не саб-диспатч

results = []
for tag, seed in (("canary-A", "351601"), ("canary-B", "351515")):
    inputs = {
        "seed": seed,
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
                 "Content-Type": "application/json", "User-Agent": "main-519-canary"})
    try:
        with urllib.request.urlopen(req) as r:
            print(tag, "POST", r.status, "seed=" + seed, "rl-remaining:", r.headers.get("X-RateLimit-Remaining"))
            results.append((tag, seed, r.status))
    except urllib.error.HTTPError as e:
        print(tag, "HTTP", e.code, e.read()[:300])
        results.append((tag, seed, e.code))

print("DISPATCHED:", results)
