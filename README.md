# none-shall-pass-rustic
Artifact for [none-shall-pass][3]

#### Summary
- [`none-shall-pass-rustic`][1] is an application to validate hyperlinks in markdown files.

#### Description
- Clones the GitHub wiki pages
- Looks up all the `*.md` files
- Scans for hyperlinks using regex (`inline`, `footnote`, and `anchored`)
- Sets exit code to 1, if any of the URL fails to resolve (`GET` request)
  - Ignores failure when the URL is a `localhost`
  - More exclusions can be via commandline arguments
- Takes argument `fail` to avoid failure
- Takes argument `debug` to print failure information on screen

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
