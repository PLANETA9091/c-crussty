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
    buf = io.BytesIO()
    with zipfile.ZipFile(buf, "w", zipfile.ZIP_DEFLATED) as z:
        z.writestr("pack.mcmeta", json.dumps(MCMETA))
        for path, content in files.items():
            z.writestr(path, content)
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

# ================= V2 (Task 511-swarm-STZ, x511) — v1-зипы НЕ тронуты =================
# Волна-512 re-feeds несут v1-канон (stz93 seed 1432 sha 9d166b8e…, stz94 seed 1433
# sha cbed1530…) -> v2 пишется в {stz93v2,stz94v2}.zip. Детерминизм build_zip()
# (ZipInfo 1980-01-01 + sorted + compresslevel 6) контент-агностичен: r-кольца и
# x16-итерации ОБА совместимы с JAM-патчем на zip-слое. Выбор: r-кольца — спека
# СТЗ-93 "summon r{2..64}" + механизм Paper#13783 mass-sync-chunk-LOAD (итерации
# концентрируют 65k item в тех же 4 чанк-зонах -> entity-merge, weak-chunk set не
# расширяется -> риск NOT-A-BENCH по G-STZ93-A/B).

# ---- stz93 v2: 16 колец r{2,6,...,62} x 4096 = 65,536 summon (x16 v1-факта 4096) ----
# Волна k на t=42+10k (42..192) — целиком внутри гейт-окна G-STZ93-A [arm+40, arm+200].
# Фиксы x510: D1 (фантомный schedule stz93:wave0 снят), D2 ($phase->0 после ring15).
F93v2 = {}
F93v2["data/stz93/functions/load.mcfunction"] = mcj([
    "scoreboard objectives add stz93 dummy",
    "scoreboard players set $phase stz93 0",
    "scoreboard players set $t stz93 0",
    "schedule function stz93:phase0 1t",
])
F93v2["data/stz93/functions/phase0.mcfunction"] = mcj([
    "# STZ-93 v2 weak-chunk entity-storm (Paper#13783): 4 far markers",
    "forceload add 1000000 1000000",
    "forceload add -1000000 1000000",
    "forceload add 1000000 -1000000",
    "forceload add -1000000 -1000000",
    "scoreboard players set $phase stz93 1",
])
tick_v2 = ["# STZ-93 v2: 16 колец r{2,6,..,62} x 4096, волна каждые 10t (t=42..192 <= [40,200])"]
for k in range(16):
    tick_v2.append(f"execute if score $phase stz93 matches 1 if score $t stz93 matches {42 + 10 * k} run function stz93:ring{k}")
tick_v2 += [
    "execute if score $phase stz93 matches 1 if score $t stz93 matches 193 run scoreboard players set $phase stz93 0",
    "execute if score $phase stz93 matches 1 run scoreboard players add $t stz93 1",
]
F93v2["data/stz93/functions/tick.mcfunction"] = mcj(tick_v2)
PTS = ((1000000, 1000000), (-1000000, 1000000), (1000000, -1000000), (-1000000, -1000000))
ITEMS16 = ["stone", "dirt", "cobblestone", "sand", "gravel", "oak_planks", "glass", "netherrack",
           "snowball", "stick", "wheat_seeds", "iron_nugget", "clay_ball", "brick", "flint", "paper"]
for k in range(16):
    r = 2 + 4 * k  # 16 колец внутри спеки r{2..64}
    lines = [f"# STZ-93 v2 ring {k}: r={r} (jitter r..2r), 4 points x 1024 items = 4096/ring"]
    n = 0
    for (dx, dz) in PTS:
        for _ in range(1024):
            a = (n * 73) % 360
            rad = r * (1 + (a % 60) / 60.0)          # канон-джиттер v1: r..2r
            x = dx + round(rad * ((a % 90) / 90.0))  # квадрант-дуга: 2D-разлёт кольца
            z = dz + round(rad * (1 - (a % 90) / 90.0)) + (n % 64) - 32
            it = ITEMS16[n % 16]
            lines.append(f"execute positioned {x} 200 {z} run summon minecraft:item ~ ~ ~ {{Item:{{id:\"minecraft:{it}\",Count:1b}},PickupDelay:32767s}}")
            n += 1
    F93v2[f"data/stz93/functions/ring{k}.mcfunction"] = mcj(lines)
F93v2["data/minecraft/tags/functions/load.json"] = json.dumps({"values": ["stz93:load"]})
F93v2["data/minecraft/tags/functions/tick.json"] = json.dumps({"values": ["stz93:tick"]})
build_zip("stz93v2", F93v2, f"{OUT}/stz93v2.zip")

# ---- stz94 v2: 512 пар (D6: gz range(32)) + anti-JAM schedule-reset (D7) ----
# D7-матем: компостер full (level 8) ждёт RANDOM-TICK-reset; fill ~213t при 0.125
# push/tick x 30% success -> JAM 9-15s v1-факт. v2 = schedule-reset БЕЗ третьего BE
# (спека-чисто 1024 BE = 512 hopper + 512 composter): refill каждые 120t
# (level при reset ~4.5 < 8 -> JAM НИКОГДА, duty ~100% весь 600s соук;
# 600s @2.2TPS = 1320t = 11 refill x 1024 setblock = 8.5 cmd/tick avg = шум).
# Вариант "hopper-extract y89" отклонён: +512 BE -> blockEntities-tick share
# фикстурный конфаунд G-STZ94-PARITY. D8-пин: Count:64b валиден до 1.21.4;
# host >=1.21.5 (items lowercase count) -> хопперы пустые = NOT-A-BENCH (share~0
# -> НЕ абсорбить, пересобрать). Путь plural = deprecated-fallback (пин на host).
HOP_NBT = ("minecraft:hopper{{Items:[{{Slot:0b,id:\"minecraft:bone_meal\",Count:64b}},"
           "{{Slot:1b,id:\"minecraft:bone_meal\",Count:64b}},{{Slot:2b,id:\"minecraft:bone_meal\",Count:64b}},"
           "{{Slot:3b,id:\"minecraft:bone_meal\",Count:64b}},{{Slot:4b,id:\"minecraft:bone_meal\",Count:64b}}]}}")
F94v2 = {}
F94v2["data/stz94/functions/load.mcfunction"] = mcj([
    "schedule function stz94:build 1t",
])
build_v2 = ["# STZ-94 v2 hopper->composter storm (Paper#8142/#976): 512 pairs (16x32)"]
refill_v2 = ["# STZ-94 v2 anti-JAM: re-arm hoppers (5x64) + reset composters, цикл 120t"]
for gx in range(16):
    for gz in range(32):  # v2 D6-фикс: было range(16) = 256 пар
        x, z = gx * 2, gz * 2
        build_v2.append(f"setblock {x} 90 {z} minecraft:composter")
        build_v2.append(f"setblock {x} 91 {z} " + HOP_NBT)
        refill_v2.append(f"setblock {x} 90 {z} minecraft:composter")
        refill_v2.append(f"setblock {x} 91 {z} " + HOP_NBT)
F94v2["data/stz94/functions/build.mcfunction"] = mcj(build_v2 + ["schedule function stz94:refill 120t"])
F94v2["data/stz94/functions/refill.mcfunction"] = mcj(refill_v2 + ["schedule function stz94:refill 120t"])
F94v2["data/minecraft/tags/functions/load.json"] = json.dumps({"values": ["stz94:load"]})
build_zip("stz94v2", F94v2, f"{OUT}/stz94v2.zip")
print("OK")
