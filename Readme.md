# Strsplit

[![Rust](https://img.shields.io/badge/Rust-1.0%2B-blue.svg)](https://www.rust-lang.org)
[![Crates.io](https://img.shields.io/crates/v/strsplit.svg)](https://crates.io/crates/strsplit)
[![Docs.rs](https://docs.rs/strsplit/badge.svg)](https://docs.rs/strsplit)

`strsplit` is a crate that provides a `Strsplit` struct and a utility function `until_char` for splitting strings efficiently.

## Usage

Add this to your `Cargo.toml`:

```toml
[dependencies]
strsplit = "0.1.1"

```
Then you can use it in your code:

```rust
use strsplit::Strsplit;

let haystack = "a,b,c,d,e,f";
let letters: Vec<_> = Strsplit::new(&haystack, ",").collect();
assert_eq!(letters, vec!["a", "b", "c", "d", "e", "f"]);

```
#### Function: until_char

The until_char function returns the string before the first instance of the delimiter is found.

```rust
use strsplit::until_char;
let haystack = "hello";
let trimmed = until_char(&haystack, 'o');
assert_eq!(trimmed, "hell");

```