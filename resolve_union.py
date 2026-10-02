#!/usr/bin/env python3
"""TASK-444-C union resolver: merge STRICT-OR tail conflicts head(mega)+sense.
Boolean tails: keep HEAD lines, append sense-only flag fragments (same style).
Match-arm maps (=>): keep HEAD lines, append sense-only lines verbatim.
"""
import re, sys, glob

TOK = re.compile(r'cmp\d{3}[0-9_a-z]*')

def resolve(path):
    src = open(path).read()
    out, i, hunks = [], 0, 0
    lines = src.split('\n')
    while i < len(lines):
        ln = lines[i]
        if ln.startswith('<<<<<<<'):
            hunks += 1
            i += 1
            head, sense = [], []
            state = 'head'
            while i < len(lines):
                l2 = lines[i]
                if l2.startswith('======='):
                    state = 'sense'; i += 1; continue
                if l2.startswith('>>>>>>>'):
                    i += 1; break
                (head if state == 'head' else sense).append(l2)
                i += 1
            head_toks = set(TOK.findall('\n'.join(head)))
            sense_toks = set(TOK.findall('\n'.join(sense)))
            extras = [t for t in TOK.findall('\n'.join(sense)) if t not in head_toks]
            if '=>' in '\n'.join(sense) or '=>' in '\n'.join(head):
                # match-arm map union: HEAD lines + sense-only lines verbatim
                head_set = set(head)
                merged = head + [l for l in sense if l not in head_set and l.strip()]
            else:
                merged = list(head)
                if merged and extras:
                    last = len(merged) - 1
                    # style detection from sense side
                    for t in extras:
                        frag = None
                        for sl in sense:
                            m = re.search(r'((?:\|\|\s*)?[\w.()]+?\s*==\s*"%s"|Ok\("%s"\))' % (t, t), sl)
                            if m:
                                frag = m.group(1); break
                        if frag is None:
                            frag = 'Ok("%s")' % t
                        merged[last] = merged[last].rstrip() + (' | ' + frag if not merged[last].rstrip().endswith('||') else ' ' + frag)
            out.extend(merged)
        else:
            out.append(ln)
            i += 1
    open(path, 'w').write('\n'.join(out))
    return hunks

total = 0
for p in sys.argv[1:]:
    h = resolve(p)
    print(f'{p}: {h} hunks resolved')
    total += h
print(f'TOTAL {total}')
