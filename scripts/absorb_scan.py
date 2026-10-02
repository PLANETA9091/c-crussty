#!/usr/bin/env python3
"""absorb_scan.py — generic round absorb-scan (v3, generalized from absorb_scan_476.py).

Scans recent GH Actions runs of the repo; splits them into:
  * fresh completed  (status==completed AND updated_at >= --since)
  * live             (status in {in_progress, queued, ...})

Generalization vs tick-476 one-off:
  --round round-476   round tag -> branch regex (branch containing "round-476" => in_round),
                      also derives default board/registry paths:
                      /home/z/rounds/ROUND-476/board, /home/z/rounds/ROUND-476/absorb/registry_476.json
  --branch-regex RE   override the branch regex outright (beats --round)
  --since ISO8601Z    freshness boundary (default: now-24h)
  --needs-absorb      cross-check fresh+live legs against board/CLM-*.md: a leg is COVERED if its
                      run_id, full branch name or normalized branch core (round-/retry suffixes
                      stripped) appears in any CLM-*.md; otherwise flagged needs-absorb.
                      master CI runs (branch==master, workflow 'ci') are excluded from the check.

Output: registry JSON (--out, default /home/z/rounds/ROUND-<N>/absorb/registry_<N>.json) + stdout table.
No writes to git; no pushes.
"""
import argparse, glob, json, os, re, sys, time, urllib.request, urllib.error

REPO_DEFAULT = "PLANETA9091/c-crussty"
TOKEN_FILE = "/tmp/gh_token"
API = "https://api.github.com"
# merge-candidate keywords (round-independent; --round regex is applied separately as in_round)
MERGE_PAT_DEFAULT = r"(itemidle|dnt|gold-l9|gold-l15|p31snap|poi|comp4|c98|coll)"


def token(path=TOKEN_FILE):
    return open(path).read().strip()


def api(tok, url):
    req = urllib.request.Request(API + url, headers={
        "Authorization": f"Bearer {tok}", "Accept": "application/vnd.github+json"})
    for attempt in range(3):
        try:
            with urllib.request.urlopen(req, timeout=60) as r:
                return json.loads(r.read() or b"{}")
        except urllib.error.HTTPError as e:
            if e.code in (403, 429) and attempt < 2:
                time.sleep(5 * (attempt + 1)); continue
            raise
        except urllib.error.URLError:
            if attempt < 2: time.sleep(3); continue
            raise


def branch_keys(branch):
    """Keys used for CLM matching: full branch + core with round-/retry suffixes stripped."""
    if not branch:
        return set()
    b = branch.strip().lower()
    keys = {b}
    core = re.sub(r"^round-\d+-", "", b)
    keys.add(core)
    keys.add(re.sub(r"-r\d+$", "", core))   # strip -r3-style retry suffix
    return keys


def load_board_index(board_dir):
    """Concatenated lowercase text of all board/CLM-*.md files (run_id/branch coverage index)."""
    chunks = []
    files = sorted(glob.glob(os.path.join(board_dir, "CLM-*.md")))
    for f in files:
        try:
            chunks.append(open(f, encoding="utf-8", errors="replace").read().lower())
        except OSError:
            pass
    return files, "\n".join(chunks)


def main():
    ap = argparse.ArgumentParser(description="generic absorb scan")
    ap.add_argument("--repo", default=REPO_DEFAULT)
    ap.add_argument("--round", default="round-476",
                    help="round tag; builds branch regex + default board/out paths (e.g. round-476)")
    ap.add_argument("--branch-regex", default=None,
                    help="override branch regex for in_round flag (default: contains --round)")
    ap.add_argument("--since", default=None,
                    help="ISO8601Z boundary for fresh completed runs (default: now-24h)")
    ap.add_argument("--pages", type=int, default=3)
    ap.add_argument("--per-page", type=int, default=100)
    ap.add_argument("--merge-pattern", default=MERGE_PAT_DEFAULT,
                    help="regex for merge_track flag on branch names")
    ap.add_argument("--board", default=None,
                    help="board dir with CLM-*.md (default: /home/z/rounds/ROUND-<N>/board)")
    ap.add_argument("--out", default=None,
                    help="registry json path (default: /home/z/rounds/ROUND-<N>/absorb/registry_<N>.json)")
    ap.add_argument("--token-file", default=TOKEN_FILE)
    ap.add_argument("--needs-absorb", action="store_true",
                    help="cross-check fresh+live legs against board/CLM-*.md coverage")
    ap.add_argument("--only-in-round", action="store_true",
                    help="with --needs-absorb: check only legs matching the branch regex")
    ap.add_argument("--include-master", action="store_true",
                    help="do not exclude master ci runs from needs-absorb check")
    args = ap.parse_args()

    m = re.search(r"(\d+)", args.round)
    rnum = m.group(1) if m else "476"
    board_dir = args.board or f"/home/z/rounds/ROUND-{rnum}/board"
    out_path = args.out or f"/home/z/rounds/ROUND-{rnum}/absorb/registry_{rnum}.json"
    branch_re = re.compile(args.branch_regex) if args.branch_regex else re.compile(re.escape(args.round), re.I)
    merge_re = re.compile(args.merge_pattern, re.I)
    since = args.since or time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime(time.time() - 24 * 3600))

    tok = token(args.token_file)
    runs = []
    for p in range(1, args.pages + 1):
        d = api(tok, f"/repos/{args.repo}/actions/runs?per_page={args.per_page}&page={p}")
        runs.extend(d.get("workflow_runs", []))

    fresh, live = [], []
    for r in runs:
        rec = {
            "run_id": r["id"], "branch": r.get("head_branch"),
            "workflow": (r.get("name") or "").strip(),
            "conclusion": r.get("conclusion"), "status": r.get("status"),
            "event": r.get("event"), "updated": r.get("updated_at"),
            "created": r.get("created_at"),
            "in_round": bool(branch_re.search(r.get("head_branch") or "")),
            "merge_track": bool(merge_re.search(r.get("head_branch") or "")),
        }
        if r.get("status") == "completed":
            if rec["updated"] and rec["updated"] >= since:
                fresh.append(rec)
        else:
            live.append(rec)

    needs_absorb = []
    board_files, board_text = [], ""
    if args.needs_absorb:
        board_files, board_text = load_board_index(board_dir)
        for rec in fresh + live:
            if not args.include_master and rec["branch"] == "master" and rec["workflow"].lower() == "ci":
                continue
            if args.only_in_round and not rec["in_round"]:
                continue
            covered = any((k and k in board_text) or str(rec["run_id"]) in board_text
                          for k in branch_keys(rec["branch"]))
            if not covered:
                needs_absorb.append({"run_id": rec["run_id"], "branch": rec["branch"],
                                     "status": rec["status"], "conclusion": rec["conclusion"],
                                     "updated": rec["updated"], "live": rec["status"] != "completed"})

    os.makedirs(os.path.dirname(out_path), exist_ok=True)
    json.dump({"generated": time.strftime("%Y-%m-%dT%H:%M:%SZ", time.gmtime()), "repo": args.repo,
               "round": args.round, "branch_regex": branch_re.pattern, "since": since,
               "scanned": len(runs), "fresh_completed_since": since, "fresh": fresh, "live": live,
               "needs_absorb": needs_absorb, "board_files_checked": [os.path.basename(f) for f in board_files]},
              open(out_path, "w"), indent=1)

    print(f"scanned={len(runs)} fresh_completed={len(fresh)} live={len(live)} since={since} round={args.round}")
    in_round_fresh = [f for f in fresh if f["in_round"]]
    print(f"in_round fresh={len(in_round_fresh)} merge_track fresh={sum(1 for f in fresh if f['merge_track'])}")
    print("--- LIVE (in_progress/queued) ---")
    for l in sorted(live, key=lambda x: x["created"] or "", reverse=True):
        tag = " <<ROUND" if l["in_round"] else ""
        print(f'{l["run_id"]} | {l["branch"]} | {l["status"]} | {l["workflow"][:28]}{tag}')
    print("--- FRESH COMPLETED (newest first) ---")
    for f in sorted(fresh, key=lambda x: x["updated"], reverse=True):
        flags = (" <<ROUND" if f["in_round"] else "") + (" <<MERGE-TRACK" if f["merge_track"] else "")
        print(f'{f["run_id"]} | {f["branch"]} | {f["workflow"][:28]} | {f["conclusion"]} | {f["updated"]}{flags}')
    if args.needs_absorb:
        print(f"--- NEEDS-ABSORB ({len(needs_absorb)}) vs {len(board_files)} CLM files in {board_dir} ---")
        for n in needs_absorb:
            print(f'{n["run_id"]} | {n["branch"]} | {n["status"]}/{n["conclusion"]} | {n["updated"] or "-"}')
        if not needs_absorb:
            print("all legs covered by CLM-*")
    print(f"registry: {out_path}")


if __name__ == "__main__":
    main()
