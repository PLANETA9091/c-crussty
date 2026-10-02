#!/usr/bin/env python3
"""AG-185 harvest: RAM-only artifact download + key-number extraction."""
import sys, io, zipfile, json, subprocess, re

TOK = open('/tmp/gh_token').read().strip()
REPO = "PLANETA9091/c-crussty"

def get_artifact(art_id):
    url = f"https://api.github.com/repos/{REPO}/actions/artifacts/{art_id}/zip"
    for attempt in range(4):
        p = subprocess.run(["curl", "-sL", "-H", f"Authorization: token {TOK}", url],
                           capture_output=True)
        data = p.stdout
        if len(data) > 1000 and data[:2] == b'PK':
            return zipfile.ZipFile(io.BytesIO(data))
        print(f"  retry {attempt}: len={len(data)} head={data[:40]!r}", file=sys.stderr)
    return None

def main():
    art_id, tag = sys.argv[1], sys.argv[2]
    z = get_artifact(int(art_id))
    if z is None:
        print(f"[{tag}] DOWNLOAD-FAIL"); return
    names = z.namelist()
    print(f"[{tag}] files ({len(names)}):")
    for n in names[:60]:
        print("   ", n, z.getinfo(n).file_size)
    # dump everything small (<2MB) to workdir
    import os
    os.makedirs(f"/home/z/c-crussty/work/AG-185/harvest_{tag}", exist_ok=True)
    for n in names:
        if z.getinfo(n).file_size < 2_000_000:
            try:
                with open(f"/home/z/c-crussty/work/AG-185/harvest_{tag}/{os.path.basename(n) or 'f'}", "wb") as f:
                    f.write(z.read(n))
            except Exception as e:
                print("  skip", n, e)
    # print tail of main log if present
    for cand in ("server-stdout.log", "bench_summary.json", "summary.json", "result.json"):
        matches = [n for n in names if n.endswith(cand)]
        for m in matches:
            data = z.read(m).decode('utf-8', 'replace')
            print(f"[{tag}] === {m} (last 80 lines) ===")
            print("\n".join(data.splitlines()[-80:]))

if __name__ == "__main__":
    main()
