#!/usr/bin/env python3
"""dispatch_450c.py — TASK-450-C (chunk-pipeline axis, закон 8) — union carrier
cmp450_chunk = chunk4 (send snapshot) ⊕ chunk5 (encode-cache) on fresh master.
УРОК ×447: argv-guard ПЕРЕД любым действием. --dry-run = только префлайт."""
import json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

LEGS = [
    ("chunkU-1", "round-450c-chunk-1", "round-450c-chunk", "cmp450_chunk"),
    ("chunkU-2", "round-450c-chunk-2", "round-450c-chunk", "cmp450_chunk"),
    ("chunkU-3", "round-450c-chunk-3", "round-450c-chunk", "cmp450_chunk"),
]

EXPECTED_SHA = {"round-450c-chunk": "9997b5d6"}

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
    return re.match(r"^https://[^:]+:([^@]+)@github.com/", url).group(1)


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
        if e.code != 404:
            print(f"HTTP {e.code}: {e.read()[:200]}")
        raise
    return json.loads(body) if body else {}


def sha_of(tok, ref):
    return api(tok, f"/repos/{REPO}/git/ref/heads/{ref}")["object"]["sha"]


def ancestry_ok(tok, pin, ref):
    try:
        cmp = api(tok, f"/repos/{REPO}/compare/{pin}...{ref}")
        return cmp.get("status") in ("ahead", "identical")
    except Exception:
        return False


def ensure_branch(tok, branch, base):
    try:
        return sha_of(tok, branch)
    except urllib.error.HTTPError:
        base_sha = sha_of(tok, base)
        api(tok, f"/repos/{REPO}/git/refs", method="POST",
            data={"ref": f"refs/heads/{branch}", "sha": base_sha})
        return base_sha


def main():
    args = sys.argv[1:]
    if "--help" in args or "-h" in args:
        print("usage: dispatch_450c.py [--dry-run] [--leg name:branch:base:lever ...]")
        print("  --dry-run  preflight only, NO dispatch (always safe)")
        raise SystemExit(0)
    dry = "--dry-run" in args
    extra = []
    args_l = list(args)
    i = 0
    while i < len(args_l):
        if args_l[i] == "--leg" and i + 1 < len(args_l):
            extra.append(args_l[i + 1]); i += 2
        else:
            i += 1
    legs = list(LEGS)
    for spec in extra:
        leg, branch, base, lever = spec.split(":")
        legs.append((leg, branch, base, lever))

    tok = token_from_remote()
    print("=== TASK-450-C DISPATCH (chunk-pipeline union cmp450_chunk) ===", flush=True)
    bases = {b for _, _, b, _ in legs}
    for base in sorted(bases):
        exp = EXPECTED_SHA.get(base)
        live = sha_of(tok, base)[:8]
        if exp is None:
            raise SystemExit(f"NO PIN for base {base} — add EXPECTED_SHA entry (argv-guard canon)")
        # ancestry-only canon (x440c step-3c: dispatcher commit moves the tip,
        # self-referential sha pins always abort) — pin must be an ANCESTOR.
        if not ancestry_ok(tok, exp, base):
            raise SystemExit(f"ANCESTRY MISMATCH: {base} live={live} pin={exp}")
        print(f"preflight OK (ancestry): {base} @ {live} (pin {exp})", flush=True)
    if dry:
        print("DRY-RUN OK — no dispatches", flush=True)
        return
    for leg, branch, base, lever in legs:
        sha = ensure_branch(tok, branch, base)
        inputs = dict(INPUTS)
        inputs["lever_flag"] = lever
        inputs["lever_arg"] = "1"
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": branch, "inputs": inputs})
        print(f"dispatched {leg}: {branch} <- {base} lever='{lever}' sha={sha[:8]}", flush=True)
        time.sleep(4)
    print(f"=== TASK-450-C BATCH COMPLETE: {len(legs)} диспатчей ===", flush=True)


if __name__ == "__main__":
    main()
