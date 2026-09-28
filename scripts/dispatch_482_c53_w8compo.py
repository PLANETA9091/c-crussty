#!/usr/bin/env python3
"""dispatch_482_c53_w8compo.py — [482-C53] Commander C53 КОМПО-СУП-4: c98ai ⊕ W8@r480 (тик ×482).

CLAIM (prereg, закон 14a/16 — зафиксирован ДО диспатча):
  Компо-суп-4 = c98ai-компо (уже в master 3666a793 — МЕРЖ №19, бесплатно) ⊕ W8-канал
  (region_threads=8 + r480-стратум = Amdahl +23.7пп gen-ось, Л-481-C34).
  c98ai-гейт: STRICT-OR паритет 40/40 (Л-480-C72/C99) — ОДИН lever-флаг cmp466_c98ai/16
  армит все 3 плоскости (climb5-resident ⊕ sensn16 ⊕ collide); это env-вход workflow
  (lever_flag/lever_arg inputs -> CRUSSTY_LEVER_FLAG/CRUSSTY_LEVER_ARG) -> ДОБАВЛЯЕМ
  lever_flag=cmp466_c98ai, lever_arg=16 (cert-конфиг МЕРЖ №19 «lever cmp466_c98ai/16»).

Вектор: burst73-ваниль канона x466-C98 (300s/fp4/guard1/ic1/fd1/bc1/pop150k/seed42/
  xms4G/band[6.0,9.5]M, default MineShield-3 Min URL) с дельтами миссии:
  radius=480 (r480-стратум, 4096 чанков, S52-домен), region_threads=8 (W8),
  gc_tune=6 (ParallelGC+MetaspaceSize256M+RCC512M, S99-gcw), server_xmx=12G
  (19b-heap канон Л180k: 10G->12G peak 80%).
  Код-дельт НЕТ (ветка = bare pin 3666a793) -> push-гейты cargo/javap не применимы.

ПРОГНОЗ (prereg): W8-norm канал +23.7пп (Amdahl rt8@r480, S52: стратум-класс r480a
  norm −3.00 + 23.7 = +20.7) + c98ai-эффект уже в базе (compо-leg банка +19.59..+23.56).
  Свежие якоря ×482: 15 burst73-фидов, медиана norm −2.9 → pair = leg − anchor:
  leg_norm ≈ +21 → pair ≈ +24 → КЛИМБ-кандидат ≥+20 (min-of-3, этот ран = 1/3).
  Честный band: банк MAE 5.91 → [+15, +27]; фальсификатор: pair < +15 или M1-срыв.

Usage: dispatch_482_c53_w8compo.py [--dry-run]
"""
import json, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"

BRANCH = "round-482-c53-w8compo"
PIN = "3666a7931703e24a36af4887c41a585918667b7a"  # master post-МЕРЖ №19 (c98ai в коде)

INPUTS = {
    "radius": "480",  # r480-стратум (W8-домен S52: N_regions<=4 шапка снята)
    "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "6",  # W8-конфиг миссии (ParallelGC+MD256M+RCC512M)
    "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0",
    "region_threads": "8",  # W8 Amdahl +23.7пп gen-ось
    "batch_collector": "1", "inside_bitmask": "0", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "12G", "server_xms": "4G",
    "cpu_band_min": "6000000", "cpu_band_max": "9500000",
    # c98ai-компо-гейт: STRICT-OR 40/40 — один флаг армит climb5⊕sensn16⊕collide
    "lever_flag": "cmp466_c98ai", "lever_arg": "16",  # cert-конфиг МЕРЖ №19
}


def token():
    return open("/tmp/gh_token").read().strip()


def api(tok, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    payload = json.dumps(data).encode() if data is not None else None
    if payload:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, payload, timeout=60) as r:
            body = r.read()
        return json.loads(body) if body else {}
    except urllib.error.HTTPError as e:
        print(f"HTTP {e.code}: {e.read()[:300]}", file=sys.stderr)
        return {"_http_error": e.code}


def ref_sha(tok, branch):
    r = api(tok, f"/repos/{REPO}/git/ref/heads/{branch}")
    return r.get("object", {}).get("sha")


def runs_on_branch(tok, br):
    runs = api(tok, f"/repos/{REPO}/actions/runs?event=workflow_dispatch&per_page=30")
    return [(r["id"], r.get("status"), r.get("created_at"), r.get("head_sha"))
            for r in runs.get("workflow_runs", []) if r.get("head_branch") == br]


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args
    tok = token()

    live_master = ref_sha(tok, "master")
    print(f"origin/master live = {live_master} -> pin {PIN} (МЕРЖ №19 c98ai base, mission)", flush=True)

    pre = runs_on_branch(tok, BRANCH)
    if pre:
        raise SystemExit(f"RUN-SNAPSHOT DIRTY: {BRANCH} has runs {pre}")

    cur = ref_sha(tok, BRANCH)
    if cur != PIN:
        if cur is None:
            api(tok, f"/repos/{REPO}/git/refs", method="POST",
                data={"ref": f"refs/heads/{BRANCH}", "sha": PIN})  # FULL-sha (урок S20: 422 на коротком)
            print(f"ref CREATED {BRANCH} @ {PIN[:8]}", flush=True)
        else:
            api(tok, f"/repos/{REPO}/git/refs/heads/{BRANCH}", method="PATCH",
                data={"sha": PIN, "force": True})
            print(f"ref PATCHED {BRANCH} -> {PIN[:8]}", flush=True)
    got = ref_sha(tok, BRANCH)
    if got != PIN:
        raise SystemExit("POST-CREATE VERIFY FAIL (Л188a)")
    print(f"GET-verify OK object.sha == {got}", flush=True)

    if dry:
        print("DRY-INPUTS: " + json.dumps(INPUTS, sort_keys=True), flush=True)
        print("DRY-RUN OK — no dispatch", flush=True)
        return

    r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
            method="POST", data={"ref": BRANCH, "inputs": INPUTS})
    if r == {}:
        print("dispatch 204-OK", flush=True)
    else:
        raise SystemExit(f"dispatch failed: {r}")

    rid = None
    deadline = time.time() + 240
    while time.time() < deadline and rid is None:
        time.sleep(10)
        for rid_, st, ca, hs in runs_on_branch(tok, BRANCH):
            if hs == PIN:
                rid = rid_
                break
    if rid is None:
        print("run-id not visible in 240s (204 принят, discovery по head_sha позже)", flush=True)
        json.dump({"branch": BRANCH, "pin": PIN, "run_id": None,
                   "inputs": INPUTS}, open("/home/z/rounds/ROUND-482/c53_dispatch.json", "w"), indent=1)
        return
    print(f"RUN-ID {rid}", flush=True)
    json.dump({"branch": BRANCH, "pin": PIN, "run_id": rid,
               "inputs": INPUTS}, open("/home/z/rounds/ROUND-482/c53_dispatch.json", "w"), indent=1)


if __name__ == "__main__":
    main()
