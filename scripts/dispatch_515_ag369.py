#!/usr/bin/env python3
"""dispatch_515_ag369.py — BENCH-V2 SIMULTANEITY-TAX A/B (AG-369, волна-515).

Свободная ячейка: dims-count ось (all-3-dims одновременно vs overworld-only).
Не дублирую xmx-293 (оба плеча all-3-dims, ось heap), dims-62/141/167
(nether/end-only), dp-tax 52/95/131/284/376/387/440, spawn-149, vd-176, rt-291,
scale-294, cadence-54. Proven харнесс AG-62 @09396b99 (knob v2dims; парсер
терпит ПРОБЕЛ-раздельный мульти-дим: tr ',;' '\n\n' режет только запятые, а
внутренний for пере-сплитит по пробелу; overworld = base sweep = 3-е измерение).
0 код-дельт, ветки-алиасы swarm-515-369/369b = sha 09396b99 (POST refs + GET
вериф, Л188a/b; df 100% → OFFLINE-паттерн, без checkout/worktree).
Same-seed 42/42 (pair канон AG-103). Prereg-гейты: work/AG-369/SPEC.md ДО
диспатча. Квота: 2 диспатча ≤2, залп 5 POST ≤40.
"""
import json, subprocess, time

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "world-bench-parallel.yml"

SHA = "09396b9976567d9a6b3c6919da6d3ce461782086"  # proven harness AG-62 (v2dims knob)
WORLD = "https://github.com/PLANETA9091/c-crussty/releases/download/v466-stress-c100/world466-stress-v1.zip"
STELLARITY = "https://cdn.modrinth.com/data/bZgeDzN8/versions/zudQ7s97/Stellarity-5.1.3.zip"
STELLARITY_SHA256 = "920d1fa4e910e9ef8dc0e0706c443466afe3c6433a3fe0d3f36f80db40d07968"

def v2_inputs(lever_arg):
    return {
        "world_url": WORLD, "radius": "640", "seconds": "300", "fake_players": "4",
        "fluid_guard": "1", "gc_tune": "6", "inside_cache": "1", "flush_diet": "1",
        "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
        "batch_collector": "1", "skip_store_bb": "0", "region_steal": "0",
        "bu_defer": "0", "population_target": "0", "population_seed": "42",
        "server_xmx": "12G", "server_xms": "4G",
        "cpu_band_min": "6000000", "cpu_band_max": "9500000",
        "lever_flag": "cmp515_v2dims",
        "lever_arg": lever_arg,
        "datapack_url": STELLARITY,
    }

LEG_A = "v2dims=minecraft:the_nether minecraft:the_end,v2vd=32,v2sd=32,v2spawn=1"  # 3 dims simultaneous (ov = base sweep)
LEG_B = "v2vd=32,v2sd=32,v2spawn=1"                                               # overworld-only anchor (v2dims absent = canon)

legs = [
    {"branch": "swarm-515-369",  "role": "ALL-3-DIMS-sim", "lever_arg": LEG_A, "inputs": v2_inputs(LEG_A)},
    {"branch": "swarm-515-369b", "role": "OV-only-anchor", "lever_arg": LEG_B, "inputs": v2_inputs(LEG_B)},
]

def gh(method, url, payload=None):
    cmd = ["curl", "-s", "-X", method, "-H", f"Authorization: token {TOK}",
           "-H", "Accept: application/vnd.github+json", url]
    if payload:
        cmd += ["-d", json.dumps(payload)]
    return subprocess.run(cmd, capture_output=True, text=True).stdout

# 1) алиас-ветки обоих плеч = proven sha (POST refs + GET-верификация, Л188a/b)
for l in legs:
    r = gh("POST", f"{API}/git/refs",
           {"ref": f"refs/heads/{l['branch']}", "sha": SHA})
    if "No common ancestor" in r or "422" in r[:20] or "already_exists" in r:
        print(f"ref-post {l['branch']}: note {r[:100]}", flush=True)
    g = json.loads(gh("GET", f"{API}/git/ref/heads/{l['branch']}") or "{}")
    got = g.get("object", {}).get("sha", "")
    l["ref_ok"] = (got == SHA)
    l["head"] = got
    print(f"ref {l['branch']}: match={got == SHA} ({got[:12]})", flush=True)

# 2) диспатчи (2 POST, разные concurrency-группы по ref)
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

# 3) run-id capture (1 GET; run может ещё не появиться — тогда пусто, poll следующей волной)
run_ids = {}
if dispatched:
    time.sleep(20)
    rs = gh("GET", f"{API}/actions/runs?event=workflow_dispatch&per_page=15")
    try:
        for w in json.loads(rs).get("workflow_runs", []):
            hb = w.get("head_branch", "")
            if hb in ("swarm-515-369", "swarm-515-369b") and hb not in run_ids:
                run_ids[hb] = {"id": w.get("id"), "url": w.get("html_url"),
                               "status": w.get("status"), "created": w.get("created_at")}
    except Exception as e:
        print(f"run-id parse: {e}", flush=True)
    print(f"run_ids: {json.dumps(run_ids)}", flush=True)

payload = {"tool": "dispatch_515_ag369", "wave": 515, "agent": 369,
           "plane": "BENCH-V2 SIMULTANEITY-TAX A/B (all-3-dims simultaneous vs overworld-only)",
           "sha": SHA, "world_sha256": "0fd4c132edfa74b9 (release v466-stress-c100)",
           "datapack": {"name": "Stellarity 5.1.3", "url": STELLARITY,
                        "sha256": STELLARITY_SHA256, "verified": "pin AG-35/AG-62 download-verified"},
           "lever_args": {"A_all3dims": LEG_A, "B_ovonly": LEG_B},
           "same_seed": "42/42 (pair canon AG-103)",
           "prereg_spec": "work/AG-369/SPEC.md (записан ДО диспатча)",
           "dispatched": dispatched, "fails": fails, "run_ids": run_ids, "legs": legs,
           "ts_utc": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime())}
json.dump(payload, open("/home/z/rounds/ROUND-515/work/AG-369/dispatch_515_ag369.json", "w"),
          indent=1, ensure_ascii=False)
print(f"DONE dispatched={dispatched}/{len(legs)} fails={fails} run_ids={list(run_ids)}")
