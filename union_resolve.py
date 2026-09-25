#!/usr/bin/env python3
"""union_resolve.py — TASK-455-B STRICT-OR conflict resolver for the
chunk-union rebaze-3 merge (round-455b-chunk).

Resolves every conflicted text file by UNION semantics:
  * comment-only hunks           -> ours lines then theirs lines (dedup)
  * lever-gate line hunks        -> union of lever ids (ordered, ours first),
                                    new ids cloned via ours' reference pattern
                                    (rust `v == "id"` / java `f.trim().equals("id")`)
  * generic hunks                -> ours then theirs, dedup identical lines
NEVER drops a lever needle from either side (mirror-drift lesson x451/x452).
Usage: python3 union_resolve.py            (run inside worktree with conflicts)
"""
import re
import subprocess
import sys

ID_RE = re.compile(r"cmp\d+_[a-z0-9]+")
MARK_O, MARK_M, MARK_C = "<<<<<<<", "=======", ">>>>>>>"


def split_hunks(text):
    """Yield (kind, payload) — kind in {'plain','hunk'}; hunk payload =
    (ours_lines, theirs_lines)."""
    lines = text.split("\n")
    out, buf, mode = [], [], 0  # 0 plain, 1 ours, 2 theirs
    ours, theirs = [], []
    for ln in lines:
        if ln.startswith(MARK_O):
            if buf:
                out.append(("plain", buf)); buf = []
            mode, ours, theirs = 1, [], []
        elif ln.startswith(MARK_M) and mode == 1:
            mode = 2
        elif ln.startswith(MARK_C) and mode == 2:
            out.append(("hunk", (ours, theirs)))
            mode = 0
        else:
            if mode == 0:
                buf.append(ln)
            elif mode == 1:
                ours.append(ln)
            else:
                theirs.append(ln)
    if buf:
        out.append(("plain", buf))
    return out


def is_comment(ln):
    s = ln.strip()
    return (not s) or s.startswith("///") or s.startswith("//") or s.startswith("*") or s.startswith("/*")


def pattern_for(ours_line, missing_id):
    """Clone the reference pattern of an existing id in ours_line for missing_id."""
    for m in re.finditer(r'[A-Za-z_\)\]]*\s*(?:==|\.equals\()\s*"[^"]*"', ours_line):
        seg = m.group(0).strip()
        return seg.replace(re.search(r'"([^"]*)"', seg).group(1), missing_id)
    return f'v == "{missing_id}"'


def union_gate_line(ours, theirs):
    """Union lever ids of two gate lines; ours structure + theirs' new ids."""
    ids_o, ids_t = [], []
    for m in ID_RE.finditer(ours):
        if m.group(0) not in ids_o:
            ids_o.append(m.group(0))
    for m in ID_RE.finditer(theirs):
        if m.group(0) not in ids_t:
            ids_t.append(m.group(0))
    missing = [i for i in ids_t if i not in ids_o]
    if not missing:
        return ours
    # insertion point: before trailing // comment (if any), else at end
    cpos = ours.rfind("//")
    if cpos == -1:
        head, tail = ours, ""
    else:
        head, tail = ours[:cpos], ours[cpos:]
    # find reference pattern from head
    segs = []
    ref = None
    for m in re.finditer(r'[A-Za-z_\)\]]*\s*(?:==|\.equals\()\s*"[^"]*"', head):
        ref = m.group(0).strip()
        break
    add = ""
    for mid in missing:
        piece = ref.replace(re.search(r'"([^"]*)"', ref).group(1), mid) if ref else f'v == "{mid}"'
        add += " || " + piece
    head = head.rstrip()
    if head.endswith(")"):
        head = head[:-1].rstrip()
    merged = head + add + " " + tail if tail else head + add
    return merged


def resolve_hunk(ours, theirs):
    if all(is_comment(l) for l in ours + theirs):
        seen, out = set(), []
        for l in ours + theirs:
            if l not in seen:
                seen.add(l); out.append(l)
        return out
    # single gate-line pair -> id union
    o = [l for l in ours if l.strip()]
    t = [l for l in theirs if l.strip()]
    if len(o) == 1 and len(t) == 1 and ID_RE.search(o[0]) and ID_RE.search(t[0]):
        return [union_gate_line(o[0], t[0])]
    # generic: ours then theirs dedup
    seen, out = set(), []
    for l in ours + theirs:
        if l not in seen:
            seen.add(l); out.append(l)
    return out


def resolve_text(text):
    parts = split_hunks(text)
    out = []
    for kind, payload in parts:
        if kind == "plain":
            out.extend(payload)
        else:
            out.extend(resolve_hunk(*payload))
    return "\n".join(out)


def conflicted():
    st = subprocess.run(["git", "status", "--porcelain"], capture_output=True, text=True).stdout
    files = []
    for ln in st.splitlines():
        code, path = ln[:2], ln[3:]
        if code in ("UU", "AA"):
            files.append(path)
    return files


def main():
    files = conflicted()
    txt = [f for f in files if not f.endswith(".class")]
    binf = [f for f in files if f.endswith(".class")]
    for f in binf:
        subprocess.run(["git", "checkout", "--ours", "--", f], check=True)
        print(f"OURS(blob) {f}")
    for f in txt:
        raw = open(f, encoding="utf-8", errors="replace").read()
        merged = resolve_text(raw)
        if MARK_O in merged or MARK_M in merged or MARK_C in merged:
            print(f"FAIL unresolved markers remain: {f}")
            sys.exit(2)
        open(f, "w", encoding="utf-8").write(merged)
        subprocess.run(["git", "add", "--", f], check=True)
        print(f"UNION {f}")
    print(f"resolved {len(txt)} text + {len(binf)} blob files")


if __name__ == "__main__":
    main()
