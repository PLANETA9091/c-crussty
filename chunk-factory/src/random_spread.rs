use crate::jrandom::{LegacyRandomSource, RandomSource};

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum SpreadType {
    Linear,
    Triangular,
}

#[derive(Clone, Copy, Debug)]
pub struct RandomSpreadStructurePlacement {
    pub spacing: i32,
    pub separation: i32,
    pub salt: i32,
    pub spread_type: SpreadType,
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
        })
    }

    /// RandomSpreadStructurePlacement.isPlacementChunk — the prescreen.
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
        let a = RandomSpreadStructurePlacement { spacing: 32, separation: 8, salt: 0, spread_type: SpreadType::Linear };
        let b = RandomSpreadStructurePlacement { spacing: 32, separation: 8, salt: 165545, spread_type: SpreadType::Linear };
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
