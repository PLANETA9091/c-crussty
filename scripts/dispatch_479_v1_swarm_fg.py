#!/usr/bin/env python3
"""479-V1 swarm idempotent foreground runner (rescue after sandbox reaped bg PID).
Reuses canon/aliases/api from prereg dispatch_479_v1_swarm.py (закон 14a/16).
Per alias: if run already registered since MC → skip POST (idempotent), else POST.
Then collect run-ids ×12, single band-dead sweep → free re-roll W3."""
import json, sys, time
sys.path.insert(0, "/home/z/c-crussty/scripts")
from dispatch_479_v1_swarm import (api, token, dispatch, list_runs, ALIASES,
                                  PAUSE_S, IDLE_POLL_S)

MC = "2026-09-28T05:10:00Z"  # волна стартовала 05:12-05:13Z (log mtime), mc с запасом


def wall_s(v, now):
    t0 = time.mktime(time.strptime(v["created"], "%Y-%m-%dT%H:%M:%SZ"))
    return now - t0


def main():
    tok = token()
    have = list_runs(tok, MC)
    print(f"pre-check: {len(have)}/{len(ALIASES)} runs already registered: "
          + ", ".join(f"{k.split('-')[-1]}={v['id']}" for k, v in sorted(have.items())),
          flush=True)

    # --- добить POST-волну (15s пауза между POST, idempotent skip) ---
    for i, a in enumerate(ALIASES):
        if a in have:
            print(f"skip {a}: run {have[a]['id']} exists", flush=True)
            continue
        if i:
            time.sleep(PAUSE_S)
        if not dispatch(tok, a):
            time.sleep(20)
            if not dispatch(tok, a):
                print(f"FATAL dispatch {a}", file=sys.stderr)
                sys.exit(2)

    # --- сбор run-id ×12 ---
    deadline = time.time() + 420
    found = have
    while time.time() < deadline and len(found) < len(ALIASES):
        time.sleep(IDLE_POLL_S)
        found = list_runs(tok, MC)
        print(f"run-ids {len(found)}/{len(ALIASES)}: "
              + ", ".join(f"{k.split('-')[-1]}={v['id']}:{v['status']}"
                          for k, v in sorted(found.items())), flush=True)

    # --- один band-dead сторож-свип: fast-fail → 1 free re-roll (закон W3) ---
    rerolled = []
    sweep_until = time.time() + 200
    while time.time() < sweep_until:
        time.sleep(25)
        now = time.time()
        snap = list_runs(tok, MC)
        for k, v in sorted(snap.items()):
            if v["status"] == "completed" and v["conclusion"] == "failure" \
                    and k not in rerolled and wall_s(v, now) < 400:
                print(f"band-dead {k} (run {v['id']}, wall {wall_s(v, now):.0f}s) "
                      f"→ free re-roll W3", flush=True)
                time.sleep(10)
                dispatch(tok, k)
                rerolled.append(k)

    print("SWARM-RESULT " + json.dumps({
        "aliases": ALIASES, "runs": found, "band_dead_rerolled": rerolled,
        "strict_window": [6900000, 7200000], "strict_hits": "post-hoc (абсорб след. тик)",
    }, indent=1))


if __name__ == "__main__":
    main()
