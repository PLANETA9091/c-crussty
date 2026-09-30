#!/usr/bin/env python3
"""dispatch_515_ag228.py — BENCH-V2 fake-player wiring (AG-228, волна-515).
Один workflow bench-v2.yml на ветке swarm-515-228 (sha dd4e654, НЕ master).
Лег-A: fake_players=4 (spawn dimension LIVE, owner-сценарий).
Лег-B: fake_players=0 (бит-в-бит база, no-player mode).
Квота 2 POST; 429/403 → payload в work/AG-228/ → DISP-INTENT."""
import json, subprocess

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "bench-v2.yml"
SHA = "dd4e654989602654c3ef0594492df8c5325a8c06"

def canon(fp):
    return {"radius_blocks": "1136", "run_seconds": "300", "seed": "351515",
            "server_xmx": "10G",
            "bench_dims": "minecraft:overworld,minecraft:the_nether,minecraft:the_end",
            "cpu_band_min": "6000000", "cpu_band_max": "9500000",
            "fake_players": fp}

def gh(method, url, payload=None):
    cmd = ["curl", "-s", "-X", method, "-H", f"Authorization: token {TOK}",
           "-H", "Accept: application/vnd.github+json", url]
    if payload is not None:
        cmd += ["-d", json.dumps(payload)]
    return subprocess.run(cmd, capture_output=True, text=True).stdout

legs = [
    {"inputs": canon("4")},
    {"inputs": canon("0")},
]
for l in legs:
    d = gh("POST", f"{API}/actions/workflows/{WF}/dispatches", {"ref": "swarm-515-228", "inputs": l["inputs"]})
    print(f"dispatch ref=swarm-515-228 fp={l['inputs']['fake_players']}: {'OK(204)' if d == '' else d[:200]}", flush=True)
    json.dump({"branch": "swarm-515-228", "sha": SHA, "inputs": l["inputs"], "dispatch_resp": d},
              open(f"/home/z/rounds/ROUND-515/work/AG-228/payload_fp{l['inputs']['fake_players']}.json", "w"), indent=1)
