#!/usr/bin/env python3
"""fetch_artifact.py — download a world-bench run artifact (S7-160).
Usage: fetch_artifact.py <run_id> <dest_dir>
Token from the origin remote URL (rule 1b, secret-free file)."""
import re
import subprocess
import sys
import urllib.request
import zipfile
import os


def token_from_remote():
    url = subprocess.run(
        ["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
        capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token in origin remote URL")
    return m.group(1)


def main():
    run_id, dest = sys.argv[1], sys.argv[2]
    tok = token_from_remote()
    hdr = {"Authorization": f"Bearer {tok}",
           "Accept": "application/vnd.github+json"}
    req = urllib.request.Request(
        f"https://api.github.com/repos/PLANETA9091/c-crussty/"
        f"actions/runs/{run_id}/artifacts", headers=hdr)
    d = __import__("json").loads(urllib.request.urlopen(req, timeout=60).read())
    arts = d.get("artifacts", [])
    if not arts:
        print("no artifacts", file=sys.stderr)
        return 2
    a = arts[0]
    print(f"artifact: {a['name']} size={a['size_in_bytes']} id={a['id']}")

    class NoRedirect(urllib.request.HTTPRedirectHandler):
        def redirect_request(self, req, fp, code, msg, headers, newurl):
            return None

    # 302 to the signed Azure blob: the auth header must NOT be forwarded
    # (fetch_artifact_s7147.py lesson).
    opener = urllib.request.build_opener(NoRedirect)
    zreq = urllib.request.Request(
        f"https://api.github.com/repos/PLANETA9091/c-crussty/"
        f"actions/artifacts/{a['id']}/zip", headers=hdr)
    os.makedirs(dest, exist_ok=True)
    zpath = os.path.join(dest, f"{a['name']}.zip")
    try:
        resp = opener.open(zreq, timeout=120)
        body = resp.read()
    except urllib.error.HTTPError as e:
        if e.code in (301, 302, 303, 307):
            loc = e.headers["Location"]
            breq = urllib.request.Request(loc)  # no auth on the blob host
            with urllib.request.urlopen(breq, timeout=600) as r, \
                    open(zpath, "wb") as f:
                while True:
                    chunk = r.read(1 << 20)
                    if not chunk:
                        break
                    f.write(chunk)
        else:
            raise
    print(f"downloaded {zpath} ({os.path.getsize(zpath)} bytes)")
    with zipfile.ZipFile(zpath) as z:
        z.extractall(dest)
    os.remove(zpath)
    print("extracted to", dest)
    return 0


if __name__ == "__main__":
    sys.exit(main())
