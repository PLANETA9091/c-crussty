#!/usr/bin/env python3
"""dispatch_456a.py — TASK-456-A leg dispatcher (spawn-carrier rebaze cycle-2).

Canon guard: EXPECTED_SHA pins the branch head; inputs = EXACT canon dict
(25-input workflow; travel_diet/fluid_dirty_ledger never sent; no
concurrency guards). Creates refs round-456a-spawn-{1,2} from
round-456a-spawn via git/refs API, then dispatches world-bench-parallel.yml.

Usage: python3 dispatch_456a.py <1|2|both> [--dry]
"""
import json
import re
import subprocess
import sys
import time
import urllib.request

BRANCH = "round-456a-spawn"
EXPECTED_SHA = "c228e058"  # rebaze-3 merge head (ancestry: code carrier 47ea8b20 ⊕ 797ae4f0)
ANCESTRY_NOTE = "rebase merge fe7e7120 (carrier c25782b4 ⊕ master fd790a17 union) + rust fixes a41d182c"

INPUTS = {
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    "lever_flag": "cmp455_spawn", "lever_arg": "1",
}

REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}"


def token():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url", "origin"],
                         capture_output=True, text=True).stdout.strip()
    return re.match(r"^https://[^:]+:([^@]+)@github.com/", url).group(1)


def api(method, path, payload=None, tok=""):
    req = urllib.request.Request(f"{API}{path}", method=method,
                                 data=json.dumps(payload).encode() if payload is not None else None,
                                 headers={"Authorization": f"token {tok}",
                                          "Accept": "application/vnd.github+json"})
    try:
        with urllib.request.urlopen(req) as r:
            body = r.read().decode()
            return r.status, (json.loads(body) if body.strip().startswith(("{", "[")) else body)
    except urllib.error.HTTPError as e:
        return e.code, e.read().decode()


def main():
    args = [a for a in sys.argv[1:] if a != "--dry"]
    dry = "--dry" in sys.argv
    if not args or args[0] not in ("1", "2", "both"):
        print(__doc__)
        sys.exit(1)
    which = args[0]
    tok = token()

    # head verification
    st, ref = api("GET", f"/git/ref/heads/{BRANCH}", tok=tok)
    head = ref["object"]["sha"][:8]
    if head != EXPECTED_SHA:
        print(f"ABORT: {BRANCH} head {head} != pinned {EXPECTED_SHA} (update EXPECTED_SHA after intentional push)")
        sys.exit(2)
    print(f"head pin OK: {BRANCH} @ {head} ({ANCESTRY_NOTE})")

    legs = ["1", "2"] if which == "both" else [which]
    for leg in legs:
        name = f"{BRANCH}-{leg}"
        st, resp = api("GET", f"/git/ref/heads/{name}", tok=tok)
        if st == 200:
            cur = resp["object"]["sha"][:8]
            if cur == head:
                print(f"ref {name} already @ head")
            else:
                st2, _ = api("PATCH", f"/git/refs/heads/{name}", {"sha": ref["object"]["sha"], "force": True}, tok=tok)
                print(f"ref {name} patched -> {head} (status {st2})")
        else:
            st2, _ = api("POST", "/git/refs", {"ref": f"refs/heads/{name}", "sha": ref["object"]["sha"]}, tok=tok)
            print(f"ref {name} created @ {head} (status {st2})")
        if dry:
            print(f"DRY: would dispatch {name} inputs={INPUTS}")
            continue
        st3, resp3 = api("POST", "/actions/workflows/world-bench-parallel.yml/dispatches",
                         {"ref": name, "inputs": INPUTS}, tok=tok)
        print(f"dispatch {name}: HTTP {st3}" + ("" if st3 == 204 else f" -> {resp3}"))
        if st3 != 204:
            sys.exit(3)
        time.sleep(2)


if __name__ == "__main__":
    main()
