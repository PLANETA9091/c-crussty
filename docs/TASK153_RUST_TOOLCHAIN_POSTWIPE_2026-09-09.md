# TASK-153 — POST-WIPE RUST TOOLCHAIN RESTORATION (agent-7625532f, 2026-09-09)

Status: **PASS (infrastructure restoration grade)** — cargo/rustc restored after
sandbox wipe #2, all agent-side gates pass on the first run, and one material
premise correction recorded: the P500 kernel `.so` files are closed-source
prebuilt binaries and were never buildable in-sandbox.

## What was done

1. **Installed** rustup per the repo README's documented path
   (`rustup-init.sh --default-toolchain stable --profile minimal`):
   **cargo/rustc 1.98.1** (2026-08/09 builds). Satisfies `Cargo.lock` v4 (needs
   cargo ≥ 1.78) and edition 2021. Pre-wipe toolchain version is unknown
   (died with the wipe, no pin file in repo) — owner may want to pin one.
2. **Gate — `cargo test --locked`: 65/65 PASS**, 0 failed, 0 ignored
   (matches the pre-wipe documented 65-test suite, including
   `whole_body_bridge_wirings_are_policy_gated`).
3. **Gate — `cargo clippy --locked`**: 14 warnings, all style-class
   (9 × doc-list-overindent, 3 × manual `is_multiple_of`, 1 × `?`-rewrite,
   1 summary). Pre-wipe documented baseline was 12; the +2 delta is **new-lint
   drift** (`is_multiple_of` landed in rustc 1.87, doc-overindent similar), not
   code change — no source was touched this task.
4. **`cargo build --release --locked` PASS** (6.45 s): `target/release/libcrussty.so`
   1,239,416 B, exports = the agent-side batch-dispatch bridge
   (`Java_crussty_batch_PaperNativeBatchDispatch_{abiVersion,run}`), consistent
   with `src/batch_api.rs`.

## Premise correction (vs the TASK-153 claim, honest scope-down)

The claim planned a kernel-`.so` rebuild + P500 drift parity gate. That premise
was **wrong**: `README.md` states the P500 kernel libraries
(`libpaper_native_jni.so`, `libpaper_native_chunk_encode_jni.so`) are
**closed-source shared libraries** — owner-published binaries (`native/` has a
single history commit, "publish Crussty CE binaries"). Verified by symbol count:
canonical main lib = 280 `Java_` exports, chunk-encode lib = 3 (§97),
280 + 3 = 283 = `JNI_EXPORTS.manifest` exactly. The repo Rust code builds the
**agent** only. Consequences:

- No `.so` was replaced → no P500 drift gate was needed → canonical `native/`
  assets untouched, working tree clean of any binary changes.
- The kernel lane was never cargo-blocked (kernels cannot be built here at
  all); what cargo restoration unblocks is **agent-side development and the
  discipline gates** (`cargo test 65/65` + clippy on every src-touching task)
  plus local agent builds for injection testing.
- New kernel variants against the registered do-not-wire optimization targets
  (§100) require owner-side kernel builds — the correct owner-ask if those
  targets are to be re-attacked.

## Value and limits

Restored capability: agent builds, workspace tests, clippy, `scripts/` Rust
tooling — all post-wipe functional again. Limits: kernel lane remains
measurement-only in-sandbox (correct by design: closed-source kernels,
BENCHFIRST §5); server-dependent channels unchanged (TASK-150 execution, C3
cadence still wait on `/home/z/server` + jdk21). INJECTS-ONLY intact: 0 server
boots, 0 product changes, 0 binary deployments.
