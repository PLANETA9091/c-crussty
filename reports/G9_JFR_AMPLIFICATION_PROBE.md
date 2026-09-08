# G9 JFR amplification probe — first live measurement of the `Ap2.fillArray` loop (S7-14)

Session S7-14 (cron 08:43+08, Job 366450). Live-boot probe against
`docs/G9_WHOLE_METHOD_HOOK_DESIGN.md` §4.2 blocker (a): the design doc's NO-GO
stands "until JFR amplification proof" — this report supplies the FIRST measured
data point. Boot 02:00:33Z 2026-09-08 (pid 24005), stock dormant hook (no
CRUSSTY arming env — the G9 §5 "profile with the hook dormant" rule), JFR
`settings=profile` via JAVA_TOOL_OPTIONS, load = 1049 fresh chunks
(`forceload add`, 4×256-chunk regions at chunk [1000..1031]², RCON-driven —
`scripts/rcon.py`; the launcher stdin fifo does NOT deliver mid-run console
commands, flood of empty prompts observed; RCON enabled in server.properties
for the probe, left ON — password is session-throwaway).

Artifacts: `reports/G9_JFR_g9probe_24005.jfr.gz` (3.3 MB, full recording),
`reports/G9_JFR_threadcpuload.txt` (raw jdk.ThreadCPULoad print),
`scripts/g9_jfr_analyze.py` (analyzer; the inline per-second breakdown below is
the load-bearing evidence).

## Measured facts (jdk.ExecutionSample, 9906 samples, 560 s window)

1. **`DensityFunctions$Ap2.fillArray` runs ENTIRELY on Paper's async gen
   worker** ("Paper Common Worker #0"): 307/307 inclusive samples, **0 samples
   on the Server thread** across the whole recording. The Server thread only
   coordinates (and stalls on the watchdog during gen bursts — 4 thread dumps).
   Any future G9 hook would execute its replacement body on WORKER threads —
   the S2 model's "per tick" framing needs re-deriving for bursty async gen
   before any economics claim.
2. **Sustained generation was 02:00:59–02:04:21 (~200 s)**, 1049 chunks,
   worker duty ≈ 40-60 samples/s at the 10 ms period (≈ 0.4-0.6 core). After
   02:04:22 the box idles (worker ≈ 1 sample/s). Steady-state (no gen)
   fillArray rate ≈ **zero** — the method does not run outside world-gen/light
   updates on an idle server.
3. **fillArray share of worker samples during generation: 5.12%** (307/5997
   over busy seconds, w ≥ 5); per-second peaks 10-23%.
4. Worker CPU per tick during gen ≈ 20-30 ms (0.4-0.6 core × 50 ms) →
   **fillArray ≈ 1.0-1.5 ms/tick DURING SUSTAINED GEN BURSTS ONLY**, ≈ 0 in
   steady state. Via the matrix §2.1 identity (R_mid = 54 ns/call) that is
   **≈ 19-29k calls/tick during bursts** — inside the §2.1 band (12k-40k),
   marginally at the §4.2 bar (≥ ~18.5k).

## Verdict update (G9 §7)

- **Blocker (a) is now MEASURED, not modeled: MARGINAL.** The ≥ ~18.5k/tick
  amplification is reached only during SUSTAINED world-gen bursts
  (first-time chunk generation via forceload/explore); it is **zero in
  steady-state ticks** — the model's 192/tick steady-state figure was never
  wrong for normal operation, the amplified band only exists as gen-burst
  behavior. A hook win would therefore be a gen-time win (~1-1.5 ms/tick
  ceiling, i.e. 2-3% of a 50 ms tick DURING BURSTS ONLY), not a steady-state
  tick win. This is ~20-30x below the pre-measurement hope that gen bursts
  would show a 96x amplification story.
- **Blocker (b) (documented `(III[J)I` scalar semantics + byte-exact parity
  gate) remains UNCLEARED** — see §3 of the design doc. Per §7, either alone
  is fatal.
- **G9 stays NO-GO**, now with the honest measured framing: even IF blocker
  (b) cleared, the payoff envelope is "gen-burst-only, ≤ ~1-1.5 ms/tick of
  JAVA code to replace with a native bridge of unknown cost" — the hook is
  NOT economically attractive, and the highest-blast-radius method in the
  codebase (§5 ADD/MUL collateral, terrain-generating arms) makes the risk
  side worse. Recommend closing as **never-batch/never-hook at steady state;
  gen-burst-only potential permanently parked behind blocker (b)** — do not
  invest parity-gate work without an operator-level requirement for gen-time
  latency.
- Method note (honest limits): sampler-based time attribution, not an
  invocation counter; the 19-29k figure converts time via R_mid and inherits
  its ±; worker duty estimated from sampler duty (under-sampling biases both
  numerator and denominator, the 5.12% share is the robust number). A counting
  probe would require building the hook itself — rejected as circular.
