#!/usr/bin/env python3
"""dispatch_515_ag362.py — BENCH-V2 ENV-MANIFEST GUARD validation leg (AG-362, волна-515).
1 POST ≤2. ref=swarm-515-362 @b51a3ee (master-диспатч запрещён — не используется).
Гейты G-ENV1..4 prereg в work/AG-362/SPEC.md ДО диспатча."""
import json, subprocess, sys, time

TOK = open("/tmp/gh_token").read().strip()
REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}/actions/workflows/bench-v2.yml/dispatches"

payload = json.load(open("/home/z/rounds/ROUND-515/work/AG-362/DISP_PAYLOAD.json"))
body = {"ref": payload["ref"], "inputs": payload["inputs"]}

r = subprocess.run([
    "curl", "-sS", "-o", "/tmp/ag362_resp.txt", "-w", "%{http_code}",
    "-X", "POST", API,
    "-H", f"Authorization: Bearer {TOK}",
    "-H", "Accept: application/vnd.github+json",
    "-d", json.dumps(body),
], capture_output=True, text=True)
code = r.stdout.strip()
print("HTTP", code)
print(open("/tmp/ag362_resp.txt").read()[:400])
if code == "204":
    time.sleep(8)
    out = subprocess.run([
        "curl", "-sS",
        f"https://api.github.com/repos/{REPO}/actions/runs?head_sha={payload['branch_sha']}&per_page=3",
        "-H", f"Authorization: Bearer {TOK}",
    ], capture_output=True, text=True).stdout
    js = json.loads(out)
    for w in js.get("workflow_runs", []):
        print("RUN", w["id"], w["status"], w["html_url"])
else:
    print("NOT-DISPATCHED -> DISP-INTENT (payload saved in work/AG-362/DISP_PAYLOAD.json)")
