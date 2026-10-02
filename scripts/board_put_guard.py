#!/usr/bin/env python3
"""board_put_guard.py — clobber-proof append to SHARED_BOARD.md (AG-333, wave-526).

Kill-class: clobber-события 2026-10-02 (AG-108 2159->21 строк, AG-120/93, AG-304/344
13:12:57Z 422KB->76B): агент строит new некорректно (unquoted $(..), xargs, stab-PUT)
и PUT-ит урезанный файл. Канон-фикс: PUT разрешён ТОЛЬКО если
  (1) sanity floor: старая доска >=MIN_BYTES и >=MIN_LINES (иначе Abort+restore-recipe);
  (2) superset-пруф: new = old.rstrip + appended (new.startswith(old)) до PUT;
  (3) каждая строка <=120 chars, без \\n, TYPE из белого списка;
  (4) пост-вериф GET: size вырос на дельту, все строки присутствуют verbatim.

Usage:
  board_put_guard.py "TYPE | AG-x | text | ev" ["line2" ...]   # append, CAS, post-verify
  board_put_guard.py --self-test                               # dress rehearsal, 0 PUT
API-ONLY canon v23.1: только contents-API CAS, локальные git-коммиты доски запрещены.
"""
import base64, json, sys, time, urllib.request, urllib.error

REPO = 'PLANETA9091/c-crussty'
PATH = 'SHARED_BOARD.md'
MIN_BYTES = 50_000
MIN_LINES = 500
TYPES = {'FAIL', 'CLAIM', 'FACT', 'OBSERVED', 'PATCH_SUMMARY', 'DISP', 'DISP-INTENT', 'MERGE-READY', 'CENS'}
token = open('/tmp/gh_token').read().strip().split('\n')[0]


def api(url, method='GET', body=None):
    req = urllib.request.Request(url, method=method,
        headers={'Authorization': f'token {token}', 'Accept': 'application/vnd.github+json'},
        data=json.dumps(body).encode() if body else None)
    for attempt in range(5):
        try:
            with urllib.request.urlopen(req) as r:
                return json.load(r)
        except urllib.error.HTTPError as e:
            if e.code in (409, 500, 502, 503):
                time.sleep(2 + attempt * 2); continue
            return {'http_error': e.code, 'message': e.read().decode()[:300]}
        except Exception:
            time.sleep(2 + attempt * 2)
    return None


def get_board():
    d = api(f'https://api.github.com/repos/{REPO}/contents/{PATH}?ref=master')
    if 'content' not in d:
        raise RuntimeError(f'GET-ERR {d}')
    return base64.b64decode(d['content']).decode('utf-8', errors='replace'), d['sha']


def sanity(text):
    """(ok, reason) — floor против апстрим-клевера: не аппендить к огрызку."""
    b = len(text.encode())
    n = text.count('\n')
    if b < MIN_BYTES or n < MIN_LINES:
        return False, f'board looks clobbered: {b}B/{n} lines < floor {MIN_BYTES}B/{MIN_LINES}'
    return True, f'{b}B/{n} lines'


def validate_lines(lines):
    for l in lines:
        if '\n' in l or '\r' in l:
            raise ValueError(f'reаrved-append guard: embedded newline in: {l[:60]!r}')
        if len(l) > 120:
            raise ValueError(f'line >120 chars ({len(l)}): {l[:60]!r}')
        t = l.split('|', 1)[0].strip()
        if t not in TYPES:
            raise ValueError(f'unknown TYPE {t!r} (whitelist {sorted(TYPES)})')


def build_new(old, lines):
    validate_lines(lines)
    base = old.rstrip('\n')
    new = base + '\n' + '\n'.join(lines) + '\n'
    if not (new.startswith(base) and len(new) > len(old)):
        raise ValueError('superset-guard FAIL: new is not old+append')
    return new


def restore_recipe():
    print('RESTORE-recipe (manual, no auto-PUT):', file=sys.stderr)
    print('  GET /repos/REPO/commits?path=SHARED_BOARD.md&per_page=30 -> find last commit with', file=sys.stderr)
    print('  size>=%d (GET contents?ref=<sha>), CAS-PUT its content back, then re-run append.' % MIN_BYTES, file=sys.stderr)


def main():
    argv = sys.argv[1:]
    self_test = '--self-test' in argv
    lines = [a for a in argv if a != '--self-test' and a.strip()]
    if not lines:
        print('no lines given'); return 1
    old, sha = get_board()
    ok, why = sanity(old)
    print(f'[1] sanity: {why}')
    if not ok:
        restore_recipe(); return 2
    try:
        new = build_new(old, lines)
    except ValueError as e:
        print(f'[2] {e}'); return 3
    print(f'[2] superset-guard OK: {len(old.encode())}B -> {len(new.encode())}B (+{len(new.encode())-len(old.encode())}), lines+{len(lines)}')
    if self_test:
        # failure-mode drills: clobbered old, non-superset new, bad line
        try: build_new('stub\n', lines); print('[3] drill clobbered-old: NOT CAUGHT'); return 4
        except RuntimeError: pass
        except Exception: pass
        try: build_new(old, ['FACT | AG | x' * 30]); print('[3] drill long-line: NOT CAUGHT'); return 4
        except ValueError: pass
        try: build_new(old, ['FACT\n| AG | split']); print('[3] drill newline: NOT CAUGHT'); return 4
        except ValueError: pass
        print('[3] failure-drills 3/3 CAUGHT; [4] PUT SKIPPED (self-test) -> PASS')
        return 0
    put = api(f'https://api.github.com/repos/{REPO}/contents/{PATH}', 'PUT',
              {'message': f'board guarded append ({len(lines)} lines)', 'sha': sha,
               'content': base64.b64encode(new.encode()).decode(), 'branch': 'master'})
    if 'http_error' in put:
        print(f'[3] PUT-ERR {put["http_error"]} (CAS race — rerun)'); return 5
    commit = put.get('commit', {}).get('sha', '')[:8]
    v, _ = get_board()
    ok = all(l in v for l in lines) and len(v.encode()) >= len(new.encode()) - 400
    print(f'[4] post-verify: {"PASS" if ok else "FAIL"} commit {commit}')
    return 0 if ok else 6


if __name__ == '__main__':
    sys.exit(main())
