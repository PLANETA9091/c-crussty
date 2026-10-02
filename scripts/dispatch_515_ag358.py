#!/usr/bin/env python3
"""dispatch_515_ag358.py — BENCH-V2 corrected-stand validation run (AG-358, wave-515).
1 POST, ref=swarm-515-358 @a704437 (0 master, canon). Prereg gates: work/AG-358/SPEC-BENCH-V2-AG358.md."""
import json, subprocess, os

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
URL = f"https://api.github.com/repos/{REPO}/actions/workflows/bench-v2.yml/dispatches"
REF = "swarm-515-358"  # head a70443728a908f6b334669f818fbbd0d66181649

payload = {
    "ref": REF,
    "inputs": {
        "run_seconds": "300",
        "seed": "351515",
        "server_xmx": "10G",
        "bench_dims": "minecraft:overworld,minecraft:the_nether,minecraft:the_end",
        "cpu_band_min": "6000000",
        "cpu_band_max": "9500000",
        "radius_blocks": "1136",
    },
}
os.makedirs("/home/z/rounds/ROUND-515/work/AG-358", exist_ok=True)
with open("/home/z/rounds/ROUND-515/work/AG-358/DISP-payload-ag358.json", "w") as f:
    json.dump(payload, f, indent=2)

r = subprocess.run([
    "curl", "-sS", "-X", "POST", URL,
    "-H", f"Authorization: token {TOK}",
    "-H", "Accept: application/vnd.github+json",
    "-d", json.dumps(payload), "-w", "\nHTTP=%{http_code}\n",
], capture_output=True, text=True, timeout=60)
print(r.stdout, r.stderr)
