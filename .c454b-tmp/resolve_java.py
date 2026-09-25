#!/usr/bin/env python3
"""TASK-454-B STRICT-OR resolver for java lever-flag conflicts."""
import re, subprocess

LEVER_RE = re.compile(r'cmp\d+_[a-z0-9]+')
CHUNK_ORDER = ['cmp434_chunkpl', 'cmp435_chunk3', 'cmp437_chunk4', 'cmp444_chunk5', 'cmp450_chunk']

def levers(t):
    return set(LEVER_RE.findall(t))

def added_sorted(hl, bl):
    added = bl - hl
    return [l for l in CHUNK_ORDER if l in added] + sorted(l for l in added if l not in CHUNK_ORDER)

def resolve_hunk(head, branch):
    hl, bl = levers(head), levers(branch)
    added = added_sorted(hl, bl)
    if not added:
        return head, 'HEAD-only'
    if 'private static final String FLAG' in head:
        # colpush constants hunk: keep master FLAG5/FLAG6, append FLAG7..FLAG11 chunk levers
        base = max(int(m) for m in re.findall(r'FLAG(\d+) = ', branch)) + 1
        start = int(re.findall(r'FLAG(\d+) = ', head)[-1]) + 1
        lines = [head.rstrip('\n')]
        for i, lev in enumerate(added):
            lines.append(f'    private static final String FLAG{start+i} = "{lev}";')
        return '\n'.join(lines) + '\n', f'constants union (+{len(added)} FLAGs from FLAG{start})'
    # boolean hunk: find idiom of last lever in HEAD
    matches = list(re.finditer(r'"(cmp\d+_[a-z0-9]+)"', head))
    last = matches[-1]
    tail = head[last.end():last.end()+40]
    m_eq_after = re.match(r'\.equals\(([A-Za-z_][A-Za-z0-9_]*)\)', tail)  # "L".equals(VAR)
    before = head[:last.start()]
    m_var = re.search(r'([A-Za-z_][A-Za-z0-9_.()]*)\.equals\(\s*$', before)  # VAR.equals("L")
    if m_eq_after:
        var = m_eq_after.group(1)
        exprs = [f'"{lev}".equals({var})' for lev in added]
        joiner = ' || '
    elif m_var:
        var = m_var.group(1)
        exprs = [f'{var}.equals("{lev}")' for lev in added]
        joiner = ' || '
    else:
        exprs = [f'"{lev}"' for lev in added]
        joiner = ' || '
    insert = ''.join(f' {joiner.strip()} {e}'.replace(' || ', ' || ', 1) if False else f' || {e}' for e in exprs)
    pos = last.end()
    # careful: idiom A needs insertion AFTER the closing paren of .equals("L")
    if m_eq_after:
        m2 = re.match(r'\.equals\([A-Za-z_][A-Za-z0-9_]*\)', head[pos:])
        pos += m2.end()
    resolved = head[:pos] + insert + head[pos:]
    return resolved, f'union (+{",".join(added)})'

def process(path):
    src = open(path).read()
    lines = src.split('\n')
    out, i, notes, hn = [], 0, [], 0
    while i < len(lines):
        if lines[i].startswith('<<<<<<<'):
            hn += 1
            head, branch = [], []
            i += 1
            while not lines[i].startswith('======='):
                head.append(lines[i]); i += 1
            i += 1
            while not lines[i].startswith('>>>>>>>'):
                branch.append(lines[i]); i += 1
            i += 1
            resolved, note = resolve_hunk('\n'.join(head), '\n'.join(branch))
            out.append(resolved)
            notes.append(f'  hunk{hn}: {note}')
        else:
            out.append(lines[i]); i += 1
    open(path, 'w').write('\n'.join(out))
    print(f'{path}:'); print('\n'.join(notes))

files = subprocess.check_output(['git', 'status', '--short'], text=True)
java = [l.split()[1] for l in files.splitlines() if l.startswith('UU') and l.split()[1].endswith('.java')]
for f in java:
    process(f)
print('DONE', len(java))
