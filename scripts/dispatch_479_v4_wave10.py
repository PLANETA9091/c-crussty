#!/usr/bin/env python3
"""dispatch_479_v4_wave10.py — COMMANDER 479-V4 wild volley (MEGA-SWARM v19.0, тик ×479).

CLAIM (закон 14a/16, прегистрируется board-коммитом ДО диспатчей):
  Безумная волна ×10 (все 0 Java/Rust код-дельт, env-каналы workflow-dispatch,
  алиасы round-479-v4-x01..x10, canon x466-C98 ЯВНЫМ JSON урок C66-C72,
  1 реф=1 диспатч Л188b FULL-sha+GET runs_before=0):
    x01 600s+gc7 повтор W1-класса (RCC-канал Л-479-W1, treatment-канал честно
        повторяется; на PIN-мастере case-7 fallthrough как в W1-триве — канал
        регистрируется как есть)
    x02 900s/640 (x8-повтор на новом master)
    x03 300s/960 (радиус-стресс)
    x04 450s/320 (узкий радиус/длинное окно)
    x05 600s/960 (двойной стресс)
    x06 150s/640 (короткое окно)
    x07 300s/640 fp4 чистый канон-повтор x466-C98
    x08 300s/640 rt8 (region_threads=8, duty-проверка на №17-мастере)
    x09 300s/640 bu_defer=1 (+region_steal=1 — S7-168 defect-fix канал)
    x10 300s/640 STEAL=1 (region_steal=1, bu_defer=0 — A/B-нога к x09)
  Канон-база x466-C98: 640/300s/fp4/gc3/ic1/fd1/rt4/bc1/pop150k/seed42/10G/
  xms4G, lever ∅ (0 код-дельт → нечего ARM-пруфить), world_url = workflow-
  default MineShield-3 Min. Band: канон-банд [6.0,9.5]M fast-fail ТОЛЬКО на
  300s-ячейках (x03/x07/x08/x09/x10); стресс-ячейки (x01/x02/x04/x05/x06,
  seconds≠300) — band НЕ гейт (пустые cpu_band_min/max).
  Каноны: закон-5 запреты (не ZGC=gc4/THP=gc5/flat_traversal/zero_alloc —
  все не тронуты), пороги v5-FROZEN не двигаются.
  Финал ≤40 мин, бенчи НЕ ждать (закон 18-iii/12e): {run-id ×10, DISPATCHED}.
  Числа — ×480 абсорб (стресс-лестница 19a обновится след. тиком).

Usage:
  dispatch_479_v4_wave10.py dispatch [--dry-run]
  dispatch_479_v4_wave10.py wait [minutes]   # run-id discovery по всем 10
  dispatch_479_v4_wave10.py poll             # статус + cpu-ценз (band-ON ячейки)
"""
import json, re, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN_SHA = "e503160cdf7a4aef55ac282d13f719d4ca05500a"  # origin/master тика ×479 (F3 №17, 0 код-дельт волны)

BAND = ("6000000", "9500000")  # x466-C98 канон-банд

# (alias, deltas) — дельты ПОВЕРХ канон-базы; band=False → стресс-ячейка
CELLS = [
    ("round-479-v4-x01", {"seconds": "600", "gc_tune": "7"}, False),          # W1-класс повтор
    ("round-479-v4-x02", {"seconds": "900"}, False),                          # x8-повтор 900s/640
    ("round-479-v4-x03", {"radius": "960"}, True),                            # 300s/960
    ("round-479-v4-x04", {"seconds": "450", "radius": "320"}, False),         # 450s/320
    ("round-479-v4-x05", {"seconds": "600", "radius": "960"}, False),         # 600s/960
    ("round-479-v4-x06", {"seconds": "150"}, False),                          # 150s/640
    ("round-479-v4-x07", {}, True),                                           # канон-повтор fp4
    ("round-479-v4-x08", {"region_threads": "8"}, True),                      # rt8 duty-check
    ("round-479-v4-x09", {"region_steal": "1", "bu_defer": "1"}, True),       # bu_defer=1
    ("round-479-v4-x10", {"region_steal": "1", "bu_defer": "0"}, True),       # STEAL=1
]

BASE_INPUTS = {
    "world_url": "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip",
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G", "gc_tune": "3",
    "lever_flag": "", "lever_arg": "",
}

INPUTS_BY_BRANCH = {}
for _br, _delta, _band in CELLS:
    _inp = dict(BASE_INPUTS, **_delta)
    if _band:
        _inp["cpu_band_min"], _inp["cpu_band_max"] = BAND
    else:
        _inp["cpu_band_min"], _inp["cpu_band_max"] = "", ""  # band НЕ гейт (стресс)
    INPUTS_BY_BRANCH[_br] = _inp


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json",
        "User-Agent": "479-v4-wave10-dispatcher"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, timeout=60, data=payload) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        if e.code != 404:
            print(f"HTTP {e.code}: {e.read()[:200]}", file=sys.stderr)
        raise


def ensure_ref(tok, br):
    """1 реф=1 диспатч Л188b: FULL-sha, GET-верификация ДО диспатча."""
    ref = f"refs/heads/{br}"
    try:
        api(tok, "/repos/{REPO}/git/refs".format(REPO=REPO), method="POST",
            data={"ref": ref, "sha": PIN_SHA})
        print(f"ref CREATED: {br} @ {PIN_SHA[:8]}", flush=True)
    except urllib.error.HTTPError as e:
        if e.code == 422:
            cur = api(tok, f"/repos/{REPO}/git/ref/heads/{br}")["object"]["sha"]
            if cur != PIN_SHA:
                if runs_on_branch(tok, br):
                    raise SystemExit(f"REF CONFLICT + runs exist: {br} @ {cur[:8]}")
                api(tok, f"/repos/{REPO}/git/refs/heads/{br}", method="PATCH",
                    data={"sha": PIN_SHA, "force": True})
                print(f"ref ALIGNED: {br} {cur[:8]} -> {PIN_SHA[:8]} (no-run)", flush=True)
            else:
                print(f"ref EXISTS-OK: {br} @ {cur[:8]}", flush=True)
        else:
            raise
    live = api(tok, f"/repos/{REPO}/git/ref/heads/{br}")["object"]["sha"]
    assert live == PIN_SHA, f"GET-verify fail {br}"
    return live


def runs_on_branch(tok, br, per=100):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page={per}")
    return [(r["id"], r.get("status"), r.get("conclusion"), r.get("created_at"))
            for r in runs.get("workflow_runs", []) if r.get("head_branch") == br]


def do_dispatch(dry):
    tok = token()
    branches = [c[0] for c in CELLS]
    for br in branches:
        ensure_ref(tok, br)
        if runs_on_branch(tok, br):
            raise SystemExit(f"Л188a VIOLATION: {br} уже имеет run — 1 реф=1 диспатч")
    if dry:
        print("DRY-RUN OK — 10 refs verified @ PIN, no dispatches", flush=True)
        return
    for br, delta, band in CELLS:
        api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches", method="POST",
            data={"ref": br, "inputs": INPUTS_BY_BRANCH[br]})
        d = ",".join(f"{k}={v}" for k, v in delta.items()) or "canon"
        print(f"DISPATCHED: {br} delta[{d}] band={'ON' if band else 'OFF'} (204)", flush=True)
        time.sleep(15)


def do_wait(minutes=10):
    tok = token()
    deadline = time.time() + minutes * 60
    pending = {c[0] for c in CELLS}
    found = {}
    while pending and time.time() < deadline:
        for br in sorted(pending):
            hits = runs_on_branch(tok, br)
            if hits:
                rid, st, concl, ca = hits[0]
                found[br] = rid
                print(f"RUN_ID: {br} -> {rid} status={st} created={ca}", flush=True)
                pending.discard(br)
        if pending:
            print(f"waiting {len(pending)}: {sorted(pending)}", flush=True)
            time.sleep(20)
    print(f"--- total {len(found)}/10 run-ids", flush=True)
    for br in sorted(found):
        print(f"{br} run_id={found[br]}", flush=True)
    for br in sorted(pending):
        print(f"{br} run_id=NOT_FOUND", flush=True)
    return found


def do_poll():
    tok = token()
    for br, delta, band in CELLS:
        hits = runs_on_branch(tok, br)
        if not hits:
            print(f"{br}: NO-RUN", flush=True)
            continue
        rid, st, concl, ca = hits[0]
        print(f"{br}: run {rid} status={st} concl={concl} band={'ON' if band else 'OFF'} "
              f"delta={delta or 'canon'} created={ca}", flush=True)


def main():
    cmd = sys.argv[1] if len(sys.argv) > 1 else "dispatch"
    if cmd == "dispatch":
        do_dispatch("--dry-run" in sys.argv)
    elif cmd == "wait":
        do_wait(int(sys.argv[2]) if len(sys.argv) > 2 else 10)
    elif cmd == "poll":
        do_poll()
    else:
        print(__doc__)
        sys.exit(2)


if __name__ == "__main__":
    main()
