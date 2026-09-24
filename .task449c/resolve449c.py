#!/usr/bin/env python3
"""TASK-449-C: resolve lever-gate conflicts (carrier HEAD + collide theirs) with unified lever retag."""
import re, sys

RETAG = {"cmp443_mega": "cmp449_mega4", "cmp445_collide": "cmp449_mega4", "cmp446_items": "cmp449_mega4"}
COMMENT443 = "// TASK-443-B: mega-composition carrier (ins4d + chunk4 union)."
COMMENT449 = "// TASK-449-C: mega4 composite (megafix ins4d+chunk4 + items-fix + collide-step2; retag cmp443_mega/cmp445_collide/cmp446_items -> cmp449_mega4)."

def retag(s):
    for a, b in RETAG.items():
        s = s.replace(f'"{a}"', f'"{b}"')
    s = s.replace(COMMENT443, COMMENT449)
    return s

pat = re.compile(r"<<<<<<< HEAD\n(.*?)\n=======\n(.*?)\n>>>>>>> [^\n]*\n", re.S)

def resolve(path, special_comp=False):
    text = open(path).read()
    out, pos, idx = [], 0, 0
    for m in pat.finditer(text):
        out.append(text[pos:m.start()])
        head, theirs = m.group(1), m.group(2)
        h = retag(head)
        # documentation comment lines from theirs not present in head
        for line in theirs.split("\n"):
            ls = line.strip()
            if ls.startswith("//") and ls not in h and ls != "// TASK-445-A: collide+broadphase+push plane round (additive STRICT-OR).":
                h = h + "\n" + line.rstrip()
        if special_comp and idx == 1:
            # items_manager comp line: theirs had collide INSIDE comp before ';'
            h = h.replace('|| flag == "cmp420_colpush";', '|| flag == "cmp420_colpush" || flag == "cmp449_mega4";', 1)
        else:
            t_tokens = set(re.findall(r'"(cmp4[0-9a-z_]+)"', theirs))
            h_tokens = set(re.findall(r'"(cmp4[0-9a-z_]+)"', h))
            for tok in sorted(t_tokens - h_tokens):
                tok2 = RETAG.get(tok, tok)
                if tok2 in h_tokens:
                    continue
                for line in theirs.split("\n"):
                    if f'"{tok}"' in line and "==" in line:
                        h = h + "\n" + retag(line.rstrip())
                        h_tokens.add(tok2)
                        break
        out.append(h + "\n")
        pos = m.end()
        idx += 1
    out.append(text[pos:])
    new = "".join(out)
    open(path, "w").write(new)
    print(f"{path}: {idx} conflicts resolved, remaining markers: {new.count('<<<<<<<')}")

if __name__ == "__main__":
    files_mech = ["src/colpush.rs", "src/queryplane.rs", "src/chunk_parse.rs", "src/mobs_grid.rs",
                  "src/mobs_manager.rs", "src/nav_plane.rs", "src/tickplane.rs", "src/items_index.rs",
                  "src/mobs_soa.rs", "src/collide_batch.rs"]
    for f in files_mech:
        resolve(f)
    resolve("src/items_manager.rs", special_comp=True)
