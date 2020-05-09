# minigrep
A minimalistic grep with rust.

### To build:

    cargo build --release
    cargo test

You'll find minigrep in `target/release/minigrep`

### Usage:

`minigrep <pattern> <filename> [case]`

case is added if the search is case sensistive

### Example:
`minigrep Ram file_to_match.txt case`





