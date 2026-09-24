#!/usr/bin/env python3
"""TASK-444-C: add cmp444_sensemega as STRICT-OR alternative at every gate site.
Rules (line-local, explicit):
- `| Ok("cmp443_mega")` tails  -> add own line `| Ok("cmp444_sensemega")`
- `|| VAR == "cmp443_mega"` tails -> add continuation `|| VAR == "cmp444_sensemega"`
- inline `... == "cmp443_mega" {` -> insert ` || VAR == "cmp444_sensemega"` before ` {`
- marker maps `Ok("cmp443_mega") => "cmp443_mega",` -> add arm for cmp444_sensemega
- label fns `"cmp443_mega" // comment` (bare expression) -> add own expression line
- mobs_sense.rs enabled() / brainhook.rs tick2_enabled() -> add Ok("cmp444_sensemega")
"""
import re, sys

CM = 'cmp444_sensemega'
TAG = '// TASK-444-C: sensemega composite carrier (STRICT OR)'

def handle(path):
    lines = open(path).read().split('\n')
    out, n = [], 0
    for ln in lines:
        # marker map arm:  Ok("cmp443_mega") => "cmp443_mega",
        if re.search(r'Ok\("cmp443_mega"\)\s*=>\s*"cmp443_mega"', ln):
            out.append(ln)
            indent = re.match(r'\s*', ln).group(0)
            out.append(f'{indent}Ok("{CM}") => "{CM}", {TAG}')
            n += 1
            continue
        # bare label expression:  "cmp443_mega" // comment   (inside if-block)
        if re.search(r'^\s*"cmp443_mega"\s*//', ln):
            out.append(ln)
            indent = re.match(r'\s*', ln).group(0)
            out.append(f'{indent}"{CM}" {TAG}')
            n += 1
            continue
        # inline if-condition ending with flag then ` {`
        m = re.search(r'^(.*== "cmp443_mega")\s*\{\s*$', ln)
        if m:
            var = 'f' if re.search(r'\bf == "cmp443_mega"', ln) else 'v'
            out.append(f'{m.group(1)} || {var} == "{CM}" {{ {TAG}')
            n += 1
            continue
        # Ok-tail:  | Ok("cmp443_mega") // comment   OR  | Ok("cmp443_mega")
        m = re.match(r'^(\s*)\|\s*Ok\("cmp443_mega"\)(.*)$', ln)
        if m:
            out.append(ln)
            out.append(f'{m.group(1)}| Ok("{CM}") {TAG.rstrip()}')
            n += 1
            continue
        # ||-tail containing the flag (with or without trailing comment)
        m = re.search(r'^(.*\|\|\s*([\w.()]+?)\s*==\s*"cmp443_mega")(.*)$', ln)
        if m:
            out.append(ln)
            var = m.group(2)
            indent = re.match(r'\s*', ln).group(0)
            out.append(f'{indent}|| {var} == "{CM}" {TAG}')
            n += 1
            continue
        out.append(ln)
    open(path, 'w').write('\n'.join(out))
    return n

total = 0
for p in sys.argv[1:]:
    c = handle(p)
    print(f'{p}: {c} sites')
    total += c
print('TOTAL', total)
