use crate::jrandom::{LegacyRandomSource, RandomSource};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SpreadType {
    Linear,
    Triangular,
}

/// StructurePlacement.FrequencyReductionMethod (CFR :148-176) — the JSON key
/// frequency_reduction_method; default "default".
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FrequencyReductionMethod {
    Default,
    LegacyType1,
    LegacyType2,
    LegacyType3,
}

/// StructurePlacement.ExclusionZone (CFR :179-185) — deprecated but shipped:
/// pillager_outposts forbids placement within chunk_count chunks of the
/// other_set's placement chunks.
#[derive(Clone, Debug)]
pub struct ExclusionZone {
    pub other_set: String,
    pub chunk_count: i32,
}

#[derive(Clone, Debug)]
pub struct RandomSpreadStructurePlacement {
    pub spacing: i32,
    pub separation: i32,
    pub salt: i32,
    pub spread_type: SpreadType,
    /// StructurePlacement.frequency (default 1.0 — no reduction).
    pub frequency: f32,
    pub frequency_reduction_method: FrequencyReductionMethod,
    pub exclusion_zone: Option<ExclusionZone>,
}

impl RandomSpreadStructurePlacement {
    pub fn parse(v: &crate::json::Json) -> Result<Self, String> {
        let placement = v
            .get("placement")
            .ok_or("structure_set missing placement")?;
        let t = placement
            .get("type")
            .and_then(|x| x.as_str())
            .unwrap_or("minecraft:random_spread");
        if t != "minecraft:random_spread" {
            return Err(format!("unsupported structure placement {t}"));
        }
        let frequency_reduction_method = match placement
            .get("frequency_reduction_method")
            .and_then(|x| x.as_str())
            .unwrap_or("default")
        {
            "legacy_type_1" => FrequencyReductionMethod::LegacyType1,
            "legacy_type_2" => FrequencyReductionMethod::LegacyType2,
            "legacy_type_3" => FrequencyReductionMethod::LegacyType3,
            _ => FrequencyReductionMethod::Default,
        };
        let exclusion_zone = placement.get("exclusion_zone").and_then(|z| {
            let other_set = z.get("other_set").and_then(|x| x.as_str())?.to_string();
            let chunk_count = z.get("chunk_count").and_then(|x| x.as_i64())? as i32;
            Some(ExclusionZone {
                other_set,
                chunk_count,
            })
        });
        Ok(RandomSpreadStructurePlacement {
            spacing: placement.get("spacing").and_then(|x| x.as_i64()).unwrap_or(32) as i32,
            separation: placement.get("separation").and_then(|x| x.as_i64()).unwrap_or(8) as i32,
            salt: placement.get("salt").and_then(|x| x.as_i64()).unwrap_or(0) as i32,
            spread_type: match placement
                .get("spread_type")
                .and_then(|x| x.as_str())
                .unwrap_or("linear")
            {
                "triangular" => SpreadType::Triangular,
                _ => SpreadType::Linear,
            },
            frequency: placement
                .get("frequency")
                .and_then(|x| x.as_f64())
                .unwrap_or(1.0) as f32,
            frequency_reduction_method,
            exclusion_zone,
        })
    }

    /// StructurePlacement.applyAdditionalChunkRestrictions (CFR :101-103):
    /// the frequency gate — a no-op while frequency >= 1.0.
    ///
    /// The four reducers (CFR :117-146):
    /// - default            = probabilityReducer: setLargeFeatureWithSalt(seed,
    ///   region, salt) then nextFloat() < frequency;
    /// - legacy_type_1      = legacyPillagerOutpostReducer (CFR :138-146):
    ///   setSeed((regionX>>4) ^ ((regionZ>>4)<<4) ^ levelSeed), DISCARD one
    ///   nextInt(), then nextInt((int)(1.0f/frequency)) == 0;
    /// - legacy_type_2      = legacyArbitrarySaltProbabilityReducer (CFR
    ///   :131-136): setLargeFeatureWithSalt with saltOverride ?? 10387320 (the
    ///   set's own salt is IGNORED), nextFloat() < frequency — vanilla confs
    ///   leave the overrides null (Paper seeds config ships null), so the
    ///   fallback salt 10387320 is the vanilla-effective one;
    /// - legacy_type_3      = legacyProbabilityReducerWithDouble (CFR
    ///   :123-129): setLargeFeatureSeed (no salt), nextDouble() < frequency.
    pub fn frequency_allows(&self, seed: i64, chunk_x: i32, chunk_z: i32) -> bool {
        if self.frequency >= 1.0 {
            return true;
        }
        match self.frequency_reduction_method {
            FrequencyReductionMethod::Default => {
                let mut rng = LegacyRandomSource::new(0);
                rng.set_large_feature_with_salt(seed, chunk_x, chunk_z, self.salt);
                rng.next_f32() < self.frequency
            }
            FrequencyReductionMethod::LegacyType1 => {
                let i = chunk_x >> 4;
                let i1 = chunk_z >> 4;
                let mut rng = LegacyRandomSource::new(0);
                // (long)(i ^ i1 << 4) ^ levelSeed — int xor sign-extended
                rng.set_seed(((i ^ (i1 << 4)) as i64) ^ seed);
                rng.next_int(); // discarded
                // (int)(1.0f / frequency): 1.0f/0.2f rounds to exactly 5.0f
                rng.next_int_bound((1.0f32 / self.frequency) as i32) == 0
            }
            FrequencyReductionMethod::LegacyType2 => {
                let mut rng = LegacyRandomSource::new(0);
                rng.set_large_feature_with_salt(seed, chunk_x, chunk_z, 10387320);
                rng.next_f32() < self.frequency
            }
            FrequencyReductionMethod::LegacyType3 => {
                let mut rng = LegacyRandomSource::new(0);
                rng.set_large_feature_seed(seed, chunk_x, chunk_z);
                rng.next_f64() < self.frequency as f64
            }
        }
    }

    /// StructurePlacement.isStructureChunk (CFR :89-107) — the FULL gate:
    /// placement prescreen && frequency reduction && exclusion zone.
    ///
    /// `exclusion_other` = the parsed placement of self.exclusion_zone's
    /// other_set (resolved by the caller from the worldgen dir). The java
    /// inner test is the OTHER set's FULL isStructureChunk (recursion); every
    /// vanilla shipped target (villages) has no frequency/exclusion of its
    /// own, so placement+frequency is exact for the shipped set graph.
    pub fn is_structure_chunk(
        &self,
        seed: i64,
        chunk_x: i32,
        chunk_z: i32,
        exclusion_other: Option<&RandomSpreadStructurePlacement>,
    ) -> bool {
        if !self.is_placement_chunk(seed, chunk_x, chunk_z) {
            return false;
        }
        if !self.frequency_allows(seed, chunk_x, chunk_z) {
            return false;
        }
        if let (Some(zone), Some(other)) = (&self.exclusion_zone, exclusion_other) {
            // ExclusionZone.isPlacementForbidden -> ChunkGeneratorStructureState
            // .hasStructureChunkInRange (CFR :268-282): the INCLUSIVE square
            // [x-range..x+range] x [z-range..z+range] is scanned.
            for i in (chunk_x - zone.chunk_count)..=(chunk_x + zone.chunk_count) {
                for j in (chunk_z - zone.chunk_count)..=(chunk_z + zone.chunk_count) {
                    if other.is_placement_chunk(seed, i, j) && other.frequency_allows(seed, i, j) {
                        return false;
                    }
                }
            }
        }
        true
    }

    /// RandomSpreadStructurePlacement.isPlacementChunk — the prescreen ONLY
    /// (no frequency/exclusion — those are is_structure_chunk's layers).
    pub fn is_placement_chunk(&self, seed: i64, chunk_x: i32, chunk_z: i32) -> bool {
        match self.potential_chunk(seed, chunk_x, chunk_z) {
            Some((cx, cz)) => cx == chunk_x && cz == chunk_z,
            None => false,
        }
    }

    /// getPotentialFeatureChunk — ONE rng, x draw then z draw (order fixed).
    pub fn potential_chunk(&self, seed: i64, chunk_x: i32, chunk_z: i32) -> Option<(i32, i32)> {
        if self.spacing <= self.separation || self.spacing <= 0 {
            return None; // degenerate config — vanilla never ships one
        }
        let i = chunk_x.div_euclid(self.spacing);
        let j = chunk_z.div_euclid(self.spacing);
        self.potential_chunk_for_region(seed, i, j)
    }

    /// Region-index entry point (i = chunk_x.div_euclid(spacing) already
    /// applied) — P5.3-pre prescan iterates REGIONS, not chunks.
    pub fn potential_chunk_for_region(&self, seed: i64, i: i32, j: i32) -> Option<(i32, i32)> {
        if self.spacing <= self.separation || self.spacing <= 0 {
            return None; // degenerate config — vanilla never ships one
        }
        let mut rng = LegacyRandomSource::new(0);
        rng.set_large_feature_with_salt(seed, i, j, self.salt);
        let d = self.spacing - self.separation;
        let (k, l) = match self.spread_type {
            SpreadType::Linear => (rng.next_int_bound(d), rng.next_int_bound(d)),
            SpreadType::Triangular => (
                (rng.next_int_bound(d) + rng.next_int_bound(d)) / 2,
                (rng.next_int_bound(d) + rng.next_int_bound(d)) / 2,
            ),
        };
        Some((i * self.spacing + k, j * self.spacing + l))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn villages_use_the_documented_salt() {
        // minecraft:villages: spacing 34, separation 8, salt 10387312
        let v = RandomSpreadStructurePlacement {
            spacing: 34,
            separation: 8,
            salt: 10387312,
            spread_type: SpreadType::Linear,
            frequency: 1.0,
            frequency_reduction_method: FrequencyReductionMethod::Default,
            exclusion_zone: None,
        };
        let a = v.potential_chunk(3053459, 5, 7).unwrap();
        let b = v.potential_chunk(3053459, 5, 7).unwrap();
        assert_eq!(a, b);
        // every chunk in the same 34-region reports the SAME potential chunk
        for dx in 0..34i32 {
            for dz in 0..34i32 {
                let (cx, cz) = v.potential_chunk(3053459, 34 + dx, 34 + dz).unwrap();
                assert!(cx >= 34 && cx < 68 && cz >= 34 && cz < 68);
                assert_eq!(
                    v.is_placement_chunk(3053459, 34 + dx, 34 + dz),
                    (34 + dx) == cx && (34 + dz) == cz
                );
            }
        }
    }

    #[test]
    fn salt_changes_positions() {
        let a = RandomSpreadStructurePlacement { spacing: 32, separation: 8, salt: 0, spread_type: SpreadType::Linear, frequency: 1.0, frequency_reduction_method: FrequencyReductionMethod::Default, exclusion_zone: None };
        let b = RandomSpreadStructurePlacement { spacing: 32, separation: 8, salt: 165545, spread_type: SpreadType::Linear, frequency: 1.0, frequency_reduction_method: FrequencyReductionMethod::Default, exclusion_zone: None };
        let mut differs = false;
        for seed in 0..50i64 {
            if a.potential_chunk(seed, 0, 0) != b.potential_chunk(seed, 0, 0) {
                differs = true;
                break;
            }
        }
        assert!(differs);
    }
}

#[cfg(test)]
mod freq_tests {
    use super::*;

    /// Oracle: java's legacyPillagerOutpostReducer at seed 90210, chunk (8,-9)
    /// must REJECT — the live server's goldenrefs (addendum 51 tick) shows no
    /// pillager_outpost start at (8,-9) (references = mineshaft only), and the
    /// independent python replication of java.util.Random agrees (draw False).
    #[test]
    fn outpost_frequency_rejects_90210_8_9() {
        let outpost = RandomSpreadStructurePlacement {
            spacing: 32,
            separation: 8,
            salt: 165745296,
            spread_type: SpreadType::Linear,
            frequency: 0.2,
            frequency_reduction_method: FrequencyReductionMethod::LegacyType1,
            exclusion_zone: Some(ExclusionZone {
                other_set: "minecraft:villages".to_string(),
                chunk_count: 10,
            }),
        };
        assert!(!outpost.frequency_allows(90210, 8, -9));
        assert!(!outpost.is_structure_chunk(90210, 8, -9, None));
    }
}
