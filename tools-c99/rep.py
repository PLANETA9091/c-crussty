#!/usr/bin/env python3
"""Exact-match replacer: python3 tools-c99/rep.py file < oldfile > newfile.
Reads old/new text from files to avoid shell-quoting pain."""
import sys

path = sys.argv[1]
old = open(sys.argv[2], encoding='utf-8').read()
new = open(sys.argv[3], encoding='utf-8').read()
src = open(path, encoding='utf-8').read()
n = src.count(old)
if n != 1:
    print(f"ERROR: {path}: {n} occurrences of old text", file=sys.stderr)
    sys.exit(1)
open(path, 'w', encoding='utf-8').write(src.replace(old, new))
print(f"OK {path}")
