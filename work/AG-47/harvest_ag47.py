#!/usr/bin/env python3
"""harvest_ag47.py — AG-47 w526: харвест терминальных SUCCESS-ног batch-1 x525.
Адаптация канона AG-173 (Azure 302 auth-strip, G4 re-grade, TPS-экстракт)."""
import base64, json, os, re, shutil, sys, urllib.request, zipfile

class _NoAuthRedirect(urllib.request.HTTPRedirectHandler):
    def redirect_request(self, req, fp, code, msg, headers, newurl):
        req.headers.pop('Authorization', None); req.remove_header('Authorization')
        return super().redirect_request(req, fp, code, msg, headers, newurl)
_OPENER = urllib.request.build_opener(_NoAuthRedirect)
TOK = open('/tmp/gh_token').read().strip()
REPO = 'PLANETA9091/c-crussty'
HDR = {'Authorization': f'token {TOK}', 'Accept': 'application/vnd.github+json', 'User-Agent': 'ag47-w526'}
HERE = os.path.dirname(os.path.abspath(__file__))
CACHE = os.path.join(HERE, 'art_cache')

TARGETS = {
 36970500736: ('ag17-g4fix-s525017', '84e6eeec'),
 36970792064: ('ag4-pair-s525004',   '877ed890'),
 36970736735: ('ag10-union-verify',  'e965bd27'),
 36970688918: ('ag38-a',             '92d09ff0'),
 36970749155: ('ag38-b',             '92d09ff0'),
 36970741819: ('ag20-a',             '401827e8'),
}

def api(url, raw=False):
    req = urllib.request.Request(f'https://api.github.com/{url}', headers=HDR)
    with _OPENER.open(req) as r:
        return r.read() if raw else json.loads(r.read())

def get_artifacts(run_id):
    root = os.path.join(CACHE, f'art-{run_id}')
    if os.path.isdir(root):
        return root
    arts = api(f'repos/{REPO}/actions/runs/{run_id}/artifacts').get('artifacts', [])
    if not arts:
        return None
    os.makedirs(root, exist_ok=True)
    for a in arts:
        req = urllib.request.Request(
            f'https://api.github.com/repos/{REPO}/actions/artifacts/{a["id"]}/zip', headers=HDR)
        with _OPENER.open(req) as r, open(os.path.join(root, a['name'] + '.zip'), 'wb') as f:
            f.write(r.read())
        try:
            with zipfile.ZipFile(os.path.join(root, a['name'] + '.zip')) as z:
                z.extractall(os.path.join(root, a['name']))
        except Exception as e:
            print(f'  zip-fail {a["name"]}: {str(e)[:60]}', flush=True)
    return root

def find_file(root, names):
    for dirpath, _d, files in os.walk(root):
        for n in names:
            if n in files:
                return os.path.join(dirpath, n)
    return None

def extract(run_id):
    tag, sha = TARGETS[run_id]
    root = get_artifacts(run_id)
    res = {'run': run_id, 'tag': tag, 'sha': sha}
    if not root:
        res['err'] = 'no-artifacts'; return res
    names = os.listdir(root)
    res['arts'] = names[:6]
    envf = find_file(root, ['run-env.txt', 'run-env'])
    benchmd = find_file(root, ['BENCHV2.md', 'WBP.md', 'BENCH.md'])
    logf = find_file(root, ['server-stdout.log', 'server.log', 'stdout.log'])
    if benchmd:
        txt = open(benchmd, encoding='utf-8', errors='replace').read()
        res['bench'] = {}
        for pat, key in [
            (r'expect ≥(\d+) = 0\.95×(\d+)×(\d+); radius-blocks side=(\d+)', 'expect_line'),
            (r'ch/s[^\d]*([\d.]+)', 'ch_s'),
            (r'TPS[^\d]*([\d.]+)', 'tps'),
            (r'[Mm]arked\D*(\d+)\D*(\d+)', 'marked'),
        ]:
            m = re.search(pat, txt)
            if m:
                res['bench'][key] = m.groups() if m.groups() else m.group(1)
        # медиана TPS: все числа tps-строк
        tps_nums = [float(x) for x in re.findall(r'(?:TPS|tps)[^\d]*?(\d+\.\d+)', txt)]
        if tps_nums:
            tps_nums.sort()
            res['tps_med'] = tps_nums[len(tps_nums)//2]
            res['tps_n'] = len(tps_nums)
        for key in ['MSPT', 'mspt']:
            m = re.search(key + r'[^\d]*?(\d+\.\d+)', txt)
            if m: res['mspt'] = m.group(1); break
    if logf:
        lines = open(logf, encoding='utf-8', errors='replace').read().splitlines()
        marked = sum(int(m.group(1)) for l in lines if (m := re.search(r'Marked (\d+) chunks', l)))
        res['marked_log'] = marked
        nc = [l for l in lines if 'NCDFE' in l][:2]
        res['ncdfe'] = ';'.join(nc)[:100]
        worlds = {m.group(1) for l in lines if (m := re.search(r"spawn point for world '([^']+)'", l))}
        res['worlds'] = sorted(worlds)[:4]
    if envf:
        txt = open(envf, encoding='utf-8', errors='replace').read()
        for k in ['radius_blocks', 'run_seconds', 'seed', 'server_xmx', 'bench_dims', 'dim_gen_window', 'drain_cap_polls']:
            m = re.search(rf'^{k}=(.*)$', txt, re.M)
            if m: res[k] = m.group(1).strip()[:40]
    return res

if __name__ == '__main__':
    out = []
    for rid in TARGETS:
        r = extract(rid)
        out.append(r)
        print(json.dumps(r, ensure_ascii=False)[:500], flush=True)
    json.dump(out, open(os.path.join(HERE, 'harvest_batch1_success.json'), 'w'), indent=1, ensure_ascii=False)
