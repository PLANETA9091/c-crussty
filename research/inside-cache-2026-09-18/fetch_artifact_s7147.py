#!/usr/bin/env python3
"""fetch_artifact_s7147.py — download a GitHub Actions artifact zip (handles the
302 to the signed Azure blob; the auth header must NOT be forwarded there)
and extract into out_dir. Token baked per owner rule 1b (Job 394666).

Usage: python3 fetch_artifact_s7147.py <artifact_id> <out_dir>
"""
import io
import os
import re
import sys
import zipfile
import urllib.request

TOKEN = __import__("os").environ.get("CRUSSTY_GH_TOKEN", "")  # set per owner rule 1b (cron directive)
REPO = "PLANETA9091/c-crussty"


def main():
    aid, outdir = sys.argv[1], sys.argv[2]
    os.makedirs(outdir, exist_ok=True)
    url = (f"https://api.github.com/repos/{REPO}/"
           f"actions/artifacts/{aid}/zip")
    req = urllib.request.Request(url, headers={
        "Authorization": f"Bearer {TOKEN}",
        "Accept": "application/vnd.github+json"})

    class NoRedirect(urllib.request.HTTPRedirectHandler):
        def redirect_request(self, req, fp, code, msg, headers, newurl):
            return None

    opener = urllib.request.build_opener(NoRedirect)
    try:
        resp = opener.open(req, timeout=120)
        body = resp.read()
    except urllib.error.HTTPError as e:
        if e.code in (301, 302, 303, 307):
            loc = e.headers["Location"]
            # signed azure blob: no auth header
            req2 = urllib.request.Request(loc)
            body = urllib.request.urlopen(req2, timeout=600).read()
        else:
            raise
    zipf = zipfile.ZipFile(io.BytesIO(body))
    zipf.extractall(outdir)
    names = zipf.namelist()
    print(f"extracted {len(names)} entries -> {outdir}")
    for n in names:
        print(" ", n)
    return 0


if __name__ == "__main__":
    sys.exit(main())
