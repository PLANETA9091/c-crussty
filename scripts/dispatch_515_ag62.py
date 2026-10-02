#!/usr/bin/env python3
"""dispatch_515_ag62.py — BENCH-V2 multi-dim ось (AG-62, волна-515).

2 ноги: Incendium (nether) + Stellarity (end) на канон-стресс-мире v466
(world466-stress-v1.zip sha256 0fd4c132edfa74b9, Terralith+Tectonic+BACAP в
мире), knob v2dims через lever_arg (run_world3.sh AG-62-патч, canon
bit-in-bit при пустом lever_arg). 1 диспатч = 1 ветка (Л188b): leg A на
swarm-515-62, leg B на алиасе swarm-515-62b ОДНОГО sha (2 concurrency-группы,
Л407r-канон). POST /git/refs FULL-sha + GET-верификация (Л188a).
Квота: 2 диспатча ≤2, залп 5 POST ≤40. Бюджет-гейты prereg (закон 14a/16):
band 6.0-9.5M fast-fail; DP-INSTALLED sha256 == пин; V2DIM begin/end маркеры;
Marked-дельта per-dim; AIOOBE=0/NCDFE-канон; TPS-tail<20 per-dim медиана."""
import json, subprocess

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "world-bench-parallel.yml"

SHA = "09396b9976567d9a6b3c6919da6d3ce461782086"  # swarm-515-62 (AG-62 патч)
WORLD = "https://github.com/PLANETA9091/c-crussty/releases/download/v466-stress-c100/world466-stress-v1.zip"
INCENDIUM = "https://cdn.modrinth.com/data/ZVzW5oNS/versions/gBoadsBv/Incendium_1.21.5_v5.4.9_UNSUPPORTED.zip"  # sha256 f71bdc30... (пин AG-35, download-verified AG-62)
STELLARITY = "https://cdn.modrinth.com/data/bZgeDzN8/versions/zudQ7s97/Stellarity-5.1.3.zip"  # sha256 920d1fa4... (пин AG-35, download-verified AG-62)

def v2_inputs(dp, dims):
    return {
        "world_url": WORLD, "radius": "640", "seconds": "300", "fake_players": "4",
        "fluid_guard": "1", "gc_tune": "6", "inside_cache": "1", "flush_diet": "1",
        "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
        "batch_collector": "1", "skip_store_bb": "0", "region_steal": "0",
        "bu_defer": "0", "population_target": "0", "population_seed": "42",
        "server_xmx": "12G", "server_xms": "4G",
        "cpu_band_min": "6000000", "cpu_band_max": "9500000",
        "lever_flag": "cmp515_v2dims",
        "lever_arg": f"v2dims={dims},v2vd=32,v2sd=32,v2spawn=1",
        "datapack_url": dp,
    }

legs = [
    {"branch": "swarm-515-62",  "sha": SHA, "inputs": v2_inputs(INCENDIUM,  "minecraft:the_nether")},
    {"branch": "swarm-515-62b", "sha": SHA, "inputs": v2_inputs(STELLARITY, "minecraft:the_end")},
]

def gh(method, url, payload=None):
    cmd = ["curl", "-s", "-X", method, "-H", f"Authorization: token {TOK}",
           "-H", "Accept: application/vnd.github+json", url]
    if payload:
        cmd += ["-d", json.dumps(payload)]
    return subprocess.run(cmd, capture_output=True, text=True).stdout

# 1) алиас-ветка 62b (POST refs + GET-верификация; 62 уже запушена локально)
for l in legs:
    r = gh("POST", f"{API}/git/refs",
           {"ref": f"refs/heads/{l['branch']}", "sha": l["sha"]})
    g = json.loads(gh("GET", f"{API}/git/ref/heads/{l['branch']}") or "{}")
    got = g.get("object", {}).get("sha", "")
    print(f"ref {l['branch']}: post={'ok' if '\"ref\"' in r else r[:100]} get={got[:12]} match={got == l['sha']}", flush=True)

# 2) диспатчи (2 POST, разные concurrency-группы)
dispatched, fails = 0, []
for l in legs:
    r = gh("POST", f"{API}/actions/workflows/{WF}/dispatches",
           {"ref": l["branch"], "inputs": l["inputs"]})
    if r.strip() == "":
        dispatched += 1
    else:
        fails.append((l["branch"], r[:200]))

json.dump({"tool": "dispatch_515_ag62", "sha": SHA, "world": WORLD,
           "dispatched": dispatched, "fails": fails, "legs": legs},
          open("/home/z/rounds/ROUND-515/work/AG-62/dispatch_515_ag62.json", "w"),
          indent=1, ensure_ascii=False)
print(f"DONE dispatched={dispatched}/{len(legs)} fails={fails}")
