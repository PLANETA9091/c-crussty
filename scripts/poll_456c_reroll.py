#!/usr/bin/env python3
"""poll_456c_reroll.py — poll re-roll legs 3/4 status + T1 NCDFE gate check."""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
BRANCHES = ["round-456c-chunkmono-3", "round-456c-chunkmono-4"]


def token_from_remote():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    return re.match(r"^https://[^:]+:([^@]+)@github.com/", url).group(1)


def api(tok, url):
    req = urllib.request.Request(API + url, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    with urllib.request.urlopen(req, timeout=60) as r:
        return json.loads(r.read())


def main():
    tok = token_from_remote()
    seen = {}
    deadline = time.time() + int(sys.argv[1] if len(sys.argv) > 1 else 3900)
    while time.time() < deadline:
        for br in BRANCHES:
            if seen.get(br, {}).get("status") == "completed":
                continue
            try:
                d = api(tok, f"/repos/{REPO}/actions/runs?branch={br}&per_page=1")
                runs = d.get("workflow_runs", [])
                if not runs:
                    print(f"{br}: no runs yet", flush=True)
                    continue
                r = runs[0]
                seen[br] = {"id": r["id"], "status": r["status"], "conclusion": r["conclusion"]}
                print(f"{br}: run {r['id']} {r['status']} {r['conclusion']} ({r['created_at']})", flush=True)
            except Exception as e:
                print(f"{br}: poll error {e}", flush=True)
        done = all(v.get("status") == "completed" for v in seen.values()) and len(seen) == len(BRANCHES)
        if done:
            print("ALL COMPLETED", flush=True)
            for br, v in seen.items():
                print(f"FINAL {br}: run {v['id']} conclusion={v['conclusion']}", flush=True)
            return
        time.sleep(90)
    print("POLL TIMEOUT", flush=True)


if __name__ == "__main__":
    main()
