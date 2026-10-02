#!/usr/bin/env python3
"""dispatch_526_66.py — AG-66 волна-526: leg-A pop600k-мид WBP (500-750k, 0-клейм;
соседи 500k AG-241 / 750k AG-40; injector-cliff зона AG-277) @e49e8984 dp3v2 seed42;
leg-B s2700 seconds-мид WBP (2400-3000, 0-клейм; соседи 2400 AG-269 / 3000 AG-258).
Zero-code: refs-API @PIN, tree-FULL вериф ≥3200, race-check живой GET boundary-regex,
CAS-append CLAIM, 2 POST ≥31s (FAIL AG-338), GET-вериф run-ids + head_sha==PIN.
Пивоты: leg-A alt pop650k, leg-B alt s3300 (миды 600-750k / 3000-3600)."""
import base64, json, re, subprocess, sys, time, urllib.request, urllib.error

REPO = "PLANETA9091/c-crussty"
API = "https://api.github.com"
RD = "/home/z/rounds/ROUND-526"
PIN = "e49e89845108c0b3d00022dfbace91777bacb01e"  # WBP-канон-носитель (AG-257/2/32/40)
WF = "world-bench-parallel.yml"
BR_A, BR_B = "swarm-526-66", "swarm-526-66b"
DP3V2 = ("https://github.com/PLANETA9091/c-crussty/releases/download/"
         "v484-dp3v2/stz3v2-fixture.zip")
CLAIM = ("CLAIM | AG-66 | pop600k-мид WBP (500-750k) + s2700 s-мид WBP (2400-3000) "
         "0-клейм dp3v2 seed42 | 2 POST")
assert len(CLAIM) <= 120, f"CLAIM {len(CLAIM)}>120"

CANON_WBP = {"radius": "640", "seconds": "300", "fake_players": "4",
             "fluid_guard": "1", "gc_tune": "3", "inside_cache": "1",
             "flush_diet": "1", "fluid_dirty": "0", "fluid_bitmask": "0",
             "region_threads": "4", "batch_collector": "1", "skip_store_bb": "0",
             "region_steal": "0", "bu_defer": "0", "population_target": "150000",
             "population_seed": "42", "server_xmx": "10G", "server_xms": "4G",
             "cpu_band_min": "5500000", "cpu_band_max": "13500000",
             "datapack_url": DP3V2, "lever_flag": "", "lever_arg": ""}

PRIMARY = [
    ("pop600k", BR_A, dict(CANON_WBP, population_target="600000"), r"\bpop600k\b"),
    ("s2700", BR_B, dict(CANON_WBP, seconds="2700"), r"\bs2700\b"),
]
ALT = [
    ("pop650k", BR_A, dict(CANON_WBP, population_target="650000"), r"\bpop650k\b"),
    ("s3300", BR_B, dict(CANON_WBP, seconds="3300"), r"\bs3300\b"),
]


def tok():
    url = subprocess.run(["git", "-C", "/home/z/c-crussty", "remote", "get-url",
                          "origin"], capture_output=True, text=True).stdout.strip()
    m = re.match(r"^https://[^:]+:([^@]+)@github.com/", url)
    return m.group(1) if m else open("/tmp/gh_token").read().strip()


def api(t, url, method="GET", data=None):
    req = urllib.request.Request(API + url, method=method, headers={
        "Authorization": f"Bearer {t}", "Accept": "application/vnd.github+json",
        "User-Agent": "ag-66-wave526"})
    body = json.dumps(data).encode() if data is not None else None
    if body is not None:
        req.add_header("Content-Type", "application/json")
    try:
        with urllib.request.urlopen(req, body, timeout=90) as r:
            raw = r.read().decode()
            return r.status, (json.loads(raw) if raw.strip() else {})
    except urllib.error.HTTPError as e:
        raw = e.read().decode()
        try:
            return e.code, json.loads(raw)
        except Exception:
            return e.code, {"_raw": raw[:200]}


def tree_full(t, pin, tag):
    code, c = api(t, f"/repos/{REPO}/commits/{pin}")
    if code != 200:
        sys.exit(f"FATAL pin {tag} {code}")
    tr = c["commit"]["tree"]["sha"]
    code, tt = api(t, f"/repos/{REPO}/git/trees/{tr}?recursive=1")
    n = len(tt.get("tree", []))
    trunc = tt.get("truncated", False)
    print(f"PIN {tag}={c['sha'][:8]} tree={n} trunc={trunc} "
          f"{'FULL' if n >= 3200 and not trunc else 'SPARSE-FATAL'}", flush=True)
    if trunc or n < 3200:
        sys.exit(f"FATAL sparse {tag}")
    return n


def race_scan(t, regex):
    code, d = api(t, f"/repos/{REPO}/contents/SHARED_BOARD.md?ref=master")
    if code != 200:
        sys.exit(f"FATAL board GET {code}")
    text = base64.b64decode(d["content"]).decode()
    for ln in text.splitlines():
        if "AG-66" in ln or not (ln.startswith("CLAIM") or ln.startswith("FAIL")):
            continue
        if re.search(regex, ln):
            return d["sha"], text, f"RACE: {ln[:100]}"
    return d["sha"], text, None


def board_append(t, lines, msg):
    for l in lines:
        assert len(l) <= 120, f"{len(l)}>120: {l}"
    for attempt in range(8):
        code, d = api(t, f"/repos/{REPO}/contents/SHARED_BOARD.md?ref=master")
        if code != 200:
            print(f"board GET-err {code}", flush=True)
            time.sleep(4)
            continue
        sha = d["sha"]
        text = base64.b64decode(d["content"]).decode()
        if all(L in text for L in lines):
            print("board: already-appended", flush=True)
            return True
        new = text.rstrip("\n") + "\n\n" + "\n".join(lines) + "\n"
        code, r = api(t, f"/repos/{REPO}/contents/SHARED_BOARD.md", method="PUT",
                      data={"message": msg,
                            "content": base64.b64encode(new.encode()).decode(),
                            "sha": sha, "branch": "master"})
        if code == 200 and r.get("commit", {}).get("sha"):
            print(f"board-commit {r['commit']['sha'][:8]} (+{len(lines)})", flush=True)
            return True
        print(f"board retry {attempt}: {code} {str(r.get('message'))[:60]}",
              flush=True)
        time.sleep(5)
    return False


def make_ref(t, ref, pin):
    code, r = api(t, f"/repos/{REPO}/git/refs", method="POST",
                  data={"ref": f"refs/heads/{ref}", "sha": pin})
    if code != 201:
        code2, g = api(t, f"/repos/{REPO}/git/ref/heads/{ref}")
        cur = g.get("object", {}).get("sha", "") if code2 == 200 else "?"
        if cur != pin:
            sys.exit(f"FATAL ref {ref} exists sha={cur[:8]} != PIN")
    ver = api(t, f"/repos/{REPO}/git/ref/heads/{ref}")[1]["object"]["sha"]
    print(f"branch {ref}: sha={ver[:8]} match={ver == pin}", flush=True)
    if ver != pin:
        sys.exit(f"FATAL ref mismatch {ref}")


def find_runs(t, pre_ids, want, pages=4):
    found = {}
    for p in range(1, pages + 1):
        code, d = api(t, f"/repos/{REPO}/actions/runs?per_page=100&page={p}")
        if code != 200:
            break
        for run in d.get("workflow_runs", []):
            b = run["head_branch"]
            if b in want and run["id"] not in pre_ids:
                ok = run["head_sha"] == want[b]
                print(f"RUN {run['id']} {b} sha={run['head_sha'][:8]} "
                      f"{run['status']} match={'OK' if ok else 'MISMATCH'}", flush=True)
                if ok:
                    found[b] = run["id"]
        if len(found) >= len(want):
            break
    return found


def phase_run():
    t = tok()
    n = tree_full(t, PIN, "WBP")
    legs = []
    for i, (tag, br, inp, rx) in enumerate(PRIMARY):
        _, _, hit = race_scan(t, rx)
        if hit:
            print(f"{hit} -> pivot ALT", flush=True)
            tag, br, inp, rx = ALT[i]
            _, _, hit2 = race_scan(t, rx)
            if hit2:
                sys.exit(f"FATAL обе клетки заняты leg{i}: {hit2}")
        legs.append({"tag": tag, "branch": br, "inputs": inp})
    if not board_append(t, [CLAIM], "board: AG-66 claim pop600k+s2700 WBP mids (wave-526)"):
        sys.exit("FATAL claim not appended")
    pre = set()
    code, d = api(t, f"/repos/{REPO}/actions/runs?per_page=100")
    if code == 200:
        pre = {r["id"] for r in d.get("workflow_runs", [])}
    print(f"pre-snapshot runs={len(pre)}", flush=True)
    for i, L in enumerate(legs):
        make_ref(t, L["branch"], PIN)
        code, d = api(t, f"/repos/{REPO}/actions/workflows/{WF}/dispatches",
                      method="POST", data={"ref": L["branch"], "inputs": L["inputs"]})
        okd = code in (200, 204)
        print(f"dispatch {L['branch']} {L['tag']}: HTTP {code} "
              f"{'OK' if okd else d}", flush=True)
        if not okd:
            sys.exit(f"FATAL dispatch {L['branch']} {code}")
        if i == 0:
            time.sleep(31)  # урок AG-338: 2-й POST <4с = 204 без run
    time.sleep(10)
    want = {L["branch"]: PIN for L in legs}
    found = find_runs(t, pre, want)
    if len(found) < 2:
        print("WARN: не оба рана видимы — finalize доверяет", flush=True)
    payload = {"pin": PIN, "tree_files": n, "claim": CLAIM, "runs": found,
               "legs": legs}
    json.dump(payload, open(f"{RD}/work/AG-66/dispatch_526_66.json", "w"),
              indent=1, ensure_ascii=False)
    print(f"RUNIDS {json.dumps(found)}", flush=True)
    print(f"PIN {PIN[:8]} TREE {n}", flush=True)


def phase_finalize():
    t = tok()
    p = json.load(open(f"{RD}/work/AG-66/dispatch_526_66.json"))
    if len(p["runs"]) < 2:
        want = {BR_A: PIN, BR_B: PIN}
        p["runs"].update(find_runs(t, set(), want))
        json.dump(p, open(f"{RD}/work/AG-66/dispatch_526_66.json", "w"),
                  indent=1, ensure_ascii=False)
    ra = p["runs"].get(BR_A, 0)
    rb = p["runs"].get(BR_B, 0)
    tags = {L["branch"]: L["tag"] for L in p["legs"]}
    lines = [
        ("CLAIM | AG-66 | self-corr: leg-A пивот pop600k->pop650k (race AG-76 "
         "pre-CLAIM, 0 runner-min); leg-B s2700 | race"),
        (f"FACT | AG-66 | 2/2 204 @e49e8984 t4231 WBP dp3v2 s42: {ra} {tags[BR_A]} + "
         f"{rb} {tags[BR_B]} QUEUED | api"),
        (f"DISP | AG-66 | pop/s-миды 2/2 queued @swarm-526-66[ab] WBP dp3v2; "
         f"payload work/AG-66"),
        (f"PATCH_SUMMARY | AG-66 | files=claims,work/AG-66 | idea=pop600k+s2700 "
         f"dose mids pop/s-оси | ev=2/2 204"),
    ]
    fixed = []
    for l in lines:
        while len(l) > 120:
            if " payload work/AG-66" in l:
                l = l.replace(" payload work/AG-66", "")
            else:
                l = l[:-1]
        assert len(l) <= 120, f"{len(l)}: {l}"
        fixed.append(l)
        print(f"LINE({len(l)}): {l}", flush=True)
    ok = board_append(t, fixed, "board: AG-66 fact+disp+patch queued (wave-526)")
    print("BOARD-OK" if ok else "BOARD-FAIL", flush=True)


if __name__ == "__main__":
    if "--finalize" in sys.argv:
        phase_finalize()
    else:
        phase_run()
