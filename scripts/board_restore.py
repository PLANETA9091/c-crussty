#!/usr/bin/env python3
"""AG-333 restore v2: scan 100 commits for last >=400KB good board, CAS-PUT good+stub-fresh, retry 409."""
import base64, json, sys, time, urllib.request, urllib.error

REPO = 'PLANETA9091/c-crussty'
PATH = 'SHARED_BOARD.md'
token = open('/tmp/gh_token').read().strip().split('\n')[0]

def api(url, method='GET', body=None):
    req = urllib.request.Request(url, method=method,
        headers={'Authorization': f'token {token}', 'Accept': 'application/vnd.github+json'},
        data=json.dumps(body).encode() if body else None)
    for attempt in range(4):
        try:
            with urllib.request.urlopen(req) as r:
                return json.load(r)
        except urllib.error.HTTPError as e:
            if e.code in (409, 500, 502, 503):
                time.sleep(1 + attempt); continue
            print(f'HTTP {e.code}: {e.read().decode()[:150]}'); return None
        except Exception:
            time.sleep(1 + attempt)
    return None

def get_raw(ref):
    d = api(f'https://api.github.com/repos/{REPO}/contents/{PATH}?ref={ref}')
    if not d or 'content' not in d:
        return None, None
    return base64.b64decode(d['content']), d['sha']

hist = api(f'https://api.github.com/repos/{REPO}/commits?path={PATH}&per_page=100')
hist = hist if isinstance(hist, list) else hist.get('commits', [])
good = None
for i, c in enumerate(hist):
    sha = c['sha']
    blob = api(f'https://api.github.com/repos/{REPO}/contents/{PATH}?ref={sha}')
    size = blob.get('size', 0)
    if size >= 400_000:
        good = (sha, size)
        print(f'GOOD: {sha[:8]} size={size} (index {i}, {c["commit"]["author"]["date"]})')
        break
if not good:
    print('NO GOOD COMMIT IN 100'); sys.exit(1)

gtext, gsha = get_raw(good[0])
gtext = gtext.decode('utf-8', errors='replace')

for attempt in range(6):
    cur, csha = get_raw('master')
    ctext = cur.decode('utf-8', errors='replace')
    if len(cur) >= 400_000:
        print('BOARD ALREADY RESTORED by peer, size', len(cur)); sys.exit(0)
    stub_lines = [l for l in ctext.splitlines() if l.strip()]
    fresh = [l for l in stub_lines if l not in gtext]
    merged = gtext.rstrip('\n') + '\n' + '\n'.join(fresh) + '\n' if fresh else gtext
    if not merged.endswith('\n'):
        merged += '\n'
    assert len(merged.encode()) >= 400_000
    put = api(f'https://api.github.com/repos/{REPO}/contents/{PATH}', 'PUT',
              {'message': f'restore board {good[0][:8]} + {len(fresh)} fresh stub lines (AG-333 w526, clobber-cascade fix)',
               'content': base64.b64encode(merged.encode()).decode(), 'sha': csha, 'branch': 'master'})
    if put and 'commit' in put:
        print('RESTORED', put['commit']['sha'][:8], 'fresh lines kept:', len(fresh))
        v, _ = get_raw('master')
        print('VERIFY size', len(v), 'lines', v.decode().count('\n'),
              'fresh-present', all(l.encode() in v for l in fresh))
        sys.exit(0)
    print('retry', attempt + 1, '(CAS race)')
    time.sleep(3)
print('FAILED 6 attempts'); sys.exit(2)
