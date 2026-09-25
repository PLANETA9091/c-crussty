#!/usr/bin/env python3
"""dispatch_436c_golden.py — GOLDEN-SLOT batch for tick 02:08 (canon v2 window
02:2x-03:3x: certification ONLY here). PAIR-COMPLETION plan for the chunkpl
line (strongest alive era line outside composite: +22.1/+17.7/+21.9 norm,
all no-pair — legs 6.65-6.71M vs anchors 6.76M+, gap 55-60k).

READY — DO NOT RUN from the C tick; the golden slot (tick 02:08) executes:
    python3 /home/z/rounds/ROUND-436/dispatch_436c_golden.py

Batch = 14 runs, big-interleaved ONE window (canon: anchors supply the pair
zone 6.60-6.76M around every leg):
  - 8 anchors  @master                  lever=""          (round-436g-anchor-*)
  - 4 legs     @round-435-c-chunkpl2    lever=cmp434_chunkpl (round-436g-chk-*)
  - 2 legs     @round-436-c-chunk3      lever=cmp435_chunk3 (round-436g-chk3-*)
After dispatch: absorb via scripts/bench4_recon/absorb_round.py, verdict gates
band 6.0-9.5M / anchors norm >= -2% / pair-by-runner Delta<=50k / min-of-3 /
selfTest==true / NCDFE=0 / AIOOBE=0 / threw=0 / ARM markers
(parse-cache first hit, biomes-cache first hit, chunk3 armed).
"""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BATCH = [
    ("anchor-1",  "round-436g-anchor-1", "master",               ""),
    ("anchor-2",  "round-436g-anchor-2", "master",               ""),
    ("chk-1",     "round-436g-chk-1",    "round-435-c-chunkpl2", "cmp434_chunkpl"),
    ("anchor-3",  "round-436g-anchor-3", "master",               ""),
    ("chk3-1",    "round-436g-chk3-1",   "round-436-c-chunk3",   "cmp435_chunk3"),
    ("anchor-4",  "round-436g-anchor-4", "master",               ""),
    ("chk-2",     "round-436g-chk-2",    "round-435-c-chunkpl2", "cmp434_chunkpl"),
    ("anchor-5",  "round-436g-anchor-5", "master",               ""),
    ("chk3-2",    "round-436g-chk3-2",   "round-436-c-chunk3",   "cmp435_chunk3"),
    ("anchor-6",  "round-436g-anchor-6", "master",               ""),
    ("chk-3",     "round-436g-chk-3",    "round-435-c-chunkpl2", "cmp434_chunkpl"),
    ("anchor-7",  "round-436g-anchor-7", "master",               ""),
    ("chk-4",     "round-436g-chk-4",    "round-435-c-chunkpl2", "cmp434_chunkpl"),
    ("anchor-8",  "round-436g-anchor-8", "master",               ""),
]

INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
}


def token_from_remote():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github\.com/", url)
    if not m:
        raise SystemExit("no token in origin remote URL")
    return m.group(1)


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:200]}")
        raise
    return json.loads(body) if body else {}


def sha_of(tok, ref):
    return api(tok, f"/repos/{REPO}/git/ref/heads/{ref}")["object"]["sha"]


def ensure_branch(tok, branch, base):
    try:
        return sha_of(tok, branch)
    except urllib.error.HTTPError:
        base_sha = sha_of(tok, base)
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{branch}", "sha": base_sha})
        return base_sha


if __name__ == "__main__":
    tok = token_from_remote()
    for leg, branch, base, lever in BATCH:
        sha = ensure_branch(tok, branch, base)
        inputs = dict(INPUTS)
        inputs["lever_flag"] = lever
        inputs["lever_arg"] = "1"
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": branch, "inputs": inputs})
        print(f"dispatched {leg}: branch={branch} base={base} lever='{lever}' sha={sha[:8]}", flush=True)
        time.sleep(4)
