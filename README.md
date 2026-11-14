# Binary Operations

A small Rust command-line program for experimenting with binary sequences and logical operations. Choose an operation, enter two sequences of bits, and see the result.

> **Learning project:** This project was made for learning purposes by a beginner Rust learner. AI suggestions were used during development. The documentation and this README were also created with AI assistance to save time, so they may not always reflect the author's own wording or understanding. The code is part of an ongoing learning process.

## What it does

The program supports these operations:

- **AND** — outputs `1` when both bits are `1`.
- **OR** — outputs `1` when at least one bit is `1`.
- **NAND** — the inverse of AND.
- **NOR** — the inverse of OR.
- **XOR** — outputs `1` when the bits differ.
- **XNOR** — outputs `1` when the bits match.

The two input sequences can have different lengths. They are aligned from the right, and the shorter one is padded with zeroes on the left before the operation is applied.

## Requirements

- [Rust](https://www.rust-lang.org/tools/install) and Cargo

## Run

Clone or download this repository, open a terminal in the project directory, then run:

```sh
cargo run
```

Follow the prompts to choose an operation, enter the length of each sequence, and provide each bit (`0` or `1`) one at a time. Operation names are entered in uppercase.

For example, choosing `XOR` with the sequences `101` and `1` right-aligns them as `101` and `001`, producing `100`.

## Run the tests

```sh
cargo test
```

The tests cover parsing and formatting bits and operation names, operation truth tables, sequences of different lengths, and empty-sequence errors.

## Project files

- [`Cargo.toml`](Cargo.toml) — package metadata and binary target.
- [`src/main.rs`](src/main.rs) — interactive command-line program and input/output.
- [`src/binary.rs`](src/binary.rs) — `Binary` and `Operation` types and sequence operations.
- [`src/lib.rs`](src/lib.rs) — library module entry point.
- [`tests/binary_test.rs`](tests/binary_test.rs) — integration tests.
- [Generated Rust API documentation](docs/binary_operations/index.html) — browse the library API, including the [`binary` module](docs/binary_operations/binary/index.html).

## License

No license has been specified yet.
