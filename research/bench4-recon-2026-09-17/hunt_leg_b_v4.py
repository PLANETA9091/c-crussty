#!/usr/bin/env python3
"""hunt_leg_b.py v4 — §125-AMENDMENT-1: slow-track baseline+pack hunt.

AMENDMENT CAUSE (measured, S7-119): the §125 bank pair (run#17 9080657,
run#21 8914646, MID class) can no longer be sampled — census of 43 legs: the
mid window [8899044,9092939] was hit only by the two bank legs themselves
(22:10Z/01:13Z era); since then 0/14 in-window (two mid legs 8869954/8875106
missed by ~0.3%); slow class 6.57-6.87M dominates (~30% of draws). Reusing the
banked MID baseline against fresh legs is unwinnable in practice — NOT by
machinery (v3 fixed that) but by runner-population shift.

AMENDMENT (all owner gates intact: >=3% MSPT, CI A/B min-of-2, median-exact
parity, world pin, +/-2% harness-cpu pairing):
  - Baseline arm #1 (FREE, already banked): run#18 35159240368, cpu 6746569,
    MSPT 85.24, FIXTURE-VALID (S7-102), pre-pack kernel (f3c82b3-era, S7-102,
    predates F1 impl S7-111 and F1 hook S7-112), world afb3a0b3 (confirmed).
  - Baseline arm #2 (B1): FRESH dispatch on PRE-PACK ref 962fc9f (S7-111,
    RandomTickOps.java banked but NOT hooked => zero pack behavior), band =
    run#18's +/-2% window [6611637,6881501] so the pair can intersect.
  - PACK WINDOW = [max(cpu18,B1)*0.98, min(cpu18,B1)*1.02] (pairs legally with
    BOTH arms; if empty => B1 honest discard, redispatch).
  - Pack arms: master ref, band = window padded 0.5%, exact-window final check.
  - VERDICT: pack median MSPT <= median(85.24, B1_mspt)*0.97 at 2 in-window
    pack legs => pack lands; else REFUTED row, zero landing.

One transition per --once call (resumable; background dies with bash call):
  no state        -> guard -> dispatch (phase-dependent ref/band) -> save
  state=dispatched-> completed: failure/gate => reject, clear (exit 1)
                             success: world pin + phase window check =>
                             baseline arm banked (exit 0) / pack LEG (exit 0)
                             / out => discard, clear (exit 2)
                   still running: early echo => in-window keep (4) /
                             out-of-window cancel (2) / no echo yet (4)
Exit codes: 0 = transition done (arm/leg found or dispatched) | 1 = gate
            reject | 2 = false leg cancelled/discarded | 3 = guard abort
            | 4 = in flight, keep polling
"""
import json, os, re, sys, time, urllib.request

API = "https://api.github.com"
REPO = "PLANETA9091/c-crussty"
TOKEN_ERROR = "token"

# ---- v4 constants (see docstring) ----
# NOTE: workflow_dispatch API accepts only branch/tag refs (SHA => HTTP 422,
# caught live S7-119) => audit tag at 962fc9f pushed to origin.
PRE_PACK_REF = "pre-pack-962fc9f"  # S7-111: impl banked, hook NOT wired
WORLD_SHA = "afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5"
ARM1 = {"run_id": 35159240368, "cpu": 6746569, "mspt": 85.24}  # run#18, banked VALID
ARM1_WIN_LO, ARM1_WIN_HI = int(ARM1["cpu"] * 0.98), int(ARM1["cpu"] * 1.02)
SLACK = 1.005  # gate band padding around final window (skew slack, v3 pattern)

HERE = os.path.dirname(os.path.abspath(__file__))
STATE = os.path.join(HERE, "leg_b_v4_state.json")
RESULT = os.path.join(HERE, "leg_b_v4.json")


def api(token, url, method="GET", data=None):
    req = urllib.request.Request(url, method=method, headers={
        "Authorization": f"Bearer {token}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"  HTTP {e.code}: {e.read()[:160]}", file=sys.stderr)
        return {}


def token_from_creds(path="~/.git-credentials"):
    import os
    line = open(os.path.expanduser(path)).read().strip().splitlines()[0]
    return re.match(r"^https://[^:]+:([^@]+)@github\.com$", line).group(1)


def log_text(token, run_id):
    url = f"{API}/repos/{REPO}/actions/runs/{run_id}/logs"
    req = urllib.request.Request(url, headers={
        "Authorization": f"Bearer {token}", "Accept": "application/vnd.github+json"})
    try:
        with urllib.request.urlopen(req, timeout=120) as r:
            data = r.read()
        import io, zipfile
        zf = zipfile.ZipFile(io.BytesIO(data))
        return "\n".join(zf.read(n).decode("utf-8", "replace")
                         for n in zf.namelist() if n.endswith(".txt"))
    except Exception:
        return ""


def read_state():
    if os.path.exists(STATE):
        return json.load(open(STATE))
    return None


def write_state(s):
    json.dump(s, open(STATE, "w"))


def clear_state():
    if os.path.exists(STATE):
        os.remove(STATE)


def pack_window(cpu2):
    lo = max(ARM1["cpu"], cpu2) * 0.98
    hi = min(ARM1["cpu"], cpu2) * 1.02
    return int(lo), int(hi)


def dispatch(tok, ref, band_lo, band_hi):
    d = api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/runs?per_page=3")
    act = [r["id"] for r in d.get("workflow_runs", [])
           if r.get("status") in ("in_progress", "queued", "waiting")]
    if act:
        print(f"ABORT: run {act[0]} in flight (cancel-in-progress guard)")
        return None
    print(f"dispatch ref={ref[:7]} band=[{band_lo},{band_hi}] fp=4...")
    api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/dispatches",
        method="POST",
        data={"ref": ref, "inputs": {"radius": "640", "seconds": "900",
              "summon_sweeps": "0", "fake_players": "4",
              "cpu_band_min": str(band_lo), "cpu_band_max": str(band_hi)}})
    time.sleep(25)
    d = api(tok, f"{API}/repos/{REPO}/actions/workflows/world-bench.yml/runs?per_page=1")
    top = d.get("workflow_runs", [{}])[0]
    rid = top.get("id")
    # VERIFY the run actually matches this dispatch (ref + fresh status):
    # a 422/failed dispatch must NOT capture a stale run as state (S7-119 lesson).
    if (top.get("head_branch") != ref
            or top.get("status") not in ("queued", "in_progress", "waiting")):
        print(f"  DISPATCH MISMATCH: top run {rid} head_branch={top.get('head_branch')} "
              f"status={top.get('status')} != ref={ref} in-flight -> NO state saved")
        return None
    print(f"  dispatched run {rid} (ref={ref}, state saved)")
    return rid


def poll(tok, st):
    """Returns (action, exit_code). action in {found_baseline, found_pack, none}."""
    rid = st["run_id"]
    s = api(tok, f"{API}/repos/{REPO}/actions/runs/{rid}")
    status, concl = s.get("status"), s.get("conclusion")
    if status != "completed":
        txt = log_text(tok, rid)
        mh = re.search(r"run-env: world_sha256=([0-9a-f]+) runner_cpu_index=(\d+) fake_players=4", txt)
        if mh:
            wsha, cpu = mh.group(1), int(mh.group(2))
            if wsha != WORLD_SHA:
                print(f"  run {rid}: WORLD DRIFT sha={wsha[:12]} -> cancel early")
                api(tok, f"{API}/repos/{REPO}/actions/runs/{rid}/cancel", method="POST")
                time.sleep(15)
                clear_state()
                return None, 2
            lo, hi = (ARM1_WIN_LO, ARM1_WIN_HI) if st["phase"] == "baseline" \
                else (st["win_lo"], st["win_hi"])
            if lo <= cpu <= hi:
                print(f"  run {rid}: harness cpu={cpu} IN {st['phase']} window — keep polling")
                return None, 4
            print(f"  run {rid}: harness cpu={cpu} OUT of {st['phase']} window -> cancel")
            api(tok, f"{API}/repos/{REPO}/actions/runs/{rid}/cancel", method="POST")
            time.sleep(15)
            clear_state()
            return None, 2
        print(f"  run {rid}: running, no harness echo yet — keep polling")
        return None, 4
    # completed
    txt = log_text(tok, rid)
    mh = re.search(r"run-env: world_sha256=([0-9a-f]+) runner_cpu_index=(\d+) fake_players=4", txt)
    if concl == "success" and mh:
        wsha, cpu = mh.group(1), int(mh.group(2))
        if wsha != WORLD_SHA:
            print(f"  run {rid}: WORLD DRIFT sha={wsha[:12]} != pin -> discard, re-baseline required")
            json.dump({"run_id": rid, "cpu": cpu, "world_sha": wsha,
                       "verdict": "WORLD-DRIFT"}, open(RESULT, "w"))
            clear_state()
            return None, 2
        if st["phase"] == "baseline":
            if ARM1_WIN_LO <= cpu <= ARM1_WIN_HI:
                win_lo, win_hi = pack_window(cpu)
                if win_lo <= win_hi:
                    json.dump({"baseline": [ARM1, {"run_id": rid, "cpu": cpu}],
                               "window": [win_lo, win_hi], "world_sha": wsha},
                              open(RESULT, "w"))
                    print(f"BASELINE COMPLETE: arm2={rid} cpu={cpu} -> pack window [{win_lo},{win_hi}]")
                    # seed pack phase (no run_id) so next call dispatches pack arm #1
                    write_state({"phase": "pack", "run_id": None,
                                 "win_lo": win_lo, "win_hi": win_hi,
                                 "baseline": [ARM1, {"run_id": rid, "cpu": cpu}],
                                 "pack_legs": [], "t": time.time()})
                    return "found_baseline", 0
                print(f"  run {rid}: cpu={cpu} in arm1 window but intersection EMPTY -> discard")
                clear_state()
                return None, 2
            print(f"  run {rid}: success cpu={cpu} OUT arm1 window {ARM1_WIN_LO}-{ARM1_WIN_HI} -> discard")
            clear_state()
            return None, 2
        # pack phase
        lo, hi = st["win_lo"], st["win_hi"]
        if lo <= cpu <= hi:
            legs = st.get("pack_legs", [])
            legs.append({"run_id": rid, "cpu": cpu})
            st["pack_legs"] = legs
            write_state(st)
            if len(legs) >= 2:
                json.dump({"pack_legs": legs, "window": [lo, hi],
                           "baseline": st["baseline"], "verdict": "PENDING-ABSORB"},
                          open(RESULT, "w"))
                print(f"PACK COMPLETE: legs {[l['run_id'] for l in legs]} window [{lo},{hi}] -> absorb MSPT, verdict")
                clear_state()
                return "found_pack", 0
            print(f"PACK LEG #1 FOUND: {rid} cpu={cpu} — need 1 more in-window leg")
            # drop run_id so next call dispatches pack arm #2 (not re-poll)
            st["run_id"] = None
            write_state(st)
            return "found_pack_leg1", 0
        print(f"  run {rid}: success cpu={cpu} OUT pack window {lo}-{hi} -> discard")
        clear_state()
        return None, 2
    m = re.search(r"runner_cpu_index=(\d+) band=\[", txt)
    print(f"  run {rid}: {concl} (gate cpu={m.group(1) if m else 'n/a'}) -> reject")
    clear_state()
    return None, 1


def main():
    tok = token_from_creds()
    st = read_state()

    if st and st.get("phase") in ("baseline", "pack") and st.get("run_id"):
        _, code = poll(tok, st)
        return code

    # no state -> dispatch next leg (phase-dependent)
    if st and st.get("phase") == "pack" and st.get("pack_legs"):
        # pack arm #2 dispatch with stored window
        lo, hi = st["win_lo"], st["win_hi"]
        rid = dispatch(tok, "master", int(lo * 0.995), int(hi * SLACK))
        if rid is None:
            return 3
        write_state({"phase": "pack", "run_id": rid, "win_lo": lo, "win_hi": hi,
                     "baseline": st["baseline"], "pack_legs": st["pack_legs"],
                     "t": time.time()})
        return 0
    # fresh baseline arm #2 (arm1 = banked run#18)
    rid = dispatch(tok, PRE_PACK_REF, ARM1_WIN_LO, ARM1_WIN_HI)
    if rid is None:
        return 3
    write_state({"phase": "baseline", "run_id": rid, "t": time.time()})
    return 0


if __name__ == "__main__":
    sys.exit(main())
