# reloaded3_common_hash_cache

## Additional Setup Required!!

<!-- DELETE THIS SECTION AFTER COMPLETING SETUP -->

Additional setup might be required.
Once you do the stuff under this text, delete this section.

### Code Coverage
To setup code coverage for this project:

1. Go to [Codecov][codecov], add your project.
2. Go to [Settings -> Secrets and variables -> Actions][gh-actions-secrets] in your repo and add a repository secret named `CODECOV_TOKEN`.
3. Do the same in [Settings -> Secrets and variables -> Dependabot][gh-actions-dependabot].

The instructions on the Codecov page will provide the token to use.

### Setup Documentation

Before you make your first commit to GitHub, you'll need to go to [Settings -> Pages][gh-pages] and set
`Source` to `GitHub Actions` for the documentation builds to work.




### Enable Auto-Delete Merged Branches

Enable automatic deletion of merged branches:

```bash
gh repo edit --delete-branch-on-merge
```

Or via UI: **Settings > General > check "Automatically delete head branches"**

### Fill in README Documentation

This repository uses a two-level README structure:

| File | Purpose | What to Write |
|------|---------|---------------|
| `README.MD` (this file) | GitHub landing page | Project overview, quick start example, badges |
| [`src/reloaded3_common_hash_cache/README.MD`](./src/reloaded3_common_hash_cache/README.MD) | crates.io page | Detailed features, installation, comprehensive usage examples |

-----------------------

[![Crates.io](https://img.shields.io/crates/v/reloaded3_common_hash_cache.svg)](https://crates.io/crates/reloaded3_common_hash_cache)
[![Docs.rs](https://docs.rs/reloaded3_common_hash_cache/badge.svg)](https://docs.rs/reloaded3_common_hash_cache)
[![CI](https://github.com/TheBestAstroNOT/reloaded3_common_hash_cache/actions/workflows/rust.yml/badge.svg)](https://github.com/TheBestAstroNOT/reloaded3_common_hash_cache/actions)

A hash cache to verify integrity of mod packages

## Quick Start

Add to your `Cargo.toml`:

```toml
[dependencies]
reloaded3_common_hash_cache = "0.1.0"
```

```rust
// TODO: Add a minimal example showing the most common use case
```
For detailed usage guides, see the [documentation][docs].

## Crates

- [reloaded3_common_hash_cache](./src/reloaded3_common_hash_cache/README.MD): Core library <!-- TODO: Update description -->

## Developer Manual

For step-by-step development guidance, see the [Developer Manual](https://reloaded-project.github.io/reloaded-templates-rust/manual/).

## Contributing

We welcome contributions! See the [Contributing Guide](https://reloaded-project.github.io/reloaded-templates-rust/manual/#contributing) for details.

## License

Licensed under [](./LICENSE).

[codecov]: https://app.codecov.io
[gh-actions-secrets]: https://github.com/TheBestAstroNOT/reloaded3_common_hash_cache/settings/secrets/actions
[gh-actions-dependabot]: https://github.com/TheBestAstroNOT/reloaded3_common_hash_cache/settings/secrets/dependabot
[gh-pages]: https://github.com/TheBestAstroNOT/reloaded3_common_hash_cache/settings/pages
[crates-io-key]: https://crates.io/settings/tokens
[nuget-key]: https://www.nuget.org/account/apikeys
[docs]: https://reloaded3.common.hash.cache.github.io/reloaded3.common.hash.cache
