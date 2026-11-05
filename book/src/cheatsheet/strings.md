# String & char

- `String`: owned, growable, UTF-8. `&str`: a borrowed view into one (or a literal).
- Take `&str` in function signatures. A `&String` coerces to it automatically.
- **`len()` is bytes, not characters**, and **you can't index with `s[i]`**. For index-heavy
  problems, convert once to `as_bytes()` (ASCII) or `Vec<char>` (Unicode).

| Want | Use |
|---|---|
| iterate characters | `s.chars()` |
| iterate bytes | `s.bytes()` / `s.as_bytes()` |
| char at i, O(1) | `s.as_bytes()[i] as char` or `chars_vec[i]` |
| letter → 0..26 | `(c as u8 - b'a') as usize` |
| words | `s.split_whitespace()` |
| build up | `String::with_capacity(n)`, `push`, `push_str`, `format!` |
| reverse | `s.chars().rev().collect::<String>()` |

## Basics

```rust
{{#include ../../../src/ds/strings.rs:basics}}
```

## Indexing (the Rust way)

```rust
{{#include ../../../src/ds/strings.rs:indexing}}
```

## char helpers

```rust
{{#include ../../../src/ds/strings.rs:chars}}
```

## Split & parse

```rust
{{#include ../../../src/ds/strings.rs:split_and_parse}}
```

## Search & build

```rust
{{#include ../../../src/ds/strings.rs:search_and_build}}
```

## Positions, case & anagram keys

```rust
{{#include ../../../src/ds/strings.rs:char_indices}}
```
