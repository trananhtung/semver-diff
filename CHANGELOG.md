# Changelog

All notable changes to this project are documented here. The format is based on
[Keep a Changelog](https://keepachangelog.com/en/1.1.0/), and this project
adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [0.1.0] - 2026-06-22

### Added

- Initial release.
- `diff(a, b)` — the release-type [`Difference`] between two `semver::Version`s, or
  `None` if they compare equal (build metadata ignored, symmetric).
- `diff_str(a, b)` — parse two `&str` versions and diff them.
- `Difference` enum (`Major` / `PreMajor` / `Minor` / `PreMinor` / `Patch` /
  `PrePatch` / `PreRelease`) with `as_str` (node-semver names) and `is_prerelease`.
- Behavior matches node-semver's `diff`, including the prerelease-to-release logic.

[0.1.0]: https://github.com/trananhtung/semver-diff/releases/tag/v0.1.0
