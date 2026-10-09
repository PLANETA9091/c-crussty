# NCF RULES (owner standing order, 2026-10-09 — supersedes Job 441690)

R1 Every code commit changes code or adds a measured number to
   docs/NCF_SPEED.md. Worklog-only commits are allowed only as ride-along
   with the next code commit. Never commit just to confirm a CI run.

R2 A speed lever must show: before/after on the ledger, two runs within
   2%, bit-identical output (stagediff --gen-batch, NCF flags 0/1), gate-p2
   15/15 and ncf-staged green. Gain < 5%: revert and record the numbers.

R3 Do not touch the veccheck/stagediff gate contracts. No tokens in files
   or payloads. No decompiled Mojang sources in the repo (move
   chunk-factory/docs_overworld_biome_builder.java out and gitignore it).

R4 Worklog entries <= 20 lines. Move entries older than 3 days to
   docs/archive/. Read only the last 150 lines each tick.

R5 Hypotheses marked HYP are unproven. Prove with an ncf_profile probe
   (counts or timings) BEFORE writing the fix.
