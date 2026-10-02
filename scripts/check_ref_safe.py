#!/usr/bin/env python3
"""check_ref_safe.py — POST-safety guard (wave-519 AG-115).

Closes infra lesson x518 #4 (incident AG-10/AG-49): concurrency groups are
per-REF, so a workflow_dispatch POST on a ref that already has a live
(queued or in_progress) leg CANCELS that leg — same-ref/other-seed is NOT
covered by concurrency. Rule canon: "не диспатчить на ref с живой ногой".

This gate is the other half of POST-safety next to seed_gate.py (seeds) —
this one guards REFS. Run it BEFORE every workflow_dispatch POST:

Usage:
  python3 check_ref_safe.py <ref> [<ref> ...]   # exit 0 = all safe, 1 = UNSAFE
  python3 check_ref_safe.py --census            # live queue census (all refs)

Refuses to guard master bench POSTs (canary re-dispatch is coordinator-only,
SWARM_PROMPT x519 infra rule 3). Read-only: only GET requests, token from
/tmp/gh_token (never overwritten).
"""
import json
import os
import sys
import urllib.request

REPO = "PLANETA9091/c-crussty"
API = f"https://api.github.com/repos/{REPO}/actions/runs"
TOKEN_PATH = "/tmp/gh_token"
CANON = ("canary re-dispatch is COORDINATOR-only (SWARM_PROMPT x519 infra#3); "
         "agents must not POST bench workflows on master")


def _token():
    with open(TOKEN_PATH) as fh:
        return fh.read().strip()


def _get(url):
    req = urllib.request.Request(url, headers={
        "Authorization": f"Bearer {_token()}",
        "Accept": "application/vnd.github+json",
    })
    with urllib.request.urlopen(req, timeout=30) as resp:
        return json.load(resp)


def live_runs_on_ref(ref, per_page=100):
    """All queued/in_progress runs on head_branch==ref (both statuses polled)."""
    found = []
    for status in ("queued", "in_progress"):
        url = f"{API}?status={status}&per_page={per_page}"
        for run in _get(url).get("workflow_runs", []):
            if run.get("head_branch") == ref:
                found.append({
                    "run_id": run["id"],
                    "workflow": run.get("name"),
                    "status": run["status"],
                    "seed/event": run.get("event"),
                    "created": run.get("created_at"),
                })
    return found


def census(per_page=100):
    """Global live census: counts + per-ref breakdown of bench-critical legs."""
    out = {"queued": 0, "in_progress": 0, "refs": {}}
    for status in ("queued", "in_progress"):
        data = _get(f"{API}?status={status}&per_page=1")
        out[status] = data.get("total_count", 0)
    seen = {}
    page = 1
    while True:
        data = _get(f"{API}?status=in_progress&per_page={per_page}&page={page}")
        runs = data.get("workflow_runs", [])
        for run in runs:
            ref = run.get("head_branch") or "?"
            seen.setdefault(ref, []).append(run["id"])
        if len(runs) < per_page or page >= 10:
            break
        page += 1
    out["refs"] = {ref: ids for ref, ids in sorted(seen.items())}
    return out


def main(argv):
    if "--census" in argv:
        c = census()
        print(f"CENSUS repo={REPO}")
        print(f"  queued={c['queued']} in_progress={c['in_progress']}")
        for ref, ids in c["refs"].items():
            mark = "  [BENCH]" if ("519-" in ref or "515-" in ref or "518-" in ref) else ""
            print(f"  {ref}: {len(ids)} live {ids[:6]}{mark}")
        return 0

    refs = [a for a in argv[1:] if not a.startswith("-")]
    if not refs:
        print(__doc__)
        return 2
    rc = 0
    for ref in refs:
        if ref == "master":
            print(f"[REF-GATE] {ref}: UNSAFE — {CANON}")
            rc = 1
            continue
        live = live_runs_on_ref(ref)
        if live:
            rc = 1
            print(f"[REF-GATE] {ref}: UNSAFE — {len(live)} live leg(s), "
                  f"POST would CANCEL them (lesson x518#4):")
            for r in live:
                print(f"    run-{r['run_id']} {r['workflow']} {r['status']} {r['created']}")
        else:
            print(f"[REF-GATE] {ref}: SAFE — no queued/in_progress runs on ref")
    if rc:
        print("REF-GATE: BLOCKED — pick another ref (alias branch per leg) or wait.")
    return rc


if __name__ == "__main__":
    sys.exit(main(sys.argv))
