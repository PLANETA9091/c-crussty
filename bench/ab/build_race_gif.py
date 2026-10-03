#!/usr/bin/env python3
"""build_race_gif.py — assemble the REAL worldgen race GIF.

Left panel: Paper (vanilla). Right panel: Paper + c-crussty.
Map imagery = the real squaremap zoom-3 tiles mirrored during the race
(tilesnap/<ARM>race10/<t>/). Counters = the real RCON receipts
(race10_receipts.json). Font = Monocraft. Nothing is synthesized: each frame
shows exactly what the squaremap live map contained at that real second.
"""
import json
import os
from PIL import Image, ImageDraw, ImageFont

import os
# Paths are env-overridable so the script runs from any checkout:
#   RACE_DIR  receipts (default: this results dir)         TILE_DIR  mirrored squaremap tiles
#   FONT      Monocraft.ttc (github.com/IdreesInc/Monocraft)  OUT_GIF  output path
HERE = os.path.dirname(os.path.abspath(__file__))
RACE_DIR = os.environ.get("RACE_DIR", HERE + "/results/race")
TILE_DIR = os.environ.get("TILE_DIR", "/home/z/race-assets/tilesnap")
FONT = os.environ.get("FONT", "/home/z/race-assets/Monocraft.ttc")
OUT_FRAMES = os.environ.get("OUT_FRAMES", "/tmp/race_gif_frames")
OUT_GIF = os.environ.get("OUT_GIF", HERE + "/../../docs/assets/worldgen_race.gif")
TILE = TILE_DIR
ASSETS = RACE_DIR
FONT_PATH = FONT
RECEIPTS = json.load(open(f"{RACE_DIR}/race10_receipts.json"))

TOTAL = 3721
RADIUS = 30
STEP = 10          # real seconds per frame
FPS_MS = 330       # frame duration
T_MAX = 340

VOID = (0, 18, 45)          # squaremap ungenerated background (sampled)
BG = (9, 16, 30)            # page background
PANEL_BG = (13, 24, 43)
BORDER = (34, 52, 84)
TEXT = (232, 238, 247)
MUTED = (143, 163, 189)
ACC_A = (224, 101, 74)      # paper — warm red
ACC_B = (88, 196, 112)      # crussty — green
GOLD = (240, 196, 84)

os.makedirs(OUT_FRAMES, exist_ok=True)

def F(size):
    return ImageFont.truetype(FONT_PATH, size, index=0)

f_title = F(26)
f_sub = F(13)
f_panel = F(20)
f_stat = F(15)
f_stat_small = F(12)
f_badge = F(15)

def fmt_mmss(s):
    s = int(round(s))
    return f"{s // 60}:{s % 60:02d}"

def fmt_num(n):
    return f"{n:,}".replace(",", "_")

def arm_state(arm, t):
    rows = RECEIPTS[arm]["rows"]
    cur = None
    for r in rows:
        if r["t"] <= t:
            cur = r
        else:
            break
    if cur is None:
        cur = {"t": 0, "proc": 0, "pct": 0.0, "cps": 0.0, "mspt": None, "tps": None}
    finish = RECEIPTS[arm]["finish_s"]
    done = t >= finish or cur["proc"] >= TOTAL
    if done:
        # at/after the line: show the real completion (last receipt rows lag up
        # to one sample behind the actual finish)
        cur = dict(cur)
        cur["proc"] = TOTAL
        cur["pct"] = 100.0
        cur["cps"] = round(TOTAL / finish, 2)
    return cur, finish, done

def tile_composite(arm, t):
    """1024x1024 composite of the 4 real zoom-3 tiles at snapshot <= t, cropped to the race square."""
    snap = None
    base = f"{TILE}/{arm}race10"
    dirs = sorted(int(d) for d in os.listdir(base) if d.isdigit()) if os.path.isdir(base) else []
    for d in dirs:
        if d <= t:
            snap = d
        else:
            break
    img = Image.new("RGB", (1024, 1024), VOID)
    if snap is not None:
        sd = f"{base}/{snap:04d}"
        for name, (ox, oy) in {
            "-1_-1": (0, 0), "-1_0": (0, 512), "0_-1": (512, 0), "0_0": (512, 512),
        }.items():
            p = f"{sd}/{name}.png"
            if os.path.exists(p):
                tile = Image.open(p).convert("RGB")
                if tile.size != (512, 512):
                    tile = tile.resize((512, 512))
                img.paste(tile, (ox, oy))
    # crop to the pregeneration square: blocks -480..480 -> px 32..992
    return img.crop((32, 32, 992, 992)).resize((540, 540), Image.LANCZOS)

def draw_hud(d, x0, y0, w, arm, label, accent, t, badge=None, badge_color=None):
    cur, finish, done = arm_state(arm, t)
    proc, cps = cur["proc"], cur["cps"]
    pct = min(100.0, proc * 100.0 / TOTAL)
    eta = (TOTAL - proc) / cps if cps and cps > 0 else 0

    d.rectangle([x0, y0 + 2, x0 + 10, y0 + 16], fill=accent)
    d.text((x0 + 18, y0), label, font=f_panel, fill=TEXT)
    d.rectangle([x0, y0 + 30, x0 + w, y0 + 31], fill=BORDER)
    # progress bar
    by = y0 + 40
    d.rectangle([x0, by, x0 + w, by + 12], outline=BORDER, fill=(6, 12, 24))
    fill_w = int(w * pct / 100)
    if fill_w > 0:
        d.rectangle([x0 + 1, by + 1, x0 + fill_w, by + 11], fill=accent)
    d.text((x0, by + 16), f"{pct:5.1f}%  OF {fmt_num(TOTAL)} CHUNKS", font=f_stat_small, fill=MUTED)

    rows = [
        ("CHUNKS", f"{fmt_num(proc)}/{fmt_num(TOTAL)}", "TIME", fmt_mmss(t)),
        ("RATE", f"{cps:.1f} CHUNKS/S", "TPS", f"{cur['tps']:.1f}" if cur["tps"] else "--"),
        ("MSPT", f"{cur['mspt']:.1f} MS" if cur["mspt"] else "--", "", ""),
    ]
    ry = y0 + 74
    for l1, v1, l2, v2 in rows:
        d.text((x0, ry), l1, font=f_stat, fill=MUTED)
        d.text((x0 + 92, ry), v1, font=f_stat, fill=TEXT)
        if l2:
            d.text((x0 + 285, ry), l2, font=f_stat, fill=MUTED)
            d.text((x0 + 350, ry), v2, font=f_stat, fill=TEXT)
        ry += 21
    if done:
        d.text((x0, ry + 2), f"FINISHED IN {fmt_mmss(finish)}", font=f_badge, fill=accent)
        if badge:
            d.text((x0 + 260, ry + 2), badge, font=f_badge, fill=badge_color or GOLD)
    else:
        d.text((x0, ry + 2), f"ETA {fmt_mmss(eta)}" if cps else "ETA --", font=f_badge, fill=MUTED)

W, H = 1200, 852
frames = []
durations = []
for t in range(0, T_MAX + 1, STEP):
    img = Image.new("RGB", (W, H), BG)
    d = ImageDraw.Draw(img)
    # header
    d.text((24, 16), "WORLD GENERATION RACE", font=f_title, fill=TEXT)
    d.text((568, 16), "PAPER", font=f_title, fill=ACC_A)
    d.text((676, 16), "VS", font=f_title, fill=MUTED)
    d.text((724, 16), "C-CRUSSTY", font=f_title, fill=ACC_B)
    d.text((24, 52), "REAL CAPTURE  |  PURPUR 1.21.10  |  SEED 3053459  |  CHUNKY SPIRAL SQUARE R=30 CHUNKS (3,721)  |  TIMELAPSE 24x",
           font=f_sub, fill=MUTED)
    d.line([24, 72, W - 24, 72], fill=BORDER)

    map_y = 88
    b_fin, a_fin = RECEIPTS["B"]["finish_s"], RECEIPTS["A"]["finish_s"]
    # left panel
    d.rectangle([22, map_y - 6, 566, map_y + 552], outline=BORDER, width=1)
    img.paste(tile_composite("A", t), (24, map_y))
    draw_hud(d, 24, map_y + 552, 542, "A", "PAPER 1.21.10  (VANILLA)", ACC_A, t,
             badge=("SECOND PLACE" if b_fin <= t < a_fin else None), badge_color=MUTED)
    # right panel
    d.rectangle([634, map_y - 6, 1178, map_y + 552], outline=ACC_B if b_fin <= t else BORDER, width=3 if b_fin <= t else 1)
    img.paste(tile_composite("B", t), (636, map_y))
    draw_hud(d, 636, map_y + 552, 542, "B", "PAPER + C-CRUSSTY", ACC_B, t,
             badge=("FIRST PLACE — 10 S AHEAD" if b_fin <= t else None), badge_color=GOLD)

    d.line([24, H - 34, W - 24, H - 34], fill=BORDER)
    d.text((24, H - 26), "IDENTICAL JVM FLAGS + BYTE-IDENTICAL WORLD RESTORE PER LEG  |  ARM B DIFFERENTIAL = CRUSSTY AGENT ONLY  |  RECEIPTS: BENCH/AB/RESULTS/RACE/",
           font=f_stat_small, fill=MUTED)

    p = f"{OUT_FRAMES}/frame_{t:04d}.png"
    img.save(p)
    frames.append(p)
    durations.append(800 if t == 0 else (2500 if t == T_MAX else FPS_MS))

pil_frames = [Image.open(p).convert("RGB", dither=False) for p in frames]
pal = pil_frames[0].quantize(colors=256, method=Image.MEDIANCUT, dither=Image.NONE)
pil_frames[0].save(
    OUT_GIF, save_all=True, append_images=pil_frames[1:], duration=durations, loop=0,
    palette=pal, optimize=True,
)
mb = os.path.getsize(OUT_GIF) / 1e6
print(f"GIF: {OUT_GIF}  frames={len(pil_frames)}  {W}x{H}  {mb:.1f} MB")
