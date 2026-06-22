# semver-diff

[![All Contributors](https://img.shields.io/badge/all_contributors-1-orange.svg?style=flat-square)](#contributors-)

[![Crates.io](https://img.shields.io/crates/v/semver-diff.svg)](https://crates.io/crates/semver-diff)
[![Documentation](https://docs.rs/semver-diff/badge.svg)](https://docs.rs/semver-diff)
[![CI](https://github.com/trananhtung/semver-diff/actions/workflows/ci.yml/badge.svg)](https://github.com/trananhtung/semver-diff/actions/workflows/ci.yml)
[![License](https://img.shields.io/crates/l/semver-diff.svg)](#license)

**Find the release-type difference between two semantic versions** — was it a major,
minor, or patch change (or one of their prerelease variants)? A faithful Rust port of
[node-semver](https://github.com/npm/node-semver)'s `diff` and the
[`semver-diff`](https://www.npmjs.com/package/semver-diff) npm package, built on the
[`semver`](https://crates.io/crates/semver) crate.

```rust
use semver::Version;
use semver_diff::{diff, Difference};

assert_eq!(diff(&Version::new(1, 0, 0), &Version::new(1, 0, 1)), Some(Difference::Patch));
assert_eq!(diff(&Version::new(1, 0, 0), &Version::new(2, 0, 0)), Some(Difference::Major));
assert_eq!(diff(&Version::new(1, 2, 3), &Version::new(1, 2, 3)), None);

// Or straight from strings:
assert_eq!(semver_diff::diff_str("1.0.0", "1.1.0-rc.1").unwrap(), Some(Difference::PreMinor));
```

## Why semver-diff?

The `semver` crate parses and compares versions, but doesn't tell you *what kind* of
bump separates two of them — the thing update notifiers, changelog generators, and
dependency dashboards actually want to show ("this is a **minor** update"). The
prerelease logic is subtle, so this is a small, well-tested port that matches
node-semver exactly.

```toml
[dependencies]
semver-diff = "0.1"
```

## API

| Item | Purpose |
| --- | --- |
| `diff(a, b)` | `Option<Difference>` between two `Version`s (`None` if equal) |
| `diff_str(a, b)` | Parse two `&str` versions and diff them |
| `Difference` | `Major` / `PreMajor` / `Minor` / `PreMinor` / `Patch` / `PrePatch` / `PreRelease` |
| `Difference::as_str()` | The node-semver name (`"major"`, `"premajor"`, …) |
| `Difference::is_prerelease()` | Whether it's one of the `Pre*` variants |

## Behavior

- The most significant changed component decides the result; a `Pre*` variant is
  returned when the **higher** version is itself a prerelease.
- Stabilizing a prerelease (`1.2.0-rc` → `1.2.0`) reports the release it became
  (`Minor` here), matching node-semver.
- The result is symmetric (`diff(a, b) == diff(b, a)`) and ignores build metadata,
  just like SemVer precedence.

## Contributors ✨

This project follows the [all-contributors](https://github.com/all-contributors/all-contributors) specification. Contributions of any kind are welcome — code, docs, bug reports, ideas, reviews! See the [emoji key](https://allcontributors.org/docs/en/emoji-key) for how each contribution is recognized, and open a PR or issue to get involved.

Thanks goes to these wonderful people:

<!-- ALL-CONTRIBUTORS-LIST:START - Do not remove or modify this section -->
<!-- prettier-ignore-start -->
<!-- markdownlint-disable -->
<table>
  <tbody>
    <tr>
      <td align="center" valign="top" width="14.28%"><a href="https://github.com/trananhtung"><img src="https://avatars.githubusercontent.com/u/30992229?v=4?s=100" width="100px;" alt="Tung Tran"/><br /><sub><b>Tung Tran</b></sub></a><br /><a href="https://github.com/trananhtung/semver-diff/commits?author=trananhtung" title="Code">💻</a> <a href="#maintenance-trananhtung" title="Maintenance">🚧</a></td>
    </tr>
  </tbody>
</table>

<!-- markdownlint-restore -->
<!-- prettier-ignore-end -->

<!-- ALL-CONTRIBUTORS-LIST:END -->

## License

Licensed under either of [Apache-2.0](LICENSE-APACHE) or [MIT](LICENSE-MIT) at
your option.
