#!/usr/bin/env python3
"""AG-425 w526 queue+parser census: queued/in_progress runs by wf/event/ref;
parser md5 per unique head_sha; ci-flood share; latest natural SUCCESS."""
import json, time, hashlib, base64, urllib.request, collections, sys

REPO = 'PLANETA9091/c-crussty'
TOKEN = open('/tmp/gh_token').read().strip().split('\n')[0]
HDR = {'Authorization': f'token {TOKEN}', 'Accept': 'application/vnd.github+json'}
OUT = '/home/z/rounds/ROUND-526/work/AG-425/queue_census_ag425.json'

def api(url):
    req = urllib.request.Request(url, headers=HDR)
    for a in range(5):
        try:
            with urllib.request.urlopen(req) as r:
                return json.load(r)
        except urllib.error.HTTPError as e:
            if e.code in (409, 500, 502, 503, 429):
                time.sleep(2 + a * 2); continue
            return {'http_error': e.code}
        except Exception:
            time.sleep(2 + a * 2)
    return {'http_error': 'retry-exhausted'}

def collect(status, maxpages=6):
    runs, page = [], 1
    while page <= maxpages:
        d = api(f'https://api.github.com/repos/{REPO}/actions/runs?status={status}&per_page=100&page={page}')
        if 'workflow_runs' not in d: break
        runs += d['workflow_runs']
        if len(d['workflow_runs']) < 100: break
        page += 1
    return runs

q = collect('queued')
ip = collect('in_progress')
print(f'queued={len(q)} in_progress={len(ip)}', flush=True)

def brief(r):
    return {'id': r['id'], 'name': r['name'], 'event': r['event'], 'status': r['status'],
            'head_sha': r['head_sha'][:8], 'head_branch': r['head_branch'],
            'created_at': r['created_at'], 'run_attempt': r.get('run_attempt')}

allr = q + ip
by_wf = collections.Counter(r['name'] for r in allr)
by_event = collections.Counter(r['event'] for r in allr)
by_sha = collections.Counter(r['head_sha'][:8] for r in allr)
q_event = collections.Counter(r['event'] for r in q)
q_sha = collections.Counter(r['head_sha'][:8] for r in q)
print('all by_wf:', dict(by_wf))
print('all by_event:', dict(by_event), '| queued by_event:', dict(q_event))
print('all by_sha:', dict(by_sha.most_common(20)))
print('queued by_sha:', dict(q_sha.most_common(20)))

# parser md5 per unique head_sha (top by exposure, cap 14)
parser = {}
for sha8, cnt in by_sha.most_common(14):
    full = None
    for r in allr:
        if r['head_sha'][:8] == sha8: full = r['head_sha']; break
    d = api(f'https://api.github.com/repos/{REPO}/contents/bench/worldv2/report_benchv2.py?ref={full}')
    if 'content' in d:
        raw = base64.b64decode(d['content'])
        parser[sha8] = {'md5': hashlib.md5(raw).hexdigest(), 'size': len(raw), 'exposure': cnt}
        parts = raw.decode('utf-8', errors='replace').split('\n')
        parser[sha8]['line32'] = parts[31].strip()[:80] if len(parts) >= 32 else ''
    else:
        parser[sha8] = {'err': d.get('http_error'), 'exposure': cnt}
    print('parser', sha8, parser[sha8], flush=True)

# ci.yml paths-ignore verification on master
ci = api(f'https://api.github.com/repos/{REPO}/contents/.github/workflows/ci.yml?ref=master')
ci_verdict = None
if 'content' in ci:
    txt = base64.b64decode(ci['content']).decode('utf-8', errors='replace')
    ci_verdict = 'paths-ignore:present' if 'paths-ignore' in txt else 'paths-ignore:ABSENT'
    ci_lines = [l.strip() for l in txt.split('\n') if 'paths-ignore' in l or 'SHARED_BOARD' in l][:6]
else:
    ci_lines = ci.get('http_error')
print('ci.yml:', ci_verdict, ci_lines, flush=True)

# latest natural SUCCESS (any workflow), first page top-40
succ = []
d = api(f'https://api.github.com/repos/{REPO}/actions/runs?status=success&per_page=100&page=1')
if 'workflow_runs' in d:
    for r in d['workflow_runs'][:40]:
        succ.append({'id': r['id'], 'name': r['name'], 'event': r['event'],
                     'created_at': r['created_at'], 'head_sha': r['head_sha'][:8]})
print('latest successes (top6):', json.dumps(succ[:6]), flush=True)

res = {'ts_utc': time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime()),
       'queued_n': len(q), 'in_progress_n': len(ip),
       'by_wf': dict(by_wf), 'by_event': dict(by_event), 'q_event': dict(q_event),
       'by_sha': dict(by_sha), 'q_sha': dict(q_sha), 'parser': parser,
       'ci_verdict': ci_verdict, 'ci_lines': ci_lines, 'latest_success': succ[:12],
       'queued_detail': [brief(r) for r in q][:250], 'inprog_detail': [brief(r) for r in ip][:150]}
json.dump(res, open(OUT, 'w'), indent=1)
print('SAVED', OUT, flush=True)
