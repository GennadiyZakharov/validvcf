# validvcf
A fast and simple VCF validator

This repository now contains a Rust command-line tool template.

## Prerequisites
- Rust toolchain (install via https://rustup.rs)

## Build
- Debug: `cargo build`
- Release: `cargo build --release`

## Run
- Show help: `cargo run -- --help`
- Show version: `cargo run -- --version`
- Validate a file (placeholder): `cargo run -- path/to/file.vcf`
- Silence output with `-q`.

## Binary
After a release build, the binary will be at `target/release/validvcf`.

## Next steps
- Replace the placeholder logic in `src/main.rs` with real VCF validation.
- Add tests under `tests/` or unit tests within `src/` as functionality evolves.
