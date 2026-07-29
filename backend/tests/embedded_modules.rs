//! Deploy-readiness (composable-perspectives plan, Step 4).
//!
//! A deployed binary has no source tree on disk, so `load_perspective_modules`
//! must fall back to the copy of the modules embedded via `include_dir!`. This
//! test forces that path by pointing `SYSTEMATICS_DATA` at a directory that does
//! not exist, then asserts the full corpus (15 modules, 75 references, the 17
//! module-owned systems) is present from the binary alone — no filesystem, no
//! working-directory assumption, no volume.
//!
//! Lives in its own test binary so its process-global env var cannot race with
//! the filesystem-path tests in the other integration files.

use systematics_backend::data;

#[test]
fn embedded_modules_are_complete_without_a_filesystem() {
    // SAFETY: edition 2021, single-threaded within this one-test binary.
    std::env::set_var("SYSTEMATICS_DATA", "/systematics-nonexistent-path");

    let mut graph = data::build_graph();
    let modules = data::load_perspective_modules(&mut graph);
    if modules > 0 {
        graph.mark_bundled();
    }

    assert_eq!(
        modules, 15,
        "the deployed binary must serve all 15 modules from the embedded copy"
    );
    assert_eq!(
        graph.references.len(),
        75,
        "all 75 references must be embedded in the binary"
    );
    assert_eq!(
        graph.perspectives().len(),
        15,
        "all 15 perspectives must be embedded in the binary"
    );
    assert!(
        graph.system("system_dramatic_universe_i_triad_3").is_some(),
        "a module-owned system must be present from the embedded copy"
    );

    std::env::remove_var("SYSTEMATICS_DATA");
}
