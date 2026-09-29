#!/usr/bin/env python3
"""dispatch_488_c36_sensn16b.py — [488-C36] sensn16-нога ре-ролл на STRICT-окне [7.0,7.2]M (тик ×488, Job 415026).

CLAIM (prereg, закон 14a/16, CLM-C36.md): sensn16-пара 0/26 дроу — причина структурная:
leg c98ai @7.597M (+32.01, canon x466-C98, run 36480261240) сидит в междугорбной
впадине, а якорь-пул дрейфанул к 7.05M (C20-st8 7,058,620 / XS3 7,047,916 ценз /
cnr9 7,071,551; Δ≤50k-кластер CLM-C19 §4). Δ leg→якоря = 526-549k ≫ 50k — пара
геометрически мертва → правильный ход = ре-ролл САМОЙ ноги в текущую моду пула:
union-окно якорей [6.998,7.109]M (CLM-C20 §3) внутри STRICT-воронки [7.0,7.2]M.
1 ветка = 1 ран (Л188b): round-488-c36-sensn16b @ PIN master 13a955a3, 0 код-дельт.
ЕДИНСТВЕННЫЕ дельты vs canon x466-C98 (явным JSON: вектор A16 leg-k
scripts/dispatch_478_a16_legk.py, адаптирован под master-workflow C38 DP-DOOR
24 инпута — inside_bitmask-входа НЕТ, он dormant-0 через shell-default
run_world3.sh:88; урок 422 round-488-c17): cpu_band GLOB [6.0,9.5]M →
[7000000,7200000] STRICT-воронка (band fast-fail = бесплатный pairing-дискард
S7-96d → фид cpu_index).
lever LEG-класс НЕ ваниль: cmp466_c98ai/16 = c98ai-компо-носитель (МЕРЖ №19
sensn16⊕climb5⊕collide STRICT-OR 40/40, cert n16 Л-470-S20.1 / Л-482-C53.1).
P-модель: P(STRICT-hit) ≈ 36%/дроу (C19 §1, 3 источника); P(union-якорь) ≈ 13.4%/дроу
(C20 §3, ×2.2 от двух якорей vs 6.0% одиночного 50k-окна, против ~0% на впадине).
Поверхность: world-bench-parallel.yml (parallel per-ref, слот-война ЗАПРЕТ).

Usage: dispatch_488_c36_sensn16b.py [--dry-run]
"""
import json, re, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
WF = "world-bench-parallel.yml"
PIN = "13a955a30ae41704fc27a4a7e1597fc4202f6496"  # master ×487 учёт (канон sensn16-l1 x466-C98)
GATE_STEP = "Runner calibration band gate (pair-hunter fast-fail, S7-96d pairing law)"

BRANCHES = ["round-488-c36-sensn16b"]  # 1 ветка = 1 ран (Л188b); бриф: одна ветка "b"

# canon x466-C98 — ЯВНЫЙ JSON 24/24 инпута master-workflow (урок C66-C73: yml-дефолты
# = merge-поверхность; вектор A16 leg-k scripts/dispatch_478_a16_legk.py минус
# inside_bitmask — C38 DP-DOOR swap ×488, dormant-0 via shell-default run_world3.sh:88).
# ДЕЛЬТЫ ×488: cpu_band [7.0,7.2]M STRICT-воронка (union якорей [6.998,7.109]M внутри);
# lever cmp466_c98ai/16 = LEG (не якорь-ваниль) — sensn16-нога ре-ролла.
WORLD = "https://storage.shield.land/public.php/dav/files/twzsxN3HkBQtyED/Season%203/MineShield-3__Min--Normal.zip"
INPUTS = {
    "world_url": WORLD,
    "radius": "640", "seconds": "300", "fake_players": "4",
    "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1", "flush_diet": "1",
    "fluid_dirty": "0", "fluid_bitmask": "0", "region_threads": "4",
    "batch_collector": "1", "skip_store_bb": "0",
    "region_steal": "0", "bu_defer": "0",
    "datapack_url": "",
    "population_target": "150000", "population_seed": "42",
    "server_xmx": "10G", "server_xms": "4G",
    "cpu_band_min": "7000000",   # ← дельта: STRICT-воронка [7.0,7.2]M
    "cpu_band_max": "7200000",   # ← дельта: STRICT-воронка [7.0,7.2]M
    "lever_flag": "cmp466_c98ai", "lever_arg": "16",  # LEG: sensn16⊕climb5⊕collide /16 cert
}

ANCHORS = {"C20-st8": 7058620, "XS3": 7047916, "cnr9": 7071551}  # Δ≤50k-кластер CLM-C19 §4


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
    runs = api(tok, f"/repos/{REPO}/actions/runs?branch={br}&event=workflow_dispatch&per_page=30")
    return [(r["id"], r.get("status"), r.get("created_at"), r.get("head_sha"))
            for r in runs.get("workflow_runs", [])]


def gate_verdict(tok, run_id):
    """None = ещё не решено; 'PASS' = гейт прошёл (бенч идёт); 'FAIL' = band fast-fail."""
    jobs = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs?per_page=5")
    for j in jobs.get("jobs", []):
        for s in j.get("steps", []):
            if s.get("name") == GATE_STEP:
                c = s.get("conclusion")
                if c == "success":
                    return "PASS"
                if c in ("failure", "cancelled"):
                    return "FAIL"
    return None


def cpu_index_feed(tok, run_id):
    """Фид пула: runner_cpu_index из лога gate-джоба (дискард-данные, S7-96d)."""
    jobs = api(tok, f"/repos/{REPO}/actions/runs/{run_id}/jobs?per_page=5")
    for j in jobs.get("jobs", []):
        try:
            req = urllib.request.Request(
                f"{API}/repos/{REPO}/actions/jobs/{j['id']}/logs",
                headers={"Authorization": f"Bearer {tok}"})
            with urllib.request.urlopen(req, timeout=60) as r:
                m = re.search(r"runner_cpu_index=(\d+)", r.read().decode("utf-8", "replace"))
            if m:
                return int(m.group(1))
        except Exception as e:
            print(f"log-fetch fail job {j['id']}: {e}", file=sys.stderr)
    return None


def anchor_delta(idx):
    """Δ cpu_index к кластеру {C20, XS3, cnr9} — union-окно [6.998,7.109]M ⟺ min Δ ≤ 50k."""
    if idx is None:
        return None
    d = {k: abs(idx - v) for k, v in ANCHORS.items()}
    best = min(d, key=d.get)
    return {"min_delta": d[best], "best_anchor": best, "in_union_window": d[best] <= 50000, "deltas": d}


def roll(tok, branch, dry=False):
    cur = ref_sha(tok, branch)
    if cur != PIN:
        raise SystemExit(f"REF-VERIFY FAIL {branch}: got {cur}, want PIN (Л188a)")
    print(f"GET-verify OK {branch} object.sha == {cur[:8]} (master ×487)", flush=True)

    if dry:
        print(f"DRY-INPUTS {branch} ({len(INPUTS)}/24 полей C38-канона): " + json.dumps(INPUTS, sort_keys=True), flush=True)
        return None

    existing = [rid_ for rid_, st, ca, hs_ in runs_on_branch(tok, branch) if hs_ == PIN]
    if existing:  # re-entrant: адопт in-flight/готовый ран вместо повторного диспатча
        rid = existing[0]
        print(f"ADOPT existing run {rid} on {branch}", flush=True)
    else:
        r = api(tok, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
                method="POST", data={"ref": branch, "inputs": INPUTS})
        if r != {}:
            raise SystemExit(f"dispatch failed {branch}: {r}")
        print(f"dispatch 204-OK {branch}", flush=True)

        rid = None
        deadline = time.time() + 300
        while time.time() < deadline and rid is None:
            time.sleep(10)
            for rid_, st, ca, hs_ in runs_on_branch(tok, branch):
                if hs_ == PIN:
                    rid = rid_
                    break
        if rid is None:
            print(f"run-id not visible in 300s {branch} (204 принят)", flush=True)
            return {"branch": branch, "run_id": None, "verdict": "UNKNOWN",
                    "cpu_index": None, "anchor_delta": None}
    print(f"RUN-ID {rid} ({branch})", flush=True)

    # Ждём вердикт band-гейта (fast-fail pre-download ~1 мин после старта job)
    deadline = time.time() + 900
    while time.time() < deadline:
        time.sleep(20)
        v = gate_verdict(tok, rid)
        if v == "PASS":
            idx = cpu_index_feed(tok, rid)
            d = anchor_delta(idx)
            print(f"GATE PASS {branch} run {rid} cpu_index={idx} anchor={d} — бенч продолжается", flush=True)
            return {"branch": branch, "run_id": rid, "verdict": "IN-BENCH",
                    "cpu_index": idx, "anchor_delta": d}
        if v == "FAIL":
            idx = cpu_index_feed(tok, rid)
            print(f"GATE FAST-FAIL {branch} run {rid} — pairing discard, cpu_index={idx}", flush=True)
            return {"branch": branch, "run_id": rid, "verdict": "DISCARD",
                    "cpu_index": idx, "anchor_delta": anchor_delta(idx)}
        st = api(tok, f"/repos/{REPO}/actions/runs/{rid}").get("status")
        if st == "completed":
            concl = api(tok, f"/repos/{REPO}/actions/runs/{rid}").get("conclusion")
            print(f"run {rid} completed early conclusion={concl}", flush=True)
            idx = cpu_index_feed(tok, rid)
            return {"branch": branch, "run_id": rid,
                    "verdict": "DISCARD" if concl != "success" else "HIT",
                    "cpu_index": idx, "anchor_delta": anchor_delta(idx)}
    print(f"gate verdict timeout 900s {branch} run {rid} — in-flight (DISPATCHED 18-iii)", flush=True)
    return {"branch": branch, "run_id": rid, "verdict": "UNKNOWN",
            "cpu_index": None, "anchor_delta": None}


def main():
    args = sys.argv[1:]
    if any(a not in ("--dry-run",) for a in args):
        raise SystemExit(f"argv-guard: unknown args {args}")
    dry = "--dry-run" in args
    tok = token()

    live_master = ref_sha(tok, "master")
    print(f"origin/master live = {live_master} -> pin {PIN[:8]} (×487 учёт)", flush=True)

    for br in BRANCHES:
        rs = runs_on_branch(tok, br)
        if rs:
            print(f"RUN-SNAPSHOT: {br} has runs {[r[0] for r in rs]} — адопт-путь (ре-энтрант)", flush=True)
        else:
            print(f"RUN-SNAPSHOT: {br} чист (0 ранов)", flush=True)

    results = []
    for br in BRANCHES:
        res = roll(tok, br, dry=dry)
        if dry:
            continue
        results.append(res)
        json.dump({"pin": PIN, "inputs": INPUTS, "anchors": ANCHORS, "rolls": results},
                  open("/home/z/rounds/ROUND-488/c36_sensn16b_dispatch.json", "w"),
                  indent=1, ensure_ascii=False)
    # без ре-ролл-цикла: 1 ветка = 1 ран (Л188b); P(hit) 36%/дроу, P(union) 13.4%/дроу —
    # дискард-фид (cpu_index) идёт в пул якорей, добивка решается на абсорбе.

    if not dry:
        print("SUMMARY " + json.dumps(results, ensure_ascii=False), flush=True)


if __name__ == "__main__":
    main()
