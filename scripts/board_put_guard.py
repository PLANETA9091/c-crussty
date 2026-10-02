#!/usr/bin/env python3
"""board_put_guard.py v2 — clobber-proof append to SHARED_BOARD.md (AG-333 w526; v2 AG-46 w527).

v1 kill-class (AG-333, waves <=526): агент строит `new` некорректно (unquoted $(..), xargs,
stab-PUT) и PUT-ит урезанный файл. v1 канон: PUT разрешён ТОЛЬКО если
  (1) sanity floor: старая доска >=MIN_BYTES и >=MIN_LINES (иначе Abort+restore-recipe);
  (2) superset-пруф: new = old.rstrip + appended (new.startswith(old)) до PUT;
  (3) каждая строка <=120 chars, без \n, TYPE из белого списка;
  (4) пост-вериф GET: size вырос на дельту, все строки присутствуют verbatim.

v2 additions (AG-46 w527; clobber-каскад волны-527 15:44-15:56Z):
  (5) >1MB wall (root-clobber AG-26 w527 15:47Z: contents-GET отдаёт content:""/encoding:none
      для файлов 1-100MB -> ad-hoc скрипты аппендили к пустоте): get_board() при пустом
      content / encoding=none / size>BLOB_FALLBACK_THRESHOLD читает ПОЛНЫЙ текст через
      git/blobs/{sha} (валидно до 100MB) — append-флоу живёт и за 1MB стеной;
  (6) idempotent-dedup: строки уже присутствующие verbatim — дропаются до PUT (kill-class:
      CAS-гонка дублей AG-5/8/9 w527 self-corr; retry-409 после фактического успеха PUT
      больше не создаёт вторую копию; полный no-op = легальный PASS);
  (7) пост-вериф exact-once: каждая добавленная строка ровно 1 раз (v.count(l)==1);
  (8) --blobcheck: live репетиция fallback-пути (contents sha -> git/blobs decode ->
      byte-eq vs contents size), 0 PUT.

Usage:
  board_put_guard.py "TYPE | AG-x | text | ev" ["line2" ...]   # append, CAS, post-verify
  board_put_guard.py --self-test                               # offline drills, 0 PUT
  board_put_guard.py --blobcheck                               # live fallback check, 0 PUT
API-ONLY canon v23.1: только contents/blob-API CAS, локальные git-коммиты доски запрещены.
"""
import base64, json, sys, time, urllib.request, urllib.error

REPO = 'PLANETA9091/c-crussty'
PATH = 'SHARED_BOARD.md'
MIN_BYTES = 50_000
MIN_LINES = 500
BLOB_FALLBACK_THRESHOLD = 900_000  # contents-API content-field ненадёжен у 1MB стены — запас
TYPES = {'FAIL', 'CLAIM', 'FACT', 'OBSERVED', 'PATCH_SUMMARY', 'DISP', 'DISP-INTENT', 'MERGE-READY', 'CENS'}
token = open('/tmp/gh_token').read().strip().split('\n')[0]
API = f'https://api.github.com/repos/{REPO}'


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


def needs_blob_fallback(d):
    """GitHub docs: файлы 1-100MB => contents-GET content:"" encoding:none. Тогда blob-API."""
    content = d.get('content') or ''
    return (len(content) == 0
            or d.get('encoding') == 'none'
            or int(d.get('size') or 0) > BLOB_FALLBACK_THRESHOLD)


def get_board():
    d = api(f'{API}/contents/{PATH}?ref=master')
    if not isinstance(d, dict) or 'sha' not in d:
        raise RuntimeError(f'GET-ERR {d}')
    if needs_blob_fallback(d):
        b = api(f'{API}/git/blobs/{d["sha"]}')
        if not isinstance(b, dict) or b.get('encoding') != 'base64':
            raise RuntimeError(f'BLOB-GET-ERR {b}')
        raw = base64.b64decode(b['content'])
    else:
        raw = base64.b64decode(d['content'])
    return raw.decode('utf-8', errors='replace'), d['sha']


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
            raise ValueError(f'carved-append guard: embedded newline in: {l[:60]!r}')
        if len(l) > 120:
            raise ValueError(f'line >120 chars ({len(l)}): {l[:60]!r}')
        t = l.split('|', 1)[0].strip()
        if t not in TYPES:
            raise ValueError(f'unknown TYPE {t!r} (whitelist {sorted(TYPES)})')


def dedup_lines(old, lines):
    """(keep, dropped): уже-присутствующие verbatim — дроп (idempotent, анти-dup-гонка)."""
    keep, dropped = [], []
    for l in lines:
        (dropped if l in old else keep).append(l)
    return keep, dropped


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
    blob_check = '--blobcheck' in argv
    lines = [a for a in argv if a not in ('--self-test', '--blobcheck') and a.strip()]
    if blob_check:
        d = api(f'{API}/contents/{PATH}?ref=master')
        if not isinstance(d, dict) or 'sha' not in d:
            print(f'blobcheck: contents-GET ERR {d}'); return 1
        b = api(f'{API}/git/blobs/{d["sha"]}')
        if not isinstance(b, dict) or b.get('encoding') != 'base64':
            print(f'blobcheck: blob-GET ERR {b}'); return 1
        n = len(base64.b64decode(b['content']))
        ok = n == int(d.get('size') or -1)
        print(f'blobcheck: {"PASS" if ok else "FAIL"} sha {d["sha"][:10]} contents-size {d.get("size")} blob-decoded {n}B '
              f'({"contents-path" if not needs_blob_fallback(d) else "BLOB-FALLBACK-ACTIVE"})')
        return 0 if ok else 1
    if not lines:
        print('no lines given'); return 1
    keep, dropped, new, commit = [], [], None, None
    for attempt in range(6):
        old, sha = get_board()
        ok, why = sanity(old)
        print(f'[1] sanity: {why}')
        if not ok:
            restore_recipe(); return 2
        try:
            build_new(old, lines)
        except ValueError as e:
            print(f'[2] {e}'); return 3
        if self_test:
            # failure-mode drills: clobbered old, long line, embedded newline, >1MB wall, dup
            try:
                ok, _ = sanity('stub\n')
                if ok: raise AssertionError('floor passed on stub')
            except Exception:
                print('[3] drill clobbered-old: NOT CAUGHT'); return 4
            try: build_new(old, ['FACT | AG | x' * 30]); print('[3] drill long-line: NOT CAUGHT'); return 4
            except ValueError: pass
            try: build_new(old, ['FACT\n| AG | split']); print('[3] drill newline: NOT CAUGHT'); return 4
            except ValueError: pass
            if needs_blob_fallback({'content': 'aGk=', 'encoding': 'base64', 'size': 1000}):
                print('[3] drill fallback-falsepositive: NOT CAUGHT'); return 4
            if not (needs_blob_fallback({'content': '', 'encoding': 'none', 'size': 1_200_000})
                    and needs_blob_fallback({'content': 'aGk=', 'encoding': 'base64', 'size': 1_200_000})):
                print('[3] drill fallback-wall: NOT CAUGHT'); return 4
            k, dr = dedup_lines('FACT | A | x\nBB\n', ['FACT | A | x', 'CC'])
            if k != ['CC'] or dr != ['FACT | A | x']:
                print('[3] drill dedup: NOT CAUGHT'); return 4
            print('[3] failure-drills 5/5 CAUGHT; [4] PUT SKIPPED (self-test) -> PASS')
            return 0
        validate_lines(lines)
        keep, dropped = dedup_lines(old, lines)
        if dropped:
            print(f'[2] dedup: {len(dropped)} line(s) already in board — skipped (idempotent):')
            for l in dropped: print(f'     = {l[:100]}')
        if not keep:
            print('[2] nothing to append (all lines already present) -> idempotent no-op PASS')
            return 0
        new = build_new(old, keep)
        print(f'[2] superset-guard OK: {len(old.encode())}B -> {len(new.encode())}B (+{len(new.encode())-len(old.encode())}), lines+{len(keep)}')
        put = api(f'{API}/contents/{PATH}', 'PUT',
                  {'message': f'board guarded append v2 ({len(keep)} lines)', 'sha': sha,
                   'content': base64.b64encode(new.encode()).decode(), 'branch': 'master'})
        if put and 'commit' in put:
            commit = put['commit']['sha'][:8]
            break
        print(f'[3] PUT retry {attempt+1} (CAS race / 409) — re-GET sha'); time.sleep(2 + attempt)
    else:
        print('[3] PUT failed 6 attempts (stampede) — rerun'); return 5
    v, _ = get_board()
    ok = (all(l in v for l in keep)
          and all(v.count(l) == 1 for l in keep)
          and len(v.encode()) >= len(new.encode()) - 400)
    print(f'[4] post-verify (exact-once x{len(keep)}): {"PASS" if ok else "FAIL"} commit {commit}')
    return 0 if ok else 6


if __name__ == '__main__':
    sys.exit(main())
