#!/usr/bin/env python3
"""board_append.py — AG-150 w526: contents-API CAS append в SHARED_BOARD.md (канон v23.1).
Usage: python3 board_append.py "TYPE | AG-113 | текст | число" ["строка2" ...]
"""
import base64, json, sys, time, urllib.request

TOK = open('/tmp/gh_token').read().strip()
REPO = 'PLANETA9091/c-crussty'
PATH = 'SHARED_BOARD.md'
HDR = {'Authorization': f'token {TOK}', 'Accept': 'application/vnd.github+json',
       'User-Agent': 'ag150-w526'}

def api(url, method='GET', body=None):
    req = urllib.request.Request(f'https://api.github.com/{url}', headers=HDR, method=method)
    data = json.dumps(body).encode() if body is not None else None
    with urllib.request.urlopen(req, data) as r:
        return json.loads(r.read())

def append(lines):
    for attempt in range(6):
        try:
            cur = api(f'repos/{REPO}/contents/{PATH}')
            sha = cur['sha']
            text = base64.b64decode(cur['content']).decode('utf-8')
            new = text.rstrip('\n') + '\n' + '\n'.join(lines) + '\n'
            body = {'message': f'board: AG-150 w526 append x{len(lines)} (CAS r{attempt})',
                    'content': base64.b64encode(new.encode()).decode(), 'sha': sha}
            api(f'repos/{REPO}/contents/{PATH}', 'PUT', body)
            return True
        except urllib.error.HTTPError as e:
            if e.code == 409 or e.code == 422:
                time.sleep(2 + attempt * 2)
                continue
            raise
    return False

if __name__ == '__main__':
    lines = [a for a in sys.argv[1:]]
    for a in lines:
        if len(a) > 120:
            print(f'LEN-REJECT ({len(a)}): {a}')
            sys.exit(2)
    ok = append(lines)
    print('OK' if ok else 'RETRY-EXHAUSTED', '| lines:', len(lines))
