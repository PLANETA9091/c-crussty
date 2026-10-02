#!/usr/bin/env python3
"""dispatch_round398.py — TASK-398 ноги: ре-анкер банк-v5 + репликации 2-й ногой.
Usage: python3 dispatch_round398.py <leg>
  legs: anchor1 | anchor2 | sweep2 | soa | footprint | jsubsys2 | compose

Ре-анкер банк-v5 = банк v4 + items_oss (lever_flag=items_oss, master 4f927bd+).
Репликации = 2-я нога веток РАУНДА-2 (sweep2 +8.1% / soa +7.5% / footprint +7.2%).
anchor1/anchor2 ОДИН ref=master → ПОСЛЕДОВАТЕЛЬНО (concurrency per-ref, cancel-in-progress).
Остальные — разные refs → параллельно. НЕ слать travel_diet/fluid_dirty_ledger (удалены).
"""
import json, re, subprocess, sys, time, urllib.request

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

LEGS = {
    "anchor1":   {"ref": "master",                "lever": "items_oss"},
    "anchor2":   {"ref": "master",                "lever": "items_oss"},
    "sweep2":    {"ref": "round-397-e-sweep2",    "lever": "items_sweep2"},
    "soa":       {"ref": "round-397-c-soa",       "lever": "items_soa"},
    "footprint": {"ref": "round-397-g-footprint", "lever": "items_footprint"},
    "jsubsys2":  {"ref": "round-398-j-subsys2",   "lever": "items_subsys2"},
    "compose":   {"ref": "round-398-compose",     "lever": "items_compose398"},
}

# БАНК v4 (бит-в-бит из world-bench-parallel.yml: world_url default; natives default)
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
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {}


def main():
    leg = sys.argv[1] if len(sys.argv) > 1 else ""
    if leg not in LEGS:
        print(f"unknown leg {leg!r}; one of {list(LEGS)}", file=sys.stderr)
        return 1
    spec = LEGS[leg]
    tok = token_from_remote()

    d = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?per_page=20")
    clash = [r for r in d.get("workflow_runs", [])
             if r["head_branch"] == spec["ref"].split("/")[-1] and
             r["status"] in ("in_progress", "queued", "waiting")]
    if clash:
        print(f"per-ref concurrency clash on {spec['ref']}: run {clash[0]['id']} "
              f"{clash[0]['status']} — dispatch BLOCKED (same ref runs sequentially)")
        return 1

    inputs = dict(INPUTS)
    inputs["lever_flag"] = spec["lever"]
    inputs["lever_arg"] = "1"
    api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
        method="POST", data={"ref": spec["ref"], "inputs": inputs})
    print(f"dispatch POST sent: leg={leg} ref={spec['ref']} lever={spec['lever']}")
    for _ in range(12):
        time.sleep(5)
        d = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/runs?per_page=30")
        for r in d.get("workflow_runs", []):
            if r["head_branch"] == spec["ref"].split("/")[-1] and \
               r["created_at"] > time.strftime("%Y-%m-%dT%H:%M", time.gmtime(time.time() - 300)):
                print(json.dumps({"leg": leg, "run_id": r["id"], "status": r["status"],
                                  "head_sha": r["head_sha"][:7], "created": r["created_at"]}))
                return 0
    print("no run appeared after 60s", file=sys.stderr)
    return 2


if __name__ == "__main__":
    sys.exit(main())
