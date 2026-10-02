#!/usr/bin/env python3
"""dispatch_515_ag343.py — BENCH-V2 JVM-GC plane (AG-343, волна-515).
Leg-A base: ref=swarm-515-343 = канон-харнесс @e8b6a08 (0 код-дельт).
Leg-B patch: ref=swarm-515-343b = 00b6ea7b (jvm_extra passthrough).
Квота 2 POST; 429/403 → payload в work/AG-343/ → DISP-INTENT."""
import json, subprocess

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "bench-v2.yml"

A_SHA = "e8b6a08d5018b23701e35f3b7445256bee185895"
B_SHA = "00b6ea7b43ae93c6cb019ead01f8624065873d29"
JVM_EXTRA = ("-XX:+ParallelRefProcEnabled -XX:MaxGCPauseMillis=200 "
             "-XX:+UnlockExperimentalVMOptions -XX:+DisableExplicitGC "
             "-XX:+AlwaysPreTouch -XX:G1NewSizePercent=40 "
             "-XX:G1MaxNewSizePercent=50 -XX:G1HeapRegionSize=16M "
             "-XX:G1ReservePercent=15 -XX:G1HeapWastePercent=5 "
             "-XX:G1MixedGCCountTarget=4 -XX:InitiatingHeapOccupancyPercent=15 "
             "-XX:G1MixedGCLiveThresholdPercent=90 "
             "-XX:G1RSetUpdatingPauseTimePercent=5 -XX:SurvivorRatio=32 "
             "-XX:+PerfDisableSharedMem -XX:MaxTenuringThreshold=1")

def canon(extra=""):
    v = {"radius_blocks": "1136", "run_seconds": "300", "seed": "351515",
         "server_xmx": "10G",
         "bench_dims": "minecraft:overworld,minecraft:the_nether,minecraft:the_end",
         "cpu_band_min": "6000000", "cpu_band_max": "9500000"}
    if extra:
        v["jvm_extra"] = extra
    return v

def gh(method, url, payload=None):
    cmd = ["curl", "-s", "-X", method, "-H", f"Authorization: token {TOK}",
           "-H", "Accept: application/vnd.github+json", url]
    if payload is not None:
        cmd += ["-d", json.dumps(payload)]
    return subprocess.run(cmd, capture_output=True, text=True).stdout

legs = [
    {"branch": "swarm-515-343",  "sha": A_SHA, "inputs": canon()},
    {"branch": "swarm-515-343b", "sha": B_SHA, "inputs": canon(JVM_EXTRA)},
]

for l in legs:
    r = gh("POST", f"{API}/git/refs", {"ref": f"refs/heads/{l['branch']}", "sha": l["sha"]})
    ok_ref = '"ref"' in r or "already_exists" in r
    g = json.loads(gh("GET", f"{API}/git/ref/heads/{l['branch']}") or "{}")
    got = g.get("object", {}).get("sha", "")
    print(f"ref {l['branch']}: post={'ok' if ok_ref else r[:120]} get={got[:12]} match={got == l['sha']}", flush=True)
    d = gh("POST", f"{API}/actions/workflows/{WF}/dispatches", {"ref": l["branch"], "inputs": l["inputs"]})
    print(f"dispatch {l['branch']}: {'OK(204)' if d == '' else d[:200]}", flush=True)
    json.dump({"branch": l["branch"], "sha": l["sha"], "inputs": l["inputs"], "dispatch_resp": d},
              open(f"/home/z/rounds/ROUND-515/work/AG-343/payload_{l['branch'].split('-')[-1]}.json", "w"), indent=1)
