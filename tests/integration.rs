//! End-to-end behavioral spec for the public `semver-diff` API.
//!
//! Expected values are cross-checked against node-semver's `diff()`.

use semver::Version;
use semver_diff::{diff, diff_str, Difference};

fn v(s: &str) -> Version {
    Version::parse(s).unwrap()
}

// ---------------------------------------------------------------------------
// Release-level changes (no prerelease)
// ---------------------------------------------------------------------------

#[test]
fn release_level_diffs() {
    assert_eq!(diff(&v("1.0.0"), &v("1.0.1")), Some(Difference::Patch));
    assert_eq!(diff(&v("1.0.0"), &v("1.1.0")), Some(Difference::Minor));
    assert_eq!(diff(&v("1.0.0"), &v("2.0.0")), Some(Difference::Major));
    assert_eq!(diff(&v("1.2.3"), &v("1.2.4")), Some(Difference::Patch));
}

#[test]
fn equal_versions_are_none() {
    assert_eq!(diff(&v("1.2.3"), &v("1.2.3")), None);
    // Build metadata is ignored, just like semver comparison.
    assert_eq!(diff(&v("1.2.3+build.1"), &v("1.2.3+build.2")), None);
}

#[test]
fn diff_is_symmetric() {
    let pairs = [
        ("1.0.0", "2.3.4"),
        ("1.0.0-alpha", "1.0.0"),
        ("1.0.0", "1.1.0-rc.1"),
        ("1.2.0-alpha", "1.2.0"),
    ];
    for (a, b) in pairs {
        assert_eq!(diff(&v(a), &v(b)), diff(&v(b), &v(a)), "{a} vs {b}");
    }
}

// ---------------------------------------------------------------------------
// Going TO a prerelease → the `pre*` variants
// ---------------------------------------------------------------------------

#[test]
fn changes_to_a_prerelease() {
    assert_eq!(
        diff(&v("1.0.0"), &v("2.0.0-pre")),
        Some(Difference::PreMajor)
    );
    assert_eq!(
        diff(&v("1.0.0"), &v("1.1.0-pre")),
        Some(Difference::PreMinor)
    );
    assert_eq!(
        diff(&v("1.0.0"), &v("1.0.1-pre")),
        Some(Difference::PrePatch)
    );
    // same main version, both prereleases that differ → prerelease
    assert_eq!(
        diff(&v("1.0.0-alpha"), &v("1.0.0-beta")),
        Some(Difference::PreRelease)
    );
    assert_eq!(
        diff(&v("1.0.0-alpha.1"), &v("1.0.0-alpha.2")),
        Some(Difference::PreRelease)
    );
}

// ---------------------------------------------------------------------------
// Releasing FROM a prerelease (node-semver's quirky branch)
// ---------------------------------------------------------------------------

#[test]
fn releasing_from_a_prerelease() {
    // X.0.0-pre → X.0.0 is a major release
    assert_eq!(
        diff(&v("1.0.0-alpha"), &v("1.0.0")),
        Some(Difference::Major)
    );
    // X.Y.0-pre → X.Y.0 is a minor release
    assert_eq!(
        diff(&v("1.2.0-alpha"), &v("1.2.0")),
        Some(Difference::Minor)
    );
    // X.Y.Z-pre → X.Y.Z is a patch release
    assert_eq!(
        diff(&v("1.2.3-alpha"), &v("1.2.3")),
        Some(Difference::Patch)
    );
}

// Regression: when the MAIN version also changes, a prerelease→release diff must
// fall through to the standard comparison (matches live node-semver 7.7.2).
#[test]
fn releasing_from_a_prerelease_with_changed_main() {
    assert_eq!(diff(&v("1.2.3-pre"), &v("2.3.5")), Some(Difference::Major));
    assert_eq!(diff(&v("1.5.0-pre"), &v("2.3.0")), Some(Difference::Major));
    assert_eq!(diff(&v("1.2.0-pre"), &v("1.3.5")), Some(Difference::Minor));
    assert_eq!(diff(&v("1.0.5-pre"), &v("1.2.3")), Some(Difference::Minor));
    assert_eq!(diff(&v("0.0.1-0"), &v("1.0.1")), Some(Difference::Major));
    assert_eq!(diff(&v("0.0.1-0"), &v("1.1.0")), Some(Difference::Major));
    assert_eq!(diff(&v("0.0.1-0"), &v("0.1.1")), Some(Difference::Minor));
    assert_eq!(diff(&v("0.1.3-pre"), &v("1.2.0")), Some(Difference::Major));
    assert_eq!(diff(&v("0.1.0-pre"), &v("0.2.5")), Some(Difference::Minor));
}

// ---------------------------------------------------------------------------
// Difference helpers + string form (matches node-semver)
// ---------------------------------------------------------------------------

#[test]
fn difference_as_str() {
    assert_eq!(Difference::Major.as_str(), "major");
    assert_eq!(Difference::PreMajor.as_str(), "premajor");
    assert_eq!(Difference::Minor.as_str(), "minor");
    assert_eq!(Difference::PreMinor.as_str(), "preminor");
    assert_eq!(Difference::Patch.as_str(), "patch");
    assert_eq!(Difference::PrePatch.as_str(), "prepatch");
    assert_eq!(Difference::PreRelease.as_str(), "prerelease");
}

#[test]
fn difference_is_prerelease() {
    assert!(!Difference::Major.is_prerelease());
    assert!(!Difference::Patch.is_prerelease());
    assert!(Difference::PreMajor.is_prerelease());
    assert!(Difference::PreRelease.is_prerelease());
}

// ---------------------------------------------------------------------------
// diff_str convenience
// ---------------------------------------------------------------------------

#[test]
fn diff_str_parses_and_diffs() {
    assert_eq!(diff_str("1.0.0", "1.0.1").unwrap(), Some(Difference::Patch));
    assert_eq!(diff_str("1.0.0", "1.0.0").unwrap(), None);
    assert!(diff_str("not-a-version", "1.0.0").is_err());
}
