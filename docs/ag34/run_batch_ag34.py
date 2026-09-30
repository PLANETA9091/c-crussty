#!/usr/bin/env python3
"""run_batch_ag34.py — GATE-3 банк-харвест AG-34 (волна-517), GET-only, 0 диспатчей.
Фазы: 1) метаданные артефактов (параллельно) 2) загрузки zip (параллельно, retry)
3) normtool_478.parse (кэш-zip, параллельно) → norm_batch_*.jsonl
"""
import json, os, subprocess, sys
from concurrent.futures import ThreadPoolExecutor

sys.path.insert(0, "/home/z/c-crussty/scripts")
os.chdir("/home/z/rounds/ROUND-517/work/AG-34")
os.makedirs("art", exist_ok=True)
os.makedirs("meta", exist_ok=True)

TOKEN = open("/tmp/gh_token").read().strip()
BASE = "https://api.github.com/repos/PLANETA9091/c-crussty"
RUNS = ["36755725909", "36755741812", "36757148594", "36757163912",
        "36760406926", "36760415164", "36760450715", "36760454052",
        "36755165921", "36752032802", "36752909115", "36750126063", "36750140047"]


def curl(url, out=None, extra=None):
    cmd = ["curl", "-s", "-H", f"Authorization: token {TOKEN}"]
    if extra:
        cmd += extra
    if out:
        cmd += ["-o", out]
    cmd.append(url)
    return subprocess.run(cmd, capture_output=True, text=True)


def get_url(rid):
    r = curl(f"{BASE}/actions/runs/{rid}/artifacts")
    try:
        d = json.loads(r.stdout)
        a = next((a for a in d.get("artifacts", []) if a["name"] == "world3-bench"), None)
        return rid, (a["archive_download_url"] if a else None)
    except Exception as e:
        return rid, f"META-ERR {e}"


def download(rid_url):
    rid, url = rid_url
    if not url or not url.startswith("http"):
        return rid, f"SKIP {url}"
    zpath = f"art/art_{rid}.zip"
    for attempt in range(4):
        r = subprocess.run(["curl", "-sL", "-H", f"Authorization: token {TOKEN}",
                            "--retry", "2", "-o", zpath, url])
        if r.returncode == 0:
            try:
                import zipfile
                zipfile.ZipFile(zpath).namelist()
                return rid, f"OK {os.path.getsize(zpath)}"
            except Exception as e:
                err = f"zip-bad {e}"
        else:
            err = f"curl rc={r.returncode}"
    return rid, f"FAIL {err}"


def parse(rid):
    r = subprocess.run([sys.executable, "/home/z/c-crussty/scripts/normtool_478.py",
                        "--run-id", rid, "--workdir", "art", "--biomes-exempt"],
                       capture_output=True, text=True)
    if r.stdout.strip():
        with open(f"norm_batch_{rid}.jsonl", "w") as f:
            f.write(r.stdout.strip() + "\n")
        return rid, "PARSED"
    return rid, f"PARSE-ERR {r.stderr.strip()[:200]}"


with ThreadPoolExecutor(6) as ex:
    urls = list(ex.map(get_url, RUNS))
print("== meta:", urls, flush=True)

with ThreadPoolExecutor(6) as ex:
    dl = list(ex.map(download, urls))
print("== dl:", dl, flush=True)

good = [rid for rid, st in dl if st.startswith("OK")]
with ThreadPoolExecutor(6) as ex:
    ps = list(ex.map(parse, good))
print("== parse:", ps, flush=True)
