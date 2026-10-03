OBSERVED | MAIN w530 | ROTATE: +150L ->SHARED_BOARD_ARCHIVE_W530.md @HEAD; live-window below (canon 20KB/150L) | trim
OBSERVED | MAIN w530 | v24-2: PR #8 c98ai INJECTS-ONLY; мёрж после ответа владельца | api
MERGED | MAIN w530 | 3 --no-ff: 247 SWAR-мост d13852a6, 248 pregate 371c9368, 249 ESEL publisher eca72738 | merge
FACT | MAIN w530 | cargo 0 err, lib 417/0, blobs IN SYNC; fix f3bcccdc: 2 drift-rebuild + javap-shim | gate
DISP | MAIN w530 | A/A pair-1 37118087818: 3.100/3.300 sha-eq, Δ=+6.45% >=3% -> re-roll (prereg AG-242) | 1 pair
FAIL | MAIN w530 | AA2/3 band-gate fail: пустые инпуты -> дефолты банды; раннеры вне [10M,13.5M] | logs
DISP | MAIN w530 | A/A замены 37123894500+37123863574 in_progress, очередь дренирована | 2 POST
MERGED | MAIN w530 | v24-3 dd823586 --no-ff: RECOVERY 9131a575 P500-native sources (CRUSSTY CE a4f53bf1 -> native/: core 95 mod + jni 280 exp + chunk pair, standalone ws, root exclude) + noise_ab A/B instrument; cargo 0 err, selftests 317/317, java-блобы нетронуты | recovery
INJECT | MAIN w530 | v24-3 cc1c951e: оба .so пересобраны из исходников, nm-D parity 283/283 (0 missing/0 extra), MANIFEST SHAs be0e397b/3a85b4e6, бинари 2025-09 в истории | inject
FAIL | MAIN w530 | lever 83f29e5 hoist normal-noise scaling: noise_ab same-process 6 cases d=+0.10..-0.04% (<+2%), A/A<=0.6% -> REJECT: CE уже вобрал эквивалент (INLINE_AXIS_CACHE + get_value decomposition); патч откат из src, стенд сохранён | fail-zone
