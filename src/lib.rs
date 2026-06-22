//! # semver-diff — the release-type difference between two versions
//!
//! Given two [`semver::Version`]s, report *what kind* of change separates them — a
//! major, minor, or patch bump, or one of their prerelease variants. A faithful Rust
//! port of [node-semver](https://github.com/npm/node-semver)'s `diff` (and the
//! [`semver-diff`](https://www.npmjs.com/package/semver-diff) npm package), useful
//! for update notifiers, changelog tooling, and dependency dashboards.
//!
//! ```
//! use semver::Version;
//! use semver_diff::{diff, Difference};
//!
//! assert_eq!(diff(&Version::new(1, 0, 0), &Version::new(1, 0, 1)), Some(Difference::Patch));
//! assert_eq!(diff(&Version::new(1, 0, 0), &Version::new(2, 0, 0)), Some(Difference::Major));
//! assert_eq!(diff(&Version::new(1, 2, 3), &Version::new(1, 2, 3)), None);
//! ```
//!
//! Or work straight from strings with [`diff_str`]:
//!
//! ```
//! assert_eq!(semver_diff::diff_str("1.0.0", "1.1.0-rc.1").unwrap(), Some(semver_diff::Difference::PreMinor));
//! ```

#![doc(html_root_url = "https://docs.rs/semver-diff/0.1.0")]

// Compile-test the README's examples as part of `cargo test`.
#[cfg(doctest)]
#[doc = include_str!("../README.md")]
struct ReadmeDoctests;

use core::cmp::Ordering;
use semver::{BuildMetadata, Version};

/// The kind of change between two versions.
///
/// The `Pre*` variants mean the *higher* version is a prerelease. [`Difference::as_str`]
/// renders the same lowercase names node-semver uses (`"major"`, `"premajor"`, …). The
/// derived `Ord` follows declaration order (`Major < PreMajor < Minor < … < PreRelease`);
/// it is a stable total order, not a ranking of "impact".
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Difference {
    /// A major bump (`1.x.y` → `2.0.0`).
    Major,
    /// A major bump to a prerelease (`1.0.0` → `2.0.0-rc`).
    PreMajor,
    /// A minor bump (`1.0.x` → `1.1.0`).
    Minor,
    /// A minor bump to a prerelease (`1.0.0` → `1.1.0-rc`).
    PreMinor,
    /// A patch bump (`1.0.0` → `1.0.1`).
    Patch,
    /// A patch bump to a prerelease (`1.0.0` → `1.0.1-rc`).
    PrePatch,
    /// Only the prerelease identifiers differ (`1.0.0-a` → `1.0.0-b`).
    PreRelease,
}

impl Difference {
    /// The node-semver name for this difference (`"major"`, `"premajor"`, `"minor"`,
    /// `"preminor"`, `"patch"`, `"prepatch"`, `"prerelease"`).
    #[must_use]
    pub const fn as_str(self) -> &'static str {
        match self {
            Difference::Major => "major",
            Difference::PreMajor => "premajor",
            Difference::Minor => "minor",
            Difference::PreMinor => "preminor",
            Difference::Patch => "patch",
            Difference::PrePatch => "prepatch",
            Difference::PreRelease => "prerelease",
        }
    }

    /// Whether this is one of the prerelease variants (`PreMajor`, `PreMinor`,
    /// `PrePatch`, `PreRelease`).
    #[must_use]
    pub const fn is_prerelease(self) -> bool {
        matches!(
            self,
            Difference::PreMajor
                | Difference::PreMinor
                | Difference::PrePatch
                | Difference::PreRelease
        )
    }
}

/// The release-type difference between `a` and `b`, or `None` if they compare equal.
///
/// The result is symmetric (`diff(a, b) == diff(b, a)`) and, like semver comparison,
/// ignores build metadata. Mirrors node-semver's `diff`.
///
/// ```
/// use semver::Version;
/// use semver_diff::{diff, Difference};
/// assert_eq!(diff(&Version::new(1, 0, 0), &Version::new(1, 1, 0)), Some(Difference::Minor));
/// ```
#[must_use]
pub fn diff(a: &Version, b: &Version) -> Option<Difference> {
    let ordering = cmp_ignoring_build(a, b);
    if ordering == Ordering::Equal {
        return None;
    }

    let (high, low) = if ordering == Ordering::Greater {
        (a, b)
    } else {
        (b, a)
    };
    let high_has_pre = !high.pre.is_empty();
    let low_has_pre = !low.pre.is_empty();

    // Stabilizing a prerelease into a release. Following node-semver: a prerelease of
    // a clean major (`X.0.0-pre`) becomes a major release; if the main version is
    // otherwise unchanged, the prerelease's own shape decides; and if the main version
    // *did* change, fall through to the standard component comparison below.
    if low_has_pre && !high_has_pre {
        if low.minor == 0 && low.patch == 0 {
            return Some(Difference::Major);
        }
        if low.major == high.major && low.minor == high.minor && low.patch == high.patch {
            if low.minor != 0 && low.patch == 0 {
                return Some(Difference::Minor);
            }
            return Some(Difference::Patch);
        }
    }

    // Otherwise, the most significant changed component decides, with a `pre` prefix
    // when the higher version is itself a prerelease.
    if a.major != b.major {
        return Some(if high_has_pre {
            Difference::PreMajor
        } else {
            Difference::Major
        });
    }
    if a.minor != b.minor {
        return Some(if high_has_pre {
            Difference::PreMinor
        } else {
            Difference::Minor
        });
    }
    if a.patch != b.patch {
        return Some(if high_has_pre {
            Difference::PrePatch
        } else {
            Difference::Patch
        });
    }

    // Same major.minor.patch, but not equal → only the prerelease tags differ.
    Some(Difference::PreRelease)
}

/// Order two versions by SemVer precedence, ignoring build metadata (which the
/// `semver` crate otherwise treats as a tiebreaker, unlike node-semver and the spec).
fn cmp_ignoring_build(a: &Version, b: &Version) -> Ordering {
    if a.build == b.build {
        a.cmp(b)
    } else {
        let a = Version {
            build: BuildMetadata::EMPTY,
            ..a.clone()
        };
        let b = Version {
            build: BuildMetadata::EMPTY,
            ..b.clone()
        };
        a.cmp(&b)
    }
}

/// Parse `a` and `b` as [`semver::Version`]s and return their [`diff`].
///
/// # Errors
///
/// Returns the [`semver::Error`] from the first input that fails to parse.
pub fn diff_str(a: &str, b: &str) -> Result<Option<Difference>, semver::Error> {
    Ok(diff(&Version::parse(a)?, &Version::parse(b)?))
}
