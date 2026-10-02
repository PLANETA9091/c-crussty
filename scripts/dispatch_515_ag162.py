#!/usr/bin/env python3
"""dispatch_515_ag162.py — BENCH-V2 smoke-canary (AG-162, wave-515).

1 диспатч workflow bench-v2-smoke.yml на СВОЕЙ ветке swarm-515-162
(ref=master диспатч ЗАПРЕЩЁН каноном). Смоук дерискрит полный стенд
20k×3dim волны за ~15 мин runner-времени. 429/403 → payload в
work/AG-162/dispatch/ → легальный финал DISP-INTENT.
"""
import json, subprocess, time

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"
REF = "swarm-515-162"
WF = "bench-v2-smoke.yml"

INPUTS = {
    "chunks_per_dim": "900",
    "batch_size": "50",
    "spread": "0",
    "seconds": "60",
    "seed": "42",
    "xmx": "8G",
}

def gh(method, url, payload=None):
    cmd = ["curl", "-s", "-X", method, "-H", f"Authorization: token {TOK}",
           "-H", "Accept: application/vnd.github+json", url]
    if payload:
        cmd += ["-d", json.dumps(payload)]
    return subprocess.run(cmd, capture_output=True, text=True).stdout

payload = {"ref": REF, "inputs": INPUTS}
json.dump({"workflow": WF, "payload": payload}, open("dispatch_payload.json", "w"), indent=1)

r = gh("POST", f"{API}/actions/workflows/{WF}/dispatches", payload)
print("POST status-body:", r[:200] if r else "(empty=202 OK)")

# GET run-id (poll a few times; dispatch is async)
run_id = None
for _ in range(6):
    time.sleep(4)
    runs = gh("GET", f"{API}/actions/workflows/{WF}/runs?event=workflow_dispatch&per_page=5")
    try:
        for w in json.loads(runs).get("workflow_runs", []):
            if w.get("head_branch") == REF:
                run_id = w["id"]
                break
    except Exception as e:
        print("runs parse err:", e)
    if run_id:
        break

json.dump({"run_id": run_id, "payload": payload},
          open("dispatch_result.json", "w"), indent=1)
print("RUN_ID:", run_id)
