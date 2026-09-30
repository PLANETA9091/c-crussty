#!/usr/bin/env python3
"""dispatch_515_ag158.py — dp21-press@50k «мир под давлением» (AG-158, волна-515).

1 нога: Structory v1.3.17 datapack (Modrinth FULL URL, sha256 55f1281b…a40,
download-verified локально, pack_format 48..107 ⇒ MC 1.21.10) на канон-стресс-мире
v466, pop=50000, seed 1813, 300s, fp4, gc3, band 6.0-9.5M. Нуль кода: datapack_url
= DP-DOOR C38 input; lever_arg ∅ = канон бит-в-байт (law 4). Ref = swarm-515-158
@7d81675 (НЕ master). Квота: 1/2 диспатча, залп 1 POST ≤40.
Prereg-гейты: work/AG-158/SPEC.md (G1-G5) — записан ДО диспатча.
"""
import json, subprocess

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "world-bench-parallel.yml"

BRANCH = "swarm-515-158"
DP21 = "https://cdn.modrinth.com/data/aKCwCJlY/versions/OIcllpSf/Structory_v1.3.7.zip"

inputs = {
    "world_url": "https://github.com/PLANETA9091/c-crussty/releases/download/v466-stress-c100/world466-stress-v1.zip",
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "skip_store_bb": "0", "region_steal": "0",
    "bu_defer": "0", "population_target": "50000", "population_seed": "1813",
    "server_xmx": "12G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "", "lever_arg": "",
    "datapack_url": DP21,
}

def gh(method, url, payload=None):
    cmd = ["curl", "-s", "-X", method, "-H", f"Authorization: token {TOK}",
           "-H", "Accept: application/vnd.github+json", url]
    if payload:
        cmd += ["-d", json.dumps(payload)]
    return subprocess.run(cmd, capture_output=True, text=True).stdout

ref = json.loads(gh("GET", f"{API}/git/ref/heads/{BRANCH}") or "{}")
got = ref.get("object", {}).get("sha", "")
print(f"ref {BRANCH}: sha={got[:12]}", flush=True)

r = gh("POST", f"{API}/actions/workflows/{WF}/dispatches",
       {"ref": BRANCH, "inputs": inputs})
ok = r.strip() == ""
print(f"dispatch: {'OK' if ok else r[:200]}", flush=True)

json.dump({"tool": "dispatch_515_ag158", "branch": BRANCH, "sha": got,
           "dp21_url": DP21, "dp21_sha256": "55f1281b61e556b80771788bb251a3723b3b771681f03149b049de13c89b6a40",
           "dispatched": 1 if ok else 0, "err": None if ok else r[:200],
           "inputs": inputs},
          open("/home/z/rounds/ROUND-515/work/AG-158/dispatch_515_ag158.json", "w"),
          indent=1, ensure_ascii=False)
print("DONE")
