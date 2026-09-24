#!/usr/bin/env python3
"""Fix malformed appended fragments from resolve_union.py:
1. flag accidentally placed inside a trailing line comment -> move to own line above
2. `| || var == "flag"` syntax error -> own line
Then ensure a clean union: every sense flag lives on its own valid line.
"""
import re, sys

TOK = re.compile(r'cmp\d{3}[0-9_a-z]*')

def fix(path):
    lines = open(path).read().split('\n')
    out = []
    fixed = 0
    for ln in lines:
        # case 2: malformed `| || expr` inside a line (rust boolean or-chain)
        m = re.search(r'\|\s*\|\|\s*([\w.()]+?\s*==\s*"cmp\d{3}[0-9_a-z]*")', ln)
        if m:
            frag = m.group(1)
            ln2 = re.sub(r'\s*\|\s*\|\|\s*' + re.escape(frag), '', ln)
            out.append(ln2.rstrip())
            out.append('                || ' + frag + ' // TASK-444-C: sense family union')
            fixed += 1
            continue
        # case 1: `| Ok("cmpX")` (or multiple) stuck after a // comment
        if '//' in ln:
            code, comment = ln.split('//', 1)
            bad = re.findall(r'\|\s*Ok\("cmp\d{3}[0-9_a-z]*"\)', code + ' ' + comment)
            if bad and re.search(r'\|\s*Ok\("cmp\d{3}[0-9_a-z]*"\)', comment):
                # flags inside comment: strip them from comment, put as own line BEFORE the commented line
                flags = re.findall(r'Ok\("cmp\d{3}[0-9_a-z]*"\)', comment)
                base = re.sub(r'\s*\|\s*Ok\("cmp\d{3}[0-9_a-z]*"\)', '', comment)
                for f in flags:
                    out.append('            | ' + f + ' // TASK-444-C: sense family union')
                    fixed += 1
                out.append(code.rstrip() + ' // ' + base)
                continue
        out.append(ln)
    open(path, 'w').write('\n'.join(out))
    return fixed

total = 0
for p in sys.argv[1:]:
    n = fix(p)
    if n:
        print(f'{p}: {n} fixed')
    total += n
print('TOTAL', total)
