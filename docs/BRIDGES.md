# BRIDGES — the Java bridge material

`bridges/` holds the kernel-side Java material of the module. Everything here
mirrors a **kernel package tree** (net/minecraft/**, ca/spottedleaf/**) —
these sources and payloads are derived from the kernel (vanilla/Paper/Moonrise)
and are hot-patched or defined into the running JVM by the Rust module. They
are deliberately kept out of the Rust language statistics (`.gitattributes`
marks them `linguist-vendored`) — the module itself is the Rust code.

## Layout

```
bridges/
├── <name>/
│   ├── net/minecraft/... or ca/spottedleaf/...   # Java sources (rebuild inputs)
│   └── build/                                    # compiled .class payloads
```

Two kinds of `build/` payloads exist:

1. **Bridge ops classes** (`*Ops.class`, `*NativeOps.class`, …) — public
   static helpers the module defines into the kernel loader at runtime. The
   compiled bytes are the canonical artifact: `src/*.rs` embeds them via
   `include_bytes!("../bridges/<name>/build/...")` and (re)defines +
   `RegisterNatives`-binds them at runtime. Delivery tests in `src/`
   assert byte-parity between the embedded copy and the build dir.
2. **Patched kernel classes** (e.g. `paletted/build/PalettedContainer.patched.class`)
   — reference outputs of the patch pipeline, used by parity tests.

## Current bridges

| dir | wired by (src/) | purpose |
|---|---|---|
| `area-map/` | `area_map.rs` | SingleUserAreaMap.update hot-patch (native ops batch) |
| `noise/` | `improved_noise.rs`, `perlin_noise.rs`, `noise_fill.rs` | ImprovedNoise/PerlinNoise/NormalNoise native bridges |
| `entityinside/` | region/inside/emap/zero* modules | inside-blocks, entity map, traversal, region-tick lanes |
| `entityquery/`, `entitygoalquery/`, `bulkjni/`, `entityselector/` | `entity_query.rs`, `entity_index*.rs`, `selector_bulk.rs` | entity query/index bulk kernels |
| `mobai/`, `mobpush/`, `mobs*` consumers | `mobs_ai.rs`, `mobs_manager.rs`, … | mob AI/push/scan lanes |
| `chunksend/`, `chunkparse/`, `chunksched/` | `chunk_send*.rs`, `chunk_parse.rs`, `chunk_sched.rs` | chunk send/parse/schedule lanes |
| `fluid/`, `colpush/`, `stagger/`, `sense/`, `sscan/`, `randomtick/`, `goalops/`, `items/`, `poi/`, `prepare/`, `queryplane/`, `paletted/` | respective `src/*.rs` | experimental/architecture lanes (dormant unless gated) |

`allocdiet/harness/`, `tests/area_map_smoke/` and `cplug-sdk/asm-src/` are
kernel-coupled harnesses/fixtures of the same nature (also `linguist-vendored`).

## Rebuilding payloads

Each bridge has a pinned rebuild script (JDK `--release` pinned so the class
file major version stays compatible with the kernel JVM):

```bash
scripts/build_noise.sh            # noise bridges
scripts/build_area_map.sh         # area-map ops (+ _budget/_probe variants)
scripts/build_emap_ops.sh         # entity map ops
scripts/build_<name>_ops.sh       # one script per ops family (see scripts/)
```

Requirements: a JDK with `javac` (or the vendored ECJ compiler at
`tools/ecj.jar`), sources from `bridges/<name>/`, output into
`bridges/<name>/build/`. After rebuilding, `cargo test` re-checks the
byte-parity gates — a payload mismatch fails the delivery tests loudly.

## History note

Before 2026-10 these directories lived at the repository root
(`area-map/`, `noise/`, `entityinside/`, …); the restructure consolidated
them under `bridges/` and updated every `include_bytes!`/delivery-test path.
Old paths remain valid in git history (tag `pre-restructure-2026-10`).
