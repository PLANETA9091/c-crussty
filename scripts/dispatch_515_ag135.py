#!/usr/bin/env python3
"""dispatch_515_ag135.py — BENCH-V2 activation-ceiling leg (AG-135, wave-515).
Branch swarm-515-135 @374b02a6 (fork @09396b99 + v2act knob, +25 shell lines).
Gates preregistered in work/AG-135/SPEC.md BEFORE this POST. Quota 1/2, volley 1 POST.
"""
import json, subprocess

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "world-bench-parallel.yml"
BR = "swarm-515-135"
WORLD = "https://github.com/PLANETA9091/c-crussty/releases/download/v466-stress-c100/world466-stress-v1.zip"
INCENDIUM = "https://cdn.modrinth.com/data/ZVzW5oNS/versions/gBoadsBv/Incendium_1.21.5_v5.4.9_UNSUPPORTED.zip"

inputs = {
    "world_url": WORLD, "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "6", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "skip_store_bb": "0", "region_steal": "0",
    "bu_defer": "0", "population_target": "0", "population_seed": "2135",
    "server_xmx": "12G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "cmp515_v2act",
    "lever_arg": "v2dims=minecraft:the_nether,v2vd=32,v2sd=32,v2spawn=1,v2act=1",
    "datapack_url": INCENDIUM,
}

def gh(method, url, payload=None):
    cmd = ["curl", "-s", "-X", method, "-H", f"Authorization: token {TOK}",
           "-H", "Accept: application/vnd.github+json", url]
    if payload:
        cmd += ["-d", json.dumps(payload)]
    return subprocess.run(cmd, capture_output=True, text=True).stdout

g = json.loads(gh("GET", f"{API}/git/ref/heads/{BR}") or "{}")
got = g.get("object", {}).get("sha", "")
print(f"ref {BR}: {got[:12]} match374b={got.startswith('374b02a6799e46c09b88')}", flush=True)
if not got.startswith("374b02a6799e46c09b88"):
    print("ABORT: branch sha mismatch"); raise SystemExit(1)

r = gh("POST", f"{API}/actions/workflows/{WF}/dispatches", {"ref": BR, "inputs": inputs})
ok = r.strip() == ""
print("dispatch:", "OK" if ok else r[:200], flush=True)

json.dump({"tool": "dispatch_515_ag135", "branch": BR, "sha": got,
           "inputs": inputs, "dispatched": ok, "resp": r[:400]},
          open("/home/z/rounds/ROUND-515/work/AG-135/dispatch_515_ag135.json", "w"),
          indent=1, ensure_ascii=False)
print("DONE ok=", ok)
