# bit-board: Bugs, Refactors, and Testing TODO

## Potential Bugs

- [x] **Zero-sized board causes underflow panic** (`bitboard.rs:21`)
  `index_of` computes `self.n_rows() - 1` which underflows if `n_rows` or `n_cols` is 0. Same issue in `set_cardinal_neighbors` (line 111), `set_diagonals` (lines 135, 140, 145). Neither constructor prevents `new(0, n)` or `new(n, 0)`.

- [x] **`set_col` / `set_row` skip bounds checking** (`bitboard.rs:73-95`)
  These methods compute `(r_idx * self.n_cols()) + col` directly instead of calling `self.index_of()`. If you call `set_col(col_out_of_bounds, true)`, it silently writes to wrong bit positions rather than panicking. For a 3x3 board, `set_col(5, true)` sets bits at indices 5, 8, 11 — corrupting unrelated cells.

- [x] **`row_col_of` has no validation** (`bitboard.rs:32-36`)
  No check that `index < n_rows * n_cols`. Returns a bogus (row, col) pair for out-of-range indices. Also panics with division-by-zero if `n_cols == 0`.

- [x] **Cross-type `or`/`and` panics despite matching dimensions**
  `BitBoardStatic<1>::or(&some_dyn_board)` is allowed by the `&impl BitBoard` signature, but will panic inside bitvec's `bitor_assign` because the backing slices have different lengths (64 bits for static W=1 vs exactly `n_rows*n_cols` bits for dyn).

- [x] **Public fields allow invariant violations**
  Both `BitBoardStatic` and `BitBoardDyn` expose `n_rows`, `n_cols`, and `board` as `pub`. A user can set `bb.n_rows = 1000` on a tiny board, breaking every method.

- [x] **Inconsistent OOB behavior: `get` returns false, `set` panics**
  `get(100, 100)` silently returns `false`; `set(100, 100, true)` panics. This is a footgun — it's easy to have silent bugs in read paths.

## Refactor Opportunities

- [ ] **Duplicated `Display` implementations**
  `BitBoardStatic::fmt` and `BitBoardDyn::fmt` are identical. Could be a blanket impl or a shared free function.

- [ ] **Duplicated test suites**
  The ~90 tests in `bitboardstatic.rs` and `bitboarddyn.rs` are nearly copy-paste identical. A macro like `bitboard_tests!(BitBoardDyn::new)` or a generic test function could eliminate the duplication.

- [ ] **`set_col`/`set_row` should use `self.index_of()`**
  They manually duplicate the index formula. Using `index_of` would get bounds checking for free and reduce the chance of formula drift.

- [ ] **`DimensionMismatch` carries no context**
  The error says "Dimensions do not match." but doesn't tell you what the actual vs expected dimensions were. Including them would make debugging easier.

- [ ] **Missing `not()` / `xor()` operations**
  These are natural bitwise operations for a bitboard library. `not()` in particular would enable De Morgan's law properties and complement-based algorithms.

## Proptest Opportunities

- [ ] **`index_of` / `row_col_of` roundtrip**
  For any valid (row, col), `row_col_of(index_of(row, col)) == (row, col)`.

- [ ] **`set`/`get` roundtrip and isolation**
  For any valid (r, c) and value: `set(r, c, v)` then `get(r, c) == v`, and no other cell is modified.

- [ ] **`or`/`and` algebraic laws**
  - Commutativity: `a.or(&b) == b.or(&a)`
  - Associativity: `a.or(&b).or(&c) == a.or(&b.or(&c))`
  - Identity: `a.or(&empty) == a`, `a.and(&full) == a`
  - Annihilation: `a.and(&empty) == empty`
  - Idempotence: `a.or(&a) == a`, `a.and(&a) == a`
  - Absorption: `a.or(&a.and(&b)) == a`

- [ ] **`fill` property**
  After `fill(true)`, every `get(r, c)` returns true. After `fill(false)`, every one returns false.

- [ ] **Neighbor count bounds**
  For any valid (r, c), the number of bits set by `set_cardinal_neighbors` is <= 4, and `set_all_neighbors` is <= 8. The center bit is never set.

## Kani Verification Opportunities

- [ ] **Index arithmetic never overflows**
  Prove that `row * n_cols + col` does not overflow for valid inputs within reasonable bounds, and that the result is always `< n_rows * n_cols`.

- [ ] **`row_col_of` always produces valid coordinates**
  Prove that for any `index < n_rows * n_cols`, the returned (row, col) satisfies `row < n_rows` and `col < n_cols`.

- [ ] **Neighbor operations never access out-of-bounds**
  Prove that `set_cardinal_neighbors` and `set_diagonals` never call `set()` with an out-of-bounds (row, col) — i.e., the boundary guards are correct and complete.

- [ ] **`or`/`and` dimension check is sufficient**
  Prove that if the dimension check passes, the underlying bitvec operations won't panic (specifically that slice lengths match for `BitBoardDyn`).
