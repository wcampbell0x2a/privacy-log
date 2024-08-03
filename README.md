# privacy-log
[<img alt="github" src="https://img.shields.io/badge/github-wcampbell0x2a/privacy_log-8da0cb?style=for-the-badge&labelColor=555555&logo=github" height="20">](https://github.com/wcampbell0x2a/privacy-log)
[<img alt="crates.io" src="https://img.shields.io/crates/v/privacy-log.svg?style=for-the-badge&color=fc8d62&logo=rust" height="20">](https://crates.io/crates/privacy-log)
[<img alt="docs.rs" src="https://img.shields.io/badge/docs.rs-privacy_log-66c2a5?style=for-the-badge&labelColor=555555&logo=docs.rs" height="20">](https://docs.rs/privacy-log)
[<img alt="build status" src="https://img.shields.io/github/actions/workflow/status/wcampbell0x2a/privacy-log/main.yml?branch=master&style=for-the-badge" height="20">](https://github.com/wcampbell0x2a/privacy-log/actions?query=branch%3Amaster)

[rust-lang/log](https://github.com/rust-lang/log), but without file names and line information.

## Usage
### In libraries
* The `privacy-log` macros *still* uses `log` internals, you will need to `$ cargo add log` to use this crate.
* Follow https://github.com/rust-lang/log?tab=readme-ov-file#in-libraries, but replace `log` with `privacy-log`.

### In executables
* The `privacy-log` macros *still* uses `log` internals, you will need to `$ cargo add log` to use this crate.
* Follow https://github.com/rust-lang/log?tab=readme-ov-file#in-executables, but replace `log` with `privacy-log`.

## Compile Time Filters
After adding `log`, you may set [Compile Time Filters](https://docs.rs/log/latest/log/#compile-time-filters) to further limit output for different profiles.

## Other Differences
* No support for `kv` feature
