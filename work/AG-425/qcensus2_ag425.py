#!/usr/bin/env python3
"""AG-425 tail: parser md5 for ALL unique shas; completed-since-0644Z verdict split; branch exposure."""
import json, time, hashlib, base64, urllib.request, collections

REPO = 'PLANETA9091/c-crussty'
TOKEN = open('/tmp/gh_token').read().strip().split('\n')[0]
HDR = {'Authorization': f'token {TOKEN}', 'Accept': 'application/vnd.github+json'}
BASE = '/home/z/rounds/ROUND-526/work/AG-425'
res = json.load(open(f'{BASE}/queue_census_ag425.json'))

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

# sha8 -> full sha map from details
full_map = {}
for k in ('queued_detail', 'inprog_detail'):
    for r in res[k]:
        full_map.setdefault(r['head_sha'], None)
# need full shas: refetch minimal list of runs pages 1-6 (cheap via head_sha full)
fulls = {}
for st in ('queued', 'in_progress'):
    page = 1
    while page <= 7:
        d = api(f'https://api.github.com/repos/{REPO}/actions/runs?status={st}&per_page=100&page={page}')
        if 'workflow_runs' not in d: break
        for r in d['workflow_runs']:
            fulls[r['head_sha'][:8]] = r['head_sha']
        if len(d['workflow_runs']) < 100: break
        page += 1

by_sha = res['by_sha']
parser = res['parser']
BUG = '762ceee8f0633d9251a52058ac57676d'
todo = [s for s in by_sha if s not in parser]
print('shas to fetch:', len(todo), flush=True)
for s in todo:
    f = fulls.get(s)
    d = api(f'https://api.github.com/repos/{REPO}/contents/bench/worldv2/report_benchv2.py?ref={f}')
    if 'content' in d:
        raw = base64.b64decode(d['content'])
        parser[s] = {'md5': hashlib.md5(raw).hexdigest(), 'size': len(raw), 'exposure': by_sha[s]}
    else:
        parser[s] = {'err': d.get('http_error'), 'exposure': by_sha[s]}
    print('parser', s, parser[s], flush=True)

bugged = {s: v for s, v in parser.items() if v.get('md5') == BUG}
bugged_n = sum(v.get('exposure', 0) for v in bugged.values())
fixed = {s: v for s, v in parser.items() if v.get('md5') and v['md5'] != BUG}
fixed_n = sum(v.get('exposure', 0) for v in fixed.values())
unk_n = sum(v.get('exposure', 0) for v in parser.values() if 'err' in v)
print(f'BUGGED={bugged_n} FIXED={fixed_n} UNKNOWN={unk_n} total_fetched={bugged_n+fixed_n+unk_n}/{res["queued_n"]+res["in_progress_n"]}', flush=True)
print('fixed shas:', {s: (v.get('md5', '')[:8], v.get('exposure')) for s, v in fixed.items()}, flush=True)

# completed since 06:44Z verdict split
comp = collections.Counter(); oldest = None; page = 1; seen = 0
while page <= 10:
    d = api(f'https://api.github.com/repos/{REPO}/actions/runs?status=completed&per_page=100&page={page}')
    if 'workflow_runs' not in d: break
    stop = False
    for r in d['workflow_runs']:
        if r['created_at'] <= '2026-10-02T06:44:30Z':
            stop = True; break
        comp[(r['name'], r['event'], r['conclusion'])] += 1
        seen += 1
    oldest = d['workflow_runs'][-1]['created_at']
    if stop or len(d['workflow_runs']) < 100: break
    page += 1
print(f'completed since 0644Z: n={seen} by (wf,event,conclusion):', dict(comp), flush=True)

# exposure by head_branch top-16
br = collections.Counter()
for r in res['queued_detail'] + res['inprog_detail']:
    br[(r['head_branch'], r['head_sha'])] += 1
print('top branches:', br.most_common(16), flush=True)

res['parser'] = parser
res['bugged_n'] = bugged_n; res['fixed_n'] = fixed_n; res['unknown_n'] = unk_n
res['bugged_shas'] = {s: v.get('exposure') for s, v in bugged.items()}
res['fixed_shas'] = {s: (v.get('md5', '')[:8], v.get('exposure')) for s, v in fixed.items()}
res['completed_since_0644'] = {'n': seen, 'split': {f'{a}|{b}|{c}': v for (a, b, c), v in comp.items()}}
res['branch_exposure'] = {f'{b}|{s}': n for (b, s), n in br.most_common(40)}
res['census2_ts'] = time.strftime('%Y-%m-%dT%H:%M:%SZ', time.gmtime())
json.dump(res, open(f'{BASE}/queue_census_ag425.json', 'w'), indent=1)
print('SAVED updated', flush=True)
