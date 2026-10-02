#!/usr/bin/env python3
"""poll_479_b1.py — ожидание complete ран 479-B1 + normtool-вердикт пары.

Гейты (v5-FROZEN / канон): band [6.0,9.5]M, M1 STW≤23.0s ∧ young≤200ms,
FIXTURE-VALID, NCDFE=0, AIOOBE=0, ARM-пруф «cmp456_chunkmono_p31snap: ARMED»
в leg server-stdout (из артефакта), vanilla-якорь без lever.
Pair = leg_norm − anchor_norm, legal only if Δcpu ≤50k; ≥+20 → POST-MERGE-PAIR.
"""
import json, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
RUNS = {"round-479-b1-a1": 36376489317, "round-479-b1-leg": 36376515194}
WORKDIR = "/home/z/rounds/ROUND-479/b1/art"
DEADLINE = time.time() + 26 * 60  # общий бюджет тика


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url):
    req = urllib.request.Request(API + url, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json",
        "User-Agent": "479-b1-poller"})
    with urllib.request.urlopen(req, timeout=60) as r:
        return json.loads(r.read())


def main():
    tok = token()
    done = {}
    while len(done) < len(RUNS) and time.time() < DEADLINE:
        for alias, rid in RUNS.items():
            if alias in done:
                continue
            r = api(tok, f"/repos/{REPO}/actions/runs/{rid}")
            print(f"{alias} {rid}: {r['status']}/{r.get('conclusion')}", flush=True)
            if r["status"] == "completed":
                done[alias] = r.get("conclusion")
        if len(done) < len(RUNS):
            time.sleep(120)
    if len(done) < len(RUNS):
        print(json.dumps({"verdict": "DISPATCHED",
                          "runs": {a: f"{i} in-flight" for a, i in RUNS.items()}},
                         ensure_ascii=False))
        return
    print("CONCLUSIONS=" + json.dumps(done), flush=True)
    ids = list(RUNS.values())
    out = subprocess.run(["python3", "/home/z/c-crussty/scripts/normtool_478.py",
                          "--run-id", str(ids[0]), "--run-id", str(ids[1]),
                          "--workdir", WORKDIR], capture_output=True, text=True)
    print(out.stdout)
    print(out.stderr[-1500:], file=sys.stderr)
    try:
        lines = [json.loads(l) for l in out.stdout.strip().splitlines() if l.startswith("{")]
        a1 = next(x for x in lines if x["run_id"] == ids[0])
        leg = next(x for x in lines if x["run_id"] == ids[1])
        pair = (round(leg["norm_v5"] - a1["norm_v5"], 2)
                if a1["norm_v5"] is not None and leg["norm_v5"] is not None else None)
        dcpu = abs(leg["cpu_index"] - a1["cpu_index"])
        print("PAIR_JSON=" + json.dumps({
            "anchor": {"run": ids[0], "norm": a1["norm_v5"], "cpu": a1["cpu_index"],
                       "verdict": a1["verdict"], "stw": a1["host_M1"]["stw_total_s"]},
            "leg": {"run": ids[1], "norm": leg["norm_v5"], "cpu": leg["cpu_index"],
                    "verdict": leg["verdict"], "stw": leg["host_M1"]["stw_total_s"],
                    "armed": leg.get("armed_marker")},
            "pair": pair, "dcpu": dcpu, "legal_pair": dcpu <= 50000},
            ensure_ascii=False), flush=True)
    except Exception as e:
        print(f"PAIR-PARSE-FAIL: {e}", file=sys.stderr)


if __name__ == "__main__":
    main()
