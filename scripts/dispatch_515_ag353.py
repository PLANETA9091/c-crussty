#!/usr/bin/env python3
"""dispatch_515_ag353.py — BENCH-V2 io-ratio plane (AG-353, волна-515).
Leg-A base: ref=swarm-515-353 @0425b1d5, io_ratio empty = vanilla bit-in-bit.
Leg-B patch: ref=swarm-515-353 @0425b1d5, io_ratio=75 (same-sha pair).
Квота 2 POST; 429/403 → payload в work/AG-353/ → DISP-INTENT."""
import json, subprocess, time

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "bench-v2.yml"
BR = "swarm-515-353"
SHA = "0425b1d55216f07382b5eaa7f4ad352c743d2d2f"

def canon(io_ratio=None):
    v = {"soak_seconds": "240", "server_xmx": "12G", "seed": "515045",
         "gen_cap_s": "2700", "ov_n": "100", "nether_n": "71", "end_n": "71"}
    if io_ratio is not None:
        v["io_ratio"] = io_ratio
    return v

def gh(method, url, payload=None):
    cmd = ["curl", "-s", "-X", method, "-H", f"Authorization: token {TOK}",
           "-H", "Accept: application/vnd.github+json", url]
    if payload is not None:
        cmd += ["-d", json.dumps(payload)]
    return subprocess.run(cmd, capture_output=True, text=True).stdout

legs = [
    {"tag": "A_base",  "inputs": canon()},
    {"tag": "B_io75",  "inputs": canon("75")},
]

for l in legs:
    g = json.loads(gh("GET", f"{API}/git/ref/heads/{BR}") or "{}")
    got = g.get("object", {}).get("sha", "")
    print(f"ref {BR}: get={got[:12]} match={got == SHA}", flush=True)
    d = gh("POST", f"{API}/actions/workflows/{WF}/dispatches", {"ref": BR, "inputs": l["inputs"]})
    print(f"dispatch {l['tag']} {BR}: {'OK(204)' if d == '' else d[:200]}", flush=True)
    json.dump({"branch": BR, "sha": SHA, "tag": l["tag"], "inputs": l["inputs"],
               "dispatch_resp": d, "workflow": WF},
              open(f"/home/z/rounds/ROUND-515/work/AG-353/payload_{l['tag']}.json", "w"), indent=1)
    time.sleep(2)

# best-effort run-id capture (GET, not counted toward POST quota)
for attempt in range(6):
    time.sleep(8)
    r = json.loads(gh("GET", f"{API}/actions/runs?event=workflow_dispatch&branch={BR}&per_page=5") or "{}")
    runs = [(x["id"], x["name"], x["status"], x["created_at"]) for x in r.get("workflow_runs", [])]
    print(f"runs poll#{attempt}: {runs}", flush=True)
    if len(runs) >= 2:
        break
