# How to use this guide

My Rust reference for DSA interviews. The pattern breakdown follows *Beyond Cracking the
Coding Interview*.

- **Patterns**: one page per question category: when to reach for it, the template, and
  the Rust-specific traps.
- **Rust Cheat Sheet**: the methods I actually use, one page per data structure.

## The rule

Every code block in this book is pulled out of the `rust_mastery` crate with
`\{{#include file.rs:anchor}}`. That code is compiled and asserted by `cargo test`, so if
it's in the book, it works.

```sh
cargo test          # check the code
mdbook serve book   # live-reload the book at http://localhost:3000
```

To add code to a page, wrap it in the `.rs` file:

```rust,ignore
// ANCHOR: my_snippet
fn my_snippet() {}
// ANCHOR_END: my_snippet
```

Then put `\{{#include ../../../src/dsa/file.rs:my_snippet}}` inside a code block on a
page in `patterns/` or `cheatsheet/`. The path is relative to the `.md` file.

Each pattern page ends with a practice checklist. Solve those in `src/code_practice/`.
