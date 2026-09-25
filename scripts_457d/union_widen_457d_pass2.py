#!/usr/bin/env python3
"""union_widen_457d_pass2.py — leftover pattern shapes for TASK-457-D.

  P1 rust arm-lists:   `| Ok("cmp450_chunk")`  (no body on line)  -> append `| Ok("cmp457_noisesimd")`
  P2 rust arm-lists:   `| Some("cmp450_chunk")`                  -> append `| Some("cmp457_noisesimd")`
  P3 java str-first:   `"cmp450_chunk".equals(X)`                -> `... || "cmp457_noisesimd".equals(X)`
  P4 colpush constant: FLAG_457D + chain element in leverEnabled()
  P5 raw-cp needles:   CARRIER_UNION_457D constant in chunkparse/chunksend blobs
  P6 sync needles:     check_blobs_sync.sh lists get "cmp457_noisesimd" after "cmp450_chunk"
"""
import re, pathlib

ROOT = pathlib.Path("/home/z/rounds/ROUND-457/agent-d")
OLD = "cmp450_chunk"
NEW = "cmp457_noisesimd"

def p12(txt):
    out = []
    n = 0
    for line in txt.split("\n"):
        if OLD in line and NEW not in line and "==" not in line:
            if re.search(r'Ok\("' + OLD + r'"\)(?!.*=>)', line):
                line = line.replace(f'Ok("{OLD}")', f'Ok("{OLD}") | Ok("{NEW}")')
                n += 1
            elif re.search(r'Some\("' + OLD + r'"\)(?!.*=>)', line):
                line = line.replace(f'Some("{OLD}")', f'Some("{OLD}") | Some("{NEW}")')
                n += 1
        out.append(line)
    return "\n".join(out), n

def p3(txt):
    rx = re.compile(r'"' + OLD + r'"\.equals\((\w+)\)')
    n = 0
    def sub(m):
        nonlocal n
        n += 1
        return f'"{OLD}".equals({m.group(1)}) || "{NEW}".equals({m.group(1)})'
    return rx.sub(sub, txt), n

def main():
    for name in ["src/entity_query.rs", "src/mobs_ai.rs", "src/mobs_sense.rs",
                 "src/mobs_sscan.rs", "src/stagger.rs"]:
        p = ROOT / name
        txt = p.read_text(encoding="utf-8")
        txt2, n = p12(txt)
        if txt2 != txt:
            p.write_text(txt2, encoding="utf-8")
            print(f"{name}: P1/P2 arms added x{n}")
    p = ROOT / "entityinside/net/minecraft/world/entity/ItemEntityManager.java"
    txt = p.read_text(encoding="utf-8")
    txt2, n = p3(txt)
    if txt2 != txt:
        p.write_text(txt2, encoding="utf-8")
        print(f"entityinside ItemEntityManager.java: P3 x{n}")

    # P4 colpush: constant + chain element
    p = ROOT / "colpush/net/minecraft/world/entity/ColpushOps.java"
    txt = p.read_text(encoding="utf-8")
    if NEW not in txt:
        anchor = '    private static final String FLAG14 = "cmp450_chunk";'
        assert anchor in txt, "colpush anchor missing"
        txt = txt.replace(anchor, anchor +
                          '\n    /** TASK-457-D noise-SIMD carrier (STRICT-OR; raw-cp needle). */\n'
                          '    private static final String FLAG_457D = "cmp457_noisesimd";')
        txt = txt.replace("f.trim().equals(FLAG_DIET));",
                          'f.trim().equals(FLAG_DIET) || f.trim().equals(FLAG_457D));')
        p.write_text(txt, encoding="utf-8")
        print("colpush ColpushOps.java: P4 constant+chain")

    # P5 raw-cp needle constants (chunkparse / chunksend x2)
    for name, anchor in [
        ("chunkparse/net/minecraft/world/level/chunk/storage/ChunkParseOps.java",
         '    static final String CARRIER_UNION_450 = "cmp450_chunk";'),
        ("chunksend/net/minecraft/server/network/ChunkPacketEncodeOps.java",
         '    static final String CARRIER_UNION_450 = "cmp450_chunk";'),
        ("chunksend/net/minecraft/server/network/ChunkSendOps.java",
         '    static final String CARRIER_UNION_450 = "cmp450_chunk";'),
    ]:
        p = ROOT / name
        txt = p.read_text(encoding="utf-8")
        if NEW not in txt:
            assert anchor in txt, f"anchor missing in {name}"
            txt = txt.replace(anchor, anchor +
                              '\n    /** TASK-457-D noise-SIMD carrier (STRICT-OR; raw-cp needle). */\n'
                              '    static final String CARRIER_UNION_457D = "cmp457_noisesimd";')
            p.write_text(txt, encoding="utf-8")
            print(f"{name}: P5 needle constant")

    # P6 check_blobs_sync.sh needles
    p = ROOT / "scripts/check_blobs_sync.sh"
    txt = p.read_text(encoding="utf-8")
    if NEW not in txt:
        n = txt.count(f'"{OLD}"')
        txt = txt.replace(f'"{OLD}"', f'"{OLD}" "{NEW}"')
        p.write_text(txt, encoding="utf-8")
        print(f"scripts/check_blobs_sync.sh: P6 needles added x{n}")

if __name__ == "__main__":
    main()
