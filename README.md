# privacy-log

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
