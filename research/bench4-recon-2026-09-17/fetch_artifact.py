#!/usr/bin/env python3
"""fetch_artifact.py — download a GitHub Actions artifact zip (handles the
302 to the signed Azure blob; the auth header must NOT be forwarded there)
and extract log-ish entries for marker grepping.

Usage: python3 fetch_artifact.py <artifact_id> <out_dir>
"""
import io
import os
import re
import sys
import zipfile
import urllib.request


def token_from_creds(path="~/.git-credentials"):
    line = open(os.path.expanduser(path)).read().strip().splitlines()[0]
    return re.match(r"^https://[^:]+:([^@]+)@github\.com$", line).group(1)


def main():
    aid, outdir = sys.argv[1], sys.argv[2]
    tok = token_from_creds()
    url = (f"https://api.github.com/repos/PLANETA9091/c-crussty/"
           f"actions/artifacts/{aid}/zip")
    req = urllib.request.Request(url, headers={
        "Authorization": f"Bearer {tok}",
        "Accept": "application/vnd.github+json"})

    class NoRedirect(urllib.request.HTTPRedirectHandler):
        def redirect_request(self, req, fp, code, msg, headers, newurl):
            return None  # surface the 302 as HTTPError with its headers

    opener = urllib.request.build_opener(NoRedirect)
    loc = None
    try:
        opener.open(req, timeout=120)
        print("unexpected: 200 without redirect")
        return 1
    except urllib.error.HTTPError as e:
        if e.code in (301, 302, 303, 307):
            loc = e.headers.get("Location")
        if not loc:
            print(f"HTTP {e.code}, no Location")
            return 1
    # follow the signed blob URL WITHOUT the auth header
    data = urllib.request.urlopen(loc, timeout=300).read()
    print("downloaded", len(data), "bytes")
    os.makedirs(outdir, exist_ok=True)
    zf = zipfile.ZipFile(io.BytesIO(data))
    names = zf.namelist()
    print("entries:", len(names))
    for n in names:
        # S7-128: + collapsed stacks + bottleneck report — profile ranking
        # for the ARCH-ATTACK queue needs cpu/wall/alloc collapsed stacks
        # and BOTTLENECKS_3.md, not only log-ish marker carriers.
        if re.search(r"(log|console|latest|run-env|entity-recon|tickmonitor|collapsed|BOTTLENECKS)", n, re.I):
            try:
                content = zf.read(n)
                if len(content) < 40_000_000:
                    out = os.path.join(outdir, n.replace("/", "_"))
                    open(out, "wb").write(content)
                    print("saved", out, len(content))
            except Exception as ex:
                print("skip", n, ex)
    return 0


if __name__ == "__main__":
    sys.exit(main())
