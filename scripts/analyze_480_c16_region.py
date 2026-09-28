#!/usr/bin/env python3
"""480-C16: RegionTickOps domain decomposition (cpu) + wall-window composition.
Inputs: B4 cpu-collapsed (run 36378524664 @ed705c5d, 116,936 — proxy канарейки 479-F1:
forEach 21.97 vs 22.19 F1), G6 wall-collapsed (run 36376470764, 61,254 — та же wall-методика F1).
Метод = F1 (f1_lane_capture_479.py) bit-совместим."""
import collections, json, sys

def agg(path):
    inc = collections.Counter(); lf = collections.Counter(); tot = 0; stacks = []
    for line in open(path, encoding='utf-8', errors='replace'):
        line = line.rstrip('\n'); m = line.rfind(' ')
        if m < 0: continue
        cnt = int(line[m+1:]); tot += cnt
        frames = line[:m].split(';')
        s = set()
        for fr in frames:
            if fr not in s: inc[fr] += cnt; s.add(fr)
        lf[frames[-1]] += cnt
        stacks.append((cnt, frames))
    return tot, inc, lf, stacks

CT, cinc, cleaf, cstacks = agg('/home/z/rounds/ROUND-479/B4/art/cpu-collapsed.txt')
WT, winc, wleaf, wstacks = agg('/home/z/rounds/ROUND-479/G6/g6/wall-collapsed.txt')

def pct(x, t): return round(100*x/t, 2) if t else 0.0

RTO = 'net/minecraft/world/entity/RegionTickOps.'
out = {"cpu_total": CT, "wall_total": WT, "src": {"cpu": "B4 r36378524664", "wall": "G6 r36376470764"}}

# --- 1. Region domain (cpu): main-thread forEach side vs worker side ---
fe = cinc.get(RTO+'forEach', 0)
worker = sum(c for c, fr in cstacks if any('RegionTickOps.lambda$ensureHelpers$5' in f for f in fr))
rto_all = sum(c for c, fr in cstacks if any(f.startswith(RTO) for f in fr))
tickBucket_all = cinc.get(RTO+'tickBucket', 0)
out['region'] = {
  'forEach_incl_cpu': fe, 'forEach_pct': pct(fe, CT),
  'worker_ensureHelpers_cpu': worker, 'worker_pct': pct(worker, CT),
  'tickBucket_incl_cpu': tickBucket_all, 'tickBucket_pct': pct(tickBucket_all, CT),
  'rto_any_frame_cpu': rto_all, 'rto_any_pct': pct(rto_all, CT),
}

# --- 2. Worker-side decomposition: next frame under tickBucket on worker stacks ---
wk_callee = collections.Counter(); wk_self = 0
for c, fr in cstacks:
    if not any('RegionTickOps.lambda$ensureHelpers$5' in f for f in fr): continue
    for i, f in enumerate(fr):
        if f == RTO+'tickBucket':
            if i+1 < len(fr): wk_callee[fr[i+1]] += c
            else: wk_self += c
            break
out['worker_next_under_tickBucket'] = [(k.split('.')[-1][:60], v, pct(v, CT)) for k, v in wk_callee.most_common(8)]

# --- 3. Entity consumer decomposition (lambda$tick$4) — worker+main ---
L4 = 'net/minecraft/server/level/ServerLevel.lambda$tick$4'
l4 = cinc.get(L4, 0)
out['lambda_tick4'] = {'incl': l4, 'pct': pct(l4, CT)}
l4_next = collections.Counter()
for c, fr in cstacks:
    for i, f in enumerate(fr):
        if f == L4 and i+1 < len(fr):
            l4_next[fr[i+1]] += c; break
out['tick4_next'] = [(k.split('/')[-1][:70], v, pct(v, CT)) for k, v in l4_next.most_common(14)]

# --- 4. Barrier / midTick / steal / sched lanes (cpu incl) ---
def lane_any(*keys):
    return sum(c for c, fr in cstacks if any(any(k in f for f in fr) for k in keys))
out['lanes_cpu'] = {
  'barrier_incl': pct(lane_any('CyclicBarrier.await'), CT),
  'midTickTasks_incl': pct(cinc.get(RTO+'midTickTasks', 0), CT),
  'stealTick_incl': pct(cinc.get(RTO+'stealTick', 0), CT),
  'drainScheduledTicks_incl': pct(cinc.get(RTO+'drainScheduledTicks', 0), CT),
  'onTickingStart_incl': pct(cinc.get(RTO+'onTickingStart', 0), CT),
  'onTickingEnd_incl': pct(cinc.get(RTO+'onTickingEnd', 0), CT),
  'GuardedNavigatingMobs_incl': pct(cinc.get('net/minecraft/world/entity/RegionTickOps$GuardedNavigatingMobs.tick', 0), CT),
  'aiStep_agg': pct(sum(c for l, c in cinc.items() if '.aiStep' in l), CT),
  'gc_self_leaf': pct(sum(c for l, c in cleaf.items() if any(k in l for k in ("PSCardTable","OopOopIterate","PSPromotion","GCTask","ScavengeRoots"))), CT),
}

# --- 5. WALL window: forEach wall, barrier wall, tickBucket worker wall ---
fe_w = winc.get(RTO+'forEach', 0)
bar_w = sum(c for line in open('/home/z/rounds/ROUND-479/G6/g6/wall-collapsed.txt', encoding='utf-8', errors='replace')
            for c in [int(line.rstrip().rsplit(' ', 1)[1])] if 'CyclicBarrier' in line)
wk_wall = sum(c for c, fr in wstacks if any('RegionTickOps.lambda$ensureHelpers$5' in f for f in fr))
eh_w = winc.get(RTO+'parallelTick', 0)
tb_w = winc.get(RTO+'tickBucket', 0)
out['wall'] = {
  'forEach_wall': fe_w, 'forEach_wall_pct': pct(fe_w, WT),
  'parallelTick_wall_pct': pct(eh_w, WT),
  'tickBucket_wall_pct': pct(tb_w, WT),
  'worker_wall_pct': pct(wk_wall, WT),
  'barrier_stack_wall': bar_w, 'barrier_wall_pct': pct(bar_w, WT),
}

# --- 6. Region-domain TOTAL CPU (worker+main, dedup per stack) ---
out['region_domain_total_cpu'] = {'incl': rto_all, 'pct': pct(rto_all, CT)}

# --- 7. burst / capture math ---
fe_cpu_pct = pct(fe, CT); fe_wall_pct = pct(fe_w, WT)
out['capture'] = {
  'forEach_burst': round(fe_cpu_pct/max(fe_wall_pct, 0.01), 1),
  'forEach_ceil_pp_wall': fe_wall_pct,
  'barrier_ceil_pp_wall': pct(bar_w, WT),
  'combined_naive_pp': round(fe_wall_pct + pct(bar_w, WT), 2),
}

json.dump(out, open('/home/z/rounds/ROUND-480/c16/c16_decomp.json', 'w'), indent=1, ensure_ascii=False)
print(json.dumps(out, indent=1, ensure_ascii=False))
