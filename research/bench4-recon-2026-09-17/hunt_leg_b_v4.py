#!/usr/bin/env python3
"""hunt_leg_b.py v4.3 — §125-AMENDMENT-1: slow-track baseline+pack hunt.

v4.3 FIX (recon bug#6+#7, S7-123): (bug#6) logs can be undownloadable for a
short window right after run completion — log_text returns "", the completed
run fell to the reject branch and state was WIPED (caught live on pack arm#1
35213299343; outcome coincided with an honest discard there, classification
was still wrong). Fix: empty logs on a successful run => retry next call.
(bug#7, bug#5 family) every pack-phase discard/cancel/reject path called
clear_state(), losing phase/window/baseline — the next dispatch then fell to
the fresh-baseline branch (wrong kernel on wrong ref). Fix:
restore_pack_or_clear() — pack phase is restored with run_id=None, baseline
clears as before.

v4.2 FIX (recon bug#5, S7-122): v4.1 main() dispatch branch required
`st.get("pack_legs")` truthy — the PACK ARM#1 case (phase=pack, run_id=None,
pack_legs=[]) fell through to the fresh-baseline branch and re-dispatched a
pre-pack leg, CLOBBERING the pack state (caught live S7-122: run
35213099261 dispatched on the wrong ref, cancelled, state restored from
leg_b_v4.json). Fix: pack dispatch keyed on (phase=pack AND run_id is None),
covering both arm#1 (pack_legs=[]) and arm#2 (pack_legs=[leg1]).

v4.1 FIX (recon bug#4, S7-121): the band gate samples harness cpu at ~30s,
but the pairing law pairs on the FINAL run-env cpu — measured drift between
the two (same-log gate line vs final echo): run#24 -5.5%, leg#8 -2.0%,
leg#9 +0.3%, run#23 +1.5%, run#21(bank) +1.9%, B1#6 +3.4%. Non-constant,
up to ±5.5%. Consequence: tight bands reject legs whose FINALS would land
in-window (run#21 itself would have been killed by the v3 band), and
in-gate legs can finish out-of-window (B1#6 under v4.0). Fix: the dispatch
band is DRIFT-COMPENSATED — [win_lo/(1+DRIFT_HI), win_hi/(1+DRIFT_LO)] —
admitting every gate value that can still finish in-window; the FINAL echo
check stays exact-window (honest discards for the rest, ~3 boots/leg).

AMENDMENT CAUSE (measured, S7-119): the §125 bank pair (run#17 9080657,
run#21 8914646, MID class) can no longer be sampled — census of 43 legs: the
mid window [8899044,9092939] was hit only by the two bank legs themselves;
since then 0/14 in-window; slow class dominates. Reusing the banked MID
baseline against fresh legs is unwinnable in practice — population shift.

AMENDMENT (all owner gates intact: >=3% MSPT, CI A/B min-of-2, median-exact
parity, world pin, +/-2% harness-cpu pairing on FINAL values):
  - Baseline arm #1 (RECYCLED, S7-121): run 35205343087 (B1#6) on audit tag
    pre-pack-962fc9f: SUCCESS, FIXTURE-VALIDITY VALID, world afb3a0b3,
    MSPT avg 83.40, final cpu 7057150 (gate 6828367, drift +3.4%).
  - Baseline arm #2 (B2): fresh pre-pack dispatch, drift-compensated band.
  - PACK WINDOW = [max(cpu1,B2)*0.98, min(cpu1,B2)*1.02] (empty => B2 discard).
  - Pack arms: master ref, drift-compensated band, exact-window final check.
  - VERDICT: pack median MSPT <= median(83.40, B2_mspt)*0.97 at 2 in-window
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

# ---- v4.1 constants (bug#4: gate-vs-final cpu DRIFT, measured S7-121) ----
# The band gate samples cpu at ~30s; the PAIRING-LAW value is the FINAL
# run-env echo. Measured drift (same-log pairs): run#24 -5.5%, leg#8 -2.0%,
# run#23 +1.5%, run#21(bank) +1.9%, leg#9 +0.3%, B1#6 +3.4%  => [-5.5%, +3.4%].
# Smoking gun: run#21 gate 8745625 would be KILLED by v3 band [8850000,9120000]
# while its final 8914646 IS in the v3 window => the gate was rejecting
# legally-pairable finals. Fix: band = [win_lo/(1+DRIFT_HI), win_hi/(1+DRIFT_LO)].
DRIFT_LO = 0.945   # final can sit 5.5% BELOW gate value
DRIFT_HI = 1.034   # final can sit 3.4% ABOVE gate value
# NOTE: workflow_dispatch API accepts only branch/tag refs (SHA => HTTP 422,
# caught live S7-119) => audit tag at 962fc9f pushed to origin.
PRE_PACK_REF = "pre-pack-962fc9f"  # S7-111: impl banked, hook NOT wired
WORLD_SHA = "afb3a0b3ba78397b833032574ed2a1af8c4279dc7cfa31d9cffa0f217a6112d5"
# Baseline arm #1 = B1#6 RECYCLED (S7-121): run 35205343087 on pre-pack tag,
# SUCCESS, FIXTURE-VALIDITY VALID, world afb3a0b3, MSPT avg 83.40, final cpu
# 7057150 (gate 6828367, drift +3.4%). Fresh window anchored on TODAY's class.
# (run#18 arm mothballed: its 6.75M class is stale in the current pool.)
ARM1 = {"run_id": 35205343087, "cpu": 7057150, "mspt": 83.40}
ARM1_WIN_LO, ARM1_WIN_HI = int(ARM1["cpu"] * 0.98), int(ARM1["cpu"] * 1.02)

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


def restore_pack_or_clear(st):
    """bug#7 (S7-123): a pack-phase discard/cancel/reject must NOT wipe the
    phase — clear_state() sent the next dispatch to the fresh-baseline branch
    (wrong kernel on wrong ref — bug#5 family). Pack phase is RESTORED with
    run_id=None so the hunt continues with the next pack arm; baseline phase
    clears (fresh-baseline branch re-derives from ARM1 constants)."""
    if st.get("phase") == "pack":
        write_state({"phase": "pack", "run_id": None,
                     "win_lo": st["win_lo"], "win_hi": st["win_hi"],
                     "baseline": st.get("baseline", []),
                     "pack_legs": st.get("pack_legs", []),
                     "t": time.time()})
    else:
        clear_state()


def pack_window(cpu2):
    lo = max(ARM1["cpu"], cpu2) * 0.98
    hi = min(ARM1["cpu"], cpu2) * 1.02
    return int(lo), int(hi)


def band_for(win_lo, win_hi):
    """Drift-compensated gate band: admit every gate value whose FINAL cpu
    can possibly land inside [win_lo, win_hi] given measured drift bounds."""
    return int(win_lo / DRIFT_HI), int(win_hi / DRIFT_LO)


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
                restore_pack_or_clear(st)
                return None, 2
            lo, hi = (ARM1_WIN_LO, ARM1_WIN_HI) if st["phase"] == "baseline" \
                else (st["win_lo"], st["win_hi"])
            if lo <= cpu <= hi:
                print(f"  run {rid}: harness cpu={cpu} IN {st['phase']} window — keep polling")
                return None, 4
            print(f"  run {rid}: harness cpu={cpu} OUT of {st['phase']} window -> cancel")
            api(tok, f"{API}/repos/{REPO}/actions/runs/{rid}/cancel", method="POST")
            time.sleep(15)
            restore_pack_or_clear(st)
            return None, 2
        print(f"  run {rid}: running, no harness echo yet — keep polling")
        return None, 4
    # completed
    txt = log_text(tok, rid)
    # v4.3 (bug#6): logs can be UNDOWNLOADABLE for a short window right after
    # completion (zip not yet finalized) — log_text returns "" and the run
    # used to fall to the reject branch, wiping state (caught live S7-123 on
    # pack arm#1 35213299343; outcome coincided with an honest discard there,
    # but the classification was wrong). Treat empty logs on a successful run
    # as retry-later, keeping state.
    if concl == "success" and not txt.strip():
        print(f"  run {rid}: success but logs not yet downloadable — retry next call")
        return None, 4
    mh = re.search(r"run-env: world_sha256=([0-9a-f]+) runner_cpu_index=(\d+) fake_players=4", txt)
    if concl == "success" and mh:
        wsha, cpu = mh.group(1), int(mh.group(2))
        if wsha != WORLD_SHA:
            print(f"  run {rid}: WORLD DRIFT sha={wsha[:12]} != pin -> discard, re-baseline required")
            json.dump({"run_id": rid, "cpu": cpu, "world_sha": wsha,
                       "verdict": "WORLD-DRIFT"}, open(RESULT, "w"))
            restore_pack_or_clear(st)
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
        restore_pack_or_clear(st)
        return None, 2
    m = re.search(r"runner_cpu_index=(\d+) band=\[", txt)
    print(f"  run {rid}: {concl} (gate cpu={m.group(1) if m else 'n/a'}) -> reject")
    restore_pack_or_clear(st)
    return None, 1


def main():
    tok = token_from_creds()
    st = read_state()

    if st and st.get("phase") in ("baseline", "pack") and st.get("run_id"):
        _, code = poll(tok, st)
        return code

    # no state -> dispatch next leg (phase-dependent)
    # v4.2: keyed on run_id is None so pack ARM#1 (pack_legs=[]) also lands here
    if st and st.get("phase") == "pack" and st.get("run_id") is None:
        # pack arm #1/#2 dispatch with stored window (drift-compensated band)
        lo, hi = st["win_lo"], st["win_hi"]
        b_lo, b_hi = band_for(lo, hi)
        rid = dispatch(tok, "master", b_lo, b_hi)
        if rid is None:
            return 3
        write_state({"phase": "pack", "run_id": rid, "win_lo": lo, "win_hi": hi,
                     "baseline": st["baseline"], "pack_legs": st["pack_legs"],
                     "t": time.time()})
        return 0
    # fresh baseline arm #2 (arm1 = recycled B1#6 on pre-pack tag)
    b_lo, b_hi = band_for(ARM1_WIN_LO, ARM1_WIN_HI)
    rid = dispatch(tok, PRE_PACK_REF, b_lo, b_hi)
    if rid is None:
        return 3
    write_state({"phase": "baseline", "run_id": rid, "t": time.time()})
    return 0


if __name__ == "__main__":
    sys.exit(main())
