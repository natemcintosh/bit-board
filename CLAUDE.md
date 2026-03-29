# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project

bit-board is a Rust library wrapping `bitvec` to provide 2D bit array operations with row/column-based access. Published on crates.io as `bit-board`.

## Build & Test Commands

```bash
cargo build                        # build
cargo test                         # run all tests (~180 tests)
cargo test <test_name>             # run a single test by name
cargo test --lib bitboardstatic    # run tests in one module
cargo clippy                       # lint
cargo fmt                          # format
```

Uses `rstest` for parameterized tests. Edition 2024.

## Development Process

New features use red/green test-driven development: write a failing test first, then write the minimal code to make it pass.

## Architecture

The crate has three main pieces:

- **`BitBoard` trait** (`src/bitboard.rs`) — Defines all 2D operations (get/set cells, rows, columns, neighbors, bitwise OR/AND). Uses row-major linear storage: `index = row * n_cols + col`. Most methods have default implementations on the trait; implementors only need to provide `n_rows()`, `n_cols()`, `board()`, `board_mut()`, `or()`, and `and()`.

- **`BitBoardStatic<const W: usize>`** (`src/bitboardstatic.rs`) — Fixed-size implementation backed by `BitArray<[usize; W]>`. Stack-allocated, `Copy`. W is the number of `usize` words (each 64 bits on 64-bit systems). Panics at construction if the board doesn't fit.

- **`BitBoardDyn`** (`src/bitboarddyn.rs`) — Dynamic implementation backed by `BitVec`. Heap-allocated, no size limit beyond memory.

`DimensionMismatch` (`src/lib.rs`) is the only error type, used when `or()`/`and()` receive boards of different dimensions.

## Key Design Details

- Boundaries are hard (no wrapping).
- `get()` returns `false` for out-of-bounds coordinates; `set()` panics on out-of-bounds.
- Neighbor methods (`set_cardinal_neighbors`, `set_diagonals`, `set_all_neighbors`) silently skip neighbors that would be out of bounds.
- `or()`/`and()` take `&impl BitBoard`, so cross-type operations compile but may panic due to backing slice length mismatches between static and dynamic implementations.
