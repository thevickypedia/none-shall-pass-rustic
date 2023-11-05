# none-shall-pass-rustic
Artifact for [none-shall-pass][3]

#### Summary
- [`none-shall-pass-rustic`][1] is an application written in Rust.
- This application validates hyperlinks in markdown files.
- It accepts inputs provided as command-line arguments.
- The application extracts hyperlinks from Markdown content, validates them concurrently using multithreading, and logs the validation results.
- It can differentiate between local Markdown files and Wiki pages within the repository, expanding its validation scope.
- Usage via GitHub actions can be found in [GitHub Marketplace][4]

#### Description
- Clones the GitHub wiki pages
- Looks up all the `*.md` files
- Scans for hyperlinks using regex (`inline`, `footnote`, and `anchored`)
- Sets exit code to 1, if any of the URL fails to resolve (`GET` request)
  - Ignores failure when the URL is a `localhost`
  - More exclusions can be via commandline arguments
- Takes argument `fail` to avoid failure
- Takes argument `debug` to print failure information on screen
- Takes argument `excludeHostnames` to exclude URLs to have known failures

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
