#!/usr/bin/env python3
"""poll_457a.py — poll world-bench-parallel.yml runs for agent-a legs (TASK-457-A2).
Usage: python3 poll_457a.py 11 12 13"""
import json, re, subprocess, sys, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    return re.match(r"^https://[^:]+:([^@]+)@github\.com/", url).group(1)


def api(tok, url):
    req = urllib.request.Request(f"{API}{url}", headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    with urllib.request.urlopen(req, timeout=60) as r:
        return json.loads(r.read())


def main():
    legs = [int(x) for x in sys.argv[1:]] or [11, 12, 13]
    tok = token()
    runs = api(tok, f"/repos/{REPO}/actions/workflows/world-bench-parallel.yml/runs?per_page=40").get("workflow_runs", [])
    for leg in legs:
        br = f"round-456c-chunkmono-{leg}"
        match = [r for r in runs if r["head_branch"] == br]
        if not match:
            print(f"leg {leg}: NO RUN FOUND for {br}")
            continue
        r = match[0]  # most recent
        print(f"leg {leg}: run_id={r['id']} status={r['status']} conclusion={r['conclusion']} "
              f"created={r['created_at']} event={r.get('event')}")


if __name__ == "__main__":
    main()
