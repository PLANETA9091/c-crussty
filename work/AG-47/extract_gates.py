import json, os, re
HERE = '/home/z/rounds/ROUND-526/work/AG-47'
out = []
for rid, tag in [(36970500736,'ag17-g4fix-s525017'), (36970792064,'ag4-pair-s525004'),
                 (36970736735,'ag10-union-verify'), (36970688918,'ag38-a'),
                 (36970749155,'ag38-b'), (36970741819,'ag20-a')]:
    root = os.path.join(HERE, f'art_cache/art-{rid}')
    md = None
    for dp, _d, fs in os.walk(root):
        if 'BENCHV2.md' in fs: md = os.path.join(dp, 'BENCHV2.md'); break
    txt = open(md, encoding='utf-8', errors='replace').read()
    r = {'run': rid, 'tag': tag}
    for pat, k in [
        (r'ch/s \(drain-def[^*]*\*\*([\d.]+)\*\*', 'chs'),
        (r'forceload-marked chunks total: \*\*(\d+)\*\* \(expect ≥(\d+) = 0\.95×(\d+)×(\d+)', 'marked_expect'),
        (r'MSPT: idle≈([\d.]+), sustain-median≈([\d.]+) \(spark mspt samples n=(\d+)\)', 'mspt'),
        (r'TPS samples \(spark tps\): n=(\d+), min=([\d.]+), last=([\d.]+)', 'tps'),
        (r'NCDFE=(\d+) \(canon T1=0 gate: (\w+)\)', 'ncdfe'),
        (r'AIOOBE=(\d+)', 'aioobe'),
        (r'G3 datapacks-enabled markers: (\d+)/\d+ \((\w+)\)', 'g3'),
        (r'G4 marked≥95%: (\w+); G5 drain: (\w+)', 'g45'),
        (r'G-DIM: ov=(\d+) ne=(\d+) en=(\d+) total=(\d+) \(gate per-dim>=\d+ total>=\d+\)', 'gdim'),
        (r'G6-FPV2 verdict: \*\*(\S[^*]*)\*\*', 'g6'),
    ]:
        m = re.search(pat, txt)
        if m: r[k] = m.groups()
    out.append(r)
    print(json.dumps(r, ensure_ascii=False), flush=True)
json.dump(out, open(os.path.join(HERE, 'harvest_batch1_gates.json'), 'w'), indent=1, ensure_ascii=False)
