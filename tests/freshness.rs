//! Freshness compares the complete Rust projection from the pinned producer,
//! not the contract's source digest. Mutations stay in memory.

use ethos_zero::{Actualizing, File, Generating, Potential};

const SOURCE: &str = include_str!("../ethos/signal.ethos");
const ARTIFACT: &str = include_str!("../src/generated/signal.rs");

#[test]
fn both_committed_artifacts_are_the_pinned_producers_complete_output() {
    let library = Potential::<File>::from(include_str!("../ethos/library.ethos").to_owned())
        .actualize()
        .expect("read authored Library");
    assert_eq!(
        library.generate().expect("generate authored Library"),
        include_str!("../src/generated/library.rs"),
    );
    let file = Potential::<File>::from(SOURCE.to_owned())
        .actualize()
        .expect("read authored Signal");
    let generated = file.generate().expect("generate authored Signal");
    assert_eq!(generated, ARTIFACT);
}

#[test]
fn a_changed_authored_type_makes_the_committed_artifact_stale() {
    let source = SOURCE.replacen("ArtifactPath.String", "ArtifactPath.Integer", 1);
    assert_ne!(source, SOURCE, "the source mutation must take effect");
    let file = Potential::<File>::from(source)
        .actualize()
        .expect("read changed Signal");
    let generated = file.generate().expect("generate changed Signal");
    assert_ne!(generated, ARTIFACT, "old Rust must not pass freshness");
}

#[test]
fn a_changed_rust_type_is_stale_even_with_unchanged_authored_source() {
    let artifact = ARTIFACT.replacen(
        "pub type ArtifactPath = String;",
        "pub type ArtifactPath = bool;",
        1,
    );
    assert_ne!(artifact, ARTIFACT, "the artifact mutation must take effect");
    let file = Potential::<File>::from(SOURCE.to_owned())
        .actualize()
        .expect("read unchanged Signal");
    let generated = file.generate().expect("generate unchanged Signal");
    assert_eq!(generated, ARTIFACT, "the original artifact is fresh");
    assert_ne!(generated, artifact, "edited Rust must not pass freshness");
}

#[test]
fn a_changed_library_type_makes_the_committed_library_stale() {
    let source = include_str!("../ethos/library.ethos");
    let changed = source.replacen("SourceName.String", "SourceName.Integer", 1);
    assert_ne!(changed, source, "the library mutation must take effect");
    let file = Potential::<File>::from(changed)
        .actualize()
        .expect("read changed Library");
    assert_ne!(
        file.generate().expect("generate changed Library"),
        include_str!("../src/generated/library.rs"),
        "old Library Rust must not pass freshness",
    );
}

#[test]
fn a_changed_library_artifact_is_stale_with_unchanged_authored_library() {
    let artifact = include_str!("../src/generated/library.rs");
    let changed = artifact.replacen(
        "pub type SourceName = String;",
        "pub type SourceName = bool;",
        1,
    );
    assert_ne!(
        changed, artifact,
        "the library artifact mutation must take effect"
    );
    let file = Potential::<File>::from(include_str!("../ethos/library.ethos").to_owned())
        .actualize()
        .expect("read unchanged Library");
    let generated = file.generate().expect("generate unchanged Library");
    assert_eq!(generated, artifact);
    assert_ne!(
        generated, changed,
        "edited Library Rust must not pass freshness"
    );
}
