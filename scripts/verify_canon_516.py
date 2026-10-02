#!/usr/bin/env python3
# AG-49 x516 — arbiter: offline MERGE-READINESS verifier for BENCH-V2 canon branches.
# Checks every swarm-516-* branch against the 6 canon components (wave goal #1)
# and commit-author law. Zero CI dispatches, pure local git. Read-only.
import subprocess, sys, re, json

REPO = "/home/z/c-crussty"
MASTER = "master"
CANON = {
    "tect025":      ("Tectonic 3.0.25 sha-pinned, 3.0.29 purged", "must"),
    "plugindim":    ("plugin setChunkForceLoaded (nether/end)", "must"),
    "async_driver": ("async driver / wall-clock heartbeat (anti FAKE-GREEN)", "must"),
    "fakeplayers":  ("fake-players lane (0 players = 0 natural spawn)", "must"),
    "seedgate":     ("seed-gate vs SEED-LEDGER in dispatch scripts", "must"),
    "canonbase":    ("view/sim 32 + 20k forceload + yml concurrency per-LEG intact", "must"),
}
AUTHOR_OK = "PLANETA9091"

def git(*args, data=False):
    r = subprocess.run(["git", "-C", REPO, *args], capture_output=True, text=True)
    return r.stdout if data else r.stdout.strip()

def branches():
    out = git("branch", "-r", "--format=%(refname:short)", data=True)
    return sorted(l.strip() for l in out.splitlines() if re.search(r"/swarm-516-", l))

def branch_content(b, path):
    r = subprocess.run(["git", "-C", REPO, "show", f"{b}:{path}"], capture_output=True)
    if r.returncode != 0 or b"\x00" in r.stdout[:4096]:
        return None  # binary or missing
    return r.stdout.decode("utf-8", errors="replace")

def run():
    rows, shortlist = [], []
    for b in branches():
        short = b.split("/")[-1]
        files = git("ls-tree", "-r", "--name-only", b, data=True).splitlines()
        blob_all = ""
        # gather only relevant files to keep it fast
        keys = ("run_benchv2", "seed_gate", "dispatch", "bench-v2.yml", "plugin", "fake", "dimload", "forceload")
        rel = [f for f in files if any(k in f.lower() for k in keys)]
        for f in rel:
            c = branch_content(b, f)
            if c: blob_all += f"\n--{f}--\n" + c
        runner = branch_content(b, "bench/worldv2/run_benchv2.sh") or ""
        yml = branch_content(b, ".github/workflows/bench-v2.yml") or ""
        checks = {}
        # C1 tectonic 3.0.25 canon sha 7b3c5dee, no 3.0.29/pxgiJaJp
        checks["tect025"] = ("3.0.25" in runner and "7b3c5dee" in runner
                             and "pxgiJaJp" not in runner
                             and "tectonic-datapack-3.0.29" not in runner)
        # C2 plugin-dim-forceload (setChunkForceLoaded or addPluginChunkTicket)
        checks["plugindim"] = ("setChunkForceLoaded" in blob_all
                               or "addPluginChunkTicket" in blob_all)
        # C3 async driver / heartbeat sampler (not vanilla sync forceload-only)
        hb = ("heartbeat" in blob_all.lower()) or ("G-HB" in blob_all)
        sync_vanilla = re.search(r"forceload add \$", runner) is not None
        checks["async_driver"] = hb and not sync_vanilla
        # C4 fake players
        checks["fakeplayers"] = ("fakeplayers" in blob_all.lower() and "plugin.yml" in blob_all) or "FAKE_PLAYERS" in runner
        # C5 seed gate
        checks["seedgate"] = ("seed_gate" in blob_all) or ("SEED-LEDGER" in blob_all) or ("seed_ledger" in blob_all.lower())
        # C6 canon base params intact
        checks["canonbase"] = ("view-distance=32" in runner and "simulation-distance=32" in runner
                               and "1136" in runner and "cancel-in-progress" in yml
                               and "${{ github.ref }}" in yml)
        # commits ahead of master + author law
        ahead = git("rev-list", "--count", f"{MASTER}..{b}")
        bad_authors = set()
        for ln in git("log", "--format=%an|%ae|%h", f"{MASTER}..{b}", data=True).splitlines():
            if AUTHOR_OK not in ln: bad_authors.add(ln.strip())
        score = sum(checks.values())
        missing = [k for k, v in checks.items() if not v]
        verdict = ("MERGE-READY" if score == 6 and not bad_authors else
                   "CANDIDATE" if score >= 4 and not bad_authors else
                   "AUTHOR-FAIL" if bad_authors else "INCOMPLETE")
        rows.append({"branch": short, "score": f"{score}/6", "missing": missing,
                     "ahead": ahead, "files": len(files), "bad_authors": sorted(bad_authors),
                     "verdict": verdict})
        if verdict in ("MERGE-READY", "CANDIDATE"): shortlist.append((verdict, short, score, ahead))
    return rows, shortlist

if __name__ == "__main__":
    rows, shortlist = run()
    print(json.dumps(rows, indent=1, ensure_ascii=False))
    print("\n=== SHORTLIST ===")
    for v, b, s, a in sorted(shortlist, key=lambda x: -x[2]):
        print(f"{v:12s} {b:18s} {s}/6 ahead={a}")
