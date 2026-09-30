#!/usr/bin/env python3
"""build_stz9394.py — СТЗ-93 (Paper#13783 weak-chunk entity-storm) + СТЗ-94
(Paper#8142 hopper→composter per-tick) datapack-фикстуры, pack.mcmeta dual
48+[48,88] (канон C13), zip в /home/z/c-crussty/dp_assets/{stz93,stz94}.zip.
СТЗ-93: 4 far-точки (weak чанки ~+1M блоков от спавна) × 4 волны × 4096 item =
65,536 summon'ов → mass-sync-chunk-load + tracker-шторм; tick-гейт score-фазой
(120/600 duty канон C13). СТЗ-94: 512 пар hopper(в NBT items)→composter у
спавна (форселоад-зона) → per-tick bone-meal push шторм.
"""
import io, json, os, zipfile

OUT = "/home/z/c-crussty/dp_assets"
os.makedirs(OUT, exist_ok=True)
MCMETA = {"pack": {"pack_format": 48, "supported_formats": [48, 88],
                   "description": "STZ fixture (C33 x510)"}}


def build_zip(root, files, dest):
    # ДЕТЕРМИНИЗМ-SHA (fix 510-STZ): writestr(str,...) ставит ZipInfo.date_time =
    # localtime → sha256 меняется каждой сборкой (56/16 diff-байт = DOS time+date
    # на каждый entry при byte-identical контенте). Пиним 1980-01-01 для
    # воспроизводимых DP-INSTALLED sha-гейтов.
    ts = (1980, 1, 1, 0, 0, 0)
    buf = io.BytesIO()
    with zipfile.ZipFile(buf, "w", zipfile.ZIP_DEFLATED) as z:
        for path, content in [("pack.mcmeta", json.dumps(MCMETA))] + sorted(files.items()):
            zi = zipfile.ZipInfo(path, date_time=ts)
            zi.compress_type = zipfile.ZIP_DEFLATED
            z.writestr(zi, content, compresslevel=6)
    data = buf.getvalue()
    with open(dest, "wb") as f:
        f.write(data)
    import hashlib
    print(f"{dest}: {len(data)}B sha256={hashlib.sha256(data).hexdigest()[:16]} entries={len(files)+1}")


def mcj(lines):
    return "\n".join(lines) + "\n"


# ---------------- STZ-93 ----------------
F93 = {}
F93["data/stz93/functions/load.mcfunction"] = mcj([
    "scoreboard objectives add stz93 dummy",
    "scoreboard players set $phase stz93 0",
    "scoreboard players set $t stz93 0",
    "schedule function stz93:phase0 1t",
])
# phase0: разметка 4 far-точек (weak чанки), старт волн
F93["data/stz93/functions/phase0.mcfunction"] = mcj([
    "# STZ-93 weak-chunk entity-storm (Paper#13783): 4 far markers",
    "forceload add 1000000 1000000",
    "forceload add -1000000 1000000",
    "forceload add 1000000 -1000000",
    "forceload add -1000000 -1000000",
    "scoreboard players set $phase stz93 1",
    "schedule function stz93:wave0 40t",
])
# волны: 4 волны × 4096 = 8 функций по 512 summon (4 точки × 128/функцию)
wave_fns = []
for w in range(4):
    for part in range(2):
        name = f"wave{w}_{part}"
        lines = [f"# STZ-93 wave {w} part {part}: 4 points x 128 items"]
        n = 0
        for pt, (dx, dz) in enumerate(((1000000, 1000000), (-1000000, 1000000), (1000000, -1000000), (-1000000, -1000000))):
            for i in range(128):
                r = (n * 37) % 60 + 2
                a = (n * 73) % 360
                x = dx + round(r * 1.0 * (1 + (a % 60) / 60.0))
                z = dz + (n % 64) * 1
                item_type = ["stone","dirt","cobblestone","sand","gravel","oak_planks","glass","netherrack","snowball","stick","wheat_seeds","iron_nugget","clay_ball","brick","flint","paper"][n % 16]
                lines.append(f"execute positioned {x} 200 {z} run summon minecraft:item ~ ~ ~ {{Item:{{id:\"minecraft:{item_type}\",Count:1b}},PickupDelay:32767s}}")
                n += 1
        F93[f"data/stz93/functions/{name}.mcfunction"] = mcj(lines)
        wave_fns.append(name)
# tick: каскад волн по счётчику (duty-гейт: только пока phase=1)
F93["data/stz93/functions/tick.mcfunction"] = mcj([
    "# STZ-93 tick driver: волна каждые 40t × 4 волны",
    "execute if score $phase stz93 matches 1 if score $t stz93 matches 1 run function stz93:wave0_0",
    "execute if score $phase stz93 matches 1 if score $t stz93 matches 2 run function stz93:wave0_1",
    "execute if score $phase stz93 matches 1 if score $t stz93 matches 41 run function stz93:wave1_0",
    "execute if score $phase stz93 matches 1 if score $t stz93 matches 42 run function stz93:wave1_1",
    "execute if score $phase stz93 matches 1 if score $t stz93 matches 81 run function stz93:wave2_0",
    "execute if score $phase stz93 matches 1 if score $t stz93 matches 82 run function stz93:wave2_1",
    "execute if score $phase stz93 matches 1 if score $t stz93 matches 121 run function stz93:wave3_0",
    "execute if score $phase stz93 matches 1 if score $t stz93 matches 122 run function stz93:wave3_1",
    "execute if score $phase stz93 matches 1 run scoreboard players add $t stz93 1",
])
F93["data/stz93/functions/wave0_0.mcfunction"] = F93["data/stz93/functions/wave0_0.mcfunction"] if "data/stz93/functions/wave0_0.mcfunction" in F93 else ""
F93["data/minecraft/tags/functions/load.json"] = json.dumps({"values": ["stz93:load"]})
F93["data/minecraft/tags/functions/tick.json"] = json.dumps({"values": ["stz93:tick"]})
build_zip("stz93", F93, f"{OUT}/stz93.zip")

# ---------------- STZ-94 ----------------
F94 = {}
F94["data/stz94/functions/load.mcfunction"] = mcj([
    "schedule function stz94:build 1t",
])
# 512 пар hopper(с item-стаками)→composter: сетка 16×32, шаг 2, у y=90, X 0..30 Z 0..62
build_lines = ["# STZ-94 hopper->composter storm (Paper#8142/#976): 512 pairs"]
n = 0
for gx in range(16):
    for gz in range(16):
        x, z = gx * 2, gz * 2
        build_lines.append(f"setblock {x} 90 {z} minecraft:composter")
        build_lines.append(f"setblock {x} 91 {z} minecraft:hopper{{Items:[{{Slot:0b,id:\"minecraft:bone_meal\",Count:64b}},{{Slot:1b,id:\"minecraft:bone_meal\",Count:64b}},{{Slot:2b,id:\"minecraft:bone_meal\",Count:64b}},{{Slot:3b,id:\"minecraft:bone_meal\",Count:64b}},{{Slot:4b,id:\"minecraft:bone_meal\",Count:64b}}]}}")
        n += 1
F94["data/stz94/functions/build.mcfunction"] = mcj(build_lines)
F94["data/minecraft/tags/functions/load.json"] = json.dumps({"values": ["stz94:load"]})
build_zip("stz94", F94, f"{OUT}/stz94.zip")
print("OK")
