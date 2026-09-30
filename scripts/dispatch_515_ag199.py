#!/usr/bin/env python3
"""dispatch_515_ag199.py — BENCH-V2 BATCH-COLLECTOR ось (AG-199, волна-515).

A/B: bc1 (канон BENCH-V2) vs bc0 (vanilla collector) на proven-харнессе
@09396b99 (swarm-515-62 патч, валид-ветка волны). 0 код-дельт: обе ноги =
алиас-ветки ОДНОГО sha (ref-POST + GET-вериф, Л188a/b; 1 диспатч = 1 ветка
Л188b). Overworld v2-профиль: v2vd=32/v2sd=32/v2spawn=1, r640/300s/fp4/gc6/
xmx12G, world466-stress-v1 (sha256 0fd4c132), same-seed population_seed=515199.
Квота: 2 диспатча <=2, залп 4 POST <=40. Prereg-гейты: work/AG-199/SPEC.md
(зарегистрированы ДО диспатча). Финал: DISP run-ids / 429-403 -> DISP-INTENT.
"""
import json, subprocess

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "world-bench-parallel.yml"

SHA = "09396b9976567d9a6b3c6919da6d3ce461782086"  # swarm-515-62 (AG-62 bench-v2 патч, proven)
WORLD = "https://github.com/PLANETA9091/c-crussty/releases/download/v466-stress-c100/world466-stress-v1.zip"
POPS = "515199"  # same-seed оба плеча, кенсус claims = 0 коллизий

def v2_inputs(bc):
    return {
        "world_url": WORLD, "radius": "640", "seconds": "300", "fake_players": "4",
        "fluid_guard": "1", "gc_tune": "6", "inside_cache": "1", "flush_diet": "1",
        "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
        "batch_collector": bc, "skip_store_bb": "0", "region_steal": "0",
        "bu_defer": "0", "population_target": "0", "population_seed": POPS,
        "server_xmx": "12G", "server_xms": "4G",
        "cpu_band_min": "6000000", "cpu_band_max": "9500000",
        "lever_flag": "cmp515_v2dims",
        "lever_arg": "v2dims=minecraft:overworld,v2vd=32,v2sd=32,v2spawn=1",
        "datapack_url": "",
    }

legs = [
    {"branch": "swarm-515-199",  "sha": SHA, "inputs": v2_inputs("1")},  # bc1 канон
    {"branch": "swarm-515-199b", "sha": SHA, "inputs": v2_inputs("0")},  # bc0 ваниль
]

def gh(method, url, payload=None):
    cmd = ["curl", "-s", "-X", method, "-H", f"Authorization: token {TOK}",
           "-H", "Accept: application/vnd.github+json", url]
    if payload:
        cmd += ["-d", json.dumps(payload)]
    return subprocess.run(cmd, capture_output=True, text=True).stdout

# 1) алиас-ветки (POST refs idempotent + GET-верификация Л188a)
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
        fails.append((l["branch"], r[:300]))

out = {"tool": "dispatch_515_ag199", "sha": SHA, "world_sha256": "0fd4c132",
       "popseed": POPS, "dispatched": dispatched, "fails": fails,
       "legs": [{"branch": l["branch"], "inputs": l["inputs"]} for l in legs]}
json.dump(out, open("/home/z/rounds/ROUND-515/work/AG-199/dispatch_515_ag199.json", "w"),
          indent=1, ensure_ascii=False)
print(f"DONE dispatched={dispatched}/{len(legs)} fails={fails}")
