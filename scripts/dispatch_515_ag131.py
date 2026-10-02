#!/usr/bin/env python3
"""dispatch_515_ag131.py — BENCH-V2 END-DP-TAX A/B (AG-131, волна-515).

Свободная ячейка матрицы атрибуции: nether-dp-tax=AG-284, end-ON/ON=AG-167,
spawn-AB=AG-149, vd-pair=AG-176. Я = Stellarity-ON vs vanilla-OFF в the_end
при ПОЛНОМ BENCH-V2 профиле (v2vd=32/v2sd=32/v2spawn=1) на proven harness
AG-62 @09396b99 (0 код-дельт). Ref-POST swarm-515-131/131b (2 concurrency-
группы, Л188b). Same-seed 42/42 (pair канон AG-103). Prereg-гейты: SPEC.md
ДО диспатча. Квота: 2 диспатча ≤2, залп 4 POST ≤40.
"""
import json, subprocess, time

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "world-bench-parallel.yml"

SHA = "09396b9976567d9a6b3c6919da6d3ce461782086"  # proven harness AG-62
WORLD = "https://github.com/PLANETA9091/c-crussty/releases/download/v466-stress-c100/world466-stress-v1.zip"
STELLARITY = "https://cdn.modrinth.com/data/bZgeDzN8/versions/zudQ7s97/Stellarity-5.1.3.zip"
STELLARITY_SHA256 = "920d1fa4e910e9ef8dc0e0706c443466afe3c6433a3fe0d3f36f80db40d07968"
LEVER_ARG = "v2dims=minecraft:the_end,v2vd=32,v2sd=32,v2spawn=1"

def v2_inputs(dp):
    return {
        "world_url": WORLD, "radius": "640", "seconds": "300", "fake_players": "4",
        "fluid_guard": "1", "gc_tune": "6", "inside_cache": "1", "flush_diet": "1",
        "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
        "batch_collector": "1", "skip_store_bb": "0", "region_steal": "0",
        "bu_defer": "0", "population_target": "0", "population_seed": "42",
        "server_xmx": "12G", "server_xms": "4G",
        "cpu_band_min": "6000000", "cpu_band_max": "9500000",
        "lever_flag": "cmp515_v2dims",
        "lever_arg": LEVER_ARG,
        "datapack_url": dp,
    }

legs = [
    {"branch": "swarm-515-131",  "expect": "",
     "role": "ON-Stellarity",  "inputs": v2_inputs(STELLARITY)},
    {"branch": "swarm-515-131b", "expect": SHA,
     "role": "OFF-vanilla-end", "inputs": v2_inputs("")},
]

def gh(method, url, payload=None):
    cmd = ["curl", "-s", "-X", method, "-H", f"Authorization: token {TOK}",
           "-H", "Accept: application/vnd.github+json", url]
    if payload:
        cmd += ["-d", json.dumps(payload)]
    return subprocess.run(cmd, capture_output=True, text=True).stdout

# 1) ветки: 131 = запушена локально (proven sha + plumbing-коммит),
#    131b = алиас proven sha через POST refs (2-я concurrency-группа, Л188b)
for l in legs:
    if l["expect"] != SHA:
        pass  # 131: своя ветка уже запушена, POST не нужен
    else:
        r = gh("POST", f"{API}/git/refs",
               {"ref": f"refs/heads/{l['branch']}", "sha": SHA})
        print(f"ref-post {l['branch']}: {r[:80]}", flush=True)
    g = json.loads(gh("GET", f"{API}/git/ref/heads/{l['branch']}") or "{}")
    got = g.get("object", {}).get("sha", "")
    l["ref_ok"] = (got == l["expect"]) if l["expect"] else (got != "")
    l["head"] = got
    print(f"ref {l['branch']}: match={got == l['expect']} ({got[:12]})", flush=True)

# 2) диспатчи (2 POST, разные concurrency-группы)
time.sleep(1)
dispatched, fails = 0, []
for l in legs:
    if not l.get("ref_ok"):
        fails.append((l["branch"], "ref-mismatch, skip"))
        continue
    r = gh("POST", f"{API}/actions/workflows/{WF}/dispatches",
           {"ref": l["branch"], "inputs": l["inputs"]})
    if r.strip() == "":
        dispatched += 1
        l["dispatched"] = True
    else:
        fails.append((l["branch"], r[:200]))
        l["dispatched"] = False
    print(f"dispatch {l['branch']} [{l['role']}]: {'OK' if l['dispatched'] else r[:160]}", flush=True)

payload = {"tool": "dispatch_515_ag131", "wave": 515, "agent": 131,
           "plane": "BENCH-V2 END-DP-TAX A/B (Stellarity ON vs vanilla OFF, the_end)",
           "sha": SHA, "world_sha256": "0fd4c132edfa74b9 (release v466-stress-c100)",
           "datapack": {"name": "Stellarity 5.1.3", "url": STELLARITY,
                        "sha256": STELLARITY_SHA256, "verified_local": True},
           "lever_arg": LEVER_ARG, "same_seed": "42/42 (pair canon)",
           "prereg_spec": "work/AG-131/SPEC.md (записан ДО диспатча)",
           "dispatched": dispatched, "fails": fails, "legs": legs,
           "ts_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}
json.dump(payload, open("/home/z/rounds/ROUND-515/work/AG-131/dispatch_515_ag131.json", "w"),
          indent=1, ensure_ascii=False)
print(f"DONE dispatched={dispatched}/{len(legs)} fails={fails}")
