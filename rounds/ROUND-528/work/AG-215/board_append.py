import json, base64, sys, time, urllib.request

TOK = open('/tmp/gh_token').read().strip()
REPO = 'PLANETA9091/c-crussty'
PATH = 'SHARED_BOARD.md'
LINE = sys.argv[1]
assert len(LINE) <= 120, f"line too long: {len(LINE)}"

def api(url, method='GET', data=None):
    req = urllib.request.Request(f'https://api.github.com/repos/{REPO}/{url}',
        headers={'Authorization': f'Bearer {TOK}', 'Accept': 'application/vnd.github+json'},
        method=method, data=data)
    try:
        with urllib.request.urlopen(req) as r:
            return r.status, json.load(r)
    except urllib.error.HTTPError as e:
        return e.code, json.loads(e.read().decode())

for attempt in range(4):
    st, d = api(f'contents/{PATH}?ref=master')
    if st != 200: print('GET fail', st, d); sys.exit(1)
    sha = d['sha']
    txt = base64.b64decode(d['content']).decode('utf-8')
    if LINE in txt:
        print('DUP: line already present, abort'); sys.exit(2)
    if len(txt.encode()) + len(LINE) + 1 > 900_000:
        print('BOARD WALL: too big'); sys.exit(3)
    new = txt.rstrip('\n') + '\n' + LINE + '\n'
    body = json.dumps({'message': 'board: AG-215 w528 append (CAS)', 'content': base64.b64encode(new.encode()).decode(), 'sha': sha}).encode()
    st2, d2 = api(f'contents/{PATH}', 'PUT', body)
    print('PUT attempt', attempt, 'status', st2)
    if st2 in (200, 201):
        print('OK commit', d2['commit']['sha'][:10]); sys.exit(0)
    if st2 == 409:
        time.sleep(3); continue
    print('PUT body err', d2); time.sleep(3)
sys.exit(4)
