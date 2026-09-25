#!/usr/bin/env python3
"""TASK-454-B STRICT-OR conflict resolver for rust lever gates.

For each conflict hunk: union of lever ids (branch-only levers appended to
HEAD side, preserving master verbatim + style). Two special cases:
- entity_query.rs hunk with flag_enabled(Some(s)) pin -> HEAD only.
- queryplane.rs marker-id match arms -> HEAD + branch verbatim.
"""
import re, sys, subprocess

LEVER_RE = re.compile(r'cmp\d+_[a-z0-9]+')

def levers(text):
    return set(LEVER_RE.findall(text))

def resolve_hunk(head, branch, path, idx):
    hl, bl = levers(head), levers(branch)
    added = bl - hl
    # special case 1: flag_enabled pin (master one-source-of-truth)
    if 'flag_enabled(Some(s))' in head:
        return head, 'HEAD-only (flag_enabled pin; chunk levers live in flag_enabled)'
    # special case 2: queryplane marker-id match arms -> concatenate both
    if '=> "' in head and '=> "' in branch:
        return head + branch, 'HEAD+branch (marker-id match arms union)'
    if not added:
        return head, 'HEAD-only (no branch-only levers)'
    # find last quoted lever occurrence in HEAD, extract operator prefix style
    matches = list(re.finditer(r'"(cmp\d+_[a-z0-9]+)"', head))
    last = matches[-1]
    # operator prefix: text from previous delimiter (|| or |) to the quote
    before = head[:last.start()]
    m = re.search(r'(\|\|\s*[A-Za-z_.()=!]*\s*==\s*|\|\s*(?:Ok|Some)\()\s*$', before)
    if m:
        prefix = m.group(1)
        sep = ' ' if prefix.startswith('| O') or prefix.startswith('| S') else ''
    else:
        prefix = '|| v == '
        sep = ' '
    parts = []
    for lev in sorted(added):
        parts.append(f'{sep}{prefix}"{lev}"')
    insert = ''.join(parts)
    pos = last.end()
    resolved = head[:pos] + insert + head[pos:]
    return resolved, f'union (+{",".join(sorted(added))})'

def process(path):
    src = open(path).read()
    lines = src.split('\n')
    out, i, notes, hunk_no = [], 0, [], 0
    while i < len(lines):
        if lines[i].startswith('<<<<<<<'):
            hunk_no += 1
            head, branch = [], []
            i += 1
            while not lines[i].startswith('======='):
                head.append(lines[i]); i += 1
            i += 1
            while not lines[i].startswith('>>>>>>>'):
                branch.append(lines[i]); i += 1
            i += 1  # skip >>>>>>> line
            resolved, note = resolve_hunk('\n'.join(head), '\n'.join(branch), path, hunk_no)
            out.append(resolved)
            notes.append(f'  hunk{hunk_no}: {note}')
        else:
            out.append(lines[i]); i += 1
    open(path, 'w').write('\n'.join(out))
    print(f'{path}:')
    print('\n'.join(notes))

files = subprocess.check_output(
    ['git', 'status', '--short'], text=True)
rust = [l.split()[1] for l in files.splitlines()
        if l.startswith('UU') and l.split()[1].startswith('src/')]
for f in rust:
    process(f)
print('DONE', len(rust), 'files')
