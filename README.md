# c-crussty — Crussty CE native surface as a plugin

A module of the [CRUSSTY](https://github.com/PLANETA9091/CRUSSTY) platform that
injects the full Crussty CE JNI bridge (98 bridge classes, 283 natives) into
the kernel, plus an `ImprovedNoise` hot-patch via ASM weaving.

## Layout

- `src/` — Rust (jni_table, bridge_class, classfile, improved_noise, area_map)
- `area-map/` — `ca/` — Java sources of the Moonrise bridge classes, `build/` — compiled (embedded via `include_bytes!`)
- `noise/` — `net/` — Java sources of the noise bridge, `build/` — compiled
- `tests/fixtures/` — test fixtures

## Build & install

Clone into `<crussty>/modules/crussty` and build:

```bash
cargo build
cp target/debug/libcrussty.so libcrussty.so
```

Enable the noise patch with `CRUSSTY_NATIVE_IMPROVED_NOISE=1` in the env at
server start.

Note: runtime dependencies `native/libpaper_native_jni.so` come from the
original Crussty CE (not committed here).
