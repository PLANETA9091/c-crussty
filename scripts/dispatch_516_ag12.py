#!/usr/bin/env python3
"""dispatch_516_ag12.py — BENCH-V2 canon MERGE-READY validation (AG-12, волна-516).
Branch swarm-516-12 = canon line AG-433/104 (master d80f0d3) + AG-342 fake-players
+ AG-248 DimForceload (nether/end tickets) + AG-93/234 async driver (ticket
marking + wall-clock heartbeat) + Tectonic 3.0.25 repin (FATAL-alias purge).
Leg-1 CANON-SANITY seed 351515 FP=0 — merge-validation leg.
Leg-2 SPAWN-LANE    seed 351601 FP=4 — fake-players wiring proof.
Квота 2 POST; 429/403 → payload в work/AG-12/ → DISP-INTENT."""
import json, subprocess, time, sys

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
WF = "bench-v2.yml"
BR = "swarm-516-12"
SHA = "edb599bef6e848290f25562843b0bb949d5b3104"

def gh(method, url, payload=None):
    cmd = ["curl", "-s", "-X", method, "-H", f"Authorization: token {TOK}",
           "-H", "Accept: application/vnd.github+json", url]
    if payload is not None:
        cmd += ["-d", json.dumps(payload)]
    return subprocess.run(cmd, capture_output=True, text=True).stdout

legs = [
    {"tag": "L1_canon_sanity", "inputs": {"seed": "351515", "fake_players": "0"}},
    {"tag": "L2_spawn_lane",   "inputs": {"seed": "351601", "fake_players": "4"}},
]

seeds = [int(l["inputs"]["seed"]) for l in legs]
g = subprocess.run([sys.executable, "/home/z/c-crussty/scripts/seed_gate.py"] + [str(s) for s in seeds],
                   capture_output=True, text=True)
print(g.stdout.strip() or g.stderr.strip(), flush=True)
if g.returncode != 0:
    sys.exit("SEED-GATE BLOCKED DISPATCH")

for l in legs:
    gg = json.loads(gh("GET", f"{API}/git/ref/heads/{BR}") or "{}")
    got = gg.get("object", {}).get("sha", "")
    print(f"ref {BR}: get={got[:12]} match={got == SHA}", flush=True)
    d = gh("POST", f"{API}/actions/workflows/{WF}/dispatches", {"ref": BR, "inputs": l["inputs"]})
    print(f"dispatch {l['tag']} {BR}: {'OK(204)' if d == '' else d[:200]}", flush=True)
    json.dump({"branch": BR, "sha": SHA, "tag": l["tag"], "inputs": l["inputs"],
               "dispatch_resp": d, "workflow": WF},
              open(f"/home/z/rounds/ROUND-516/work/AG-12/payload_{l['tag']}.json", "w"), indent=1)
    time.sleep(2)

for attempt in range(6):
    time.sleep(8)
    r = json.loads(gh("GET", f"{API}/actions/runs?event=workflow_dispatch&branch={BR}&per_page=5") or "{}")
    runs = [(x["id"], x["status"], x["created_at"]) for x in r.get("workflow_runs", [])]
    print(f"runs poll#{attempt}: {runs}", flush=True)
    if len(runs) >= 2:
        break
