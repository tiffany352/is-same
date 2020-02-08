# is-same

[![Crates.io](https://img.shields.io/crates/v/is-same)](https://crates.io/crates/is-same)
![License: MIT/Apache-2.0](https://img.shields.io/crates/l/is-same)
![build](https://github.com/tiffany352/is-same/workflows/build/badge.svg)
[![Coverage Status](https://img.shields.io/codecov/c/github/tiffany352/is-same)](https://codecov.io/gh/tiffany352/is-same)

is-same is a crate that exposes one trait: IsSame. The trait is similar
to PartialEq, but is designed for the usecase of diffing information
(such as in a tree structure). It has two important advantages:

- `NaN.is_same(NaN)` will return true, preventing your data from
  permanently being marked as different from its previous version.
- It takes advantage of referential equality for `Rc<T>`.

## Install

Add to your `Cargo.toml`:

```toml
is-same = "0.1"
is-same-derive = "0.1"
```

## Usage

```rust
use is_same::IsSame;
use is_same_derive::IsSame;

#[derive(IsSame)]
struct MyStruct {
    text: String,
    foo: usize,
    bar: char,
}

fn diff(left: &MyStruct, right: &MyStruct) {
    println!("is_same? {}", left.is_same(right));
}
```
