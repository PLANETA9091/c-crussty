#!/usr/bin/env python3
"""fetch_job_logs.py — download GitHub Actions job log (NoRedirect pattern).
Usage: fetch_job_logs.py <run_id> <out_file>
Token from the origin remote URL (rule 1b, secret-free file)."""
import re
import subprocess
import sys
import urllib.request
import json


def token_from_remote():
    url = subprocess.run(
        ["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
        capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token in origin remote URL")
    return m.group(1)


def main():
    run_id, out = sys.argv[1], sys.argv[2]
    tok = token_from_remote()
    hdr = {"Authorization": f"Bearer {tok}",
           "Accept": "application/vnd.github+json",
           "User-Agent": "agent"}
    req = urllib.request.Request(
        f"https://api.github.com/repos/PLANETA9091/c-crussty/"
        f"actions/runs/{run_id}/jobs", headers=hdr)
    d = json.loads(urllib.request.urlopen(req, timeout=60).read())
    jobs = d.get("jobs", [])
    if not jobs:
        print("no jobs", file=sys.stderr)
        return 2
    jid = jobs[0]["id"]

    class NoRedirect(urllib.request.HTTPRedirectHandler):
        def redirect_request(self, req, fp, code, msg, headers, newurl):
            return None

    opener = urllib.request.build_opener(NoRedirect)
    zreq = urllib.request.Request(
        f"https://api.github.com/repos/PLANETA9091/c-crussty/"
        f"actions/jobs/{jid}/logs", headers=hdr)
    try:
        resp = opener.open(zreq, timeout=60)
        data = resp.read()
    except urllib.error.HTTPError as e:
        if e.code in (301, 302, 303, 307, 308):
            loc = e.headers["Location"]
            r2 = urllib.request.Request(loc, headers={"User-Agent": "agent"})
            data = urllib.request.urlopen(r2, timeout=120).read()
        else:
            raise
    with open(out, "wb") as f:
        f.write(data)
    print(f"job log {jid}: {len(data)} bytes -> {out}")
    return 0


if __name__ == "__main__":
    sys.exit(main())
