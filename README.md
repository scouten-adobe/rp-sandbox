# Sandbox where we can play with release-plz

## Important links

* [release-plz: Introduction](https://release-plz.ieni.dev/docs)
* [Conventional Commits: Summary](https://www.conventionalcommits.org/en/v1.0.0/#summary)
* [release-plz: How should I write my commits?](https://release-plz.ieni.dev/docs/changelog/format#how-should-i-write-my-commits)
* [release-plz: FAQ](https://release-plz.ieni.dev/docs/faq)

## tl;dr: Mapping from our existing behavior to new

| Old | New |
| --- | --- |
| Commit message: `(MINOR) Do something` | `feat: Do something` |
| Commit message: `(IGNORE) Change build system` | `chore: Change build system` |
| Release: Find Action and trigger it manually | Release by merging release PR |
| Changelog: Never ever edit it | OK to edit (*) |

(*) Note that the changelog will be rewritten (force-pushed) if the release PR branch gets updated (i.e. something new gets pushed to `main`).

As a back-up strategy, `release-plz` uses `cargo-semver-checks` to look for API enhancements or breaking changes and will bump the crate version accordingly even if the commits were mis-categorized.

## NOTE: Custom fork

The current published version of `release-plz` follows a Conventional Commits behavior that is at odds with how our team has used semantic versioning:

When making a **feature** addition to an existing crate **with a 0.x.y version number,** the published version of `release-plz` will treat that as a **patch** version bump (i.e. 0.x.y+1). We have been treating those as a **minor** version bump (i.e. 0.x+1.0).

I've submitted a pull request to change this [MarcoIeni/release-plz#1657: Add `features_always_increment_minor` flag](https://github.com/MarcoIeni/release-plz/pull/1657). Hopefully that gets merged someday; until then, I've figured out how to run our releases using a fork.
