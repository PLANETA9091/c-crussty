# LEDGER-07 — chunk-send/serialization (TASK-459-L07)
- round-458m-delta @d13eb80a = RESEARCH-ONLY (1 коммит, +54 строки, 0 кода); cmp458_chdelta strings в блобах = 0/всё — вайринга НЕТ, шаги 1-6 плана NOT STARTED.
- chkmono457-14 wall (63661 сэмплов): chunk-send serialize лейн **0.00%**; entity-sync send лейн **0.68%** wall / **2.6%** CPU-окна (sendChanges 1.1% + SynchedEntityData 3.0%); players_packets 0.01%.
- Capture-матем: ID-M1 монстр-нога 0.00%×85% = **+0.0пп, потолок +0.0пп** (не диспатчить); joins-burst 1.75%×85% = **+1.5пп, потолок +1.75пп**; P26 entity-sync 2.6%×35% = **+0.9пп, потолок +2.6пп**.
- javap 4 контракта: 2-arg extractChunkData = redirect sites:1; 3-arg = чистый section-loop + ISE writerIndex==capacity; LevelChunkSection.write = самодостаточный пер-секционный payload (bit-in-byte по построению); ChunkSendOps.sendChunk = chunk4 hook-сигнатура, слой ниже whole-packet CACHE cap-2048.
- До world-bench: bridge → redirect → rust(STRICT+EARLY-define) → carrier-union в paldelta-сайты (21 rust + 10 blob java) → блобы + javap 10/10 + strings-gate ≥1/блоб → dispatch world-bench-parallel leg (lever_flag=cmp458_chdelta), НЕ монстр-soak.
