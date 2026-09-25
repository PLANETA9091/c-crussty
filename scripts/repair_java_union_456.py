#!/usr/bin/env python3
"""TASK-456-A: repair java flag-list unions after resolve_union.py damage.
The generic resolver appended rust-style Ok("cmp455_spawn") fragments into
java files. This script replaces them with proper java STRICT-OR syntax
(canon ×455-A: same-list additions only, needles verbatim).
"""
import re, sys

DAMAGE = re.compile(r'\s*\|\s*Ok\("cmp455_spawn"\)')

def fix(path):
    src = open(path).read()
    lines = src.split('\n')
    out = []
    for ln in lines:
        if 'Ok("cmp455_spawn")' not in ln:
            out.append(ln)
            continue
        clean = DAMAGE.sub('', ln).rstrip()
        if clean.endswith('));'):
            # boolean chain: insert java-form before closing
            body = clean[:-3].rstrip()
            body += ' || f.trim().equals("cmp455_spawn")'
            out.append(body + '));')
        elif clean.rstrip().endswith('; //') or clean.rstrip().endswith(';'):
            # LEVER_FLAG chain: insert before the ';'
            idx = clean.rfind(';')
            body = clean[:idx].rstrip()
            body += ' || "cmp455_spawn".equals(LEVER_FLAG)'
            out.append(body + clean[idx:])
        else:
            out.append(clean + '  # NEEDS-MANUAL')
    open(path, 'w').write('\n'.join(out))

for p in sys.argv[1:]:
    fix(p)
    print(f'fixed {p}')
