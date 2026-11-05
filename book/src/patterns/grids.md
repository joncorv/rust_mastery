# Grids

## Triggers

- The input is a **2D matrix**: a board, map, image, or maze.
- You move between **adjacent cells** (4 or 8 directions), or walk rows, columns, or diagonals.
- "Number of islands / regions", "shortest path in a maze", "rotate / transpose / spiral".

## Keywords

| matrix | board | cell | adjacent / neighbor | island / region |
|---|---|---|---|---|

## The big idea

A grid is a **graph in disguise**: every cell is a node, its in-bounds neighbors are the
edges. Once you can list neighbors safely, every graph algorithm (DFS, BFS, Dijkstra) just works.

## Rust toolkit

- `Vec<Vec<T>>` with `grid[r][c]`. Get `rows = grid.len()`, `cols = grid[0].len()`, but
  check that `grid` isn't empty first.
- A `const DIRS4: [(isize, isize); 4]` array plus `usize::checked_add_signed` for neighbors.
- Mark visited by **overwriting the cell** (if allowed), or use `vec![vec![false; cols]; rows]`.
- `HashSet<(usize, usize)>` for visited when the grid is huge or sparse.
- Strings to grid: `lines.iter().map(|r| r.chars().collect()).collect::<Vec<Vec<char>>>()`.

## Template: safe neighbors

```rust
{{#include ../../../src/dsa/_16_grids.rs:neighbors}}
```

## Template: flood fill

```rust
{{#include ../../../src/dsa/_16_grids.rs:num_islands}}
```

## Template: in-place matrix transform

```rust
{{#include ../../../src/dsa/_16_grids.rs:rotate}}
```

## Rust gotchas

- **`r - 1` when `r == 0` panics.** This is the #1 grid bug in Rust. Use `checked_sub` /
  `checked_add_signed`, or do the math in `i32` and check `>= 0` before casting back.
- **`grid[0].len()` on an empty grid panics.** `grid.first().map_or(0, |r| r.len())`.
- **Two `&mut` into the same `Vec<Vec<_>>`** (e.g. swapping `m[i][j]` and `m[j][i]`) won't
  compile. Use a temp copy, or `split_at_mut` on the outer Vec.
- **Don't recurse for flood fill on big grids.** A 1000×1000 all-land grid will overflow the
  stack. Use an explicit `Vec` stack or a `VecDeque`.

## Practice

- [ ] [Number of Islands (200)](https://leetcode.com/problems/number-of-islands/)
- [ ] [Flood Fill (733)](https://leetcode.com/problems/flood-fill/)
- [ ] [Rotate Image (48)](https://leetcode.com/problems/rotate-image/)
- [ ] [Spiral Matrix (54)](https://leetcode.com/problems/spiral-matrix/)
- [ ] [Set Matrix Zeroes (73)](https://leetcode.com/problems/set-matrix-zeroes/)
- [ ] [Rotting Oranges (994)](https://leetcode.com/problems/rotting-oranges/), multi-source BFS
- [ ] [Valid Sudoku (36)](https://leetcode.com/problems/valid-sudoku/)
