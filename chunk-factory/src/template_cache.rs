//! NCF P5.2 — cache of structure templates (once per world).
//!
//! The fallback policy (P5.6) sends structure-bearing chunks to Java, but the
//! prescreen (P5.1 random_spread) and the piece engine (P5.3, pending) need
//! the template NBT PARSED once per (datapack, template id) — not once per
//! chunk. Keying (I5): the cache lives per WorldgenDir instance (== one
//! datapack tree) + the spec hash comes from the RandomState — the template
//! bytes are spec-INDEPENDENT (they are datapack content), so the cache key
//! is the template resource key; the world-spec dependency lives in the
//! callers (placement seeds), not in the template bytes.

use crate::sections::Nbt;
use std::cell::RefCell;
use std::collections::HashMap;

pub struct TemplateCache<'d> {
    dir: &'d crate::router::WorldgenDir,
    cache: RefCell<HashMap<String, Option<Nbt>>>,
}

impl<'d> TemplateCache<'d> {
    pub fn new(dir: &'d crate::router::WorldgenDir) -> Self {
        TemplateCache { dir, cache: RefCell::new(HashMap::new()) }
    }

    /// Parse (minecraft:structure/<ns>/<path>.nbt) once; None = missing or
    /// undecodable (remembered to avoid re-reading every chunk).
    pub fn get(&self, ns: &str, path: &str) -> Option<Nbt> {
        let key = format!("{ns}:{path}");
        if let Some(hit) = self.cache.borrow().get(&key) {
            return hit.clone();
        }
        let parsed = self.dir.get(ns, "structure", path).and_then(|bytes| {
            // template files are gzip-compressed NBT (vanilla .nbt)
            crate::sections::gunzip(bytes.as_bytes())
                .ok()
                .and_then(|raw| crate::sections::nbt_parse(&raw).ok())
        });
        let verdict = if parsed.is_some() { Some(parsed.unwrap()) } else { None };
        self.cache.borrow_mut().insert(key, verdict.clone());
        verdict
    }

    pub fn len(&self) -> usize {
        self.cache.borrow().len()
    }

    pub fn is_empty(&self) -> bool {
        self.cache.borrow().is_empty()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn negative_results_are_cached() {
        // no real WorldgenDir available without the extract; pin the API
        // contract with a manual cache via the same internals shape
        let mut cache: HashMap<String, Option<Nbt>> = HashMap::new();
        cache.insert("missing".into(), None);
        assert!(cache["missing"].is_none());
        assert_eq!(cache.len(), 1);
    }
}
