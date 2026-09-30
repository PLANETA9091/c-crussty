#!/usr/bin/env python3
"""dispatch_515_ag54.py — BENCH-V2 cadence-sweep A/B (AG-54, волна-515).

Leg A base (v2sleep=0.4, канон) на swarm-515-54; leg B burst (v2sleep=0) на
алиасе swarm-515-54b — ОДИН sha 0d56798 (Л188b: 1 диспатч = 1 ветка, 2
concurrency-группы). Nether+Incendium (пин sha256 AG-35, verified AG-62),
view/sim 32, v2spawn=1. Prereg-гейты: work/AG-54/SPEC.md (ДО диспатча).
Квота: 2 POST ≤2/агент, залп ≤40. ref=master ЗАПРЕЩЁН — оба ref мои ветки."""
import json, subprocess

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "world-bench-parallel.yml"
SHA = "0d56798cd7b9"  # swarm-515-54 == swarm-515-54b

WORLD = "https://github.com/PLANETA9091/c-crussty/releases/download/v466-stress-c100/world466-stress-v1.zip"
INCENDIUM = "https://cdn.modrinth.com/data/ZVzW5oNS/versions/gBoadsBv/Incendium_1.21.5_v5.4.9_UNSUPPORTED.zip"

def inputs(sleep):
    return {
        "world_url": WORLD, "radius": "640", "seconds": "300", "fake_players": "4",
        "fluid_guard": "1", "gc_tune": "6", "inside_cache": "1", "flush_diet": "1",
        "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
        "batch_collector": "1", "skip_store_bb": "0", "region_steal": "0",
        "bu_defer": "0", "population_target": "0", "population_seed": "42",
        "server_xmx": "12G", "server_xms": "4G",
        "cpu_band_min": "6000000", "cpu_band_max": "9500000",
        "lever_flag": "cmp515_v2dims",
        "lever_arg": f"v2dims=minecraft:the_nether,v2vd=32,v2sd=32,v2spawn=1,v2sleep={sleep}",
        "datapack_url": INCENDIUM,
    }

legs = [
    {"branch": "swarm-515-54",  "sleep": "0.4", "inputs": inputs("0.4")},
    {"branch": "swarm-515-54b", "sleep": "0",   "inputs": inputs("0")},
]

def gh(method, url, payload=None):
    cmd = ["curl", "-s", "-X", method, "-H", f"Authorization: token {TOK}",
           "-H", "Accept: application/vnd.github+json", url]
    if payload:
        cmd += ["-d", json.dumps(payload)]
    return subprocess.run(cmd, capture_output=True, text=True).stdout

# ref-верификация: обе ветки на сервере == SHA (анти-алиас, урок dp13-16)
for l in legs:
    g = json.loads(gh("GET", f"{API}/git/ref/heads/{l['branch']}") or "{}")
    got = g.get("object", {}).get("sha", "")
    print(f"ref {l['branch']}: got={got[:12]} match_full={got == SHA or got.startswith(SHA)}", flush=True)
    if not got.startswith(SHA):
        raise SystemExit(f"FATAL: {l['branch']} not at {SHA} — abort before POST")

dispatched, fails = 0, []
for l in legs:
    r = gh("POST", f"{API}/actions/workflows/{WF}/dispatches",
           {"ref": l["branch"], "inputs": l["inputs"]})
    if r.strip() == "":
        dispatched += 1
    else:
        fails.append((l["branch"], r[:200]))

json.dump({"tool": "dispatch_515_ag54", "sha": SHA, "world": WORLD,
           "dp": INCENDIUM,
           "dp_sha256": "f71bdc3003bc7549cd87224bace7a70cdaea4366b9c317f7d1b9e3362da134a5",
           "dispatched": dispatched, "fails": fails, "legs": legs},
          open("/home/z/rounds/ROUND-515/work/AG-54/dispatch_515_ag54.json", "w"),
          indent=1, ensure_ascii=False)
print(f"DONE dispatched={dispatched}/{len(legs)} fails={fails}")
