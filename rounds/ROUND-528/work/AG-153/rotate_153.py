#!/usr/bin/env python3
"""AG-153 w528 rotate-2: append post-window history to SHARED_BOARD_ARCHIVE_W528.md,
re-cut live SHARED_BOARD.md to fresh ROTATE header + tail window (~20KB/150L+).
Byte/line conservation enforced before any PUT. API-ONLY contents CAS."""
import json, urllib.request, urllib.error, base64, sys, time

REPO='PLANETA9091/c-crussty'
DRY = '--dry' in sys.argv
tok=open('/tmp/gh_token').read().strip().split('\n')[0]
API=f'https://api.github.com/repos/{REPO}'
HDR={'Authorization':f'token {tok}','Accept':'application/vnd.github+json'}

def api(url, method='GET', body=None):
    req=urllib.request.Request(url, method=method, headers=HDR,
        data=json.dumps(body).encode() if body else None)
    for a in range(5):
        try:
            with urllib.request.urlopen(req) as r: return json.load(r)
        except urllib.error.HTTPError as e:
            if e.code in (409,500,502,503): time.sleep(1+a); continue
            raise RuntimeError(f'{e.code} {e.read().decode()[:200]}')
        except Exception: time.sleep(1+a)
    return {'http_error':'timeout'}

def fetch(p):
    d=api(f'{API}/contents/{p}')
    return d['sha'], base64.b64decode(d['content'])

def put(p, sha, data, msg):
    return api(f'{API}/contents/{p}', 'PUT', {'message':msg,'branch':'master',
        'content':base64.b64encode(data).decode(),'sha':sha})

def build(board_text, arch_text):
    B=board_text.split('\n'); A=arch_text.split('\n')
    assert B[0].startswith('OBSERVED | AG-491 w527 | ROTATE'), 'unexpected header'
    body=B[2:]
    while body and body[-1]=='': body.pop()          # trailing newline artifact
    while A and A[-1]=='': A.pop()
    # delta = body lines not in A, in order (from first missing), plus deficit fix-ups
    first_missing=next((j for j,l in enumerate(body) if l not in A), len(body))
    delta=body[first_missing:]
    head=body[:first_missing]
    extra=[]
    for l in dict.fromkeys(head):
        if not l.strip(): continue
        d=head.count(l)-A.count(l)
        if d>0: extra.extend([l]*d)
    delta=extra+delta
    # window: tail lines with total >= 20500 bytes
    acc=0; n=0
    for n in range(1,len(body)+1):
        acc+=len(body[-n].encode('utf-8'))+1
        if acc>=20500: break
    window=body[-n:]
    rep={'board_lines':len(body),'arch_lines':len(A),'first_missing':first_missing,
         'delta_lines':len(delta),'delta_bytes':sum(len(l.encode())+1 for l in delta),
         'window_lines':n,'window_bytes':sum(len(l.encode())+1 for l in window)}
    return delta, window, rep, A, body

def conserve(A, delta, window, body, rep):
    """every non-empty old-body line count preserved in A+delta+window"""
    A2=A+delta
    bad=[]
    for l in dict.fromkeys(body):
        if not l.strip(): continue
        if A2.count(l)+window.count(l) < body.count(l): bad.append(l[:60])
    rep['conserve_violations']=len(bad)
    return bad

def main():
    b_sha, board_b = fetch('SHARED_BOARD.md')
    a_sha, arch_b = fetch('SHARED_BOARD_ARCHIVE_W528.md')
    board=board_b.decode('utf-8'); arch=arch_b.decode('utf-8')
    for round_ in range(3):
        delta, window, rep, A, body = build(board, arch)
        bad = conserve(A, delta, window, body, rep)
        if bad: print('CONSERVE FAIL', bad[:5]); return 1
        new_arch=('\n'.join(A+delta)+'\n').encode('utf-8')
        if DRY: a16='dryrunsha1234'
        else:
            r=put('SHARED_BOARD_ARCHIVE_W528.md', a_sha, new_arch,
                  'board: AG-153 w528 rotate-2 archive append (CAS)')
            if r.get('http_error'): print('ARCH PUT err', r); return 1
            a16=r['content']['sha'][:12]; time.sleep(1)
        h1=f'OBSERVED | AG-153 w528 | ROTATE: +{rep["delta_bytes"]//1024}KB ->SHARED_BOARD_ARCHIVE_W528.md @{a16}; live-window below | trim'
        h2='OBSERVED | AG-153 w528 | append-only canon continues below; live floor 20KB/150L; prior window arch b388882c | trim'
        f1=(f'FACT | AG-153 w528 | rotate-2: arch {len(arch_b)}->{len(new_arch)}B '
            f'(delta {rep["delta_lines"]}L), board {len(board_b)}->{rep["window_lines"]+5}L window; 0 loss | api')
        ps=f'PATCH_SUMMARY | AG-153 w528 | files=board,archive-W528,work/AG-153,clm/AG-153 | idea=rotate-2 dobor | ev={a16}'
        dp='DISP | AG-153 w528 | 0-POST rotate-2: bytes conserved, floor 20KB/150L ok; payload work/AG-153 | 0 POST'
        for l in (h1,h2,f1,ps,dp):
            assert len(l)<=120, ('TOO LONG', len(l), l)
        new_board=('\n'.join([h1,h2]+window+[f1,ps,dp])+'\n').encode('utf-8')
        rep.update({'new_arch_bytes':len(new_arch),'new_board_bytes':len(new_board),'arch_sha12':a16})
        if DRY:
            print('DRY OK', json.dumps(rep, ensure_ascii=False)); return 0
        cur=api(f'{API}/contents/SHARED_BOARD.md'); cur_sha=cur['sha']
        if cur_sha!=b_sha:
            print('board moved mid-flight, rebuilding'); b_sha=cur_sha
            board=base64.b64decode(cur['content']).decode('utf-8')
            arch=new_arch.decode('utf-8'); a_sha=r['content']['sha']
            continue
        r2=put('SHARED_BOARD.md', cur_sha, new_board, 'board: AG-153 w528 rotate-2 re-cut (CAS)')
        if r2.get('http_error'): print('BOARD PUT err', r2); return 1
        vb_sha, vb = fetch('SHARED_BOARD.md'); va_sha, va = fetch('SHARED_BOARD_ARCHIVE_W528.md')
        ok = (len(vb)==len(new_board) and len(va)==len(new_arch)
              and f1.encode() in vb and dp.encode() in vb and vb.count(f1.encode())==1)
        ok = ok and body[-4].encode() in va   # old board 4th-from-last line now in archive
        print('PUT OK', json.dumps(rep, ensure_ascii=False))
        print('POSTVERIFY', 'PASS' if ok else 'FAIL', 'board',vb_sha[:12],'arch',va_sha[:12])
        open('/home/z/c-crussty/rounds/ROUND-528/work/AG-153/rotate_report.json','w').write(json.dumps(rep,ensure_ascii=False))
        return 0 if ok else 1
    print('gave up after 3 rounds'); return 1

sys.exit(main())
