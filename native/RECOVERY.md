# Native sources recovery (v24)

Full recovery of the P500-era native sources from `PLANETA9091/CRUSSTY`
(CE release `a4f53bf1`, 2026-06-06). That repo deleted `native/` from its
master by September 2026 — the CE snapshot is the only complete surviving
copy. c-crussty previously shipped only prebuilt binaries (see
`MANIFEST.md` provenance: built 2025-09 from the then-closed crates).

## Recovered crates (buildable workspace)

| Crate | Content |
|---|---|
| `paper-native-core` | 95 modules (aquifer, area_map, climate, improved/perlin/normal noise, tickets, ...) |
| `paper-native-jni` | 13061-line JNI surface, 280 exports → `libpaper_native_jni.so` |
| `paper-native-chunk-encode-core` | chunk/light packet encode + microbench bin |
| `paper-native-chunk-encode-jni` | 3 exports → `libpaper_native_chunk_encode_jni.so` |

Build: `cargo build --release` here. Parity gate: `nm -D` export set must
cover all 283 rows of `../JNI_EXPORTS.manifest` (280 + 3).

## Provenance chain (June 2026, CRUSSTY)

- `1cede7cd` codex: batch shifted normal noise in native (adds core noise + jni surface)
- `c18bda92` codex: optimize native normal noise batching
- `83f29e53` codex: hoist normal noise batch scaling — **NOT merged upstream, applied here as v24 lever** (see bench_ab)
- `bb8cf743` codex: Add native chunk/light encode prototype (chunk-encode pair)
- `a4f53bf1` Paper Native Accelerator CE — initial community release (full snapshot, recovery base)

The workspace is excluded from the root c-crussty workspace (`Cargo.toml`
root `[workspace].exclude`) — it builds standalone.
