# none-shall-pass-rustic

[![made-with-rust](https://img.shields.io/badge/Made%20with-Rust-black?style=for-the-badge&logo=Rust)][rust]

[![build](https://github.com/thevickypedia/none-shall-pass-rustic/actions/workflows/rust.yml/badge.svg)][build]

Artifact for [none-shall-pass][3]

#### Summary
- [`none-shall-pass-rustic`][1] is an application written in Rust.
- Validates hyperlinks in markdown files.
- Accepts inputs provided as command-line arguments.
- Extracts hyperlinks from Markdown content, validates them concurrently using multithreading, and logs the validation results.
- Differentiates between local Markdown files and Wiki pages within the repository, expanding its validation scope.
- Usage via GitHub actions can be found in [GitHub Marketplace][4]

#### Description
- Clones the GitHub wiki pages
- Looks up all the `*.md` files
- Scans for hyperlinks using regex (`inline`, `footnote`, and `anchored`)
- Sets exit code to 1, if any of the URL fails to resolve (`GET` request)
  - Ignores failure when the URL is a `localhost`
  - More exclusions can be added via commandline argument
- Takes argument `debug` _(as boolean)_ to enable debug level logging
- Takes argument `excludeHostnames` _(as comma separated list)_ to exclude URLs to have known failures

## Crate
https://crates.io/crates/none-shall-pass

## Linting
### Requirement
```shell
rustup component add clippy
```
### Usage
```shell
cargo clippy --no-deps --fix --allow-dirty
```

## License & copyright

&copy; Vignesh Rao

Licensed under the [MIT License][2]

[1]: https://github.com/thevickypedia/none-shall-pass-rustic
[2]: https://github.com/thevickypedia/none-shall-pass-rustic/blob/main/LICENSE
[3]: https://github.com/thevickypedia/none-shall-pass
[4]: https://github.com/marketplace/actions/none-shall-pass
[build]: https://github.com/thevickypedia/none-shall-pass-rustic/actions/workflows/rust.yml
[rust]: https://www.rust-lang.org/
