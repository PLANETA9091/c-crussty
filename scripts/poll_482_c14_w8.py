#!/usr/bin/env python3
"""poll_482_c14_w8.py — опрос run 36419896337 ≤20 мин; при SUCCESS — вердикт-числа."""
import json, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RUN = 36419896337
BRANCH = "round-482-c14-w8"
PIN = "3666a7931703e24a36af4887c41a585918667b7a"
DEADLINE_S = int(sys.argv[1]) if len(sys.argv) > 1 else 1200

def tok():
    return open("/tmp/gh_token").read().strip()

def api(url):
    req = urllib.request.Request(API + url, headers={
        "Authorization": f"Bearer {tok()}", "Accept": "application/vnd.github+json"})
    with urllib.request.urlopen(req, timeout=60) as r:
        return json.loads(r.read())

t0 = time.time()
last = None
while time.time() - t0 < DEADLINE_S:
    try:
        r = api(f"/repos/{REPO}/actions/runs/{RUN}")
    except Exception as e:
        print(f"poll-err {e}", flush=True); time.sleep(30); continue
    st, conc = r.get("status"), r.get("conclusion")
    if (st, conc) != last:
        print(f"[{int(time.time()-t0)}s] status={st} conclusion={conc} start={r.get('run_started_at')}", flush=True)
        last = (st, conc)
    if st == "completed":
        arts = api(f"/repos/{REPO}/actions/runs/{RUN}/artifacts").get("artifacts", [])
        out = {"run": RUN, "branch": BRANCH, "pin": PIN, "status": st,
               "conclusion": conc, "run_started_at": r.get("run_started_at"),
               "updated_at": r.get("updated_at"),
               "artifacts": [{"id": a["id"], "name": a["name"], "size": a["size_in_bytes"],
                              "expired": a["expired"], "download": a["archive_download_url"]}
                             for a in arts]}
        json.dump(out, open("/home/z/rounds/ROUND-482/c14_poll.json", "w"), indent=1)
        print("ARTIFACTS:", json.dumps(out["artifacts"], indent=1), flush=True)
        print(f"VERDICT-BASIS: conclusion={conc}", flush=True)
        break
    time.sleep(60)
else:
    print(f"TIMEOUT {DEADLINE_S}s — DISPATCHED run-{RUN} (закон 12e/18-iii)", flush=True)
