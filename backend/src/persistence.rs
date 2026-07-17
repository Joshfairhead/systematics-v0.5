//! Persistence for the writable user store.
//!
//! Canonical content ships bundled (see `data::canonical_content`); this module
//! handles the *user* slice — perspectives, semantic vocabularies, characters, and
//! coordinates a user creates at runtime. It serialises to a JSON file (default
//! `./data/store.json`, overridable via `SYSTEMATICS_STORE`) using the same
//! `GraphContent` shape as the canonical seed.

use std::path::{Path, PathBuf};

use crate::core::{Graph, GraphContent};

/// Default store path, relative to the process working directory.
const DEFAULT_STORE_PATH: &str = "./data/store.json";

/// Resolve the user store path: `SYSTEMATICS_STORE` if set, else the default.
pub fn resolve_store_path() -> PathBuf {
    std::env::var("SYSTEMATICS_STORE")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from(DEFAULT_STORE_PATH))
}

/// Load the user store (if it exists) and apply it over the graph. Missing file
/// is fine (fresh start). Returns the number of items applied.
pub fn load_into(graph: &mut Graph, path: &Path) -> std::io::Result<usize> {
    if !path.exists() {
        tracing::info!("No user store at {}; starting from canonical only", path.display());
        return Ok(0);
    }
    let raw = std::fs::read_to_string(path)?;
    let content: GraphContent = serde_json::from_str(&raw).map_err(|e| {
        std::io::Error::new(
            std::io::ErrorKind::InvalidData,
            format!("store at {} is not valid GraphContent: {}", path.display(), e),
        )
    })?;
    let count = content.ids().len();
    graph.apply_content(&content);
    tracing::info!("Loaded {} user item(s) from {}", count, path.display());
    Ok(count)
}

/// Persist the graph's user slice (everything not canonical) to the store path.
/// Creates the parent directory if needed. A no-op-safe write on every mutation.
pub fn save(graph: &Graph, path: &Path) -> std::io::Result<()> {
    let content = graph.user_content();
    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }
    let json = serde_json::to_string_pretty(&content).map_err(|e| {
        std::io::Error::new(std::io::ErrorKind::InvalidData, e.to_string())
    })?;
    std::fs::write(path, json)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::{Character, Perspective, SemanticVocabulary};
    use crate::data;

    fn temp_path(name: &str) -> PathBuf {
        // Unique-ish per test name; std::env::temp_dir avoids polluting the repo.
        std::env::temp_dir().join(format!("systematics_test_{}.json", name))
    }

    #[test]
    fn test_save_then_load_roundtrip() {
        let path = temp_path("roundtrip");
        let _ = std::fs::remove_file(&path);

        // Build canonical graph, add a user perspective + its deps.
        let mut graph = data::build_graph();
        graph.upsert_character(Character::new("char_word_immanent", "word", "Immanent"));
        graph.upsert_character(Character::new("char_word_omniscient", "word", "Omniscient"));
        graph.upsert_character(Character::new(
            "char_word_transcendental",
            "word",
            "Transcendental",
        ));
        graph.add_semantic_vocab(SemanticVocabulary::new(
            "semvocab_theology_triad_3",
            "Theology Triad",
            3,
            vec![
                "char_word_immanent".into(),
                "char_word_omniscient".into(),
                "char_word_transcendental".into(),
            ],
            vec![
                "char_word_generation".into(),
                "char_word_decision".into(),
                "char_word_consent".into(),
            ],
        ));
        graph.add_perspective(Perspective::new(
            "perspective_theology_triad_3",
            "Theology Triad",
            3,
            "Trinity",
            "Persons",
            "Perichoresis",
            "topvocab_3",
            "geovocab_3",
            "semvocab_theology_triad_3",
        ));

        // The user slice excludes canonical (Will/Function/Being stay canonical).
        let user = graph.user_content();
        assert!(user.perspectives.iter().any(|g| g.id == "perspective_theology_triad_3"));
        assert!(user.characters.iter().any(|c| c.id == "char_word_immanent"));
        assert!(
            !user.characters.iter().any(|c| c.id == "char_word_will"),
            "canonical characters must not be in the user slice"
        );

        save(&graph, &path).unwrap();

        // Fresh canonical graph + load store → theology perspective reappears.
        let mut reloaded = data::build_graph();
        assert!(reloaded.perspective("perspective_theology_triad_3").is_none());
        let n = load_into(&mut reloaded, &path).unwrap();
        assert!(n >= 5); // 3 chars + 1 semvocab + 1 perspective
        assert!(reloaded.perspective("perspective_theology_triad_3").is_some());
        assert!(reloaded.validate_perspective("perspective_theology_triad_3").is_ok());
        assert_eq!(
            reloaded
                .character_at_point("semvocab_theology_triad_3", "point_3_2")
                .unwrap()
                .value,
            "Omniscient"
        );

        let _ = std::fs::remove_file(&path);
    }

    #[test]
    fn test_load_missing_file_is_ok() {
        let path = temp_path("missing");
        let _ = std::fs::remove_file(&path);
        let mut graph = data::build_graph();
        let n = load_into(&mut graph, &path).unwrap();
        assert_eq!(n, 0);
    }
}
