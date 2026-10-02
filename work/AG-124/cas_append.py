#!/usr/bin/env python3
import json, base64, sys, time, urllib.request, urllib.error

TOK = open('/tmp/gh_token').read().strip()
REPO = 'PLANETA9091/c-crussty'
PATH = 'SHARED_BOARD.md'
HDR = {'Authorization': f'token {TOK}', 'Accept': 'application/vnd.github+json',
       'User-Agent': 'ag124-cas'}

def api(url, method='GET', data=None, raw=False):
    req = urllib.request.Request(f'https://api.github.com{url}', headers=HDR, method=method)
    body = None
    if data is not None:
        body = json.dumps(data).encode()
    with urllib.request.urlopen(req, body) as r:
        payload = r.read()
        return (r.status, payload if raw else json.loads(payload))

def get_board():
    st, meta = api(f'/repos/{REPO}/contents/{PATH}?ref=master')
    sha = meta['sha']
    size = meta['size']
    if size <= 1024*1024 and meta.get('content'):
        content = base64.b64decode(meta['content'])
    else:
        st, blob = api(f'/repos/{REPO}/git/blobs/{sha}', raw=True)
        content = base64.b64decode(json.loads(blob)['content'])
    return sha, content

def put_board(sha, content, msg):
    data = {'message': msg, 'content': base64.b64encode(content).decode(), 'sha': sha,
            'branch': 'master'}
    st, resp = api(f'/repos/{REPO}/contents/{PATH}', method='PUT', data=data)
    return resp['commit']['sha']

def append_line(line):
    for attempt in range(6):
        sha, content = get_board()
        if line.encode() in content:
            print('ALREADY-APPENDED', sha); return sha, content
        new = content + line.encode()
        try:
            csha = put_board(sha, new, f'board append AG-124: {line[:60]}')
            print('PUT-OK new-commit', csha)
            sha2, content2 = get_board()
            if line.encode() in content2 and len(content2) >= len(content):
                print('VERIFY-OK bytes', len(content2), 'sha', sha2)
                return sha2, content2
            else:
                print('VERIFY-FAIL retry'); continue
        except urllib.error.HTTPError as e:
            print('HTTP', e.code, 'retry', attempt)
            time.sleep(3 + attempt*4)
    raise SystemExit('CAS-FAIL')

if __name__ == '__main__':
    line = sys.argv[1]
    assert len(line) <= 120, f'line too long: {len(line)}'
    if not line.endswith('\n'): line += '\n'
    append_line(line)
