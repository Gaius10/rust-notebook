# Controlling test workflow

## General syntax:

`cargo test <cargo_test_flags> -- <flags_of_created_binary>`

## Running tests syncronously:

`cargo test -- --test-threads=1`

## Outputs:

By default, Rust shows only failed tests outputs.
We can change it with: `cargo test -- --show-output`

## Running single tests:

`cargo test <test_name>`
try `cargo test one_hundred` in this repository

## Filtering tests:

try `cargo test add`

## Running ignored tests:

`cargo test -- --ignored`
`cargo test -- --include-ignored`

